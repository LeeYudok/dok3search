//! Character trie for common-prefix lookup of dictionary surfaces.
//!
//! Nodes are numbered breadth first, so the children of node `i` are the contiguous ids
//! `child_start[i]..child_start[i + 1]` with labels in ascending code point order, and the
//! dictionary entries of node `i` are `entry_start[i]..entry_start[i + 1]`. Node 0 is the
//! root. The three arrays are stored in the compiled dictionary as they are.
//!
//! In memory, `child_start` and `entry_start` are interleaved per node ([`Node`]), so one
//! step of a lookup reads both ranges from one place instead of two arrays of 1.2 million
//! entries. Every lookup starts at the root, which has thousands of children (8,229 first
//! characters in mecab-ko-dic), so the root's children are also indexed directly by code
//! point ([`RootIndex`], derived when the trie is built or loaded). Deeper nodes have few
//! children and are binary searched.

use super::error::{Error, Result};

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct Trie {
    pub labels: Vec<u32>,
    /// One per node plus an end marker: node `i` spans `nodes[i]..nodes[i + 1]`.
    nodes: Vec<Node>,
    root: RootIndex,
}

/// Start of a node's children and of its dictionary entries.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Node {
    child_start: u32,
    entry_start: u32,
}

/// Root child node per code point in `base..base + nodes.len()` (0 = no child; node 0 is
/// the root and never a child). Empty when the root's labels span more than
/// [`RootIndex::MAX_SPAN`] code points; lookups then binary search the root too.
#[derive(Debug, Default, PartialEq, Eq)]
struct RootIndex {
    base: u32,
    nodes: Vec<u32>,
}

impl RootIndex {
    /// 128 Ki code points (512 KiB). mecab-ko-dic spans U+0021..U+FA0B.
    const MAX_SPAN: u32 = 1 << 17;

    /// Indexes the root children `labels[lo..hi]` (nodes `lo..hi`). Arrays that do not
    /// describe a valid root give an empty index; [`Trie::validate`] rejects them.
    fn new(labels: &[u32], nodes: &[Node]) -> RootIndex {
        let (Some(lo), Some(hi)) = (
            nodes.first().map(|n| n.child_start),
            nodes.get(1).map(|n| n.child_start),
        ) else {
            return RootIndex::default();
        };
        let Some(children) = labels.get(lo as usize..hi as usize) else {
            return RootIndex::default();
        };
        let (Some(&first), Some(&last)) = (children.first(), children.last()) else {
            return RootIndex::default();
        };
        if last < first || last - first >= Self::MAX_SPAN || lo == 0 {
            return RootIndex::default();
        }
        let mut nodes = vec![0u32; (last - first + 1) as usize];
        for (k, &label) in children.iter().enumerate() {
            // Unsorted labels (a damaged file) can fall outside first..=last; validation
            // rejects them before any lookup.
            if let Some(slot) = nodes.get_mut(label.wrapping_sub(first) as usize) {
                *slot = lo + k as u32;
            }
        }
        RootIndex { base: first, nodes }
    }
}

/// Unique keys in ascending order, flattened: key `k` is `chars[offsets[k]..offsets[k + 1]]`.
pub(crate) struct Keys {
    pub chars: Vec<u32>,
    pub offsets: Vec<u32>,
}

impl Keys {
    pub fn new() -> Self {
        Keys {
            chars: Vec::new(),
            offsets: vec![0],
        }
    }

    pub fn push(&mut self, key: &str) {
        self.chars.extend(key.chars().map(|c| c as u32));
        self.offsets.push(self.chars.len() as u32);
    }

    pub fn len(&self) -> usize {
        self.offsets.len() - 1
    }

    fn key(&self, k: usize) -> &[u32] {
        &self.chars[self.offsets[k] as usize..self.offsets[k + 1] as usize]
    }
}

impl Trie {
    /// Builds a trie over sorted unique non-empty `keys`, where key `k` owns
    /// `entry_counts[k]` entries. Returns the trie and the key order in which the caller
    /// must lay out the entries (breadth-first node order).
    pub fn build(keys: &Keys, entry_counts: &[u32]) -> (Trie, Vec<u32>) {
        debug_assert_eq!(keys.len(), entry_counts.len());
        let mut labels = vec![0u32];
        let mut nodes = Vec::new();
        let mut key_order = Vec::with_capacity(keys.len());
        // (first key, end key, depth) per node, in node id order.
        let mut queue: Vec<(u32, u32, u32)> = vec![(0, keys.len() as u32, 0)];
        let mut entries = 0u32;
        let mut node = 0;
        while node < queue.len() {
            let (mut lo, hi, depth) = queue[node];
            nodes.push(Node {
                child_start: queue.len() as u32,
                entry_start: entries,
            });
            // Keys share the node's prefix; the one equal to it sorts first.
            if lo < hi && keys.key(lo as usize).len() == depth as usize {
                key_order.push(lo);
                entries += entry_counts[lo as usize];
                lo += 1;
            }
            while lo < hi {
                let label = keys.key(lo as usize)[depth as usize];
                let mut end = lo + 1;
                while end < hi && keys.key(end as usize)[depth as usize] == label {
                    end += 1;
                }
                queue.push((lo, end, depth + 1));
                labels.push(label);
                lo = end;
            }
            node += 1;
        }
        nodes.push(Node {
            child_start: queue.len() as u32,
            entry_start: entries,
        });
        (Trie::from_nodes(labels, nodes), key_order)
    }

    /// Takes the stored arrays and derives the in-memory layout. Call [`Trie::validate`]
    /// before lookups when the arrays come from a file.
    pub fn from_arrays(
        labels: Vec<u32>,
        child_start: Vec<u32>,
        entry_start: Vec<u32>,
    ) -> Result<Trie> {
        if child_start.len() != entry_start.len() {
            return Err(Error::Format("trie: array lengths do not match".into()));
        }
        let nodes = child_start.into_iter().zip(entry_start).map(|(c, e)| Node {
            child_start: c,
            entry_start: e,
        });
        Ok(Trie::from_nodes(labels, nodes.collect()))
    }

    fn from_nodes(labels: Vec<u32>, nodes: Vec<Node>) -> Trie {
        let root = RootIndex::new(&labels, &nodes);
        Trie {
            labels,
            nodes,
            root,
        }
    }

    /// The stored `child_start` array.
    pub fn child_starts(&self) -> Vec<u32> {
        self.nodes.iter().map(|n| n.child_start).collect()
    }

    /// The stored `entry_start` array.
    pub fn entry_starts(&self) -> Vec<u32> {
        self.nodes.iter().map(|n| n.entry_start).collect()
    }

    pub fn node_count(&self) -> usize {
        self.labels.len()
    }

    /// Nodes that carry dictionary entries (distinct surfaces).
    pub fn surface_count(&self) -> usize {
        self.nodes
            .windows(2)
            .filter(|w| w[0].entry_start < w[1].entry_start)
            .count()
    }

    /// Checks the invariants lookups rely on, so a corrupt file cannot cause a panic.
    pub fn validate(&self, entry_count: usize) -> Result<()> {
        let n = self.labels.len();
        let bad = |what: &str| Err(Error::Format(format!("trie: {what}")));
        if n == 0 || self.nodes.len() != n + 1 {
            return bad("array lengths do not match");
        }
        if self.nodes[n].child_start as usize != n
            || self.nodes[n].entry_start as usize != entry_count
        {
            return bad("end offsets do not match");
        }
        for i in 0..n {
            let (lo, hi) = (self.nodes[i].child_start, self.nodes[i + 1].child_start);
            if lo > hi || hi as usize > n || (lo as usize) <= i && lo < hi {
                return bad("children out of order");
            }
            if self.nodes[i].entry_start > self.nodes[i + 1].entry_start {
                return bad("entries out of order");
            }
            if !self.labels[lo as usize..hi as usize]
                .windows(2)
                .all(|w| w[0] < w[1])
            {
                return bad("labels not sorted");
            }
        }
        Ok(())
    }

    /// Calls `f(length_in_chars, entry_start, entry_end)` for every key that is a prefix of
    /// `chars`, shortest first.
    #[inline]
    pub fn prefixes(&self, chars: &[char], mut f: impl FnMut(usize, u32, u32)) {
        let Some((&first, rest)) = chars.split_first() else {
            return;
        };
        let Some(mut node) = self.root_child(first as u32) else {
            return;
        };
        let mut depth = 1;
        loop {
            let (start, end) = (
                self.nodes[node].entry_start,
                self.nodes[node + 1].entry_start,
            );
            if start < end {
                f(depth, start, end);
            }
            let Some(&c) = rest.get(depth - 1) else {
                return;
            };
            match self.child(node, c as u32) {
                Some(next) => node = next,
                None => return,
            }
            depth += 1;
        }
    }

    #[inline]
    fn root_child(&self, label: u32) -> Option<usize> {
        if self.root.nodes.is_empty() {
            return self.child(0, label);
        }
        let node = *self
            .root
            .nodes
            .get(label.wrapping_sub(self.root.base) as usize)?;
        (node != 0).then_some(node as usize)
    }

    #[inline]
    fn child(&self, node: usize, label: u32) -> Option<usize> {
        let lo = self.nodes[node].child_start as usize;
        let hi = self.nodes[node + 1].child_start as usize;
        self.labels[lo..hi]
            .binary_search(&label)
            .ok()
            .map(|k| lo + k)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_all_prefixes() {
        let mut keys = Keys::new();
        let words = ["가", "가나", "가나다", "나", "다라"];
        for w in words {
            keys.push(w);
        }
        let counts = [1, 2, 1, 1, 1];
        let (trie, order) = Trie::build(&keys, &counts);
        trie.validate(6).unwrap();
        // Breadth first: depth-1 keys (가, 나) before 가나, then 가나다.
        assert_eq!(order, [0, 3, 1, 4, 2]);
        let text: Vec<char> = "가나다라".chars().collect();
        let mut got = Vec::new();
        trie.prefixes(&text, |len, s, e| got.push((len, e - s)));
        assert_eq!(got, [(1, 1), (2, 2), (3, 1)]);
        let mut none = Vec::new();
        trie.prefixes(&['라'], |len, _, _| none.push(len));
        assert!(none.is_empty());
        trie.prefixes(&['a', '가'], |len, _, _| none.push(len));
        trie.prefixes(&[], |len, _, _| none.push(len));
        assert!(none.is_empty());
    }

    #[test]
    fn wide_root_falls_back_to_binary_search() {
        let mut keys = Keys::new();
        let words = ["a", "ab", "가", "\u{30000}", "\u{30000}b"];
        for w in words {
            keys.push(w);
        }
        let (trie, _) = Trie::build(&keys, &[1, 1, 1, 1, 1]);
        assert!(
            trie.root.nodes.is_empty(),
            "U+0061..U+30000 is wider than the index"
        );
        let lengths = |text: &str| {
            let chars: Vec<char> = text.chars().collect();
            let mut got = Vec::new();
            trie.prefixes(&chars, |len, _, _| got.push(len));
            got
        };
        assert_eq!(lengths("abc"), [1, 2]);
        assert_eq!(lengths("\u{30000}b"), [1, 2]);
        assert_eq!(lengths("가나"), [1]);
        assert!(lengths("b").is_empty());

        let mut narrow = Keys::new();
        for w in ["a", "ab", "가"] {
            narrow.push(w);
        }
        let (narrow, _) = Trie::build(&narrow, &[1, 1, 1]);
        assert_eq!(narrow.root.base, 'a' as u32);
        assert_eq!(
            narrow.root.nodes.len(),
            ('가' as u32 - 'a' as u32 + 1) as usize
        );
    }
}
