# `thiserror` and `anyhow`

The [`thiserror`](https://docs.rs/thiserror/) and [`anyhow`](https://docs.rs/anyhow/) crates are widely used to simplify error handling.

- `thiserror` is often used in libraries to create custom error types that implement `From<T>`.
- `anyhow` is often used by applications to help with error handling in functions, including adding contextual information to your errors.

컴파일 오류 예제

``` rust
use anyhow::{bail, Context, Result};
use std::fs;
use std::io::Read;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{0}에서 사용자 이름을 찾을 수 없습니다.")]
struct EmptyUsernameError(String);

fn read_username(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .with_context(|| format!("{path}을(를) 열지 못했습니다."))?
        .read_to_string(&mut username)
        .context("읽지 못했습니다.")?;
    if username.is_empty() {
        bail!(EmptyUsernameError(path.to_string()));
    }
    Ok(username)
}

fn main() {
    //fs::write("config.dat", "").unwrap();
    match read_username("config.dat") {
        Ok(username) => println!("사용자 이름: {username}"),
        Err(err) => println!("오류: {err:?}"),
    }
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> ## `thiserror`
>
> - The `Error` derive macro is provided by `thiserror`, and has lots of useful attributes to help define error types in a compact way.
> - The `std::error::Error` trait is derived automatically.
> - The message from `#[error]` is used to derive the `Display` trait.
>
> ## `anyhow`
>
> - `anyhow::Error`는 `Box<dyn Error>`의 래퍼 타입이라 할 수 있습니다. 따라서 라이브러리의 공개 API로서 사용하기에 부적합하다고 할 수 있지만 많은 애플리케이션에 널리 사용되고 있습니다.
> - `anyhow::Result<V>`는 `Result<V, anyhow::Error>`의 타입 앨리어스(alias)입니다.
> - 필요하다면 `anyhow::Error`에 저장된 진짜 에러 타입을 꺼내어 검사할 수도 있습니다.
> - `anyhow::Result<T>`가 제공하는 기능들이 Go 언어 개발자들에게는 익숙할 것입니다. Go언어에서 반환 값으로 사용하는 `(T, error)` 패턴과 비슷하기 때문입니다.
> - `anyhow::Context` is a trait implemented for the standard `Result` and `Option` types. `use anyhow::Context` is necessary to enable `.context()` and `.with_context()` on those types.
