# 빌드 규칙

안드로이드 빌드 시스템(Soong)은 다음과 같은 여러 모듈을 통해 러스트를 지원합니다:

| Module Type | Description |
|----|----|
| `rust_binary` | 러스트 바이너리를 생성합니다. |
| `rust_library` | 러스트 라이브러리(rlib혹은 dylib)를 생성합니다. |
| `rust_ffi` | cc 모듈에서 사용할 수 있는 C library (정적 혹은 동적)를 생성합니다. |
| `rust_proc_macro` | `proc-macro`를 구현하는 러스트 라이브러리를 생성합니다. 컴파일러의 플러그인으로 생각해도 좋습니다. |
| `rust_test` | 표준 러스트 테스트 러너를 사용하는 테스트 바이너리를 생성합니다. |
| `rust_fuzz` | `libfuzzer`를 사용하여 fuzz 바이너리를 생성합니다. |
| `rust_protobuf` | 프로토버프(protobuf) 인터페이스를 제공하는 러스트 라이브러리를 생성합니다. |
| `rust_bindgen` | C 라이브러리에 대한 러스트 바인딩을 제공하는 러스트 라이브러리를 생성합니다. |

다음은 `rust_binary`와 `rust_library`를 살펴봅니다.

> **발표자 노트**
>
> 발표자가 언급할 수 있는 추가 항목:
>
> - Cargo는 다국어 저장소에 최적화되지 않았으며 인터넷에서 패키지를 다운로드합니다.
>
> - Android에서는 규정상, 그리고 빌드 속도를 위해, 크레이트들이 Android 소스코드 트리 안에 미리 포함되어 있어야 합니다. 빌드 시 다운로드 받을 수 없습니다. 또한 C/C++/Java 코드와 상호 운용되어야 합니다. Android 빌드 시스템인 Soong이 이 공백을 메웁니다.
>
> - Soong은 Blaze(google3에서 사용됨)의 오픈소스 변형인 Bazel과 많이 비슷합니다.
>
> - [Android](https://source.android.com/docs/setup/build/bazel/introduction), [ChromeOS](https://chromium.googlesource.com/chromiumos/bazel/), [Fuchsia](https://source.android.com/docs/setup/build/bazel/introduction)를 Bazel로 전환할 계획입니다.
>
> - Bazel과 유사한 빌드 규칙을 배우는 것은 모든 Rust OS 개발자에게 유용합니다.
>
> - 재미있는 사실: 스타트렉의 캐릭터 중 한명인 데이터(Data)는 사실 Soong 타입의 안드로이드(Android)입니다.
