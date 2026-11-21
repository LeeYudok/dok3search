# 러스트 생태계

러스트의 생태계는 여러가지 도구들로 구성되어 있으며, 그 중 중요한 것들은 아래와 같습니다:

- `rustc`: `.rs` 확장자 파일을 바이너리 혹은 다른 중간 형식으로 변환해주는 Rust 컴파일러입니다.

- `cargo`: 러스트 의존성 관리자이자 빌드 시스템 입니다. 여러분의 프로젝트에 명시된 의존성들을 <https://crates.io>에서 자동으로 다운로드 받고, 그 소스코드를 `rustc`로 전달하여 빌드를 시킵니다. 또한 유닛 테스트를 실행하는 테스트 러너를 내장하고 있습니다.

- `rustup`: the Rust toolchain installer and updater. This tool is used to install and update `rustc` and `cargo` when new versions of Rust are released. In addition, `rustup` can also download documentation for the standard library. You can have multiple versions of Rust installed at once and `rustup` will let you switch between them as needed.

> **발표자 노트**
>
> 키 포인트:
>
> - 러스트는 6주마다 새로운 릴리즈가 발표되며 이전 릴리즈와의 호환성을 유지하고 있습니다.
>
> - 릴리즈는 3가지 버전으로 제공됩니다: “stable”, “beta” 그리고 “nightly”.
>
> - 새로운 기능은 “nightly” -\> “beta” -(6주 후)-\> “stable” 로 변경됩니다.
>
> - 의존성은 다양한 [저장소](https://doc.rust-lang.org/cargo/reference/registries.html), git 프로젝트, 디렉터리 등에서 제공될 수 있습니다.
>
> - 러스트는 [에디션](https://doc.rust-lang.org/edition-guide/)으로 구분됩니다. 현재는 Rust 2021 에디션입니다. 이 전 에디션으로 Rust 2015와 Rust 2018이 있습니다.
>
>   - 에디션은 이전 에디션과 호환이 되지 않을 수 있습니다.
>
>   - 에디션이 바뀌면서 프로그램이 의도치 않게 깨지는 문제를 막기 위해, 각 프로그램은 자신이 빌드될 에디션을 명시적으로 `Cargo.toml`에 지정해야 합니다.
>
>   - 러스트 생태계가 에디션 별로 파편회 되는 것을 막기 위해, 러스트 컴파일러는 서로 다른 에디션에서 작성된 코드들을 하나의 바이너리로 묶을 수 있습니다.
>
>   - `cargo`를 사용하지 않고 컴파일러를 직접 사용하는 경우는 거의 없음을 언급해 주시기 바랍니다.
>
>   - It might be worth alluding that Cargo itself is an extremely powerful and comprehensive tool. It is capable of many advanced features including but not limited to:
>
>     - 프로젝트/패키지 구조화
>     - [워크스페이스](https://doc.rust-lang.org/cargo/reference/workspaces.html)
>     - 개발/런타임 종속성 관리 및 캐싱
>     - [빌드 스크립트](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
>     - [전역 설치](https://doc.rust-lang.org/cargo/commands/cargo-install.html)
>     - [cargo clippy](https://github.com/rust-lang/rust-clippy)와 같은 하위 플러그인으로 확장 가능.
>
>   - [공식 Cargo Book](https://doc.rust-lang.org/cargo/)에서 자세한 사항을 확인하시기 바랍니다.
