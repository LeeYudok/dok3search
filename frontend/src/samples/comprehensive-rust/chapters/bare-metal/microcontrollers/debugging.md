# 디버깅

*Embed.toml*:

``` toml
[default.general]
chip = "nrf52833_xxAA"

[debug.gdb]
enabled = true
```

`src/bare-metal/microcontrollers/example/` 에 터미널을 열고:

``` sh
cargo embed --bin board_support debug
```

In another terminal in the same directory:

gLinux 또는 Debian에서:

``` sh
gdb-multiarch target/thumbv7em-none-eabihf/debug/board_support --eval-command="target remote :1337"
```

MacOS에서:

``` sh
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/board_support --eval-command="target remote :1337"
```

> **발표자 노트**
>
> GDB에서 다음을 실행해 보세요.
>
> ``` gdb
> b src/bin/board_support.rs:29
> b src/bin/board_support.rs:30
> b src/bin/board_support.rs:32
> c
> c
> c
> ```
