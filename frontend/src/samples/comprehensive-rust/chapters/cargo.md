# 카고 사용하기

러스트를 시작하려고하면 당신은 곧 [Cargo](https://doc.rust-lang.org/cargo/)라는, 러스트 생태계에서 사용하는 표준 빌드/실행 도구를 만날 것 입니다. 여기서는 카고가 무엇인지, 그리고 카고가 러스트 생태계에서 어떤 역할을 하는지, 그리고 이 강의에서 어떻게 사용될 지에 대해 간략히 설명하겠습니다.

## 설치하기

> **<https://rustup.rs/>의 설치 방법을 따르세요.**

This will give you the Cargo build tool (`cargo`) and the Rust compiler (`rustc`). You will also get `rustup`, a command line utility that you can use to install to different compiler versions.

Rust를 설치한 후에는 Rust와 호환되도록 편집기나 IDE를 구성해야 합니다. 대부분의 편집기는 [VS Code](https://code.visualstudio.com/), [Emacs](https://rust-analyzer.github.io/manual.html#emacs), [Vim/Neovim](https://rust-analyzer.github.io/manual.html#vimneovim) 등에 자동 완성 및 정의로 이동 기능을 제공하는 [rust-analyzer](https://rust-analyzer.github.io/)와 통신하여 이를 실행합니다. [RustRover](https://www.jetbrains.com/rust/)라는 다른 IDE도 사용할 수 있습니다.

> **발표자 노트**
>
> - 데비안/우분투 시스템에서는 `apt`를 이용해서 카고, 러스트 소스, [러스트 포매터](https://github.com/rust-lang/rustfmt)를 설치할 수 있습니다. 그러나 이 방법을 따를 경우 최신 버전이 아닌 러스트를 사용게되며, 그 결과 예상치 못한 문제를 겪을 수도 있습니다. 설치 명령어는 아래와 같습니다:
>
>   ``` shell
>   sudo apt install cargo rust-src rustfmt
>   ```
