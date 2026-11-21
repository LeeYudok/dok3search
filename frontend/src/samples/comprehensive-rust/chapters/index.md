# Comprehensive Rust에 오신 것을 환영합니다 🦀

[](https://github.com/google/comprehensive-rust/actions/workflows/build.yml?query=branch%3Amain) [](https://github.com/google/comprehensive-rust/graphs/contributors) [](https://github.com/google/comprehensive-rust/stargazers)

이 강의는 무료이며, Google의 Android 팀이 만들었습니다. 기본 문법부터 제네릭, 에러 핸들링과 같은 고급주제까지 러스트의 모든 것을 포함합니다.

> 이 과정의 최신 버전은 <https://google.github.io/comprehensive-rust/>에서 확인할 수 있습니다. 다른 곳에서 읽고 있는 경우 이곳에서 업데이트를 확인하시기 바랍니다.
>
> The course is also available [as a PDF](https://google.github.io/comprehensive-rust/ko/comprehensive-rust.pdf).

강의는 당신이 러스트에 대해서 아무것도 모른다고 가정하고 아래의 목표를 가지고 있습니다:

- 러스트 구문과 언어에 대한 포괄적인 이해를 제공합니다.
- 기존 프로그램을 수정하고 러스트에서 새 프로그램을 작성할 수 있습니다.
- 일반적인 러스트 관용구를 보여줍니다.

We call the first four course days Rust Fundamentals.

그 후에는, 아래와 같은 개별 주제를 심화해서 공부할 수 있습니다:

- [Android](android.md): Android 플랫폼 개발(AOSP) 시 Rust 사용에 관한 반나절 과정입니다. 여기에는 C, C++, Java와의 상호 운용성이 포함됩니다.
- [Chromium의 Rust](chromium.md) 심층 분석은 Chromium 브라우저의 일부로 Rust를 사용하는 방법에 관한 반나절 과정입니다. 여기에는 Chromium의 ‘gn’ 빌드 시스템에서 Rust를 사용하여 서드 파티 라이브러리(“crates”)와 C++ 상호 운용성을 가져오는 방법이 포함되어 있습니다.
- [Bare-metal](bare-metal.md): bare-metal(임베디드) 개발 시 Rust 사용에 관한 종일 과정입니다. 마이크로컨트롤러와 애플리케이션 프로세서를 모두 다룹니다.
- [동시성](concurrency.md): Rust의 동시성에 관한 종일 과정입니다. 여기서는 고전적인 동시성(스레드와 뮤텍스를 사용하여 선점형 스케줄링을 하는 것)과 async/await 동시성(future를 사용하는 협력적인 멀티태스킹)을 모두 다룹니다.

## 제외사항

러스트는 며칠만에 모든 것을 다루기에는 너무 큰 언어입니다. 그래서 아래와 같은것을 목표로 하지 않습니다:

- 매크로 만들기: 매크로와 관련한 자세한 내용은 [러스트 프로그래밍 언어, 19.1절](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)과 [Rustonomicon](https://doc.rust-lang.org/nomicon/)를 참조하세요.

## 독자 수준에 대한 가정

본 강의는 여러분이 프로그래밍 자체에 대해서는 알고 있다고 가정합니다. 러스트는 정적타입 언어이며, 강좌에서는 C/C++ 와의 비교, 대조를 통해 러스트를 설명할 것입니다.

C/C++을 모르더라도 동적 타입 언어(Python이나 JavaScript 등) 프로그래밍 경험이 있다면 따라오는데 큰 문제는 없을 것입니다.

> **발표자 노트**
>
> 이것은 “발표자 노트“의 예제입니다. 이 부분을 이용해서 추가 정보를 제공합니다. 주로 강의실에서 제기되는 일반적인 질문에 대한 답변과 강사가 다루어야 할 키 포인트일 수 있습니다.
