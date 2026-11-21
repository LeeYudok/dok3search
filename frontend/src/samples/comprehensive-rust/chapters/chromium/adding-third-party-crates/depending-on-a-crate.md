# 크레이트에 따라 다름

서드 파티 크레이트를 추가하고 빌드 규칙을 생성하고 나면 크레이트에 따른 작업은 간단합니다. `rust_static_library` 타겟을 찾고 크레이트 내의 `:lib` 타겟에 `dep`를 추가합니다.

Specifically,

![름 · //서드 파티/rust · /v · /:lib · 크 · 레 · 이 · 트이 · 메 · 저semver · 버 · 전](../../../assets/chromium/adding-third-party-crates/depending-on-a-crate-diagram-1.svg)

예를 들면 다음과 같습니다.

``` gn
rust_static_library("my_rust_lib") {
  crate_root = "lib.rs"
  sources = [ "lib.rs" ]
  deps = [ "//third_party/rust/example_rust_crate/v1:lib" ]
}
```
