# 크레이트를 추가하도록 `Cargo.toml` 파일 구성

Chromium에는 중앙에서 관리되는 직접 크레이트 종속 항목의 단일 세트가 있습니다. 이는 단일 [`Cargo.toml`](https://source.chromium.org/chromium/chromium/src/+/main:third_party/rust/chromium_crates_io/Cargo.toml)을 통해 관리됩니다.

``` toml
[dependencies]
bitflags = "1"
cfg-if = "1"
cxx = "1"
# lots more...
```

다른 `Cargo.toml`과 마찬가지로 [종속 항목에 관한 자세한 내용](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)을 지정할 수 있습니다. 가장 흔하게는 크레이트에서 사용 설정하려는 `features`를 지정하는 것이 좋습니다.

Chromium에 크레이트를 추가할 때는 다음 단계에서 다룰 추가 파일 `gnrt_config.toml`에 몇 가지 정보를 추가로 제공해야 하는 경우가 많습니다.
