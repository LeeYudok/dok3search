# Welcome to Bare Metal Rust

이 과정은 Rust에 대해 어느정도 경험이 있고 (아마도 Comprehensive Rust 과정을 통해) C와 같은 다른 언어로 bare-metal 프로그래밍을 해 본 사용자를 대상으로 하는 bare-metal Rust에 관한 독립적인 1일 과정입니다.

오늘은 OS를 사용하지 않고 Rust 코드를 실행하는 ‘bare-metal’ Rust에 관해 알아봅니다. 본 강의의 구성은 다음과 같습니다.

- `no_std` Rust란 무엇인가요?
- 마이크로컨트롤러용 펌웨어 작성
- 애플리케이션 프로세서를 위한 부트로더 / 커널 코드 작성
- bare-metal Rust 개발을 위한 유용한 크레이트

이 강의에서는 [BBC micro:bit](https://microbit.org/) v2마이크로컨트롤러를 사용합니다. 이 마이크로컨트롤러는 Nordic nRF51822 마이크로컨트롤러에 기반한 [개발 보드](https://tech.microbit.org/hardware/)로써, LED와 버튼, I2C 연결 가속도계 및 나침반, 온보드 SWD 디버거를 포함하고 있습니다.

시작하기전에, 앞으로 사용할 도구를 설치해야 합니다. gLinux 또는 Debian를 사용하고 있다면 아래와 같이 하세요.

``` bash
sudo apt install gcc-aarch64-linux-gnu gdb-multiarch libudev-dev picocom pkg-config qemu-system-arm
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils cargo-embed
```

`plugdev` 그룹의 사용자에게 micro:bit 프로그래머 장치에 대한 액세스 권한을 부여합니다.

``` bash
echo 'SUBSYSTEM=="usb", ATTR{idVendor}=="0d28", MODE="0664", GROUP="plugdev"' |\
  sudo tee /etc/udev/rules.d/50-microbit.rules
sudo udevadm control --reload-rules
```

MacOS에서:

``` bash
xcode-select --install
brew install gdb picocom qemu
brew install --cask gcc-aarch64-embedded
rustup update
rustup target add aarch64-unknown-none thumbv7em-none-eabihf
rustup component add llvm-tools-preview
cargo install cargo-binutils cargo-embed
```
