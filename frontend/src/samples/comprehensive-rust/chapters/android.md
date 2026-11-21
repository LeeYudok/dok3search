# Welcome to Rust in Android

Rust is supported for system software on Android. This means that you can write new services, libraries, drivers or even firmware in Rust (or improve existing code as needed).

> 우리는 오늘 여러분의 프로젝트에서 러스트 코드를 호출해볼 것입니다. 그 프로젝트에서 러스트로 옮길만 한 작은 부분을 정하세요. 의존성이 적고 “특이한” 타입이 적을 수록 좋습니다. 바이트 몇 개를 파싱하는 코드라면 완벽합니다.

> **발표자 노트**
>
> Android에서 Rust 사용이 늘어난 점을 감안할 때 발표자는 다음 내용을 언급할 수 있습니다.
>
> - 서비스 예: [DNS over HTTP](https://security.googleblog.com/2022/07/dns-over-http3-in-android.html)
>
> - 라이브러리: [Rutabaga 가상 그래픽 인터페이스](https://crosvm.dev/book/appendix/rutabaga_gfx.html)
>
> - 커널 드라이버: [바인더](https://lore.kernel.org/rust-for-linux/20231101-rust-binder-v1-0-08ba9197f637@google.com/)
>
> - 펌웨어: [pKVM 펌웨어](https://security.googleblog.com/2023/10/bare-metal-rust-in-android.html)
