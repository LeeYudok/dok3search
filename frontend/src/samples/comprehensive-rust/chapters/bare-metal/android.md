# 안드로이드

AOSP에서 bare-metal Rust 바이너리를 빌드하려면 `rust_ffi_static` 을 사용하여 Rust 코드를 빌드하고, 링커 스크립트가 포함된 `cc_binary`를 사용하여 ELF 바이너리를 생성하고, `raw_binary`를 사용해 ELF를 곧바로 수행될 수 있는 원시(raw) 바이너리로 변환합니다.

``` soong
rust_ffi_static {
    name: "libvmbase_example",
    defaults: ["vmbase_ffi_defaults"],
    crate_name: "vmbase_example",
    srcs: ["src/main.rs"],
    rustlibs: [
        "libvmbase",
    ],
}

cc_binary {
    name: "vmbase_example",
    defaults: ["vmbase_elf_defaults"],
    srcs: [
        "idmap.S",
    ],
    static_libs: [
        "libvmbase_example",
    ],
    linker_scripts: [
        "image.ld",
        ":vmbase_sections",
    ],
}

raw_binary {
    name: "vmbase_example_bin",
    stem: "vmbase_example.bin",
    src: ":vmbase_example",
    enabled: false,
    target: {
        android_arm64: {
            enabled: true,
        },
    },
}
```
