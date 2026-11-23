# Rust 강의 11편 — 워크스페이스와 feature

> 라이브러리만 가져다 쓰는 사람이 명령줄 도구용 의존성까지 빌드하지 않게 하려면 어떻게 할까?

답부터 적습니다. 패키지의 `[dependencies]` 는 그 안의 모든 크레이트가 함께 씁니다. 그래서 실행 파일용 의존성이 라이브러리 사용자에게도 따라갑니다. 막는 길은 둘입니다. 크레이트를 워크스페이스로 나누거나, 선택 기능을 `optional` 의존성과 feature 로 묶습니다. 이 글의 예제에서 라이브러리 사용자가 빌드하는 크레이트가 27개에서 8개로, 빈 폴더 빌드가 2.9초에서 1.9초로 줄었습니다.

> [그림] 왼쪽에는 recscan 상자를 실은 수레에 clap, rayon, syn, quote, anstyle 상자가 얹혀 있고 검수 로봇이 그 짐을 본다. 나누기 전 27개 2.9초. 오른쪽 워크스페이스 틀에는 rec-core, rec-cli + clap, parallel 을 켤 때만 들어오는 rayon 이 있고 초록 로봇은 rec-core 하나만 든다. 나눈 뒤 8개 1.9초

왼쪽 수레의 짐은 실제로 빌드된 크레이트 이름 일부입니다. 라이브러리 함수 하나를 쓰려고 명령줄 파서와 병렬 처리 크레이트까지 실었습니다.

Rust 강의 11편입니다. [7편](https://blog.doksam.com/blog/rust-course-07-error-design/)에서 [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)의 `recscan` 을 라이브러리로 가져오자 clap 과 rayon 이 잠금 파일에 함께 들어왔습니다. 이번 편은 그 문제를 고칩니다. 측정은 2026-11-18, Apple M4 Pro, rustc 1.100.0 입니다.

## 패키지의 의존성은 그 안의 모든 크레이트가 함께 씁니다

`recscan` 은 한 패키지에 라이브러리와 실행 파일이 함께 있습니다. 의존성 표는 하나입니다.

```toml
[dependencies]
clap = { version = "4.6.8", features = ["derive"] }
rayon = "1.12.0"
thiserror = "2.0.21"
```

clap 은 `main.rs` 만 쓰고, rayon 도 `count` 명령만 씁니다. 그래도 라이브러리 함수 `records` 하나를 쓰는 새 패키지를 빈 폴더에서 빌드하면 크레이트 27개를 컴파일합니다. 세 번 잰 시간은 2.47~3.85초였습니다. 컴파일된 이름에 `clap`, `clap_builder`, `anstyle`, `rayon`, `crossbeam-epoch` 가 들어 있었습니다.

문서 변환 엔진도 같은 구조입니다. 한 패키지에 라이브러리와 명령줄·HTTP 서버·MCP 서버 실행 파일이 있고, `clap`·`rayon`·`tokio`·`axum`·`rmcp` 가 모두 일반 의존성입니다. 엔진을 라이브러리로 쓰는 Node 바인딩 크레이트도 이 의존성을 모두 빌드합니다. 엔진이 컴파일하는 크레이트가 215개인 데에는 이 구조도 한몫합니다. 다만 그중 몇 개가 실행 파일 전용인지는 세지 않았습니다.

## 워크스페이스로 나누면 쓰는 크레이트의 의존성만 빌드합니다

워크스페이스(workspace)는 여러 패키지를 한 저장소에서 함께 관리하는 단위입니다. 루트 `Cargo.toml` 에 구성원과 공통 설정을 적습니다.

```toml
[workspace]
members = ["core", "cli", "consumer"]
resolver = "3"

[workspace.package]
version = "0.1.0"
edition = "2024"

[workspace.dependencies]
thiserror = "2.0.21"
clap = { version = "4.6.8", features = ["derive"] }
rayon = "1.12.0"
rec-core = { path = "core" }
```

`[workspace.dependencies]` 는 버전을 한곳에 모읍니다. 구성원은 `thiserror.workspace = true` 처럼 이름만 적습니다. 구성원끼리 버전이 어긋날 일이 없습니다.

나눈 결과는 셋입니다.

| 패키지 | 하는 일 | 의존성 |
|---|---|---|
| `rec-core` | 파서 라이브러리 | `thiserror`, 선택 `rayon` |
| `rec-cli` | 명령줄 도구 | `rec-core`(parallel 켬), `clap` |
| `consumer` | 라이브러리만 쓰는 예 | `rec-core` |

`cargo build -p consumer` 를 빈 상태에서 세 번 돌리니 크레이트 8개, 1.90~1.93초였습니다. 남은 8개는 `consumer`, `rec-core`, 그리고 `thiserror` 의 절차적 매크로를 위한 `syn`·`quote` 등입니다. clap 과 rayon 은 사라졌습니다.

워크스페이스의 구성원은 잠금 파일 하나와 `target/` 폴더 하나를 함께 씁니다. 같은 의존성을 구성원마다 따로 빌드하지 않습니다. 문서 변환 엔진의 Node 바인딩은 워크스페이스 구성원이 아니라 자기 `Cargo.lock` 과 `target/` 을 따로 둡니다. 엔진을 두 번 빌드하는 구조입니다.

## 선택 기능은 optional 의존성과 feature 로 묶습니다

병렬 처리를 원하는 사용자만 rayon 을 빌드하게 했습니다.

```toml
[dependencies]
thiserror.workspace = true
rayon = { workspace = true, optional = true }

[features]
# 여러 파일을 병렬로 센다. 켜지 않으면 rayon 을 빌드하지 않는다.
parallel = ["dep:rayon"]
```

`dep:rayon` 은 "이 feature 가 rayon 의존성을 켠다"는 뜻입니다. 코드는 `#[cfg(feature = "parallel")]` 로 나눕니다.

```rust
#[cfg(feature = "parallel")]
pub fn count_many(bufs: &[Vec<u8>]) -> Vec<Result<usize, Truncated>> {
    use rayon::prelude::*;
    bufs.par_iter().map(|b| count_records(b)).collect()
}

#[cfg(not(feature = "parallel"))]
pub fn count_many(bufs: &[Vec<u8>]) -> Vec<Result<usize, Truncated>> {
    bufs.iter().map(|b| count_records(b)).collect()
}
```

`count_many` 는 feature 와 상관없이 같은 이름과 시그니처로 있습니다. 병렬 판에만 있는 `worker_threads()` 를 feature 없이 부르면 컴파일러가 이유까지 알려 줍니다.

```console
error[E0425]: cannot find function `worker_threads` in crate `rec_core`
  --> src/main.rs:3:30
   |
 3 |     println!("{}", rec_core::worker_threads());
   |                              ^^^^^^^^^^^^^^ not found in `rec_core`
   |
note: found an item that was configured out
  --> <examples>/ws/core/src/lib.rs:48:8
   |
47 | #[cfg(feature = "parallel")]
   |       -------------------- the item is gated behind the `parallel` feature
```

`rec-cli` 는 `rec-core = { workspace = true, features = ["parallel"] }` 로 켭니다. 실행하면 `worker threads: 14` 가 찍힙니다.

## feature 는 함께 빌드하는 구성원 사이에서 합쳐집니다

같은 워크스페이스에서 한쪽은 `parallel` 을 켜고 한쪽은 끕니다. 둘을 함께 빌드하면 어떻게 될까요. `cargo build --workspace -v` 의 rustc 명령을 보면 `rec-core` 는 한 번만, `feature="parallel"` 로 컴파일됩니다. `cargo build -p consumer` 로 따로 빌드하면 feature 없이 컴파일됩니다.

```console
$ cargo tree --workspace -i rayon -e features
rayon v1.12.0
└── rayon feature "default"
    └── rec-core v0.1.0
        ├── rec-core feature "default" (command-line)
        │   ├── consumer v0.1.0
        │   └── rec-cli v0.1.0
        └── rec-core feature "parallel"
            └── rec-cli v0.1.0 (*)
```

이것이 feature 통합(unification)입니다. Cargo 는 한 번에 빌드하는 그래프 안에서 같은 크레이트를 한 벌만 만들고, 요청된 feature 를 모두 켭니다. 그래서 `consumer` 는 워크스페이스 전체 빌드에서는 병렬 판을 받습니다. Cargo 문서가 "feature 는 더하기만 해야 한다"고 하는 이유가 여기에 있습니다. feature 하나를 켰다고 API 가 사라지거나 동작이 바뀌면, 같은 워크스페이스의 다른 구성원이 깨집니다. 위 `count_many` 를 두 판 모두 같은 시그니처로 둔 것도 그 때문입니다.

`cargo tree -i <크레이트>` 는 "누가 이걸 끌고 왔나"를 거꾸로 보여 줍니다. 의존성이 왜 들어왔는지 모를 때 가장 먼저 칠 명령입니다.

## 면접에서 이렇게 묻는다

**Q1. 워크스페이스는 왜 쓰나요?**
여러 패키지가 잠금 파일과 `target/` 을 함께 써서 같은 의존성을 한 번만 빌드하고 버전을 맞춥니다. 라이브러리와 실행 파일을 나누면 라이브러리 사용자가 실행 파일 의존성을 빌드하지 않습니다. 실측에서 27개가 8개로 줄었습니다.

**Q2. feature 통합이란 무엇인가요?**
한 빌드 안에서 같은 크레이트를 한 벌만 만들고, 여러 곳에서 요청한 feature 를 모두 켜는 동작입니다. 그래서 feature 는 기능을 더하기만 해야 합니다. 끄는 feature 나 API 를 바꾸는 feature 는 다른 사용자를 깨뜨립니다.

**Q3. `optional = true` 와 `dep:` 은 무엇을 하나요?**
`optional` 의존성은 어떤 feature 가 켤 때만 빌드됩니다. `dep:rayon` 은 그 feature 가 rayon 을 켠다고 적는 문법입니다. `dep:` 을 쓰면 의존성 이름이 바깥에 feature 로 드러나지 않습니다.

**Q4. `default-features = false` 는 언제 쓰나요?**
의존 크레이트의 기본 feature 가 필요 없을 때 끄고 필요한 것만 켭니다. 빌드 시간과 바이너리 크기를 줄입니다. 문서 변환 엔진도 `image` 크레이트를 `default-features = false` 로 두고 필요한 형식(png·jpeg 등)만 켭니다.

**Q5. 의존성이 왜 들어왔는지 어떻게 찾나요?**
`cargo tree -i <이름>` 으로 거꾸로 따라갑니다. `-e features` 를 더하면 어떤 feature 때문인지까지 보입니다.

## 연습

**연습 1.** `rec-core` 에 `parallel` 을 켜지 않은 채 `count_many` 를 부르면 무엇이 도나요? 켜면 결과가 달라지면 안 되는 이유는?

<details>
<summary>풀이 보기</summary>

직렬 판이 돕니다. 두 판은 같은 입력에 같은 결과를 내야 합니다. 워크스페이스 전체를 빌드하면 feature 통합으로 다른 구성원이 켠 병렬 판이 들어오기 때문입니다. 결과가 다르면 "따로 빌드하면 통과, 같이 빌드하면 실패"하는 버그가 됩니다. 5편에서 확인했듯 `par_iter().map().collect()` 는 순서를 지키므로 두 판의 출력이 같습니다.

</details>

**연습 2.** 7편의 `errdesign` 크레이트가 clap 을 왜 빌드하는지 한 줄 명령으로 찾으세요.

<details>
<summary>풀이 보기</summary>

`cargo tree -i clap` 을 `errdesign` 에서 돌리면 `clap ← recscan ← errdesign` 경로가 나옵니다. 고치는 길은 이 편의 방식대로 `recscan` 의 라이브러리를 별도 패키지로 떼어 내는 것입니다.

</details>

**연습 3.** 워크스페이스 구성원 셋이 모두 `thiserror` 를 쓰는데 한 곳만 버전을 올리고 싶다면 어떻게 될까요?

<details>
<summary>풀이 보기</summary>

`[workspace.dependencies]` 를 쓰면 한곳을 고치면 모두 바뀝니다. 한 구성원만 다르게 하려면 그 구성원에서 `thiserror = "..."` 로 직접 적을 수 있습니다. 메이저 버전이 다르면 두 버전이 모두 빌드됩니다. 같은 메이저 안이면 Cargo 가 하나로 맞춥니다. 의도한 경우가 아니면 버전을 한곳에 두는 편이 빌드 시간과 혼란을 줄입니다.

</details>

## 한 단계 더 — 나누는 비용과 기준

나누는 데도 값이 있습니다. 패키지가 늘면 `Cargo.toml` 이 늘고, 공개 API 경계가 생깁니다. `rec-core` 의 함수를 `rec-cli` 가 쓰려면 `pub` 이어야 하고, 그 순간 바깥 사용자의 약속이 됩니다. 또 크레이트 경계를 넘는 작은 함수는 기본 설정에서 인라인되지 않을 수 있습니다. 6편에서 본 LTO 가 이 경계를 넘어 최적화하는 장치입니다.

해석하면 나누는 기준은 셋입니다. 사용자가 다른가(라이브러리 사용자와 명령줄 사용자), 의존성이 무거운가(tokio·axum 처럼 수십 개를 끌고 오는가), 다시 컴파일되는 범위를 줄이고 싶은가입니다. 셋 다 아니면 한 패키지가 단순합니다. 문서 변환 엔진처럼 바인딩·서버·명령줄이 한 코어를 공유한다면 코어를 떼는 쪽의 이득이 큽니다. 이 판단은 이 편의 측정과 엔진의 `Cargo.toml` 구조에서 나온 것이고, 엔진을 실제로 나눠 재지는 않았습니다.

## 정리

라이브러리 사용자가 실행 파일 의존성을 빌드하지 않게 하려면 크레이트를 워크스페이스로 나누고, 선택 기능은 optional 의존성과 feature 로 묶습니다. feature 는 함께 빌드하면 합쳐지므로 더하기만 하게 만듭니다. 의존성이 왜 들어왔는지 모르겠으면 `cargo tree -i` 부터 칩니다.

12편에서는 코드 리뷰에서 자주 걸리는 관용구를 clippy 와 함께 고칩니다. 이 연재에서 실제로 걸렸던 경고들이 소재입니다.
