# Chromium 및 Cargo 생태계 비교

Rust 커뮤니티는 일반적으로 [crates.io](https://crates.io/)의 `cargo` 및 라이브러리를 사용합니다. Chromium은 세심하게 선별된 의존성들과 함께 `gn`, `ninja`을 이용하여 빌드됩니다.

Rust로 코드를 작성할 때 선택할 수 있는 옵션은 다음과 같습니다.

- `//build/rust/*.gni`의 템플릿(예: 나중에 다룰 `rust_static_library`)을 통해 `gn` 및 `ninja`를 사용합니다. 이는 Chromium의 감사 도구 모음 및 크레이트를 사용합니다.
- `cargo`를 사용하되 \[Chromium의 감사 도구 모음 및 크레이트로 제한\]합니다(https://chromium.googlesource.com/chromium/src/+/refs/heads/main/docs/rust.md#Using-cargo).
- `cargo`를 사용하여 [도구 모음](https://rustup.rs/) 또는 [인터넷에서 다운로드한 크레이트](https://crates.io/)를 신뢰합니다.

여기서부터는 `gn`과 `ninja`에 집중할 겁니다. 그 도구들을 써야만 Chromium 브라우저에 Rust 코드를빌드해서 넣을 수 있기 때문입니다. 이와 동시에, Cargo는 Rust 생태계에서 중요한 부분이므로 Cargo에도 익숙해야 합니다.

## Mini exercise

소규모 그룹으로 나눈 다음:

- ’cargo’가 유리할 수 있는 시나리오를 브레인스토밍하고 이러한 시나리오의 위험 프로필을 평가합니다.
- `gn` 및 `ninja`, 오프라인 `cargo` 등을 사용할 때 신뢰해야 하는 도구, 라이브러리, 사용자 그룹을 논의합니다.

> **발표자 노트**
>
> 학생들에게 연습문제를 완료하기 전에 발표자 노트를 엿보지 말라고 합니다. 과정을 수강하는 학생들이 실제로 함께 있다고 가정하고 3~4명으로 구성된 소그룹으로 토론하도록 합니다.
>
> 연습문제의 첫 부분과 관련된 참고사항/힌트(‘Cargo가 이점을 제공할 수 있는 시나리오’):
>
> - 도구를 작성하거나 Chromium 일부의 프로토타입을 제작할 때 crates.io 라이브러리의 풍부한 생태계에 액세스할 수 있다는 것은 멋진 일입니다. 거의 모든 것을 위해 크레이트가 있으며 일반적으로 사용하기 매우 좋습니다. 명령줄 파싱을 위한 `clap`, 다양한 형식 간 직렬화/역직렬화를 위한 `serde`, 반복자를 사용하는 경우의 `itertools` 등이 있습니다.
>
>   - `cargo`를 사용하면 라이브러리를 쉽게 사용해 볼 수 있습니다. `Cargo.toml`에 한 줄을 추가하고 코드 작성을 시작하면 됩니다.
>   - `perl`이 인기를 얻게 된 데 CPAN이 어떤 도움이 되었는지 비교해 보는 것도 좋습니다. `python` + `pip`와 비교해도 됩니다.
>
> - 핵심 Rust 도구(예: 나이틀리, 최신 안정화 버전, 이전 안정화 버전에서 작동해야 하는 크레이트를 테스트할 때 `rustup`을 사용하여 다른 `rustc` 버전으로 전환)뿐만 아니라 서드 파티 도구의 생태계(예: Mozilla는 보안 감사 간소화 및 공유를 위해 `cargo vet` 제공, `criterion` 크레이트는 간소화된 벤치마크 실행 방법을 제공함)를 통해서도 개발 환경이 개선됩니다.
>
>   - `cargo`를 사용하면 `cargo install --locked cargo-vet`를 통해 도구를 쉽게 추가할 수 있습니다.
>   - Chrome 확장 프로그램 또는 VScode 확장 프로그램과 비교해 보는 것이 좋습니다.
>
> - `cargo`가 적합할 수 있는 프로젝트의 광범위하고 일반적인 예:
>
>   - 놀랍게도 업계에서 Rust가 명령줄 도구 작성으로 점점 인기를 얻고 있습니다. 라이브러리의 다양함과 사용편의성은 Python과 유사하지만 풍부한 타입 시스템 덕분에 더 안전하고 더 빠르게 실행됩니다(인터프리트 언어가 아닌 컴파일된 언어로).
>   - Rust 생태계에 참여하려면 Cargo와 같은 표준 Rust 도구를 사용해야 합니다. 외부 기여를 원하고 Chromium 외부(예: Bazel 또는 Android/Soong 빌드 환경)에서 사용하고자 하는 라이브러리는 Cargo를 사용해야 할 수 있습니다.
>
> - `cargo`\\ 기반 Chromium 관련 프로젝트의 예는 다음과 같습니다.
>
>   - `serde_json_lenient`(Google의 다른 부분에서 실험하여 성능이 개선된 PR이 나옴)
>   - `font-types`와 같은 글꼴 라이브러리
>   - `gnrt` 도구(과정 후반부에서 설명)는 명령줄 파싱의 경우 `clap`, 구성 파일의 경우 `toml`에 종속됩니다.
>     - 주의: 여기서 `cargo`를 사용하는 유일한 이유는 Rust 도구 모음을 빌드할 때 Rust 표준 라이브러리를 빌드하고 부트스트랩하는 경우 `gn`을 사용할 수 없기 때문입니다.
>     - `run_gnrt.py`는 Chromium의 `cargo` 및 `rustc` 사본을 사용합니다. `gnrt`는 인터넷에서 다운로드한 서드 파티 라이브러리에 종속되며 `run_gnrt.py`는 `cargo`에 `Cargo.lock`을 통해 `--locked` 콘텐츠만 허용된다고 합니다.
>
> 학생은 다음 항목을 암시적 또는 명시적으로 신뢰할 수 있는 것으로 식별할 수 있습니다.
>
> - `rustc`(Rust 컴파일러)는 차례로 LLVM 라이브러리, Clang 컴파일러, `rustc` 소스(GitHub에서 가져옴, Rust 컴파일러팀에서 검토), 부트스트랩을 위해 다운로드한 바이너리 Rust 컴파일러에 종속됩니다.
> - `rustup`(`rustup`은 https://github.com/rust-lang/ 조직 산하에서 개발되었으며 `rustc`와 동일함)
> - `cargo`, `rustfmt` 등
> - 다양한 내부 인프라(’rustc’를 빌드하는 봇, 사전 빌드된 도구 모음을 Chromium 엔지니어에게 배포하기 위한 시스템 등)
> - `cargo audit`, `cargo vet` 등과 같은 Cargo 도구
> - `//third_party/rust`에 공급되는 Rust 라이브러리(security@chromium.org에서 감사)
> - 기타 Rust 라이브러리(일부는 틈새시장용, 일부는 매우 인기 있으며 흔히 사용됨)
