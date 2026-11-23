# Rust 강의 8편 — 컬렉션과 이터레이터, 제로 코스트를 재 보다

> `filter`·`map` 으로 이은 이터레이터는 손으로 쓴 반복문보다 느릴까? 컬렉션은 무엇을 보고 고를까?

답부터 적습니다. 느리지 않습니다. 1억 개를 처리한 시간이 인덱스 반복문, `for-in`, 이터레이터 체인 모두 15.2ms 였습니다. 기계어도 세 함수가 같은 모양으로 펼쳐졌고, 인덱스 판의 범위 검사는 사라졌습니다. 이것이 Rust 가 말하는 제로 코스트 추상화입니다. 컬렉션은 기본으로 `Vec` 을 쓰고, 키로 찾으면 `HashMap`, 키 순서가 필요하면 `BTreeMap` 을 고릅니다. 키 100만 개를 찾는 데 `HashMap` 은 14ms, `BTreeMap` 은 77ms 였습니다.

> [그림] 세 갈래 길(인덱스 반복문, for-in, filter·map·fold 이터레이터)이 같은 결승선에 15.2ms 로 닿고 검수 로봇이 기록을 본다. 아래에 키 100만 개를 찾는 데 HashMap 14ms, BTreeMap 77ms 라고 적혀 있다

세 길의 기록이 같다는 것이 이 편의 첫 결론입니다. 아래 상자는 둘째 결론, 맵을 고르는 기준입니다.

Rust 강의 8편입니다. [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)에서 파서를 `Iterator` 로 만들었습니다. 이번에는 이터레이터의 비용을 재고 표준 컬렉션을 고르는 기준을 정합니다. 외부 크레이트는 쓰지 않습니다. 측정은 2026-11-18, Apple M4 Pro, rustc 1.100.0 릴리스 빌드입니다.

## 같은 일을 세 가지로 쓰면 시간이 같습니다

짝수만 골라 제곱해 더하는 함수를 세 모양으로 썼습니다. 인라인되어 사라지지 않게 `#[inline(never)]` 를 붙였습니다.

```rust
fn sum_index(v: &[u64]) -> u64 {
    let mut s = 0u64;
    for i in 0..v.len() {
        if v[i].is_multiple_of(2) {
            s = s.wrapping_add(v[i].wrapping_mul(v[i]));
        }
    }
    s
}

fn sum_for(v: &[u64]) -> u64 {
    let mut s = 0u64;
    for &x in v {
        if x.is_multiple_of(2) {
            s = s.wrapping_add(x.wrapping_mul(x));
        }
    }
    s
}

fn sum_iter(v: &[u64]) -> u64 {
    v.iter()
        .filter(|&&x| x.is_multiple_of(2))
        .map(|&x| x.wrapping_mul(x))
        .fold(0u64, |a, b| a.wrapping_add(b))
}
```

1억 개 `u64`(800MB)로 열한 번씩 돌린 중앙값입니다.

| 모양 | 중앙값 |
|---|---:|
| 인덱스 `for i in 0..v.len()` | 15.2ms |
| `for &x in v` | 15.2ms |
| `v.iter().filter().map().fold()` | 15.2ms |

추상화 단계가 셋인 체인이 반복문과 같습니다. `filter` 와 `map` 은 클로저를 받는 제네릭 함수라 컴파일할 때 구체 타입으로 펼쳐지고 인라인됩니다(16편의 모노모피제이션). The Book 13.4절도 같은 비교를 하고 "이터레이터는 제로 코스트 추상화"라고 적습니다.

인덱스 판에는 clippy 가 `needless_range_loop` 경고를 냅니다. "루프 변수 `i` 는 `v` 를 인덱싱하는 데만 쓰인다, 이터레이터를 써라"는 뜻입니다. 시간이 같다면 경계 실수가 없는 쪽을 고르라는 권고입니다.

## 기계어를 보면 범위 검사가 사라졌습니다

시간이 같은 이유를 기계어로 확인했습니다. `cargo rustc --release -- --emit asm` 으로 어셈블리를 뽑아 세 함수를 셌습니다.

| 함수 | 명령 수 | 곱셈(`mul`) | 함수 호출(`bl`) |
|---|---:|---:|---:|
| `sum_index` | 52 | 5 | 0 |
| `sum_for` | 57 | 5 | 0 |
| `sum_iter` | 52 | 5 | 0 |

세 함수의 곱셈 수와 분기 모양이 같았습니다. 길이를 4의 배수로 자르는 명령과 곱셈 다섯 개로 보아, 반복문을 네 번씩 펼치고 나머지를 따로 처리한 것으로 읽힙니다. 인덱스 판에 `bl` 이 0 이라는 것은 범위를 넘으면 부르는 패닉 함수 호출이 없다는 뜻입니다. `i` 가 `0..v.len()` 안에 있다는 것을 컴파일러가 증명하고 검사를 지웠습니다. 해석은 이렇습니다. 이 예시에서 이터레이터의 장점은 속도가 아니라 실수할 자리가 없다는 점입니다. 인덱스를 두 배열에 섞어 쓰는 코드라면 검사가 남을 수 있습니다. 그 경우는 이번에 재지 않았습니다.

## 이터레이터는 소비하기 전에는 아무 일도 하지 않습니다

```rust
    let tags = vec![66u16, 67, 68];
    tags.iter().map(|t| println!("tag {t}"));
```

이 코드는 아무것도 찍지 않습니다. 컴파일러가 경고합니다.

```console
warning: `Iterator::map` call that discard the iterator's values
 --> src/main.rs:3:17
  |
3 |     tags.iter().map(|t| println!("tag {t}"));
  |                 ^^^^-----------------------^
...
  = note: `Iterator::map`, like many of the methods on `Iterator`, gets executed lazily, meaning that its effects won't be visible until it is iterated
help: you might have meant to use `Iterator::for_each`
```

`map` 은 일을 예약만 하고, `for_each`·`sum`·`collect` 같은 소비 메서드가 불릴 때 돕니다. 이 게으름 덕분에 중간 `Vec` 없이 체인 전체가 원소 하나씩 흘러갑니다.

## iter, iter_mut, into_iter 는 빌리느냐 가져가느냐가 다릅니다

| 메서드 | 원소 타입 | 컬렉션은 |
|---|---|---|
| `v.iter()` / `for x in &v` | `&T` | 그대로 남는다 |
| `v.iter_mut()` / `for x in &mut v` | `&mut T` | 남고, 원소를 고칠 수 있다 |
| `v.into_iter()` / `for x in v` | `T` | 이동되어 사라진다 |

`for x in v` 는 `into_iter()` 로 바뀌므로 반복 뒤에는 `v` 를 못 씁니다.

```console
error[E0382]: borrow of moved value: `tags`
...
3 |     for t in tags {
  |              ---- `tags` moved due to this implicit call to `.into_iter()`
...
help: consider iterating over a slice of the `Vec<u16>`'s content to avoid moving into the `for` loop
  |
3 |     for t in &tags {
  |              +
```

2편의 이동 규칙이 반복문에도 그대로 적용됩니다. 원소를 다른 컬렉션으로 옮길 때는 `into_iter` 가 복사 없이 넘겨서 좋고, 계속 쓸 때는 `&v` 로 빌립니다.

## 용량을 미리 잡으면 재할당이 없습니다

`Vec` 에 `push` 를 100만 번 하면서 용량이 바뀐 횟수를 셌습니다. 19번이었습니다. 용량이 두 배씩 늘기 때문에 횟수는 로그로만 늘지만, 바뀔 때마다 버퍼를 새로 잡고 옮깁니다(3편). `Vec::with_capacity(1_000_000)` 으로 시작하면 0번입니다. 크기를 미리 알면 `with_capacity` 를 씁니다. `collect` 는 이터레이터가 정확한 크기를 알려 주면 한 번에 잡습니다. `(0..1_000_000).collect()` 의 용량은 정확히 1,000,000 이었고, `filter` 를 거친 50만 개는 크기를 몰라 524,288 까지 자랐습니다.

## 맵은 찾는 속도와 순서로 고릅니다

서로 다른 `u64` 키 100만 개를 뒤섞어 넣고 다시 찾았습니다(세 번 중 뒤의 두 번).

| 컬렉션 | 넣기 | 찾기 | 도는 순서 |
|---|---:|---:|---|
| `HashMap` (`with_capacity`) | 7ms | 14ms | 정해지지 않음 |
| `BTreeMap` | 78~79ms | 77ms | 키 순서 |

`HashMap` 이 5배 넘게 빨랐습니다. `BTreeMap` 은 키가 정렬된 채로 있어서 도는 순서가 늘 같고, 범위 조회(`range`)도 됩니다. 출력이 실행마다 같아야 하는 보고서, 스냅샷 테스트, 정렬된 목록에는 `BTreeMap` 이 맞습니다. 5편 연습 1이 `BTreeMap` 을 쓴 이유입니다. 키가 네 개뿐인 낱말 세기에서는 두 맵이 46ms 와 47ms 로 거의 같았습니다. 키가 적으면 고르는 기준은 속도가 아니라 순서입니다.

세기에는 `entry` API 를 씁니다. 키를 한 번만 찾고, 없으면 넣고, 있으면 고칩니다.

```rust
    *counts.entry(w).or_insert(0) += 1;
```

`if let Some(c) = counts.get_mut(w) { ... } else { counts.insert(w, 1); }` 처럼 쓰면 키를 두 번 찾습니다.

## 면접에서 이렇게 묻는다

**Q1. 제로 코스트 추상화란 무엇인가요?**
추상화를 써도 손으로 쓴 코드보다 실행 비용이 늘지 않는다는 원칙입니다. 이터레이터 체인은 제네릭이 펼쳐지고 인라인되어 반복문과 같은 기계어가 됩니다. 실측에서 셋 다 15.2ms 였고 명령 수도 52~57로 비슷했습니다.

**Q2. `iter`, `iter_mut`, `into_iter` 의 차이는?**
각각 `&T`, `&mut T`, `T` 를 내놓습니다. 앞의 둘은 빌리고 `into_iter` 는 컬렉션을 소비합니다. `for x in v` 는 `into_iter` 라 그 뒤에 `v` 를 쓰면 E0382 입니다.

**Q3. `HashMap` 과 `BTreeMap` 은 언제 고르나요?**
찾기만 빠르면 되면 `HashMap`, 키 순서나 범위 조회가 필요하면 `BTreeMap` 입니다. 키 100만 개에서 찾기가 14ms 대 77ms 였습니다. 출력 순서가 재현돼야 하면 `BTreeMap` 입니다.

**Q4. `entry` API 를 쓰는 이유는?**
키를 한 번만 찾고 "없으면 넣고 있으면 고치기"를 한 번에 합니다. 빌림 규칙과도 잘 맞습니다. `get_mut` 뒤 `insert` 는 키를 두 번 찾습니다.

**Q5. 이터레이터가 게으르다는 것은 무슨 뜻인가요?**
어댑터는 일을 예약만 하고 소비 메서드가 불릴 때 원소 하나씩 처리합니다. 중간 컬렉션이 생기지 않습니다. 소비하지 않은 `map` 은 컴파일러가 경고합니다.

## 연습

**연습 1.** 가장 많이 나온 낱말 n 개를 돌려주는 `top_words` 를 쓰세요. 횟수가 같으면 낱말 순입니다.

<details>
<summary>풀이 보기</summary>

```rust
pub fn top_words<'a>(text: &[&'a str], n: usize) -> Vec<(&'a str, usize)> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &w in text {
        *counts.entry(w).or_insert(0) += 1;
    }
    let mut pairs: Vec<_> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    pairs.truncate(n);
    pairs
}
```

`HashMap` 은 도는 순서가 정해지지 않으므로 정렬 기준에 동점 처리(`then`)까지 넣어야 결과가 늘 같습니다. 테스트에서 `[("표", 3), ("문단", 2)]` 를 돌려줬습니다.

</details>

**연습 2.** 레코드 태그 목록에서 태그마다 나온 위치를 모으세요. 태그 순서로 돌아야 합니다.

<details>
<summary>풀이 보기</summary>

```rust
pub fn positions_by_tag(tags: &[u16]) -> BTreeMap<u16, Vec<usize>> {
    let mut groups: BTreeMap<u16, Vec<usize>> = BTreeMap::new();
    for (i, &t) in tags.iter().enumerate() {
        groups.entry(t).or_default().push(i);
    }
    groups
}
```

`[67, 66, 67, 68, 67]` 에서 `67 → [0, 2, 4]`, 키 순서 `[66, 67, 68]` 이었습니다. `or_default()` 는 없으면 빈 `Vec` 을 넣습니다.

</details>

**연습 3.** 처음 나온 순서를 지키며 중복을 빼세요.

<details>
<summary>풀이 보기</summary>

```rust
pub fn dedup_keep_order(items: &[u16]) -> Vec<u16> {
    let mut seen = HashSet::new();
    items.iter().copied().filter(|x| seen.insert(*x)).collect()
}
```

`HashSet::insert` 는 새 값이면 `true` 를 돌려줍니다. `[67, 66, 67, 68, 66]` 이 `[67, 66, 68]` 이 됐습니다. `filter` 의 클로저가 바깥 `seen` 을 고치므로 이 클로저는 `FnMut` 입니다. 5편에서 rayon 이 거부한 바로 그 종류이고, 9편에서 다룹니다.

</details>

## 한 단계 더 — collect 는 받는 쪽 타입이 정합니다

`collect` 는 무엇을 만들지 스스로 모릅니다. 받는 쪽 타입을 보고 `FromIterator` 구현을 고릅니다.

```rust
    let unique: HashSet<u16> = tags.iter().copied().collect();
    let joined: String = ["가", "나", "다"].concat();
    let parsed: Result<Vec<u16>, _> = ["66", "x7"].iter().map(|s| s.parse::<u16>()).collect();
```

같은 꼴의 호출이 집합(크기 3), 문자열(`가나다`), `Result` 를 만들었습니다. 마지막 줄은 원소마다 `Result` 를 내는 이터레이터를 `Result<Vec<_>, _>` 로 모읍니다. 하나라도 실패하면 첫 에러에서 멈추고 그 에러를 돌려줍니다(`x7` 때문에 `Err`). 5편의 `records(&data).collect::<Result<Vec<_>, _>>()` 가 같은 구조입니다. 타입이 동작을 정하는 이 방식은 트레이트와 제네릭이 맞물린 결과이고, 16편에서 연관 타입과 함께 다시 봅니다.

## 정리

이터레이터 체인은 손으로 쓴 반복문과 같은 값을 치르고, 범위 실수를 할 자리를 없앱니다. 컬렉션은 `Vec` 을 기본으로, 키로 찾으면 `HashMap`, 순서가 필요하면 `BTreeMap` 입니다. 다음에 인덱스 반복문을 쓰려 할 때는 clippy 가 말하기 전에 이터레이터로 쓸 수 있는지 봅니다.

9편에서는 이터레이터에 넘긴 클로저를 다룹니다. `Fn`, `FnMut`, `FnOnce` 가 무엇을 약속하는지, `move` 는 언제 붙이는지 봅니다.
