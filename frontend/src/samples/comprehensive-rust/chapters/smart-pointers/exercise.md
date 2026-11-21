# 연습문제: 바이너리 트리

바이너리 트리는 모든 노드에 두 개의 하위 요소(왼쪽과 오른쪽)가 있는 트리 유형 데이터 구조입니다. 각 노드가 값을 저장하는 트리를 만들겠습니다. 주어진 노드 N의 경우 N의 왼쪽 하위 트리에 있는 모든 노드는 더 작은 값을 포함하고, N의 오른쪽 하위 트리에 있는 모든 노드는 더 큰 값을 포함합니다.

지정된 테스트가 통과하도록 다음 타입을 구현합니다.

추가 크레딧: 값을 순서대로 반환하는 바이너리 트리에 대한 반복자를 구현합니다.

``` rust
/// A node in the binary tree.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// A possibly-empty subtree.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// 바이너리 트리를 사용하여 값 집합을 저장하는 컨테이너입니다.
///
/// 동일한 값이 여러 번 추가되면 한 번만 저장됩니다.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

// Implement `new`, `insert`, `len`, and `has`.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // 고유 항목이 아닙니다.
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();
        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}
```
