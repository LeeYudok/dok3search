# 연습문제

Chromium에 [uwuify](https://crates.io/crates/uwuify)를 추가하여 크레이트의 [기본 기능](https://doc.rust-lang.org/cargo/reference/features.html#the-default-feature)을 사용 중지합니다. 크레이트가 Chromium 배송에 사용되지만 신뢰할 수 없는 입력을 처리하는 데는 사용되지 않는다고 가정합니다.

다음 연습에서는 Chromium의 uwuify를 사용합니다. 원한다면 건너뛰고 지금 해 봐도 됩니다. 또는 `uwuify`를 사용하는 새로운 [`rust_executable` 타겟](https://source.chromium.org/chromium/chromium/src/+/main:build/rust/rust_executable.gni)을 만들 수 있습니다.

> **발표자 노트**
>
> 직접 및 임시 종속 항목
>
> The total crates needed are:
>
> - `instant`,
> - `lock_api`,
> - `parking_lot`,
> - `parking_lot_core`,
> - `redox_syscall`,
> - `scopeguard`,
> - `smallvec`, and
> - `uwuify`.
>
> If students are downloading even more than that, they probably forgot to turn off the default features.
>
> 이 크레이트를 제공해 주신 [다니엘 리우](https://github.com/Daniel-Liu-c0deb0t)에게 감사드립니다.
