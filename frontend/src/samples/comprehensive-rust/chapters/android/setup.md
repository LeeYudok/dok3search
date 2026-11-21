# 설치

We will be using a Cuttlefish Android Virtual Device to test our code. Make sure you have access to one or create a new one with:

``` shell
source build/envsetup.sh
lunch aosp_cf_x86_64_phone-trunk_staging-userdebug
acloud create
```

자세한 내용은 [Android Developer Codelab](https://source.android.com/docs/setup/start)을 참조하십시오.

> **발표자 노트**
>
> 키 포인트:
>
> - Cuttlefish is a reference Android device designed to work on generic Linux desktops. MacOS support is also planned.
>
> - Cuttlefish는 실제 하드웨어를 매우 충실히 재현하고 있으며, Rust 를 사용해 보기에이상적인 에뮬레이터입니다.
