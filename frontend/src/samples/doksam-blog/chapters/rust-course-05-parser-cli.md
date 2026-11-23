# Rust 강의 5편 — 실전: 파일을 뜯는 파서와 CLI

> 남이 만든 바이너리 파일을 읽는 도구를 만들 때, 무엇이 망가진 입력에서 프로그램을 지켜 줄까?

답부터 적습니다. 바이트를 인덱스로 바로 자르지 않고, 길이를 확인해 모자라면 패닉 대신 에러를 돌려주는 습관입니다. 파서를 `Iterator` 로 만들면 `filter`·`count`·`collect` 와 `rayon` 이 그대로 붙습니다. 명령줄은 `clap` 의 derive 로 만듭니다. 병렬로 돌릴 때는 스레드끼리 변수를 함께 고치는 코드를 컴파일러가 막아 줍니다.

> [그림] 검수 로봇이 돋보기로 바이트 띠의 첫 머리 칸을 읽는다. 띠는 머리와 본문이 번갈아 이어지다 끝의 잘린 조각에서 에러로 멈춤 도장이 찍혀 있다. 아래에서 rayon 이 파일 상자를 세 줄로 나눠 나르고 collect 상자에 순서대로 모은다

그림 아래 숫자는 이 글의 도구로 파일 32개(256MB)를 센 시간입니다. 직렬로 0.194초, 병렬로 0.035초였습니다.

Rust 강의 5편입니다. [4편](https://blog.doksam.com/blog/rust-course-04-types/)의 타입으로 실제 도구 `recscan` 을 만듭니다. 외부 크레이트는 `clap` 4.6.8(derive), `rayon` 1.12.0, `thiserror` 2.0.21 입니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 머리 4바이트에 태그·레벨·크기가 들어 있습니다

다룰 형식은 문서 변환 엔진이 HWP 5.0 본문을 읽을 때 쓰는 레코드 구조입니다. 레코드마다 머리 4바이트(리틀 엔디언 `u32`)가 있습니다. 아래 10비트가 태그(레코드 종류), 다음 10비트가 레벨(깊이), 위 12비트가 본문 크기입니다. 크기가 `0xFFF` 면 다음 4바이트가 실제 크기입니다. 크기가 본문 앞에 있으니, 읽는 쪽은 머리를 믿고 그만큼 잘라야 합니다. 망가진 파일에서는 이 크기가 거짓말을 합니다.

## 인덱스로 자르면 망가진 파일 하나에 프로그램이 죽습니다

머리를 인덱스로 읽는 가장 짧은 코드에 3바이트짜리 입력을 넣었습니다.

```rust
fn first_size(data: &[u8]) -> usize {
    let header = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    (header >> 20) as usize
}
```

```console
thread 'main' (30443556) panicked at src/main.rs:3:65:
index out of bounds: the len is 3 but the index is 3
```

Rust 는 인덱스마다 범위를 검사하고 넘으면 패닉합니다. 메모리는 안전하지만 프로그램은 죽습니다. 파일 1,000개를 처리하던 서버가 파일 하나 때문에 죽어도 장애입니다. 문서 변환 엔진은 `while pos + 4 <= data.len()` 처럼 길이를 먼저 확인하고, 모자라면 읽기를 멈춥니다.

## 파서를 Iterator 로 만들면 나머지 도구가 그대로 붙습니다

이 글의 파서는 레코드를 하나씩 돌려주는 이터레이터입니다. 본문은 복사하지 않고 입력을 빌립니다(3편의 `Record<'a>`).

```rust
pub struct Records<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Records<'a> {
    /// pos 에서 n 바이트를 빌린다. 모자라면 에러.
    fn take(&mut self, n: usize) -> Result<&'a [u8], ParseError> {
        let rest = &self.data[self.pos..]; // pos <= len 은 늘 지켜진다
        let bytes = rest.get(..n).ok_or(ParseError::Truncated {
            offset: self.pos,
            need: n,
            have: rest.len(),
        })?;
        self.pos += n;
        Ok(bytes)
    }

    fn read_u32(&mut self) -> Result<u32, ParseError> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    fn next_record(&mut self) -> Result<Record<'a>, ParseError> {
        let header = self.read_u32()?;
        let tag = (header & 0x3FF) as u16;
        let level = ((header >> 10) & 0x3FF) as u16;
        let mut size = (header >> 20) as usize;
        if size == 0xFFF {
            size = self.read_u32()? as usize;
        }
        let body = self.take(size)?;
        Ok(Record { tag, level, body })
    }
}

impl<'a> Iterator for Records<'a> {
    type Item = Result<Record<'a>, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.data.len() {
            return None;
        }
        let item = self.next_record();
        if item.is_err() {
            self.pos = self.data.len(); // 에러 뒤에는 멈춘다
        }
        Some(item)
    }
}
```

바이트를 자르는 곳은 `take` 한 군데뿐입니다. `get(..n)` 은 모자라면 `None` 을 주고, `ok_or` 가 이를 에러로 바꿉니다. `b[0]` 같은 인덱스는 `take(4)` 가 4바이트를 보장한 뒤에만 씁니다. 경계 검사를 한 함수에 모으면 나머지 코드는 경계를 신경 쓰지 않아도 됩니다.

`Iterator` 를 구현하려면 `type Item` 과 `next()` 만 정하면 됩니다. `map`·`filter`·`count` 같은 나머지 메서드는 기본 구현으로 따라옵니다. 텍스트 레코드 수는 `records(&data).map_while(Result::ok).filter(|r| r.tag == TAG_TEXT).count()` 한 줄입니다. `collect::<Result<Vec<_>, _>>()` 는 첫 에러에서 멈추고 그 에러를 돌려줍니다. 6바이트로 자른 입력은 `Truncated { offset: 4, need: 4, have: 2 }` 를 냈습니다. 머리 4바이트는 읽었는데 본문 4바이트 가운데 2바이트만 있다는 뜻입니다.

## UTF-16 은 코드 단위가 아니라 글자로 풉니다

텍스트 레코드의 본문은 UTF-16LE 입니다. 2바이트씩 끊어 `char::from_u32` 로 바꾸면 거의 맞습니다. 그런데 U+10000 이상의 글자는 2바이트 두 개(서로게이트 쌍)로 적힙니다. 옛 문서에 나오는 확장 한자 `𠀀`(U+20000)가 그렇습니다.

```rust
pub fn decode_utf16le(body: &[u8]) -> String {
    let (pairs, _odd) = body.as_chunks::<2>();
    let units = pairs.iter().map(|&p| u16::from_le_bytes(p));
    char::decode_utf16(units)
        .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
        .collect()
}
```

`"한𠀀"` 을 넣으면 `char::decode_utf16` 판은 `"한𠀀"` 을, 코드 단위마다 `char::from_u32` 를 부르는 판은 `"한"` 만 돌려줬습니다. 서로게이트 하나하나는 올바른 `char` 가 아니라서 `from_u32` 가 `None` 을 주고 조용히 빠집니다. 테스트가 없으면 모르고 지나갈 버그입니다.

처음에는 `body.chunks_exact(2)` 로 썼는데 clippy 1.100.0 이 `as_chunks::<2>()` 를 권했습니다. `[u8; 2]` 배열 조각을 받아 인덱스 검사가 필요 없습니다. 새 API 를 clippy 가 먼저 알려 준 셈입니다.

## clap 은 타입 정의 하나로 명령줄을 만듭니다

```rust
#[derive(Parser)]
#[command(name = "recscan", version, about = "레코드 스트림 파일을 훑는다")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 레코드 목록을 찍는다
    #[command(alias = "ls")]
    List {
        /// 읽을 파일
        file: PathBuf,
        /// 이 태그만 찍는다
        #[arg(long)]
        tag: Option<u16>,
    },
    /// 여러 파일의 레코드 수와 글자 수를 센다
    Count {
        /// 읽을 파일들
        files: Vec<PathBuf>,
        /// 병렬로 돌리지 않는다
        #[arg(long)]
        serial: bool,
        /// 실패한 파일이 있어도 종료 코드 0
        #[arg(short = 'k', long)]
        keep_going: bool,
    },
}
```

`///` 주석이 도움말이 되고, 필드 타입이 인자 규칙이 됩니다. `Option<u16>` 은 생략할 수 있는 숫자, `Vec<PathBuf>` 는 여러 개, `bool` 은 플래그입니다. 문서 변환 엔진의 CLI 도 같은 모양으로 하위 명령 여섯 개와 `cv`·`dt` 같은 별칭을 둡니다.

```console
$ recscan count --help
여러 파일의 레코드 수와 글자 수를 센다

Usage: recscan count [OPTIONS] [FILES]...

Arguments:
  [FILES]...  읽을 파일들

Options:
      --serial      병렬로 돌리지 않는다
  -k, --keep-going  실패한 파일이 있어도 종료 코드 0
  -h, --help        Print help
```

필수 인자를 빼면 clap 이 사용법을 찍고 종료 코드 2로 끝냅니다.

하나 조심할 것이 있습니다. `recscan ls 파일 | head -2` 처럼 읽는 쪽이 먼저 닫히면 `println!` 이 `failed printing to stdout: Broken pipe` 로 패닉합니다. 예제는 출력에 `writeln!` 을 쓰고 `ErrorKind::BrokenPipe` 를 받으면 조용히 끝냅니다.

## 테스트는 함수 단위와 실행 파일 단위로 나눕니다

함수 단위 테스트는 1편처럼 같은 파일의 `#[cfg(test)]` 모듈에 둡니다. 실행 파일 전체는 `tests/` 폴더의 통합 테스트로 시험합니다. cargo 는 통합 테스트를 빌드할 때 실행 파일 경로를 `CARGO_BIN_EXE_<이름>` 환경 변수로 알려 줍니다.

```rust
fn recscan() -> Command {
    Command::new(env!("CARGO_BIN_EXE_recscan"))
}
```

예제의 통합 테스트는 정상 파일과 3바이트 파일을 함께 넘깁니다. 종료 코드 1, 표준 에러의 `truncated at offset 0: need 4 bytes, have 3`, `-k` 를 주면 종료 코드 0 인지를 봅니다. 문서 변환 엔진도 출력 경로 규칙을 이 방식으로 고정합니다. 테스트가 늘면 좁혀 돌립니다. `cargo test records` 는 이름에 `records` 가 든 테스트만, `cargo test --test cli` 는 `tests/cli.rs` 만 돌립니다. 에이전트에게 "고친 곳의 테스트만 먼저"를 시키는 근거가 이 기능입니다(6편).

## rayon 은 iter 를 par_iter 로 바꾸면 됩니다

```rust
            let results: Vec<_> = if serial {
                files.iter().map(|f| count_file(f)).collect()
            } else {
                files.par_iter().map(|f| count_file(f)).collect()
            };
```

차이는 `iter` 와 `par_iter` 뿐입니다. 파일 32개(각 8MB)를 일곱 번씩 센 중앙값입니다. 프로세스 시작과 256MB 읽기가 포함된 시간입니다.

| 방식 | 중앙값 | 최솟값 |
|---|---:|---:|
| `--serial` | 0.194초 | 0.191초 |
| `par_iter` (14코어) | 0.035초 | 0.032초 |

5.5배 빨라졌습니다. 출력 순서도 직렬과 똑같았습니다(`diff` 로 확인). 일은 섞여 돌아도 `collect` 가 결과를 원래 순서대로 모으기 때문입니다. rayon 문서의 `collect_into_vec` 예시도 `(0..5)` 를 `[0, 1, 2, 3, 4]` 로 모읍니다.

실패 수를 셀 때 흔히 이렇게 씁니다. 컴파일러가 막습니다.

```console
error[E0594]: cannot assign to `failures`, as it is a captured variable in a `Fn` closure
  --> src/main.rs:8:13
...
 8 | |             failures += 1;
   | |             ^^^^^^^^^^^^^ cannot assign
...
10 | |     });
   | |______- expects `Fn` instead of `FnMut`
```

`for_each` 의 클로저는 여러 스레드에서 동시에 불립니다. 그래서 rayon 은 바깥 변수를 고치지 않는 `Fn` 클로저만 받습니다. 여러 스레드가 `failures += 1` 을 동시에 하면 데이터 경쟁입니다. 다른 언어라면 가끔 틀린 숫자가 나오는 버그가 이 코드에서는 컴파일 오류입니다. 클로저 세 종류는 9편, `Send`·`Sync` 는 17편에서 다룹니다.

## 면접에서 이렇게 묻는다

**Q1. 신뢰할 수 없는 바이트를 파싱할 때 인덱싱 대신 무엇을 쓰나요?**
`get()` 이나 길이 확인 뒤 자르기를 씁니다. 인덱싱은 메모리는 지키지만 패닉으로 프로세스를 멈춥니다. 공격자가 망가진 파일로 서비스를 멈출 수 있습니다. 경계 검사를 한 함수에 모으면 빠뜨림이 줄어듭니다.

**Q2. `Iterator` 를 직접 구현하려면 무엇이 필요한가요?**
`type Item` 과 `fn next(&mut self) -> Option<Self::Item>` 하나입니다. 나머지 어댑터는 기본 메서드로 따라옵니다. 파싱 에러는 `Item = Result<T, E>` 로 실어 보냅니다.

**Q3. rayon 은 데이터 경쟁을 어떻게 막나요?**
병렬 메서드가 `Fn + Send + Sync` 클로저를 요구합니다. 바깥 변수를 고치는 클로저는 `FnMut` 라 E0594 로 막힙니다. 공유 카운터는 `AtomicU32` 를 쓰고, 결과는 `map` 과 `collect` 로 모읍니다.

**Q4. `par_iter().map().collect()` 는 순서를 지키나요?**
`Vec` 으로 모으면 지킵니다. 32개 파일의 출력이 직렬과 같았습니다. 실행 순서는 섞이고, 모으는 자리만 원래 순서입니다.

## 연습

**연습 1.** 태그별 레코드 수를 세는 `tag_histogram(data: &[u8]) -> Result<BTreeMap<u16, usize>, ParseError>` 를 쓰세요. 에러가 나면 그 에러를 돌려줍니다.

<details>
<summary>풀이 보기</summary>

```rust
pub fn tag_histogram(data: &[u8]) -> Result<BTreeMap<u16, usize>, ParseError> {
    let mut counts = BTreeMap::new();
    for rec in records(data) {
        *counts.entry(rec?.tag).or_insert(0) += 1;
    }
    Ok(counts)
}
```

`rec?` 가 에러를 그대로 올립니다. `BTreeMap` 은 키 순서로 돌아서 출력이 늘 같습니다. `HashMap` 이면 실행마다 순서가 달라질 수 있습니다.

</details>

**연습 2.** `decode_utf16le` 는 홀수 길이의 마지막 바이트를 버립니다. 버리지 말고 에러를 내는 `decode_utf16le_strict` 를 쓰세요.

<details>
<summary>풀이 보기</summary>

```rust
pub fn decode_utf16le_strict(body: &[u8]) -> Result<String, ParseError> {
    if !body.len().is_multiple_of(2) {
        return Err(ParseError::Truncated {
            offset: body.len() - 1,
            need: 2,
            have: 1,
        });
    }
    Ok(decode_utf16le(body))
}
```

`% 2 != 0` 으로 쓰면 clippy 가 `is_multiple_of` 를 권합니다. 테스트에서 `[0x41, 0x00, 0x42]` 는 에러, `[0x41, 0x00]` 은 `"A"` 였습니다.

</details>

**연습 3.** E0594 가 난 코드를 고쳐, 읽지 못한 파일 수를 병렬로 세세요.

<details>
<summary>풀이 보기</summary>

```rust
    let failures = AtomicU32::new(0);
    files.par_iter().for_each(|f| {
        if std::fs::read(f).is_err() {
            eprintln!("error: {f}");
            failures.fetch_add(1, Ordering::Relaxed);
        }
    });
    println!("{} file(s) failed", failures.load(Ordering::Relaxed));
```

원자 정수는 `&self` 로 값을 바꾸므로 `Fn` 클로저 안에서도 씁니다. 수만 세면 되니 가장 느슨한 `Relaxed` 순서로 충분합니다. 문서 변환 엔진의 변환 명령이 정확히 이 모양입니다. 없는 파일 둘을 넣으면 `2 file(s) failed` 가 나옵니다.

</details>

## 한 단계 더 — 빌린 레코드가 복사한 레코드보다 느렸던 이유

3편에서 엔진은 본문을 `to_vec()` 으로 복사한다고 했습니다. 빌리는 쪽이 당연히 빠를 것 같아 쟀습니다. 측정마다 21번 돌린 값입니다(릴리스 빌드).

| 측정 | 256MB 전체 (중앙값) | 캐시에 드는 1MB × 256번 (중앙값) |
|---|---:|---:|
| 빌림: 머리만 훑기 | 36.0ms | 2.1ms |
| 빌림: 본문까지 읽기 | 11.8ms | 재지 않음 |
| 복사: `to_vec` | 18.2ms | 17.9ms |

256MB 에서는 머리만 훑는 쪽이 복사보다 두 배 느렸습니다. 본문 바이트를 모두 더하는, 일이 더 많은 쪽은 오히려 가장 빨랐습니다. 같은 양을 캐시에 드는 1MB 로 나눠 돌리면 빌림이 복사보다 8.5배 빨랐습니다.

해석은 이렇습니다. 다음 머리의 위치는 지금 머리를 읽어야 압니다. 머리만 훑으면 840바이트씩 건너뛰며 메모리에서 읽기를 기다리는 사슬이 됩니다. 본문을 순서대로 읽거나 복사하면 하드웨어 프리페처(prefetcher, 다음에 읽을 메모리를 미리 가져오는 장치)가 다음 머리까지 캐시에 올려 둡니다. 데이터가 캐시에 있으면 기다림이 사라지고, 일이 적은 빌림이 이깁니다. 이 해석은 위 세 측정에서 끌어낸 추론이고, 하드웨어 카운터로 확인하지는 않았습니다.

그래서 "빌림이 복사보다 빠르다"는 규칙은 메모리 접근 모양에 따라 뒤집힙니다. 엔진처럼 본문을 결국 다 읽는 파서라면 복사 비용은 생각보다 작습니다. 수명 인자를 다는 비용과 견줘 고를 문제입니다(21편).

## 정리

망가진 입력에서 프로그램을 지키는 것은 경계 검사를 한 곳에 모으고 패닉 대신 에러를 돌려주는 습관입니다. 그 위에 `Iterator`, `clap`, `rayon` 을 얹으면 도구 하나가 완성됩니다. 병렬로 바꿀 때 공유 변수를 고치는 실수는 컴파일러가 막습니다.

6편에서는 이 도구를 밖으로 내보냅니다. 릴리스 프로파일로 실행 파일 크기와 빌드 시간을 재고, Python·Node 바인딩의 구조를 보고, 에이전트와 일하는 주기를 정합니다.
