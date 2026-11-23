# Rust 강의 12편 — clippy 와 코드 리뷰 관용구

> 코드 리뷰에서 자주 걸리는 Rust 관용구는 무엇이고, 어디까지 clippy 에 맡길 수 있을까?

답부터 적습니다. 인자 타입(`&Vec` 대신 `&[T]`), 손으로 쓴 반복문과 조건, 불필요한 복사 같은 모양은 clippy 가 거의 다 잡습니다. 리뷰 예제에서 기본 경고 9개 가운데 6개는 `cargo clippy --fix` 가 자동으로 고쳤고, 시그니처와 반복문을 바꾸는 3개는 사람이 고쳤습니다. clippy 가 못 보는 것은 설계입니다. 라이브러리 공개 함수의 `unwrap`, 원인을 문자열로 뭉갠 에러, 필요 이상으로 넓은 `pub` 은 리뷰어가 봐야 합니다.

> [그림] 왼쪽 before.rs 종이에 빨간 표시 아홉 개가 있고 기본 경고 9개, pedantic 을 켜면 14개다. 종이가 cargo clippy --fix 기계를 지나 자동으로 고침 6 과 사람이 고칠 것 3(인자 타입 2, 반복문 1)으로 나뉜다. 아래 카드에 clippy 가 못 보는 것이 적혀 있고 검수 로봇이 그 카드를 본다

숫자는 이 글의 리뷰 예제를 clippy 1.100.0 으로 돌린 결과입니다.

Rust 강의 12편이고 실무 단계의 마지막 편입니다. 이 연재의 예제를 쓰는 동안 clippy 가 실제로 경고한 모양들을 소재로 씁니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 리뷰에서 걸리는 모양을 한 파일에 모았습니다

컴파일도 되고 결과도 맞지만 리뷰에서 걸릴 코드입니다.

```rust
fn count_tags(tags: &Vec<u16>) -> HashMap<u16, usize> {
    let mut counts = HashMap::new();
    for i in 0..tags.len() {
        let t = tags[i];
        if counts.contains_key(&t) {
            *counts.get_mut(&t).unwrap() += 1;
        } else {
            counts.insert(t, 1);
        }
    }
    return counts;
}

fn file_label(path: &PathBuf, title: &String) -> String {
    let name = path.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or(String::new());
    if title.len() == 0 {
        return name.clone();
    }
    format!("{} ({})", title.to_string(), name)
}

fn total_size(sizes: Vec<u64>) -> u64 {
    let mut total = 0;
    for s in sizes.iter() {
        total = total + *s;
    }
    total
}
```

`cargo clippy` 의 기본 경고는 9개였습니다.

| clippy 규칙 | 지적 | 고친 모양 |
|---|---|---|
| `ptr_arg` (2건) | `&Vec`·`&PathBuf` 인자 | `&[u16]`, `&Path` |
| `needless_range_loop` | 인덱스로만 쓰는 반복 변수 | `for &t in tags` |
| `map_entry` | `contains_key` 뒤 `insert` | `entry(t).or_insert(0)` |
| `needless_return` | 마지막 줄의 `return` | 식으로 끝낸다 |
| `unwrap_or_default` | `unwrap_or(String::new())` | `unwrap_or_default()` |
| `len_zero` | `len() == 0` | `is_empty()` |
| `to_string_in_format_args` | `format!` 안의 `to_string()` | `{title}` |
| `assign_op_pattern` | `total = total + *s` | `+=`, 더 나아가 `sum()` |

이 가운데 `ptr_arg` 와 `map_entry` 는 이 연재에서도 실제로 걸렸습니다. 7편에서 처음 쓴 `fn count(path: &PathBuf)` 가 `ptr_arg` 경고를 받았습니다. 8편은 키를 두 번 찾지 않으려고 `entry` 를 썼습니다.

`&Vec<T>` 를 받지 않는 이유는 2편의 `String` 과 `&str` 과 같습니다. `&[T]` 를 받으면 `Vec`, 배열, 다른 슬라이스를 모두 받습니다. `&Vec<T>` 는 `Vec` 만 받으면서 하는 일은 같습니다.

## --fix 는 6개를 고쳤고, 시그니처는 사람이 고쳤습니다

같은 파일 사본에 `cargo clippy --fix` 를 돌렸습니다. 남은 경고는 3개였습니다.

```console
warning: writing `&Vec` instead of `&[_]` involves a new object where a slice will do
warning: the loop variable `i` is only used to index `tags`
warning: writing `&PathBuf` instead of `&Path` involves a new object where a slice will do
```

자동으로 고친 6개는 함수 안에서 끝나는 변경입니다. 남은 3개는 함수 시그니처를 바꾸거나 반복문 구조를 다시 짜는 변경입니다. 시그니처는 호출하는 쪽까지 영향을 주므로 clippy 가 자동으로 고치지 않습니다. 에이전트에게 맡길 때도 순서가 같습니다. `--fix` 로 기계적인 것을 먼저 치우고, 남은 경고를 사람이나 에이전트가 하나씩 봅니다.

사람이 고친 판은 이렇습니다.

```rust
fn count_tags(tags: &[u16]) -> HashMap<u16, usize> {
    let mut counts = HashMap::new();
    for &t in tags {
        *counts.entry(t).or_insert(0) += 1;
    }
    counts
}

fn file_label(path: &Path, title: &str) -> String {
    let name = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    if title.is_empty() {
        return name;
    }
    format!("{title} ({name})")
}

fn total_size(sizes: &[u64]) -> u64 {
    sizes.iter().sum()
}
```

두 판의 출력에서 `report.hwp 2 6` 은 같았습니다. `HashMap` 출력 순서만 실행마다 달랐습니다(8편).

## lint 그룹은 강도가 다릅니다

clippy 규칙은 그룹으로 나뉩니다. 기본으로 켜진 것은 `style`, `correctness`, `complexity`, `perf`, `suspicious` 입니다. 같은 파일에 그룹을 더해 봤습니다.

| 켠 그룹 | 고치기 전 | 고친 뒤 | 더 잡은 것 |
|---|---:|---:|---|
| 기본 | 9 | 0 | — |
| `-W clippy::pedantic` | 14 | 1 | 값으로 받은 `Vec`(`needless_pass_by_value`), `for s in sizes.iter()` 등 |
| `-W clippy::nursery` | 기본 9 + 2종 | — | `redundant_clone`, `needless_collect` |

`pedantic` 은 깐깐합니다. 고친 판에도 `map(..).unwrap_or_default()` 를 다른 모양으로 쓰라는 경고를 하나 냈습니다. 취향에 가까운 규칙도 섞여 있다는 뜻입니다. 그래서 그룹을 통째로 켜기보다 쓸모 있는 규칙만 골라 켭니다. `nursery` 는 아직 다듬는 중인 규칙입니다. 그래도 `name.clone()` 의 불필요한 복제(`redundant_clone`)는 이 그룹만 잡았습니다.

## 규칙은 [lints] 표와 #[expect] 로 저장소에 박습니다

규칙을 명령줄에 적으면 사람마다 다르게 돌립니다. `Cargo.toml` 의 `[lints]` 표에 적으면 그 패키지를 빌드하는 모두에게 같이 걸립니다.

```toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
unwrap_used = "warn"
```

`unwrap_used` 는 기본으로 꺼진 `restriction` 그룹 규칙입니다. 라이브러리에서 켜면 공개 함수의 `unwrap` 을 찾아 줍니다.

```console
warning: used `unwrap()` on a `Result` value
 --> src/lib.rs:3:5
  |
3 |     s.trim().parse().unwrap()
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: if this value is an `Err`, it will panic
...
  = note: requested on the command line with `-W clippy::unwrap-used`
```

마지막 줄을 보면 cargo 가 `[lints]` 표를 명령줄 옵션으로 바꿔 넘깁니다. 꼭 필요한 예외는 `#[expect]` 로 이유와 함께 적습니다.

```rust
#[expect(clippy::unwrap_used, reason = "리터럴 \"8080\" 은 늘 u16 으로 읽힌다")]
pub fn default_port() -> u16 {
    "8080".parse().unwrap()
}
```

`#[allow]` 와 다른 점은 경고가 사라진 뒤에도 남아 있으면 알려 준다는 것입니다. `unwrap` 을 지운 함수에 `#[expect]` 를 남겨 두니 이렇게 나왔습니다.

```console
warning: this lint expectation is unfulfilled
  --> src/lib.rs:13:10
   |
13 | #[expect(clippy::unwrap_used, reason = "예전에 unwrap 을 썼다")]
   |          ^^^^^^^^^^^^^^^^^^^
   |
   = note: 예전에 unwrap 을 썼다
```

예외 표시가 코드보다 오래 사는 일을 막아 줍니다.

## clippy 가 못 보는 것은 설계입니다

| 리뷰 항목 | clippy 가 잡나 | 이 연재에서 본 곳 |
|---|---|---|
| 인자 타입, 반복문, 불필요한 복사 | 잡는다 | 이 편 |
| 라이브러리 공개 함수의 `unwrap` | `unwrap_used` 를 켜면 위치만 잡는다. 써도 되는지는 사람이 판단 | 7편 |
| 원인을 `to_string()` 으로 뭉갠 에러 | 잡지 않는다 | 7편 |
| 필요 이상으로 강한 클로저 요구(`Fn` 대신 `FnMut` 이면 될 때) | 잡지 않는다. 호출하는 쪽에서 E0594 로 드러난다 | 9편 |
| 실행 파일 의존성이 라이브러리로 새는 구조 | 잡지 않는다 | 11편 |
| `for` 를 `while` 로 바꾸며 `continue` 앞의 증가를 빠뜨림 | 잡지 않는다. 문서 변환 엔진에서 실제로 무한 반복이 났다 | 6편 |

해석하면 clippy 는 "이 줄을 더 낫게 쓰는 법"을 압니다. "이 함수가 이런 약속을 해도 되는가"는 모릅니다. 그래서 리뷰어의 시간은 표의 아래쪽에 씁니다.

## 면접에서 이렇게 묻는다

**Q1. 함수 인자로 `&String` 이나 `&Vec<T>` 를 받으면 왜 지적받나요?**
`&str` 과 `&[T]` 가 더 많은 입력(리터럴, 배열, 다른 슬라이스)을 받으면서 하는 일은 같습니다. 역참조 강제 변환 덕분에 `&String` 을 넘겨도 `&str` 인자로 들어갑니다. clippy `ptr_arg` 가 잡습니다.

**Q2. clippy 경고를 끄는 올바른 방법은?**
규칙이 틀렸다면 `#[expect(lint, reason = "...")]` 로 이유를 적습니다. 경고가 사라지면 `unfulfilled_lint_expectations` 로 알려 줘 낡은 예외가 남지 않습니다. 저장소 전체 규칙은 `Cargo.toml` 의 `[lints]` 에 둡니다.

**Q3. pedantic 그룹을 통째로 켜야 하나요?**
보통은 아닙니다. 실측에서 고친 코드에도 취향에 가까운 경고가 남았습니다. 쓸모 있는 규칙을 골라 `[lints]` 에 켭니다. `restriction` 그룹은 저장소 정책(예: `unwrap_used`)에 맞는 것만 켭니다.

**Q4. 불필요한 `clone` 은 어떻게 찾나요?**
기본 그룹이 일부(`clone_on_copy` 등)를 잡고, `redundant_clone` 은 `nursery` 에 있습니다. 실측에서 `name.clone()` 은 nursery 만 잡았습니다. 리뷰에서는 E0382 를 `clone` 으로 덮은 곳을 먼저 봅니다(2편).

**Q5. clippy 로 대신할 수 없는 리뷰는 무엇인가요?**
공개 API 의 약속, 에러에 원인을 남기는지, 의존성 구조, 동시성 설계입니다. clippy 는 줄 단위의 관용구를 봅니다.

## 연습

**연습 1.** `total_size(sizes: Vec<u64>)` 를 기본 clippy 는 왜 지적하지 않았을까요? 어떤 그룹이 잡았나요?

<details>
<summary>풀이 보기</summary>

값으로 받는 것은 틀린 코드가 아니라 비용 선택이라 기본 그룹에는 없습니다. `pedantic` 의 `needless_pass_by_value` 가 "소유할 필요가 없는데 값으로 받는다"고 잡았습니다. 고친 판은 `&[u64]` 를 받고 `sizes.iter().sum()` 으로 끝냅니다. 호출하는 쪽이 `vec![...]` 를 만들 필요도 사라집니다.

</details>

**연습 2.** 라이브러리 패키지에 "unsafe 금지, 공개 함수의 unwrap 경고"를 저장소 규칙으로 거세요.

<details>
<summary>풀이 보기</summary>

```toml
[lints.rust]
unsafe_code = "forbid"

[lints.clippy]
unwrap_used = "warn"
```

`cargo clippy` 를 돌리면 `parse_port` 의 `unwrap` 에 경고가 납니다. 워크스페이스라면 루트에 `[workspace.lints]` 를 두고 구성원에서 `lints.workspace = true` 로 물려받습니다.

</details>

**연습 3.** 에이전트에게 "clippy 경고를 없애라"고만 시키면 생길 수 있는 나쁜 결과를 하나 들고, 지시를 고치세요.

<details>
<summary>풀이 보기</summary>

경고를 고치지 않고 `#[allow(...)]` 를 붙여 덮을 수 있습니다. 지시는 "`cargo clippy --fix` 를 먼저 돌리고, 남은 경고는 코드를 고쳐라. 예외가 꼭 필요하면 `#[expect(..., reason = ...)]` 로 이유를 적어라. `#[allow]` 는 쓰지 마라"처럼 씁니다. `[lints.clippy] allow_attributes = "warn"` 을 켜면 `#[allow]` 자체를 경고하게 할 수도 있습니다.

</details>

## 한 단계 더 — 리뷰 비용을 clippy 쪽으로 옮기는 순서

리뷰 비용을 줄이는 순서는 세 단계로 정리됩니다. 첫째, CI 에서 `cargo clippy --all-targets -- -D warnings` 로 기본 그룹 경고를 실패로 막습니다. 6편에서 이 검사는 고친 뒤 0.15초였습니다. 둘째, 저장소 정책을 `[lints]` 에 적습니다. 셋째, 리뷰에서 되풀이되는 지적이 생기면 그것을 잡는 규칙이 있는지 찾아 켭니다.

clippy 규칙은 버전마다 늘고 바뀝니다. 5편과 8편에서 `as_chunks`·`is_multiple_of` 를 권한 경고는 1.100.0 에서 만났습니다. 툴체인을 올리면 경고가 새로 생길 수 있으니, 1편의 `rust-toolchain.toml` 로 버전을 고정하고 올릴 때 한꺼번에 정리합니다. 이 순서는 이 연재 예제에서의 경험이고, 팀 규모에 따라 단계별 비용은 다를 수 있습니다.

## 정리

줄 단위 관용구는 clippy 에 맡기고, `--fix` 가 못 고친 시그니처와 반복문은 사람이 고칩니다. 저장소 규칙은 `[lints]` 와 `#[expect]` 로 코드에 남깁니다. 리뷰어의 시간은 clippy 가 못 보는 약속과 구조에 씁니다.

13편부터는 심화 단계입니다. 3편에서 맛본 빌림 검사기와 수명을 NLL, 수명 생략 규칙, `'static` 의 두 뜻까지 깊게 다룹니다.
