# 동적인 에러 타입

Sometimes we want to allow any type of error to be returned without writing our own enum covering all the different possibilities. The `std::error::Error` trait makes it easy to create a trait object that can contain any error.

``` rust
use std::error::Error;
use std::fs;
use std::io::Read;

fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut count_str = String::new();
    fs::File::open(path)?.read_to_string(&mut count_str)?;
    let count: i32 = count_str.parse()?;
    Ok(count)
}

fn main() {
    fs::write("count.dat", "1i3").unwrap();
    match read_count("count.dat") {
        Ok(count) => println!("개수: {count}"),
        Err(err) => println!("오류: {err}"),
    }
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> `read_count` 함수는 `std::io::Error`(파일 작업에서) 또는 `std::num::ParseIntError`(`String::parse`에서)를 반환할 수 있습니다.
>
> Boxing errors saves on code, but gives up the ability to cleanly handle different error cases differently in the program. As such it’s generally not a good idea to use `Box<dyn Error>` in the public API of a library, but it can be a good option in a program where you just want to display the error message somewhere.
>
> Make sure to implement the `std::error::Error` trait when defining a custom error type so it can be boxed. But if you need to support the `no_std` attribute, keep in mind that the `std::error::Error` trait is currently compatible with `no_std` in [nightly](https://github.com/rust-lang/rust/issues/103765) only.
