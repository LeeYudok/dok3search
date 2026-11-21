# 빌드 규칙

Chromium 빌드에서 다음을 포함하는 `//ui/base/BUILD.gn`에 새 Rust 타겟을 추가합니다.

``` rust
#![allow(unused)]
fn main() {
#[no_mangle]
pub extern "C" fn hello_from_rust() {
    println!("Hello from Rust!")
}
}
```

**중요**: 여기서 `no_mangle`은 Rust 컴파일러에 의해 안전하지 않은 유형으로 간주되므로 `gn` 타겟에서 안전하지 않은 코드를 허용해야 합니다.

이 새로운 Rust 타겟을 `//ui/base:base`의 종속 항목으로 추가합니다. 이 함수를 `ui/base/resource/resource_bundle.cc`의 맨 위에서 선언합니다. 나중에 바인딩 생성 도구로 자동화하는 방법을 살펴봅니다.

``` cpp
extern "C" void hello_from_rust();
```

`ui/base/resource/resource_bundle.cc`의 어딘가에서 이 함수를 호출합니다. `ResourceBundle::MaybeMangleLocalizedString`의 상단이 좋습니다. Chromium을 빌드하고 실행하여 ’Hello from Rust!’가 여러 번 출력되는지 확인합니다.

VSCode를 사용하는 경우 이제 VSCode에서 잘 작동하도록 Rust를 설정합니다. 이후 연습에서 유용합니다. 성공하면 `println!`에서 ’정의로 이동’을 마우스 오른쪽 버튼으로 클릭할 수 있습니다.

## 도움을 받을 수 있는 곳

- [`rust_static_library` gn 템플릿](https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_static_library.gni;l=16)에서 사용 가능한 옵션
- [`#[no_mangle]`](https://doc.rust-lang.org/beta/reference/abi.html#the-no_mangle-attribute)에 관한 정보
- [`extern "C"`](https://doc.rust-lang.org/std/keyword.extern.html)에 관한 정보
- gn의 [`--export-rust-project`](https://gn.googlesource.com/gn/+/main/docs/reference.md#compilation-database) 전환 정보
- [VSCode에서 rust-analyzer를 설치하는 방법](https://code.visualstudio.com/docs/languages/rust)

> **발표자 노트**
>
> It's really important that students get this running, because future exercises will build on it.
>
> 이 예는 최소 공통분모 상호 운용성 언어인 C로 귀결되기 때문에 일반적이지 않습니다. C++와 Rust 모두 기본적으로 C ABI 함수를 선언하고 호출할 수 있습니다. 이 과정의 후반부에서 C++를 Rust에 직접 연결합니다.
>
> 여기서 `allow_unsafe = true`가 필요한 이유는 `#[no_mangle]`이 Rust가 이름이 같은 함수 두 개를 생성할 수 있도록 할 수 있고 Rust는 더 이상 올바른 함수가 호출된다고 보장할 수 없기 때문입니다.
>
> 순수한 Rust 실행 파일이 필요하면 `rust_executable` gn 템플릿을 사용하면 됩니다.
