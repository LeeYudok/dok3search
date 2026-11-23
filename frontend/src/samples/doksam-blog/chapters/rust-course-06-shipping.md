# Rust 강의 6편 — 밖으로 내보내기와 운영

> 다 만든 Rust 도구를 내보낼 때는 무엇을 정하고, 매일 고칠 때는 무엇을 돌려야 할까?

답부터 적습니다. 내보낼 때는 릴리스 프로필로 크기와 속도와 빌드 시간을 맞바꿉니다. 이 글의 도구는 `strip` 으로 22%, LTO 까지 39%, `opt-level = "z"` 까지 59% 작아졌습니다. 대신 LTO 를 켜자 고친 뒤 재빌드가 0.33초에서 3.37초로 늘었고, 실행은 오히려 느려졌습니다. 다른 언어에는 `cdylib` 위에 pyo3·napi 를 얹어 같은 코어를 붙입니다. 매일은 fmt → clippy → `test --lib` 를 돌리고(0.77초), 릴리스 빌드는 내보낼 때만 합니다.

> [그림] 왼쪽에 크기가 줄어드는 실행 파일 상자 넷(release 1.14MB, strip 0.89MB, LTO 0.70MB, z 0.47MB)이 있고 검수 로봇이 가장 작은 상자를 본다. 오른쪽 Rust 코어에서 Python pyo3, Node napi, C ABI ctypes 로 전선이 이어진다. 아래 띠에 fmt, clippy, test --lib 와 한 바퀴 0.77초가 적혀 있다

그림은 이 글의 세 부분입니다. 작게 묶기, 다른 언어에 꽂기, 짧게 확인하기입니다.

Rust 강의 6편이고 입문 단계의 마지막 편입니다. [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)의 `recscan` 을 그대로 씁니다. 측정은 2026-11-18, Apple M4 Pro 14코어, rustc 1.100.0 에서 했습니다.

## 릴리스 프로필은 크기·속도·빌드 시간을 맞바꿉니다

`Cargo.toml` 에 프로필을 더해 같은 코드를 다섯 가지로 빌드했습니다. `inherits` 로 기본 릴리스 설정을 물려받고 다른 값만 적습니다.

```toml
[profile.release-strip]
inherits = "release"
strip = true

# 문서 변환 엔진과 같은 설정
[profile.release-lto]
inherits = "release"
lto = true
strip = true

[profile.release-thin]
inherits = "release"
lto = "thin"
strip = true

[profile.release-small]
inherits = "release"
opt-level = "z"
lto = true
strip = true
codegen-units = 1
panic = "abort"
```

| 프로필 | 실행 파일 | 빈 폴더 빌드 | 고친 뒤 재빌드 | 직렬 실행 |
|---|---:|---:|---:|---:|
| `release` (기본) | 1,141,264 B | 4.46초 | 0.33초 | 0.200초 |
| `release-strip` | 886,752 B | 4.42초 | 0.33초 | 0.203초 |
| `release-thin` | 831,248 B | 6.09초 | 1.49초 | 0.197초 |
| `release-lto` | 701,120 B | 7.54초 | 3.37초 | 0.237초 |
| `release-small` | 470,320 B | 5.45초 | 2.17초 | 0.288초 |

재빌드는 `src/main.rs` 를 건드리고 세 번 잰 중앙값입니다. 실행은 파일 32개(256MB)를 `count --serial` 로 일곱 번 센 중앙값입니다.

해석은 이렇습니다. `strip` 은 빌드 시간과 속도를 그대로 두고 22%를 줄였으니 거의 공짜입니다. 기본 릴리스도 디버그 정보는 빼지만(1편), `strip = true` 는 심볼까지 뺍니다. LTO(link-time optimization, 링크할 때 크레이트 경계를 넘어 하는 최적화)는 크기를 더 줄였습니다. 대신 재빌드가 열 배로 늘었고, 이 도구에서는 실행이 약 19% 느려졌습니다. `opt-level = "z"` 는 가장 작지만 가장 느립니다. LTO 가 늘 빠르게 만든다는 믿음은 이 도구에서 맞지 않았습니다. 크레이트 경계를 넘는 작은 함수 호출이 뜨거운 반복문에 많다면 결과가 달라질 수 있습니다. 그래서 프로필은 재 보고 고릅니다.

문서 변환 엔진은 `opt-level = 3`, `lto = true`, `strip = true` 를 씁니다. 실행 파일 6.8MB 하나로 배포하는 대가로 빈 폴더 빌드 86초, 한 파일 고친 뒤 재빌드 40.8초를 냅니다.

## 빌드 시간은 무엇을 다시 컴파일하느냐로 정해집니다

`recscan` 은 디버그 빌드에서 크레이트 26개를 컴파일합니다. 문서 변환 엔진은 215개입니다. 고칠 때마다 다시 컴파일하는 범위를 줄이는 것이 빌드 시간을 줄이는 길입니다. `src/lib.rs` 를 건드린 뒤 명령마다 다섯 번 잰 중앙값입니다.

| 명령 | 시간 | 하는 일 |
|---|---:|---|
| `cargo fmt --check` | 0.05초 | 형식만 본다 |
| `cargo check` | 0.09초 | 타입 검사까지, 기계어는 만들지 않는다 |
| `cargo clippy --all-targets -- -D warnings` | 0.15초 | check 에 린트를 더한다 |
| `cargo build` | 0.37초 | 디버그 실행 파일을 만든다 |
| `cargo test` | 2.99초 | 실행 파일 4개와 통합 테스트까지 빌드한다 |
| `cargo test records` | 2.54초 | 이름으로 거르지만 빌드 범위는 같다 |
| `cargo test --lib records` | 0.52초 | 라이브러리 테스트만 빌드한다 |

눈여겨볼 줄은 아래 두 줄입니다. 테스트 이름으로 거르기만 하면 빌드는 그대로라 시간이 거의 줄지 않습니다. `--lib` 나 `--test cli` 로 빌드할 대상을 고르면 전체 `cargo test` 보다 5.8배 빠릅니다.

습관으로 정리하면 이렇습니다.

- 고치는 동안은 `cargo check` 나 `cargo clippy` 로 봅니다.
- 테스트는 대상(`--lib`, `--test 이름`)과 이름을 함께 좁힙니다.
- LTO 와 `codegen-units = 1` 은 릴리스 프로필에만 둡니다.
- 의존성과 feature 를 줄입니다. 4편에서 `thiserror` 하나가 빈 폴더 빌드에 2초를 더했습니다.
- `cargo build --timings` 가 만드는 HTML 보고서로 어느 크레이트가 오래 걸리는지 봅니다.

## 다른 언어에는 cdylib 위에 얇은 겉을 씌웁니다

문서 변환 엔진은 같은 코어를 명령줄 도구, Rust 라이브러리, Python 모듈, Node 모듈, MCP 서버로 내보냅니다. 겉은 얇고 변환 로직은 하나입니다.

Python 바인딩은 feature 로 켭니다. 켜지 않으면 pyo3 를 아예 빌드하지 않습니다.

```toml
[dependencies]
pyo3 = { version = "0.24", features = ["extension-module", "abi3-py38"], optional = true }

[features]
python = ["pyo3"]

[lib]
crate-type = ["rlib", "cdylib"]
```

```rust
// lib.rs
#[cfg(feature = "python")]
mod python;

// python.rs
#[pyfunction]
fn to_markdown(path: &str) -> PyResult<String> {
    crate::to_markdown(Path::new(path))
        .map_err(|e| PyRuntimeError::new_err(e.to_string()))
}
```

`rlib` 은 Rust 끼리 쓰는 라이브러리, `cdylib` 은 다른 언어가 불러 쓰는 동적 라이브러리입니다. Rust 의 `Result` 는 `map_err` 로 Python 예외가 됩니다. `pyproject.toml` 의 `[tool.maturin] features = ["python"]` 이 빌드할 때 feature 를 켭니다. Node 쪽은 엔진을 경로 의존성으로 가져오는 별도 크레이트입니다. `#[napi]` 함수가 같은 일을 하고 `Error::from_reason` 으로 JavaScript 예외를 만듭니다. 그 결과물 `.node` 파일은 `file` 명령으로 보면 `Mach-O 64-bit dynamically linked shared library arm64` 입니다.

pyo3·napi 바인딩은 이번에 빌드해 보지 않았습니다. 위 코드는 엔진 저장소에서 옮긴 구조입니다. 대신 두 도구가 서 있는 바탕, 즉 `cdylib` 과 C ABI 를 직접 확인했습니다. `recscan` 을 감싸는 크레이트를 `crate-type = ["cdylib"]` 로 만들었습니다.

```rust
/// 레코드 수를 돌려준다. 망가진 입력이면 -1.
///
/// # Safety
/// `ptr` 은 `len` 바이트를 읽을 수 있는 메모리를 가리켜야 한다.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn recscan_count(ptr: *const u8, len: usize) -> i64 {
    if ptr.is_null() {
        return -1;
    }
    #[cfg(feature = "trace")]
    eprintln!("recscan_count: {len} bytes");
    // SAFETY: 호출하는 쪽이 ptr 부터 len 바이트가 유효하다고 약속했다.
    let data = unsafe { std::slice::from_raw_parts(ptr, len) };
    match records(data).collect::<Result<Vec<_>, _>>() {
        Ok(v) => v.len() as i64,
        Err(_) => -1,
    }
}
```

Python 표준 라이브러리 `ctypes` 로 불렀습니다.

```console
$ python3 call.py target/release/librecscan_ffi.dylib doc000.rec
레코드 수: 19420
망가진 입력: -1
```

`unsafe` 는 남이 넘긴 포인터를 슬라이스로 바꾸는 한 줄에만 있습니다. 그 뒤는 5편의 안전한 파서 그대로입니다. 경계에서만 `unsafe` 를 쓰고 이유를 `SAFETY` 주석으로 남기는 것이 원칙입니다(19·22편). pyo3·napi 는 이 경계 코드를 대신 만들어 줍니다. 에디션 2024 에서 `#[no_mangle]` 만 쓰면 `unsafe attribute used without unsafe` 오류가 나고 `#[unsafe(no_mangle)]` 로 고치라고 안내합니다. 같은 크레이트를 `--features trace` 로 빌드하면 호출마다 `recscan_count: 8389440 bytes` 가 찍힙니다.

## 에이전트와 일하는 주기는 짧은 확인을 자주 돕니다

에이전트에게 맡기는 Rust 작업은 아래 한 줄을 고칠 때마다 돌리게 합니다. `recscan` 에서 한 바퀴가 0.77초였습니다.

```bash
cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test --lib records
```

커밋 전에는 `cargo test` 전체를, 내보내기 전에는 릴리스 빌드와 실제 파일 실행을 더합니다. 문서 변환 엔진을 운영하며 얻은 규칙도 셋 있습니다.

첫째, 테스트가 못 잡는 것은 실제 파일로 잡습니다. `for` 반복을 수동 인덱스 `while` 로 바꾸면서 `continue` 앞에 `i += 1` 을 빠뜨린 일이 있었습니다. 컴파일러도 단위 테스트도 잡지 못했고, Node 바인딩이 오류 없이 멈췄습니다. 그 뒤로 엔진은 시간 제한을 건 샘플 실행을 통과해야 합니다.

둘째, 출력에 버전을 찍습니다. 엔진은 결과물에 `extractor_version: env!("CARGO_PKG_VERSION")` 을 넣습니다. 어느 빌드가 만든 출력인지 나중에 가립니다. clap 의 `version` 도 같은 값을 써서 `recscan --version` 이 `recscan 0.1.0` 을 찍습니다.

셋째, 지키고 싶은 규칙은 컴파일러에 맡깁니다. 엔진의 `unsafe` 0줄은 지금은 사람이 지킵니다. 크레이트 맨 위에 `#![forbid(unsafe_code)]` 를 두면 컴파일러가 지킵니다.

```console
error: usage of an `unsafe` block
 --> src/main.rs:5:17
  |
5 |     let first = unsafe { *v.as_ptr() };
  |                 ^^^^^^^^^^^^^^^^^^^^^^
  |
note: the lint level is defined here
 --> src/main.rs:1:11
  |
1 | #![forbid(unsafe_code)]
  |           ^^^^^^^^^^^
```

## 면접에서 이렇게 묻는다

**Q1. LTO 는 무엇이고 언제 켜나요?**
링크할 때 크레이트 경계를 넘어 인라인과 죽은 코드 제거를 하는 최적화입니다. 크기는 줄지만 빌드와 재빌드가 길어집니다. 실측에서 크기 39% 감소, 재빌드 0.33초 → 3.37초였고 속도는 오히려 느려졌습니다. 릴리스 프로필에만 두고, 속도 이득은 재서 확인합니다.

**Q2. Rust 실행 파일을 작게 만드는 방법은?**
`strip`, `lto`, `opt-level = "z"`, `codegen-units = 1`, `panic = "abort"` 입니다. 실측에서 1.14MB 가 0.47MB 가 됐지만 실행이 44% 느려졌습니다. 의존성과 feature 를 줄이는 것도 효과가 큽니다.

**Q3. `rlib`, `cdylib`, `staticlib` 은 어떻게 다른가요?**
`rlib` 은 Rust 끼리 링크하는 라이브러리입니다. `cdylib` 은 C ABI 로 다른 언어가 여는 동적 라이브러리입니다. `staticlib` 은 C 링커에 넣는 정적 라이브러리입니다. pyo3·napi 모듈은 `cdylib` 입니다.

**Q4. 큰 Rust 프로젝트의 빌드 시간을 어떻게 줄이나요?**
고칠 때는 `check` 와 좁힌 테스트를 씁니다. 테스트는 이름만이 아니라 대상을 좁힙니다. LTO 는 릴리스에만 둡니다. 의존성·feature 를 줄이고 `--timings` 로 병목 크레이트를 찾습니다. 크게는 워크스페이스로 쪼개 다시 컴파일할 범위를 줄입니다(11편).

**Q5. 바인딩 경계에서 Rust 에러는 어떻게 넘기나요?**
`Result` 를 `map_err` 로 그 언어의 예외로 바꿉니다. pyo3 는 `PyErr`, napi 는 `napi::Error` 입니다. 경계를 넘을 때 패닉을 어떻게 다루는지는 22편에서 다룹니다.

## 연습

**연습 1.** 자기 도구에 `release-strip` 과 `release-lto` 프로필을 더하고 크기·재빌드 시간·실행 시간 표를 만드세요.

<details>
<summary>풀이 보기</summary>

본문의 프로필을 그대로 쓰고 `cargo build --profile <이름>` 으로 빌드합니다. 결과는 `target/<이름>/` 에 생깁니다. 재빌드는 파일 하나를 `touch` 한 뒤 재고, 세 번 이상 재서 중앙값을 씁니다. 판단 기준은 본문 표처럼 "크기를 얼마 줄이려고 재빌드를 얼마나 늘리나"입니다. 실행 시간이 늘면 그 프로필은 버립니다.

</details>

**연습 2.** `recscan-ffi` 에 `trace` feature 를 두고, 켤 때만 입력 크기를 찍게 하세요.

<details>
<summary>풀이 보기</summary>

```toml
[features]
trace = []
```

```rust
    #[cfg(feature = "trace")]
    eprintln!("recscan_count: {len} bytes");
```

`cargo build --release` 판은 아무것도 찍지 않고, `--features trace` 판은 호출마다 `recscan_count: 8389440 bytes` 와 `recscan_count: 3 bytes` 를 찍었습니다. feature 를 끈 빌드에는 그 코드가 아예 들어가지 않습니다.

</details>

**연습 3.** 에이전트가 고칠 때마다 돌릴 `scripts/check.sh` 를 쓰세요. 실패하면 0 이 아닌 종료 코드로 끝나야 합니다.

<details>
<summary>풀이 보기</summary>

```bash
#!/usr/bin/env bash
set -euo pipefail
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --lib "${1:-}"
```

`set -e` 가 첫 실패에서 멈추게 합니다. 인자로 테스트 이름을 받아 좁힙니다. `recscan` 에서 `scripts/check.sh records` 한 바퀴가 0.77초 안팎이었습니다.

</details>

## 한 단계 더 — LTO 가 재빌드를 열 배로 늘리는 이유

릴리스 빌드에는 증분 컴파일이 없습니다. `cargo build --release -v` 의 rustc 명령에 `-C incremental` 이 없었고, Cargo 공식 문서도 릴리스 프로필의 기본값을 `incremental = false` 로 적습니다. 그래도 LTO 가 없을 때는 고친 크레이트만 다시 컴파일하고 의존 크레이트의 결과는 그대로 씁니다. 그래서 0.33초였습니다.

Cargo 문서는 `lto = true`(fat LTO)를 "의존 그래프의 모든 크레이트에 걸쳐 최적화한다"고 설명합니다. 한 파일을 고쳐도 링크 단계에서 프로그램 전체를 다시 최적화합니다. 재빌드 시간이 고친 양이 아니라 프로그램 크기를 따라가는 이유입니다. `recscan` 의 3.37초와 엔진의 40.8초가 같은 구조에서 나옵니다. `lto = "thin"` 은 그 사이를 고릅니다. 측정에서 크기는 fat LTO 보다 19% 컸지만 재빌드는 1.49초였고 실행은 기본 릴리스와 같았습니다.

이 구조에서 나오는 실무 판단은 둘입니다. 하나, 릴리스 빌드를 고치고 확인하는 주기에 넣지 않습니다. 둘, 배포 산출물의 크기가 중요하지 않다면 thin LTO 나 LTO 없음이 개발 속도에 낫습니다. 엔진처럼 실행 파일 하나를 여러 곳에 배포한다면 크기와 빌드 시간을 재고 정합니다.

## 정리

내보낼 때 정할 것은 프로필이고, 그 선택은 크기·속도·빌드 시간 표로 합니다. 매일 돌릴 것은 fmt → clippy → 대상을 좁힌 test 입니다. 이 도구에서 그 한 바퀴는 1초가 안 걸렸습니다.

입문 단계는 여기까지입니다. 7편부터는 실무 단계입니다. 남과 같이 쓰는 코드를 위해 `thiserror` 와 `anyhow` 로 에러를 나누는 법부터 다룹니다.
