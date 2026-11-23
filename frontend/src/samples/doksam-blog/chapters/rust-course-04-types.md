# Rust 강의 4편 — 타입으로 설계하기

> 문서 형식과 실패를 어떤 타입에 담아야 컴파일러가 빠뜨린 곳을 찾아 줄까?

답부터 적습니다. 경우가 정해진 것은 `enum` 으로, 없을 수 있는 값은 `Option` 으로, 실패할 수 있는 일은 `Result` 로 적습니다. `match` 는 모든 변형을 다뤄야 컴파일되므로, 형식을 하나 더하면 손댈 곳이 오류로 나옵니다. 에러는 `thiserror` 로 `enum` 하나에 모으고 `?` 로 위로 올립니다. 형식마다 다른 파서는 트레이트 하나 뒤에 둡니다.

> [그림] 왼쪽에 Pdf, Zip, Hwp5, Unknown 네 칸 분류함이 있고, 바닥의 새 상자 Hwpx 에는 E0004 갈 곳 없음 도장이 찍혀 있다. 검수 로봇이 돋보기로 그 상자를 본다. 오른쪽에 Option 의 Some·None 과 Result 의 Ok·Err 가 있고, Err 는 물음표로 호출한 쪽에 돌아간다

왼쪽은 이 글의 E0004 예시 그대로입니다. 갈래가 네 개인 `match` 에 새 형식 `Hwpx` 가 들어오자 컴파일러가 갈 곳이 없다고 막았습니다.

Rust 강의 4편입니다. [3편](https://blog.doksam.com/blog/rust-course-03-borrowing/)에 이어 값에 모양을 줍니다. 외부 크레이트는 `thiserror` 2.0.21 하나를 씁니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 구조체는 함께 다니는 값을 묶습니다

문서의 메타데이터는 제목, 작성자, 쪽수입니다. 어느 것도 없을 수 있습니다.

```rust
/// 메타데이터. 없을 수 있는 값은 Option 으로 적는다.
#[derive(Debug, Clone, Default)]
pub struct DocInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub pages: Option<u32>,
}
```

`#[derive(...)]` 는 흔한 구현을 컴파일러가 만들게 합니다. `Default` 덕분에 `DocInfo::default()` 는 모든 필드가 `None` 인 값이 됩니다. 일부만 채울 때는 `DocInfo { title, ..DocInfo::default() }` 처럼 나머지를 기본값으로 메웁니다.

## enum 은 경우를 닫고, match 는 빠짐없이 다룹니다

문서 형식은 정해진 목록이라 `enum` 으로 닫습니다. 문자열로 들고 다니면 오타를 컴파일러가 잡지 못합니다.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Pdf,
    Zip,
    Ole,
    Hwp5,
    Txt,
    Unknown,
}

impl Format {
    pub fn name(&self) -> &'static str {
        match self {
            Format::Pdf => "PDF",
            Format::Zip => "ZIP",
            Format::Ole => "OLE",
            Format::Hwp5 => "HWP 5.0",
            Format::Txt => "Text",
            Format::Unknown => "Unknown",
        }
    }
}
```

변형을 하나 더하고 `match` 를 고치지 않으면 어떻게 될까요. 갈래 네 개짜리 예시로 돌렸습니다.

```console
error[E0004]: non-exhaustive patterns: `Format::Hwpx` not covered
  --> src/main.rs:11:11
   |
11 |     match f {
   |           ^ pattern `Format::Hwpx` not covered
...
 6 |     Hwpx, // 새로 더한 형식
   |     ---- not covered
```

`match` 가 모든 변형을 다루는지 컴파일러가 검사합니다. 이것을 완전성(exhaustiveness) 검사라고 부릅니다. 저장소에 이런 `match` 가 스무 군데 있으면 스무 군데가 모두 오류로 나옵니다. 고칠 곳 목록을 컴파일러가 만들어 주는 셈입니다. 문서 변환 엔진의 `Format::name` 은 54갈래를 와일드카드 없이 적었습니다. `_ =>` 를 쓰면 이 목록을 잃기 때문입니다. 새 변형이 자주 생기는 `enum` 에는 와일드카드를 피합니다.

## enum 변형은 데이터를 담을 수 있습니다

문서 본문은 제목, 문단, 표로 이루어집니다. 변형마다 담는 데이터가 다릅니다.

```rust
pub enum Block {
    Heading { level: u8, text: String },
    Paragraph(String),
    Table(Vec<Vec<String>>),
}

pub fn to_markdown(blocks: &[Block]) -> String {
    let mut out = String::new();
    for block in blocks {
        match block {
            Block::Heading { level, text } if *level <= 6 => {
                out.push_str(&"#".repeat(*level as usize));
                out.push(' ');
                out.push_str(text);
            }
            Block::Heading { text, .. } => out.push_str(text), // 7단계 이상은 문단으로
            Block::Paragraph(text) => out.push_str(text),
            Block::Table(rows) => { /* 행마다 `| a | b |`, 첫 행 뒤에 구분선. 줄임 */ }
        }
        out.push_str("\n\n");
    }
    out
}
```

패턴은 값을 꺼내면서 모양을 확인합니다. `if *level <= 6` 같은 조건(가드)도 붙일 수 있습니다. 가드가 있는 갈래는 완전성 검사에서 빠지므로 아래에 `Block::Heading { text, .. }` 를 따로 둡니다. 크기는 `size_of::<Block>()` 이 32바이트였습니다. 가장 큰 변형(`String` 24바이트와 `u8`)에 어느 변형인지 적는 자리를 더한 값입니다.

## 없을 수 있는 값은 Option 으로 적습니다

Rust 에는 null 이 없습니다. 없을 수 있으면 타입에 `Option` 이라고 적습니다. 바이트 파서에서 가장 흔한 쓰임은 "모자라면 없음"입니다.

```rust
/// 모자라면 None. 패닉하지 않는다.
pub fn read_u32_le(data: &[u8], pos: usize) -> Option<u32> {
    let end = pos.checked_add(4)?;
    let bytes = data.get(pos..end)?;
    Some(u32::from_le_bytes(bytes.try_into().ok()?))
}
```

`data[pos..end]` 는 범위를 벗어나면 패닉하고, `data.get(pos..end)` 는 `None` 을 돌려줍니다. `?` 는 `None` 을 만나면 그 자리에서 `None` 을 돌려주고 함수를 끝냅니다. 문서 변환 엔진의 입력 타입도 `read_u16_le` 같은 함수를 모두 `Option` 으로 돌려줍니다. 망가진 파일이 들어와도 파서가 죽지 않게 하려는 것입니다.

꺼낼 때는 `title.as_deref().unwrap_or("(없음)")` 처럼 없을 때의 값을 정합니다.

## 실패할 수 있는 일은 Result 로, 전파는 ? 로 합니다

`Result<T, E>` 는 성공하면 `Ok(T)`, 실패하면 `Err(E)` 입니다. 문서 변환 엔진의 에러 타입을 줄이면 이렇습니다.

```rust
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("unsupported format: {0}")]
    UnsupportedFormat(String),

    #[error("corrupt data: {0}")]
    CorruptData(String),

    #[error("encrypted document")]
    Encrypted,
}
```

`#[error("...")]` 는 사람에게 보일 문장이고, `#[from]` 은 `io::Error` 를 이 타입으로 바꾸는 변환을 만듭니다. `type Result<T>` 별칭 덕분에 함수마다 에러 타입을 되풀이하지 않습니다. 파일을 읽어 글자를 뽑는 함수는 이렇습니다.

```rust
pub fn filter_file(path: impl AsRef<std::path::Path>) -> Result<String> {
    let data = std::fs::read(path)?; // io::Error → Error::Io (#[from])
    if data.len() < 4 {
        return Err(Error::Truncated {
            need: 4,
            have: data.len(),
        });
    }
    let parser = get_parser(Format::detect(&data))?;
    let mut out = String::new();
    parser.filter(&data, &mut out)?;
    Ok(out)
}
```

`?` 는 `Err(e)` 를 만나면 `From::from(e)` 로 바꿔 돌려줍니다. 그래서 `io::Error` 가 `Error::Io` 가 됩니다. 네 파일로 돌린 결과입니다.

```console
  no-such-file.txt: I/O error: No such file or directory (os error 2) (source 있음: true)
  short.bin: truncated: need 4 bytes, have 2 (source 있음: false)
  memo.txt: ok "회의록\n본문"
  blob.bin: unsupported format: unknown format (source 있음: false)
```

변환이 없는 에러에 `?` 를 쓰면 컴파일러가 막습니다. 숫자 해석 에러(`ParseIntError`)를 그대로 올리려 한 경우입니다.

```console
error[E0277]: `?` couldn't convert the error to `Error`
  --> src/main.rs:11:39
   |
10 |     let text = std::fs::read_to_string(path)?; // io::Error → Error: From 이 있다
11 |     let n = text.trim().parse::<u32>()?; // ParseIntError → Error: From 이 없다
   |                         --------------^ the trait `From<ParseIntError>` is not implemented for `Error`
```

고치는 길은 둘입니다. 변형을 더하고 `#[from]` 을 붙이거나, 그 자리에서 `.map_err(|e| Error::CorruptData(e.to_string()))?` 로 바꿉니다. 원인 정보를 남기려면 앞쪽이 낫습니다. 라이브러리와 애플리케이션에서 에러를 어떻게 나눌지는 7편에서 다룹니다.

## 트레이트로 파서를 바꿔 끼웁니다

형식마다 파서가 다르지만 하는 일은 같습니다. 바이트를 받아 글자를 뽑습니다. 이 약속을 트레이트(trait)로 적습니다. 다른 언어의 인터페이스와 비슷합니다.

```rust
pub trait Parser {
    /// 본문 글자를 뽑아 out 에 붙인다.
    fn filter(&self, input: &[u8], out: &mut String) -> Result<()>;

    /// 메타데이터. 구현하지 않으면 빈 값을 쓴다.
    fn doc_info(&self, _input: &[u8]) -> DocInfo {
        DocInfo::default()
    }
}

pub struct TxtParser;

impl Parser for TxtParser {
    fn filter(&self, input: &[u8], out: &mut String) -> Result<()> {
        let text = std::str::from_utf8(input)
            .map_err(|e| Error::CorruptData(format!("not UTF-8: {e}")))?;
        out.push_str(text);
        Ok(())
    }
}
```

`doc_info` 는 기본 구현이 있어서 구현하는 쪽이 골라 덮어씁니다. 문서 변환 엔진의 `Parser` 트레이트도 이 두 메서드로 되어 있습니다. 어떤 파서를 쓸지는 실행 중에 파일을 보고 정합니다. 그래서 고르는 함수는 트레이트 객체 `Box<dyn Parser>` 를 돌려줍니다.

```rust
pub fn get_parser(format: Format) -> Result<Box<dyn Parser>> {
    match format {
        Format::Txt => Ok(Box::new(TxtParser)),
        Format::Unknown => Err(Error::UnsupportedFormat("unknown format".into())),
        other => Err(Error::UnsupportedFormat(format!(
            "{} parser not in this example",
            other.name()
        ))),
    }
}
```

같은 일을 `fn extract_with<P: Parser>(parser: &P, ...)` 처럼 제네릭으로도 쓸 수 있습니다. `fn extract_dyn(parser: &dyn Parser, ...)` 와는 파서 타입이 정해지는 때가 다릅니다.

| 비교 | 제네릭 `P: Parser` | 트레이트 객체 `dyn Parser` |
|---|---|---|
| 타입이 정해지는 때 | 컴파일할 때 | 실행할 때 |
| 메서드 찾기 | 직접 호출 | vtable(함수 표)을 거쳐 호출 |
| 참조 크기 | `&TxtParser` 8바이트 | `&dyn Parser` 16바이트 |
| 쓰는 곳 | 타입을 아는 호출 | 실행 중에 고르는 목록, `Vec<Box<dyn Parser>>` |

`&dyn Parser` 가 16바이트인 것은 데이터 주소와 vtable 주소를 함께 들고 다니기 때문입니다. 두 방식의 성능과 바이너리 크기는 16편에서 잽니다.

## 면접에서 이렇게 묻는다

**Q1. `match` 의 완전성 검사는 무엇이고 왜 중요한가요?**
`match` 가 모든 변형을 다루지 않으면 E0004 로 컴파일을 막습니다. 변형을 더하면 처리하지 않은 곳이 모두 오류로 나와서 빠뜨림이 없습니다. 와일드카드 `_` 는 이 검사를 끄므로 자주 바뀌는 `enum` 에는 피합니다.

**Q2. Rust 에 null 이 없으면 없는 값은 어떻게 다루나요?**
`Option<T>` 로 타입에 적습니다. 꺼내기 전에 `None` 을 다루도록 컴파일러가 강제합니다. `Option<&T>` 는 null 주소를 `None` 으로 쓰므로 `&T` 와 같은 8바이트입니다(실측).

**Q3. `?` 연산자는 정확히 무엇을 하나요?**
`Ok(v)` 면 `v` 를 꺼내고, `Err(e)` 면 `From::from(e)` 로 함수의 에러 타입으로 바꿔 바로 돌려줍니다. 변환이 없으면 E0277 로 막힙니다. `Option` 에도 쓸 수 있습니다.

**Q4. `Box<dyn Trait>` 와 제네릭은 언제 고르나요?**
컴파일할 때 타입을 알면 제네릭이 직접 호출이라 인라인도 됩니다. 실행 중에 고르거나 다른 타입을 한 목록에 담으면 트레이트 객체를 씁니다. 대가는 vtable 호출과 16바이트 참조입니다.

**Q5. `thiserror` 는 무엇을 해 주나요?**
에러 `enum` 에 `Display`, `std::error::Error`, `From` 구현을 만들어 줍니다. 구체적인 에러 타입을 공개하는 라이브러리에 맞습니다.

## 연습

**연습 1.** `Format` 에 `Hwpx` 를 더하고, 컴파일러가 알려 주는 곳을 고치세요.

<details>
<summary>풀이 보기</summary>

변형을 더하면 `name()` 의 `match` 에서 E0004 가 납니다. 갈래 하나를 더하면 됩니다.

```rust
            Format::Hwp5 => "HWP 5.0",
            Format::Hwpx => "HWPX",
```

`get_parser` 는 `other =>` 로 나머지를 받으므로 오류가 나지 않습니다. 와일드카드가 검사를 끈다는 증거이기도 합니다.

</details>

**연습 2.** `read_u32_le` 가 짧은 입력과 `usize::MAX` 위치에서 패닉하지 않는지 테스트를 쓰세요.

<details>
<summary>풀이 보기</summary>

```rust
    #[test]
    fn reads_or_none() {
        let d = [0x78, 0x56, 0x34, 0x12, 0xFF];
        assert_eq!(read_u32_le(&d, 0), Some(0x1234_5678));
        assert_eq!(read_u32_le(&d, 2), None);
        assert_eq!(read_u32_le(&d, usize::MAX), None);
    }
```

`checked_add` 가 없으면 `usize::MAX + 4` 가 넘쳐 디버그에서 패닉합니다(1편).

</details>

**연습 3.** `Error` 에 `Truncated { need: usize, have: usize }` 를 더하세요. 메시지는 `truncated: need 4 bytes, have 2` 꼴입니다.

<details>
<summary>풀이 보기</summary>

```rust
    #[error("truncated: need {need} bytes, have {have}")]
    Truncated { need: usize, have: usize },
```

이름 붙은 필드는 메시지에서 `{need}` 처럼 이름으로 씁니다. `Error::Truncated { need: 4, have: 2 }.to_string()` 이 위 문장과 같은지 테스트로 확인했습니다.

</details>

## 한 단계 더 — thiserror 가 대신 써 주는 코드와 그 값

`thiserror` 없이 같은 에러를 손으로 쓰면 구현 세 개가 필요합니다. 아래 둘에 원인을 돌려주는 `std::error::Error::source()` 구현이 하나 더 붙습니다.

```rust
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(e) => write!(f, "I/O error: {e}"),
            Error::Encrypted => write!(f, "encrypted document"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}
```

손으로 쓴 판도 같은 메시지와 `source 있음: true` 를 찍었습니다. `#[from]` 은 `From` 과 함께 `source()` 까지 채웁니다. 원인 사슬을 따라가는 로깅 도구가 이 `source()` 를 씁니다.

값도 있습니다. `thiserror` 는 절차적 매크로(20편)라 `syn`·`quote`·`proc-macro2` 를 데려옵니다. 빈 폴더에서 빌드하면 손으로 쓴 판은 0.16~0.18초, `thiserror` 판은 2.37~2.48초였습니다. 2초 남짓은 처음 한 번만 들고 그 뒤에는 캐시됩니다. 변형이 몇 개뿐인 작은 도구라면 손으로 써도 됩니다. 변형이 스무 개인 엔진이라면 매크로가 낫습니다. 해석의 기준은 변형 수와 바뀌는 빈도입니다.

## 정리

컴파일러가 빠뜨린 곳을 찾게 하려면 경우는 `enum` 으로 닫고, 없음은 `Option` 으로, 실패는 `Result` 로 타입에 적습니다. `match` 에 와일드카드를 쓰기 전에 이 검사를 버려도 되는지 한 번 더 봅니다.

5편에서는 이 타입들로 실제 도구를 만듭니다. 바이트를 안전하게 읽는 레코드 파서, 이터레이터, `clap` 명령줄, 테스트, `rayon` 병렬 처리까지 갑니다.
