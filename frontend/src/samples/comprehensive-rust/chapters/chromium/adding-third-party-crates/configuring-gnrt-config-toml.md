# Configuring `gnrt_config.toml`

`Cargo.toml`과 함께 [`gnrt_config.toml`](https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/gnrt_config.toml)이 있습니다. 여기에는 크레이트 처리를 위한 Chromium 전용 확장 프로그램이 포함되어 있습니다.

새 크레이트를 추가하는 경우 적어도 `group`을 지정해야 합니다. 다음 중 하나입니다.

``` toml
#   'safe': The library satisfies the rule-of-2 and can be used in any process.
#   'sandbox': The library does not satisfy the rule-of-2 and must be used in
#              a sandboxed process such as the renderer or a utility process.
#   'test': The library is only used in tests.
```

예를 들면 다음과 같습니다.

``` toml
[crate.my-new-crate]
group = 'test' # only used in test code
```

크레이트 소스 코드 레이아웃에 따라 이 파일을 사용하여 `LICENSE` 파일을 찾을 수 있는 위치를 지정해야 할 수도 있습니다.

나중에 문제를 해결하기 위해 이 파일에서 구성해야 하는 몇 가지 사항을 살펴봅니다.
