# 강의 구성

> 강사를 위한 안내 페이지입니다.

## Rust 기초

첫 4일은 [Rust 기초](../welcome-day-1.md)로 이루어지며 짧은 시간 안에 많은 내용을 다루게 됩니다.

Course schedule:

- Day 1 Morning (2 hours and 10 minutes, including breaks)

| Segment      | Duration   |
|--------------|------------|
| 개요         | 5 minutes  |
| Hello World! | 15 minutes |
| 타입 및 값   | 45 minutes |
| 흐름 제어    | 40 minutes |

- Day 1 Afternoon (2 hours and 15 minutes, including breaks)

| Segment          | Duration   |
|------------------|------------|
| 튜플 및 배열     | 35 minutes |
| 참조             | 35 minutes |
| 사용자 정의 타입 | 50 minutes |

- Day 2 Morning (2 hours and 55 minutes, including breaks)

| Segment           | Duration   |
|-------------------|------------|
| 개요              | 3 minutes  |
| 패턴 매칭         | 1 hour     |
| 메소드와 트레이트 | 50 minutes |
| 제네릭            | 40 minutes |

- Day 2 Afternoon (3 hours and 10 minutes, including breaks)

| Segment                | Duration              |
|------------------------|-----------------------|
| 표준 라이브러리 타입   | 1 hour and 20 minutes |
| 표준 라이브러리 트레잇 | 1 hour and 40 minutes |

- Day 3 Morning (2 hours and 20 minutes, including breaks)

| Segment       | Duration   |
|---------------|------------|
| 개요          | 3 minutes  |
| 메모리 관리   | 1 hour     |
| 스마트 포인터 | 55 minutes |

- Day 3 Afternoon (2 hours and 10 minutes, including breaks)

| Segment         | Duration              |
|-----------------|-----------------------|
| 대여            | 50 minutes            |
| 슬라이스와 수명 | 1 hour and 10 minutes |

- Day 4 Morning (2 hours and 40 minutes, including breaks)

| Segment   | Duration   |
|-----------|------------|
| 개요      | 3 minutes  |
| Iterators | 45 minutes |
| 모듈      | 40 minutes |
| 테스트    | 45 minutes |

- Day 4 Afternoon (2 hours and 10 minutes, including breaks)

| Segment              | Duration             |
|----------------------|----------------------|
| 오류처리             | 55 minutes           |
| 안전하지 않은 러스트 | 1 hour and 5 minutes |

## 심화 학습

In addition to the 4-day class on Rust Fundamentals, we cover some more specialized topics:

### Rust in Android

The [Rust in Android](../android.md) deep dive is a half-day course on using Rust for Android platform development. This includes interoperability with C, C++, and Java.

[AOSP 코드](https://source.android.com/docs/setup/download/downloading)를 여러분의 컴퓨터에 체크아웃해야 합니다. 그런 다음, 그 컴퓨터에서 [과정 저장소](https://github.com/google/comprehensive-rust)를 체크아웃하고 `src/android/` 디렉터리를 AOSP 코드의 루트로 이동합니다. 이렇게 하면 안드로이드 빌드 시스템에서 과제용으로 추가된 `Android.bp`파일을 인식할 수 있습니다.

`adb sync` 명렁어가 에뮬레이터 혹은 실제 장치와 작동하는지 확인합니다. 그리고 `src/android/build_all.sh`를 수행해서 모든 안드로이드 예제를 미리 빌드해 보세요. 그 쉘 스크립트를 읽고, 그 안에서 수행되는 명령어들을 확인한 후 각 명령어들을 수동으로 실행해도 잘 되는지 확인하세요.

### Chromium에서 Rust 사용

[Chromium에서 Rust 사용](../chromium.md) 이 과정은 Chromium 브라우저의 일부로 Rust를 사용하는 방법에 관한 반나절 과정입니다. 여기에는 Chromium의 gn 빌드 시스템에서 Rust를 사용하는 방법과 서드 파티 라이브러리 (“crates”)를 가져오는 방법, C++ 상호 운용성 등이 포함되어 있습니다.

Chromium을 빌드할 수 있어야 합니다. 디버그, 구성요소 빌드는 속도를 높이기 위해 [권장](../chromium/setup.md)되지만 모든 빌드가 작동합니다. 빌드한 Chromium 브라우저를 실행할 수 있는지 확인합니다.

### Bare-Metal Rust

The [Bare-Metal Rust](../bare-metal.md) deep dive is a full day class on using Rust for bare-metal (embedded) development. Both microcontrollers and application processors are covered.

마이크로컨트롤러 파트를 진행하기 위해서는 [BBC micro:bit](https://microbit.org/) v2 개발 보드를 미리 구매해야 합니다. 모든 사용자는 [시작 페이지](../bare-metal.md)에 설명된 대로 각종 패키지를 설치해야 합니다.

### Concurrency in Rust

The [Concurrency in Rust](../concurrency.md) deep dive is a full day class on classical as well as `async`/`await` concurrency.

새 크레이트를 설정하고 몇 가지 의존성을 다운로드해 두어야 합니다. 그런 다음 예제를 `src/main.rs`에 복사/붙여넣기 하여 테스트 해 볼 수 있습니다:

``` shell
cargo init concurrency
cd concurrency
cargo add tokio --features full
cargo run
```

## 강의 형식

이 강의는 강사와 수강생이 양방향으로 소통하면서 진행하도록 디자인 되었습니다. 다양한 질문을 통해 러스트의 여러 부분을 탐험할 수 있도록 하세요!
