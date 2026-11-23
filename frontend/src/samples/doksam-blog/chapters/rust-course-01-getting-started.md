# Rust 강의 1편 — 설치부터 첫 실행 파일까지

> Rust 로 실행 파일 하나를 만들려면 무엇을 깔고, 어떤 명령을 알아야 할까?

답부터 적습니다. rustup 하나를 깔면 컴파일러와 cargo, 포매터, 린터가 함께 들어옵니다. 명령은 여섯 개로 시작합니다. `cargo new` 로 만들고, `build`·`run` 으로 돌리고, `test`·`fmt`·`clippy` 로 확인합니다. 빌드가 두 종류라는 것도 처음부터 알아 둡니다. 디버그 빌드는 정수가 넘치면 멈추고, 릴리스 빌드는 같은 실수를 넘기고 틀린 값을 냅니다.

> [그림] 왼쪽 rustup 공구 상자에 rustc, cargo, fmt, clippy 가 들어 있고, 가운데 cargo build 기계에 main.rs 가 들어가 debug 상자와 release 상자가 나온다. 검수 로봇이 돋보기로 release 상자를 본다

그림의 숫자는 이 글에서 만든 도구로 256MB 파일의 체크섬을 구한 시간입니다. 같은 코드가 디버그 빌드에서 1.62초, 릴리스 빌드에서 0.63초 걸렸습니다.

Rust 강의 1편입니다. 연재는 4단계 24편이고, 1~6편이 입문 단계입니다. 예시는 제가 운영하는 문서 변환 엔진(Rust 약 5만 줄)의 패턴을 줄여 씁니다. 왜 Rust 인가는 [Go 와 Rust 를 쓰면 좋은 점](https://blog.doksam.com/blog/why-go-and-rust/)에 적었습니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## rustup 하나로 도구 네 개가 함께 깔립니다

rustup 은 툴체인(toolchain, 컴파일러와 도구 묶음) 관리자입니다. 공식 설치 명령은 한 줄입니다. 제 맥은 Homebrew 로 깔았습니다.

```console
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ rustc --version
rustc 1.100.0 (0ef1bc386 2026-11-09)
$ cargo --version
cargo 1.100.0 (3acc79c0c 2026-11-05)
```

새 안정판은 6주마다 나오고 `rustup update` 로 올립니다. 저장소마다 버전을 고정하려면 루트에 `rust-toolchain.toml` 을 둡니다.

```toml
[toolchain]
channel = "1.97.1"
components = ["rustfmt", "clippy"]
```

이 폴더 안에서 `cargo --version` 을 치면 `cargo 1.97.1` 이 나왔습니다. 사람의 맥, CI, 에이전트의 셸이 같은 컴파일러를 쓰게 됩니다.

## cargo new 는 파일 세 개를 만듭니다

첫 도구는 파일 앞 몇 바이트로 형식을 맞히는 `sniff` 입니다. `cargo new sniff` 가 만드는 파일은 `Cargo.toml`, `src/main.rs`, `.gitignore` 입니다. `.gitignore` 에는 빌드 결과 폴더 `/target` 한 줄만 있습니다.

```toml
[package]
name = "sniff"
version = "0.1.0"
edition = "2024"

[dependencies]
```

`edition` 은 언어 판입니다. cargo 1.100.0 은 2024 를 적습니다. 에디션은 크레이트마다 고르고, 에디션이 다른 크레이트끼리도 함께 링크됩니다. 문서 변환 엔진은 아직 2021 에디션입니다.

패키지(package)는 `Cargo.toml` 하나가 묶는 단위이고, 크레이트(crate)는 컴파일러가 한 번에 번역하는 단위입니다. `src/main.rs` 는 실행 파일 크레이트, `src/lib.rs` 는 라이브러리 크레이트가 됩니다. `src/bin/` 아래 파일은 하나하나가 실행 파일 크레이트입니다.

## 첫 프로그램은 파일 앞 4바이트로 형식을 맞힙니다

```rust
use std::env;
use std::fs;
use std::process;

/// 파일 앞 몇 바이트(매직 바이트)로 형식을 맞힌다.
fn guess_format(data: &[u8]) -> &'static str {
    if data.starts_with(b"%PDF") {
        "PDF"
    } else if data.starts_with(b"PK\x03\x04") {
        "ZIP(DOCX·HWPX 포함)"
    } else if data.starts_with(&[0xD0, 0xCF, 0x11, 0xE0]) {
        "OLE(DOC·HWP 5.0 포함)"
    } else {
        "알 수 없음"
    }
}

/// Adler-32 체크섬. 바이트마다 나머지 연산을 해서 CPU 를 쓴다.
fn adler32(data: &[u8]) -> u32 {
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

fn main() {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("사용법: sniff <파일>");
            process::exit(2);
        }
    };
    let data = match fs::read(&path) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("{path}: {e}");
            process::exit(1);
        }
    };
    if data.is_empty() {
        eprintln!("{path}: 빈 파일");
    }
    println!(
        "{path}: {} ({} 바이트, adler32 {:08x})",
        guess_format(&data),
        data.len(),
        adler32(&data)
    );
}
```

문법은 뒤 편에서 풉니다. 지금은 세 가지만 봅니다.

- `&[u8]` 은 바이트 묶음을 복사하지 않고 빌려 보는 타입입니다(2·3편).
- `b"%PDF"` 는 바이트 문자열입니다. 문서 형식은 글자가 아니라 바이트로 비교합니다.
- `env::args().nth(1)` 은 인자가 있으면 `Some`, 없으면 `None` 을 돌려주고, `match` 가 둘을 나눕니다(4편).

`cargo run` 은 빌드하고 바로 실행합니다. `--` 뒤의 인자는 프로그램에 넘어갑니다.

```console
$ cargo run -q; echo "exit=$?"
사용법: sniff <파일>
exit=2
$ cargo run -q -- a.pdf
a.pdf: PDF (14 바이트, adler32 192503e1)
$ cargo run -q -- none.pdf
none.pdf: No such file or directory (os error 2)
```

문서 변환 엔진의 형식 판별 함수도 이 모양입니다. 20개 넘는 형식을 `starts_with` 로 차례로 비교합니다. 다만 DRM 으로 감싼 파일을 맨 먼저 가려냅니다. 감싼 파일을 확장자만 믿고 파서에 넘기면 엉뚱한 오류가 나기 때문입니다.

## 테스트는 같은 파일 아래에 둡니다

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_pdf_and_zip() {
        assert_eq!(guess_format(b"%PDF-1.7\n"), "PDF");
        assert_eq!(guess_format(b"PK\x03\x04rest"), "ZIP(DOCX·HWPX 포함)");
        assert_eq!(guess_format(b"hello"), "알 수 없음");
    }

    #[test]
    fn adler32_known_value() {
        // 위키백과 Adler-32 항목의 예시 값
        assert_eq!(adler32(b"Wikipedia"), 0x11E6_0398);
    }
}
```

`#[cfg(test)]` 는 테스트 빌드에만 이 모듈을 넣으라는 표시입니다. `use super::*` 로 비공개 함수까지 불러 씁니다. `cargo test` 를 돌리면 `test result: ok. 2 passed` 가 나옵니다.

## fmt 와 clippy 가 사람보다 먼저 리뷰합니다

일부러 길이를 0 과 비교하는 코드를 넣어 봤습니다. `cargo fmt --check` 는 형식만, `cargo clippy` 는 관용구까지 봅니다. 1.100.0 의 clippy 규칙은 832개이고, 그중 495개가 기본으로 켜져 있습니다(`clippy-driver -W help` 로 셈).

```console
$ cargo clippy
warning: length comparison to zero
  --> src/main.rs:44:8
   |
44 |     if data.len() == 0 {
   |        ^^^^^^^^^^^^^^^ help: using `is_empty` is clearer and more explicit: `data.is_empty()`
   |
   = note: `#[warn(clippy::len_zero)]` on by default
```

경고는 세 부분으로 읽습니다. 첫 줄이 문제, `-->` 가 파일과 줄·칸, `help:` 가 고치는 방법입니다. `clippy::len_zero` 는 규칙 이름이라 검색하면 설명이 나옵니다. 위 예제의 `data.is_empty()` 가 고친 결과입니다. 이 검사는 에이전트와 일할 때 특히 쓸모 있습니다. 사람이 리뷰에서 할 지적을 기계가 먼저 끝내기 때문입니다(6편).

## Cargo.toml 에는 범위를, Cargo.lock 에는 정확한 버전을 적습니다

외부 라이브러리도 크레이트입니다. 문서 변환 엔진의 `Cargo.toml` 일부입니다.

```toml
[dependencies]
thiserror = "2"
clap = { version = "4", features = ["derive"] }
rayon = "1"
```

`thiserror = "2"` 는 "2.0.0 이상 3.0.0 미만"이라는 범위입니다. Cargo 공식 문서는 이 표기를 캐럿(caret) 요구 조건이라고 부릅니다. 세맨틱 버전에서 앞자리가 같으면 호환된다고 보기 때문입니다. `features` 는 그 크레이트의 선택 기능을 켭니다.

범위만 적으면 빌드할 때마다 다른 버전이 들어올 수 있습니다. 그래서 cargo 는 처음 고른 버전을 `Cargo.lock` 에 적습니다. 문서 변환 엔진의 잠금 파일에는 패키지 420개가 있습니다. `cargo new` 가 만든 `.gitignore` 는 잠금 파일을 빼지 않으므로 함께 커밋합니다. Cargo 공식 안내도 2023년부터 커밋을 기본으로 권합니다.

## 디버그 빌드는 틀린 계산을 멈추고, 릴리스 빌드는 넘깁니다

`src/bin/bytesum.rs` 에 바이트를 모두 더하는 프로그램을 둡니다.

```rust
use std::{env, fs};

fn main() {
    let path = env::args().nth(1).expect("파일 경로를 주세요");
    let data = fs::read(&path).expect("파일을 읽지 못했습니다");
    let mut sum: u32 = 0;
    for &b in &data {
        sum += b as u32; // u32 최댓값(4,294,967,295)을 넘으면?
    }
    println!("바이트 합: {sum}");
}
```

입력은 `0xFF` 로 채운 32MB 파일입니다. 바른 합 255 × 33,554,432 = 8,556,380,160 은 `u32` 에 들어가지 않습니다.

```console
$ ./target/debug/bytesum ff32m.bin; echo "exit=$?"
thread 'main' (30048295) panicked at src/bin/bytesum.rs:8:9:
attempt to add with overflow
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exit=101
$ ./target/release/bytesum ff32m.bin
바이트 합: 4261412864
```

디버그 빌드는 넘치는 순간 패닉(panic, 프로그램을 멈추는 오류)을 냅니다. 릴리스 빌드는 2의 보수로 감아 돌려 바른 값을 2³² 로 나눈 나머지를 냅니다. 오류 없이 틀린 값이 나오는 쪽이 더 위험합니다. 그래서 테스트는 디버그 빌드로 돌리고, 넘칠 수 있는 계산은 넓은 타입이나 `checked_add` 로 씁니다.

표의 실행 시간은 256MB 무작위 파일로 `sniff` 를 다섯 번 돌린 중앙값입니다. 컴파일러 옵션은 `cargo build -v` 가 찍은 rustc 명령에서 옮겼습니다.

| 항목 | 디버그 (`cargo build`) | 릴리스 (`cargo build --release`) |
|---|---|---|
| 프로필 이름 | `dev` | `release` |
| 최적화 | 없음 (opt-level 0) | `-C opt-level=3` |
| 정수 오버플로 | 패닉 | 감아 돌림 |
| 디버그 정보 | `-C debuginfo=2` | `-C strip=debuginfo` 로 뺌 |
| `sniff` 실행 (256MB) | 1.62초 | 0.63초 |
| `sniff` 크기 | 532,872 바이트 | 456,320 바이트 |

측정 환경은 Apple M4 Pro 14코어, macOS 26.6.2, 2026-11-18 입니다. `sniff` 는 빌드가 0.2초도 안 걸리지만 큰 프로젝트는 다릅니다. 문서 변환 엔진은 빈 폴더에서 릴리스 빌드에 86초가 걸립니다. 한 파일을 고친 뒤 다시 빌드해도 40.8초입니다. 그래서 평소에는 디버그 빌드로 일하고 릴리스 빌드는 내보낼 때만 합니다(6편).

## 면접에서 이렇게 묻는다

**Q1. 디버그 빌드와 릴리스 빌드는 무엇이 다른가요?**
최적화 수준, 디버그 정보, 정수 오버플로 검사가 다릅니다. 디버그는 빨리 빌드되고 오버플로에서 패닉합니다. 릴리스는 빌드가 느리고 실행이 빠르며 오버플로를 감아 돌립니다. `[profile.*]` 으로 하나씩 바꿀 수 있습니다.

**Q2. Rust 에서 정수 오버플로는 정의되지 않은 동작(UB)인가요?**
아닙니다. Rust Reference 는 디버그 단언이 켜진 빌드에서 오버플로를 패닉으로 잡게 합니다. 검사하지 않는 빌드에서는 2의 보수로 감아 돌린 값을 내게 정해 둡니다. C 의 부호 있는 정수 오버플로가 UB 인 것과 다릅니다. 의도를 드러내려면 `checked_*`·`wrapping_*`·`saturating_*` 을 씁니다.

**Q3. `Cargo.lock` 은 커밋하나요?**
기본은 커밋합니다. 같은 커밋이 언제나 같은 의존성으로 빌드됩니다. 라이브러리의 잠금 파일은 그 라이브러리를 쓰는 쪽 빌드에 쓰이지 않습니다. 그래도 커밋하면 자기 CI 를 재현할 수 있습니다.

**Q4. 패키지와 크레이트는 어떻게 다른가요?**
크레이트는 컴파일 단위, 패키지는 `Cargo.toml` 이 묶는 크레이트 모음입니다. 패키지에는 라이브러리 크레이트가 많아야 하나이고 실행 파일 크레이트는 여러 개일 수 있습니다.

## 연습

**연습 1.** `bytesum` 을 고쳐 32MB `0xFF` 파일에서 바른 합을 내게 하세요. 넓은 타입을 쓰는 방법과, `u32` 를 지키면서 넘치면 알리는 방법 두 가지로 풉니다.

<details>
<summary>풀이 보기</summary>

```rust
    // 풀이 A: 더 넓은 타입으로 더한다
    let wide: u64 = data.iter().map(|&b| b as u64).sum();
    println!("u64 합: {wide}");

    // 풀이 B: u32 를 지키되 넘치면 알린다
    let mut sum: u32 = 0;
    for &b in &data {
        sum = match sum.checked_add(b as u32) {
            Some(s) => s,
            None => {
                eprintln!("u32 합이 넘쳤습니다");
                process::exit(1);
            }
        };
    }
```

```console
$ cargo run -q --bin ex1_bytesum -- ff32m.bin
u64 합: 8556380160
u32 합이 넘쳤습니다
```

`checked_add` 는 넘치면 `None` 을 돌려줍니다. 빌드 종류와 상관없이 결과가 같습니다.

</details>

**연습 2.** `guess_format` 에 GZIP(`1F 8B`)과 PNG(`89 50 4E 47 0D 0A 1A 0A`)를 더하고 테스트를 쓰세요. 한 바이트짜리 입력도 시험합니다.

<details>
<summary>풀이 보기</summary>

```rust
    } else if data.starts_with(&[0x1F, 0x8B]) {
        "GZIP"
    } else if data.starts_with(b"\x89PNG\r\n\x1a\n") {
        "PNG"
    } else {
```

```rust
    #[test]
    fn detects_gzip_and_png() {
        assert_eq!(guess_format(&[0x1F, 0x8B, 0x08, 0x00]), "GZIP");
        assert_eq!(guess_format(b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR"), "PNG");
        assert_eq!(guess_format(&[0x1F]), "알 수 없음");
    }
```

`starts_with` 는 입력이 짧으면 `false` 를 돌려줍니다. `data[1]` 처럼 인덱스로 읽으면 한 바이트 입력에서 패닉합니다. 이 차이는 5편에서 다룹니다.

</details>

## 한 단계 더 — 오버플로 검사를 릴리스에서 켜면 얼마나 느려질까

결제 금액이나 파일 오프셋처럼 틀리면 안 되는 계산은 릴리스에서도 오버플로를 잡고 싶습니다. 프로필을 하나 더 만들어 쟀습니다.

```toml
[profile.release-checked]
inherits = "release"
overflow-checks = true
```

`cargo build --profile release-checked` 의 결과는 `target/release-checked/` 에 생깁니다. 이 빌드의 `bytesum` 은 디버그처럼 `attempt to add with overflow` 로 멈췄습니다. 256MB 체크섬 시간(5회 중앙값)은 `release` 와 `release-checked` 가 똑같이 0.63초였습니다. 해석은 이렇습니다. 바이트마다 하는 나머지 연산이 시간을 거의 다 쓰고, 덧셈 검사는 그에 비해 작습니다. 덧셈만 촘촘히 도는 코드라면 결과가 달라질 수 있습니다. "검사를 켜면 느리다"는 말을 믿기 전에 자기 코드로 잽니다. 프로필은 `inherits` 로 만들 수 있어 실험 비용도 작습니다.

## 정리

Rust 로 실행 파일 하나를 만드는 데 필요한 것은 rustup 과 cargo 명령 여섯 개입니다. 빌드 종류에 따라 같은 코드가 다르게 동작한다는 것도 기억합니다. 테스트는 디버그로 돌리고, 넘칠 수 있는 계산은 타입으로 막습니다.

2편에서는 첫 함수에 나온 `&[u8]` 의 `&` 를 풉니다. 값의 주인이 하나라는 소유권 규칙과, 그 규칙이 막는 실수를 실제 컴파일 오류로 봅니다.
