# Rust 오류처리

``` rust
#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn fallible(depth: usize) -> Result<String>;
    }
}

fn fallible(depth: usize) -> anyhow::Result<String> {
    if depth == 0 {
        return Err(anyhow::Error::msg("fallible1에 깊이 > 0 필요"));
    }

    Ok("완료!".into())
}
```

> **발표자 노트**
>
> - `Result`를 반환하는 Rust 함수는 C++ 측에서 예외로 변환됩니다.
> - 발생하는 예외는 항상 `rust::Error` 타입이며 주로 오류 메시지 문자열을 가져오는 방법을 노출합니다. 오류 메시지는 오류 타입의 `Display` impl에서 가져옵니다.
> - Rust에서 C++로 패닉이 해제되면 프로세스가 즉시 종료됩니다.
