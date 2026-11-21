# `Read`와 `Write`

[`Read`](https://doc.rust-lang.org/std/io/trait.Read.html)와 [`BufRead`](https://doc.rust-lang.org/std/io/trait.BufRead.html)를 사용하면 `u8` 타입의 데이터 스트림을 읽을 수 있습니다:

``` rust
use std::io::{BufRead, BufReader, Read, Result};

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn main() -> Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("슬라이스 내 줄: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("파일 내 줄: {}", count_lines(file));
    Ok(())
}
```

이와 비슷하게, `Write`를 사옹하면 `u8` 타입의 데이터를 쓸 수 있습니다:

``` rust
use std::io::{Result, Write};

fn log<W: Write>(writer: &mut W, msg: &str) -> Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

fn main() -> Result<()> {
    let mut buffer = Vec::new();
    log(&mut buffer, "안녕하세요")?;
    log(&mut buffer, "World")?;
    println!("로그 내역: {:?}", buffer);
    Ok(())
}
```
