# Rust 강의 7편 — 에러 설계: thiserror 와 anyhow

> 라이브러리와 애플리케이션은 에러를 어떻게 다르게 다뤄야 할까?

답부터 적습니다. 누가 에러를 읽느냐로 가릅니다. 라이브러리의 에러는 호출한 코드가 읽고 경우별로 가릅니다. 그래서 `thiserror` 로 만든 `enum` 을 공개합니다. 애플리케이션의 에러는 대개 사람이 읽습니다. 그래서 `anyhow::Error` 하나에 담고 무엇을 하다 실패했는지 문맥(context)을 덧붙입니다. 어느 쪽이든 원인을 문자열로 뭉개지 않고 `source` 사슬로 남깁니다. `panic` 은 프로그램 자체의 버그에만 씁니다.

> [그림] 왼쪽 라이브러리 쪽에는 Io, Parse, NotRecordFile 서랍이 있고 들어온 Err 를 코드가 match 로 가른다. 오른쪽 애플리케이션 쪽에는 counting missing.rec, cannot read missing.rec, No such file or directory 카드가 사슬로 이어지고 검수 로봇이 그 사슬을 읽는다. 아래에 panic 은 불변식이 깨진 버그에만 이라고 적혀 있다

오른쪽 카드 세 장은 이 글의 프로그램이 실제로 찍은 에러 사슬입니다.

Rust 강의 7편이고 실무 단계의 첫 편입니다. [4편](https://blog.doksam.com/blog/rust-course-04-types/)에서 `thiserror` 로 에러 `enum` 을 만들었습니다. 이번에는 [5편](https://blog.doksam.com/blog/rust-course-05-parser-cli/)의 `recscan` 라이브러리 위에 파일 읽기 층과 명령줄 층을 얹어 에러를 나눕니다. 외부 크레이트는 `anyhow` 1.0.104 와 `thiserror` 2.0.21 입니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 에러를 읽는 쪽은 둘입니다: 코드와 사람

| 비교 | 라이브러리 | 애플리케이션 |
|---|---|---|
| 에러를 읽는 쪽 | 호출한 코드 | 사람(로그, 터미널) |
| 하는 일 | 경우별로 가른다. 다시 시도, 건너뛰기, 위로 올리기 | 무엇이 왜 실패했는지 보여 주고 끝낸다 |
| 알맞은 타입 | 공개 `enum` | `anyhow::Error` |
| 만드는 도구 | `thiserror` | `anyhow` 의 `context`, `bail!` |

한 저장소에 둘이 함께 있는 일이 흔합니다. 문서 변환 엔진도 라이브러리와 명령줄 도구가 한 패키지에 있습니다. 엔진 라이브러리는 4편의 `thiserror` `enum` 을 공개합니다. 명령줄 쪽은 `anyhow` 없이 `eprintln!` 과 `process::exit(1)` 로 처리합니다.

## 라이브러리는 가를 수 있는 enum 을 공개합니다

파일을 읽어 레코드를 모으는 라이브러리 함수입니다. 실패는 세 갈래입니다.

```rust
#[derive(Debug, Error)]
pub enum LoadError {
    /// 파일을 읽지 못했다. 어느 파일인지와 원인을 함께 든다.
    #[error("cannot read {path}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// 레코드 형식이 깨졌다. 원인 에러를 그대로 감싼다.
    #[error("broken record stream")]
    Parse(#[from] ParseError),

    #[error("not a record file: {0}")]
    NotRecordFile(PathBuf),
}

pub fn load_records(path: &Path) -> Result<Vec<OwnedRecord>, LoadError> {
    let data = std::fs::read(path).map_err(|source| LoadError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    // ... 빈 파일이면 NotRecordFile, 레코드가 깨지면 ? 로 Parse
}
```

`Io` 에 경로를 함께 담은 데는 이유가 있습니다. `std::io::Error` 의 메시지는 `No such file or directory (os error 2)` 뿐이고 어느 파일인지 말하지 않습니다. 경로를 아는 곳은 이 함수이니 여기서 붙입니다. 그래서 `Io` 에는 `#[from]` 대신 `#[source]` 를 씁니다. `#[from]` 은 `?` 로 자동 변환을 만들지만 경로 같은 필드를 채울 수 없습니다. `#[source]` 는 원인만 사슬에 걸고 변환은 직접 씁니다.

메시지에 원인을 다시 넣지 않은 것도 일부러입니다. `cannot read {path}` 에 `{source}` 까지 넣으면 사슬을 찍을 때 같은 문장이 두 번 나옵니다. 원인은 `source()` 가 맡게 둡니다.

## 애플리케이션은 anyhow 로 담고 context 로 문맥을 더합니다

```rust
use anyhow::{Context, Result, bail};

fn count(path: &Path) -> Result<usize> {
    let recs = load_records(path).with_context(|| format!("counting {}", path.display()))?;
    Ok(recs.len())
}

fn main() -> Result<()> {
    let args: Vec<PathBuf> = std::env::args().skip(1).map(PathBuf::from).collect();
    if args.is_empty() {
        bail!("usage: errdesign <file>...");
    }
    let mut total = 0;
    for path in &args {
        match count(path) {
            Ok(n) => total += n,
            // 깨진 파일만 골라 건너뛴다: anyhow 안의 원래 타입을 꺼내 본다
            Err(e) if matches!(
                e.downcast_ref::<LoadError>(),
                Some(LoadError::Parse(ParseError::Truncated { .. }))
            ) =>
            {
                eprintln!("warning: skip {}: {e:#}", path.display());
            }
            Err(e) => return Err(e),
        }
    }
    println!("records: {total}");
    Ok(())
}
```

`anyhow::Result<T>` 는 `Result<T, anyhow::Error>` 입니다. `std::error::Error` 를 구현한 에러라면 무엇이든 `?` 로 담깁니다. `with_context` 는 실패했을 때만 클로저를 불러 문맥을 만듭니다. 성공 경로에서는 `format!` 비용이 들지 않습니다. 없는 파일을 넘긴 결과입니다.

```console
$ errdesign good.rec missing.rec; echo "exit=$?"
Error: counting missing.rec

Caused by:
    0: cannot read missing.rec
    1: No such file or directory (os error 2)
exit=1
```

`main` 이 `Err` 를 돌려주면 표준 라이브러리가 에러를 `Debug` 형식으로 찍고 종료 코드 1로 끝냅니다. `anyhow` 의 `Debug` 형식이 위처럼 문맥과 원인 사슬을 여러 줄로 보여 줍니다. 사람은 위에서 아래로 "무엇을 하다가, 왜, 근본 원인은"을 읽습니다.

가를 필요가 생기면 `downcast_ref` 로 원래 타입을 꺼냅니다. 위 코드는 깨진 파일만 경고로 바꿔 건너뜁니다.

```console
$ errdesign good.rec broken.rec; echo "exit=$?"
warning: skip broken.rec: counting broken.rec: broken record stream: truncated at offset 0: need 4 bytes, have 3
records: 19420
exit=0
```

`{e:#}` 는 사슬을 콜론으로 이은 한 줄입니다. 로그 한 줄에 원인까지 남길 때 씁니다.

## 원인을 문자열로 뭉개면 사슬이 끊깁니다

같은 실패를 두 방식으로 감싸고 `source()` 를 끝까지 따라가 봤습니다.

```rust
/// 원인을 문자열로 뭉갠 판
pub fn load_flat(path: &Path) -> Result<usize, FlatError> {
    load_records(path)
        .map(|v| v.len())
        .map_err(|e| FlatError(e.to_string()))
}
```

```console
[#[source] 로 감싼 판]
  0: cannot read no-such.rec
  1: No such file or directory (os error 2)
[문자열로 뭉갠 판]
  0: load failed: cannot read no-such.rec
```

뭉갠 판에서는 `No such file or directory` 가 사라졌습니다. `to_string()` 은 맨 위 메시지만 옮기기 때문입니다. 원인을 가리는 실수는 장애를 볼 때 가장 비싸게 돌아옵니다. 파일이 없는지, 권한이 없는지, 디스크가 찼는지를 로그로 가를 수 없습니다.

그래도 뭉개는 쪽을 고를 때가 있습니다. 공개 `enum` 에 의존 크레이트의 에러 타입을 넣으면 그 크레이트가 내 공개 API 의 일부가 됩니다. 의존 크레이트의 메이저 버전을 올리면 내 API 도 깨집니다. 문서 변환 엔진에도 `CorruptData(String)` 같은 문자열 변형이 있고, 의존 크레이트의 타입이 공개 API 에 드러나지 않습니다. 해석하면 기준은 이렇습니다. 표준 라이브러리 에러(`io::Error`)는 `#[source]` 로 사슬에 걸고, 외부 크레이트 에러는 공개 API 에 노출할지 먼저 정합니다.

## Box<dyn Error> 는 그대로는 스레드를 넘지 못합니다

`anyhow` 없이 아무 에러나 담으려고 `Box<dyn std::error::Error>` 를 쓰는 코드가 많습니다. 그 결과를 다른 스레드에서 돌려받으려 하면 막힙니다.

```console
error[E0277]: `dyn std::error::Error` cannot be sent between threads safely
 --> src/main.rs:6:18
  |
6 |     let handle = std::thread::spawn(|| load("a.rec"));
  |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `dyn std::error::Error` cannot be sent between threads safely
  |
  = help: the trait `Send` is not implemented for `dyn std::error::Error`
```

트레이트 객체는 적힌 트레이트만 약속합니다. `dyn Error` 에는 "스레드로 보내도 된다"는 `Send` 약속이 없습니다. `Box<dyn Error + Send + Sync>` 로 적으면 통과합니다. `anyhow::Error` 는 처음부터 `Send + Sync + 'static` 인 에러만 받습니다(anyhow 소스의 `E: StdError + Send + Sync + 'static`). 그래서 스레드와 async 작업(18편) 사이에서 그대로 쓸 수 있습니다.

## panic 은 버그에만 씁니다

`Result` 는 예상할 수 있는 실패입니다. 파일이 없거나 형식이 깨진 경우입니다. `panic` 은 코드가 스스로 세운 약속이 깨진 경우입니다. 5편의 `read_u32` 는 `take(4)` 가 4바이트를 보장한 뒤 `b[0]` 부터 `b[3]` 까지 인덱스로 읽습니다. 그 인덱스가 넘친다면 입력이 아니라 코드가 틀린 것이니 패닉이 맞습니다.

| 쓰는 곳 | `unwrap`·`expect` | 이유 |
|---|---|---|
| 테스트 | 쓴다 | 실패하면 패닉이 곧 테스트 실패다 |
| 코드가 보장한 불변식 | `expect("이유")` 로 쓴다 | 깨지면 버그다. 메시지에 약속을 적는다 |
| 사용자 입력, 파일, 네트워크 | 쓰지 않는다 | 예상할 수 있는 실패다 |
| 라이브러리 공개 함수 | 쓰지 않는다 | 호출자가 대처할 기회를 빼앗는다 |

## 면접에서 이렇게 묻는다

**Q1. `thiserror` 와 `anyhow` 는 언제 쓰나요?**
라이브러리는 호출자가 경우별로 가르도록 `thiserror` 로 구체 `enum` 을 공개합니다. 애플리케이션은 대부분 사람에게 보여 주고 끝내므로 `anyhow::Error` 에 담고 `context` 를 붙입니다. 애플리케이션에서도 가를 필요가 있으면 `downcast_ref` 로 꺼냅니다.

**Q2. `#[from]` 과 `#[source]` 는 무엇이 다른가요?**
둘 다 `source()` 사슬에 원인을 겁니다. `#[from]` 은 `From` 구현도 만들어 `?` 로 자동 변환됩니다. 경로처럼 다른 필드를 함께 담아야 하면 `#[source]` 를 쓰고 `map_err` 로 직접 감쌉니다.

**Q3. 에러를 `to_string()` 으로 감싸면 무엇을 잃나요?**
원인 사슬입니다. 실험에서 `No such file or directory` 가 사라졌습니다. 대신 의존 크레이트의 타입을 공개 API 에서 숨길 수 있습니다. 표준 라이브러리 에러는 사슬로 남기고, 외부 타입은 노출 여부를 따져 정합니다.

**Q4. `Box<dyn Error>` 대신 `anyhow::Error` 를 쓰는 이유는?**
`context` 로 문맥을 쌓고, `Send + Sync` 가 보장되고, 크기가 포인터 하나(8바이트)입니다. `Box<dyn Error>` 는 16바이트이고 `Send` 가 없어 스레드를 넘지 못합니다.

**Q5. `unwrap` 은 언제 써도 되나요?**
테스트와, 코드가 보장한 불변식에서 씁니다. 불변식이면 `expect` 에 그 약속을 적습니다. 외부 입력에서 오는 실패에는 쓰지 않습니다.

## 연습

**연습 1.** 깨진 파일에 더해 빈 파일(`NotRecordFile`)도 경고만 하고 건너뛰게 `main` 을 고치세요.

<details>
<summary>풀이 보기</summary>

```rust
            Err(e) => match e.downcast_ref::<LoadError>() {
                Some(LoadError::Parse(ParseError::Truncated { .. }))
                | Some(LoadError::NotRecordFile(_)) => {
                    eprintln!("warning: skip {}: {e:#}", path.display());
                }
                _ => return Err(e),
            },
```

`good.rec broken.rec empty.rec` 를 넣으면 경고 두 줄 뒤에 `records: 19420` 이 찍히고 종료 코드는 0 입니다. 라이브러리가 경우를 `enum` 으로 나눠 두었기에 이렇게 가를 수 있습니다.

</details>

**연습 2.** 같은 `anyhow` 에러를 `{}`, `{:#}`, `{:?}` 로 찍어 차이를 확인하세요.

<details>
<summary>풀이 보기</summary>

```console
[{}]   counting missing.rec
[{:#}] counting missing.rec: cannot read missing.rec: No such file or directory (os error 2)
[{:?}]
counting missing.rec

Caused by:
    0: cannot read missing.rec
    1: No such file or directory (os error 2)
```

`{}` 는 맨 위 문맥만, `{:#}` 는 사슬 전체를 한 줄로, `{:?}` 는 여러 줄로 찍습니다. 사용자에게는 `{:#}`, 개발자 로그에는 `{:?}` 가 알맞습니다.

</details>

**연습 3.** `FlatError` 를 고쳐 사슬이 끊기지 않게 하세요.

<details>
<summary>풀이 보기</summary>

```rust
#[derive(Debug, Error)]
#[error("load failed")]
struct WrappedError(#[source] LoadError);

fn load_wrapped(path: &Path) -> Result<usize, WrappedError> {
    load_records(path).map(|v| v.len()).map_err(WrappedError)
}
```

사슬이 `0: load failed`, `1: cannot read no-such.rec`, `2: No such file or directory (os error 2)` 세 단계로 살아납니다.

</details>

## 한 단계 더 — 에러 타입의 크기와 성공 경로

`Result<T, E>` 의 크기는 `T` 와 `E` 가운데 큰 쪽을 따라갑니다. 에러 타입 크기를 쟀습니다.

| 타입 | 크기 (바이트) |
|---|---:|
| `anyhow::Error` | 8 |
| `Result<(), anyhow::Error>` | 8 |
| `Box<dyn std::error::Error>` | 16 |
| `std::io::Error` | 8 |
| `LoadError` | 32 |

anyhow 문서는 `anyhow::Error` 를 "좁은 포인터, 정확히 한 워드"로 설명합니다. 소스를 보면 vtable 을 에러와 함께 힙에 넣어(`Error::construct(error, vtable, backtrace)`) 포인터 하나로 들고 다닙니다. `Box<dyn Error>` 는 데이터와 vtable 주소 두 워드입니다(4편의 `&dyn Parser` 와 같은 구조).

에러 변형에 256바이트 배열을 그대로 넣으면 clippy 가 `large_enum_variant` 와 `result_large_err` 로 경고합니다. 그때 `Result<u32, DecodeError>` 는 260바이트였고, 배열을 `Box` 로 감싸자 16바이트가 됐습니다. 성공 경로가 느려지는지 2억 번 호출로 쟀습니다. 큰 에러 140~166ms, 작은 에러 136~138ms 로 차이가 거의 없었습니다. 해석하면 이 호출에서는 성공할 때 꼬리표와 `u32` 만 쓰고 260바이트 전체를 옮기지 않습니다. 에러가 자주 나고 여러 층을 `?` 로 거쳐 옮겨지는 코드라면 결과가 다를 수 있습니다. 그 경우는 이번에 재지 않았습니다. 그래서 clippy 경고는 "반드시 느리다"가 아니라 "크기를 확인하라"는 신호로 읽습니다.

## 정리

에러 설계의 기준은 누가 읽느냐입니다. 코드가 읽으면 `enum`, 사람이 읽으면 `anyhow` 와 `context` 입니다. 어느 쪽이든 원인은 `source` 사슬로 남깁니다. 다음에 `map_err(|e| ... e.to_string())` 을 쓰려 할 때는 그 원인을 정말 버려도 되는지 먼저 봅니다.

8편에서는 컬렉션과 이터레이터를 다룹니다. `Vec`·`HashMap`·`BTreeMap` 을 고르는 기준과, 이터레이터가 손으로 쓴 반복문만큼 빠른지 직접 잽니다.
