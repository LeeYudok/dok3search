# Iterator

Runtime errors like connection-refused or file-not-found are handled with the `Result` type, but matching this type on every call can be cumbersome. The try-operator `?` is used to return errors to the caller. It lets you turn the common

``` rust
match some_expression {
    Ok(value) => value,
    Err(err) => return Err(err),
}
```

이렇게 짧게 쓸 수 있습니다

``` rust
some_expression?
```

이제 우리 예제에 적용해 보겠습니다:

``` rust
use std::io::Read;
use std::{fs, io};

fn read_username(path: &str) -> Result<String, io::Error> {
    let username_file_result = fs::File::open(path);
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(err) => Err(err),
    }
}

fn main() {
    //fs::write("config.dat", "alice").unwrap();
    let username = read_username("config.dat");
    println!("사용자 이름 또는 오류: {username:?}");
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> `?`를 사용하도록 `read_username` 함수를 단순화합니다.
>
> 키 포인트:
>
> - `username` 변수는 `Ok(string)`이거나 `Err(error)`일 수 있습니다.
> - `fs::write` 메서드를 사용하여 파일이 없거나, 비었거나, 중복되는 경우 등을 테스트해 봅니다.
> - Note that `main` can return a `Result<(), E>` as long as it implements `std::process::Termination`. In practice, this means that `E` implements `Debug`. The executable will print the `Err` variant and return a nonzero exit status on error.
