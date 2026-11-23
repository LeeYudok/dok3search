# Rust 강의 16편 — 트레이트 깊게: 디스패치, 연관 타입, dyn 호환성

> 제네릭과 트레이트 객체는 실행 파일 안에서 무엇이 다르고, 트레이트를 설계할 때 무엇을 정해야 할까?

답부터 적습니다. 제네릭 함수는 쓰인 타입마다 복사본이 만들어집니다(모노모피제이션). 32가지 타입으로 부른 함수의 심볼이 실행 파일에 32개 있었고, 같은 일을 `dyn` 으로 하면 1개였습니다. 크기는 392,608 대 359,648 바이트였습니다. 복사본은 직접 호출이라 인라인되고, `dyn` 은 vtable 을 거쳐 부릅니다. 설계할 때는 세 가지를 정합니다. 타입마다 결과가 하나로 정해지면 연관 타입을, 여러 개면 제네릭 인자를 씁니다. `dyn` 으로 쓸 트레이트는 vtable 을 만들 수 있게 메서드를 고릅니다. 남의 타입에 남의 트레이트는 붙일 수 없으니 새 타입으로 감쌉니다.

> [그림] 왼쪽에는 summarize 종이가 타입마다 한 장씩 32장 쌓여 실행 파일 392,608 B, 오른쪽에는 summarize_dyn 종이 한 장이 tag 와 text 칸이 있는 vtable 을 가리켜 359,648 B 이다. 가운데 검수 로봇이 두 쪽을 비교하고 아래 띠에 호출 방식의 차이가 적혀 있다

왼쪽 더미의 장수와 두 크기는 이 글의 실험에서 잰 값입니다.

Rust 강의 16편입니다. [4편](https://blog.doksam.com/blog/rust-course-04-types/)에서 `&dyn Parser` 가 16바이트이고 제네릭과 디스패치 방식이 다르다고만 했습니다. [9편](https://blog.doksam.com/blog/rust-course-09-closures/)에서는 호출 비용을 쟀습니다. 이번 편은 크기와 설계를 봅니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## 제네릭은 타입마다 복사본을 만듭니다

블록 형식 32가지가 같은 트레이트를 구현합니다. 매크로로 구조체 32개를 만들었습니다.

```rust
pub trait Block {
    fn tag(&self) -> u16;
    fn text(&self) -> &str;
}

/// 제네릭 판: 타입마다 복사본이 만들어진다.
#[inline(never)]
pub fn summarize_generic<B: Block>(items: &[B]) -> String {
    let mut out = String::new();
    let mut chars = 0usize;
    for (i, b) in items.iter().enumerate() {
        chars += b.text().chars().count();
        if i < 3 {
            out.push_str(&format!("[{}] {}\n", b.tag(), b.text().trim()));
        }
    }
    out.push_str(&format!("blocks={} chars={}\n", items.len(), chars));
    out
}
```

`dyn` 판은 `items: &[&dyn Block]` 을 받고 몸통은 같습니다. 한 실행 파일은 제네릭 판을 32가지 타입으로 한 번씩 부르고, 다른 실행 파일은 `dyn` 판을 32번 부릅니다. 출력은 둘 다 `918` 로 같았습니다.

| 실행 파일 | `summarize` 심볼 수 | 크기 (strip 후) |
|---|---:|---:|
| 제네릭 판 32가지 | 32 | 392,608 B |
| `dyn` 판 | 1 | 359,648 B |

심볼은 strip 하지 않은 빌드에서 `nm` 으로 셌습니다. 복사본 32개가 33KB 남짓을 더했습니다. 한 복사본에 1KB 꼴입니다. 컴파일러가 `B` 자리에 실제 타입을 넣은 함수를 따로 만든다는 것이 모노모피제이션(monomorphization)입니다. The Book 10.1절도 제네릭 코드가 구체 타입마다 펼쳐진다고 설명합니다.

해석하면 이렇습니다. 복사본마다 `b.tag()` 가 어떤 함수인지 컴파일할 때 알기 때문에 직접 부르고 인라인할 수 있습니다. 9편에서 제네릭 클로저 호출이 28ms, `&dyn Fn` 이 67ms 였던 이유입니다. 대신 타입이 늘수록 실행 파일과 컴파일 시간이 늘어납니다. 문서 변환 엔진처럼 형식이 수십 가지면 이 비용이 커집니다. 엔진이 파서를 `Box<dyn Parser>` 로 고르는 것(4편)은 이 균형에서 크기와 컴파일 시간 쪽을 고른 셈입니다.

## dyn 은 데이터 주소와 vtable 주소를 함께 듭니다

`&B00` 은 8바이트, `&dyn Block` 과 `Box<dyn Block>` 은 16바이트였습니다. 늘어난 8바이트는 vtable 주소입니다. vtable 은 그 타입의 `tag`, `text` 구현과 크기·`drop` 정보를 담은 표입니다. The Rust Reference 는 트레이트 객체를 이런 "넓은 포인터"로 설명합니다. 그래서 서로 다른 타입을 한 `Vec<Box<dyn Block>>` 에 담아 같은 메서드로 부를 수 있습니다. 실행에서 `B00` 과 `B01` 을 섞은 벡터의 태그가 `[0, 1]` 로 나왔습니다.

## 모든 트레이트가 dyn 이 되지는 않습니다

`dyn` 으로 쓰려면 vtable 을 만들 수 있어야 합니다. 4편의 `Parser` 에 두 메서드를 더하자 막혔습니다.

```rust
trait Parser {
    fn filter(&self, input: &[u8]) -> String;
    fn boxed_clone(&self) -> Self;
    fn with_option<T: ToString>(&self, opt: T) -> String;
}
```

```console
error[E0038]: the trait `Parser` is not dyn compatible
...
 3 |     fn boxed_clone(&self) -> Self;
   |                              ^^^^ ...because method `boxed_clone` references the `Self` type in its return type
 4 |     fn with_option<T: ToString>(&self, opt: T) -> String;
   |        ^^^^^^^^^^^ ...because method `with_option` has generic type parameters
```

두 이유 모두 vtable 칸을 정할 수 없어서입니다. `Self` 를 돌려주면 크기를 모르는 값을 돌려줘야 하고, 제네릭 메서드는 `T` 마다 복사본이 필요해 칸 수를 정할 수 없습니다. 예전 오류 문구는 "object safe"였고, rustc 1.100.0 은 "dyn compatible"이라고 씁니다.

고치는 길은 그 메서드를 vtable 에서 빼는 것입니다.

```rust
trait Parser {
    fn filter(&self, input: &[u8]) -> String;
    fn boxed_clone(&self) -> Self
    where
        Self: Sized;
    fn with_option<T: ToString>(&self, opt: T) -> String
    where
        Self: Sized;
}
```

`where Self: Sized` 가 붙은 메서드는 구체 타입에서만 부를 수 있습니다. 이렇게 고치자 `Vec<Box<dyn Parser>>` 가 컴파일됐고, `dyn` 으로는 `filter` 를, 구체 타입 `Txt` 로는 `with_option(42)` 를 불렀습니다.

## 결과가 하나면 연관 타입, 여럿이면 제네릭 인자입니다

바이트를 무언가로 푸는 트레이트를 두 모양으로 쓸 수 있습니다.

```rust
trait Decoder {
    type Output;
    fn decode(&self, b: &[u8]) -> Self::Output;
}

trait DecodeInto<T> {
    fn decode(&self, b: &[u8]) -> T;
}
```

연관 타입은 "이 타입의 출력은 하나"라는 약속입니다. 같은 타입에 두 번 구현하면 막힙니다.

```console
error[E0119]: conflicting implementations of trait `Decoder` for type `Le`
```

제네릭 인자는 여러 번 구현할 수 있습니다. `impl DecodeInto<u32> for Le` 와 `impl DecodeInto<String> for Le` 가 함께 섭니다. 대신 부르는 쪽이 무엇을 원하는지 알려야 합니다.

```console
error[E0283]: type annotations needed
...
note: multiple `impl`s satisfying `Le: DecodeInto<_>` found
```

`let n: u32 = Le.decode(...)` 처럼 받는 타입을 적거나 `<Le as DecodeInto<u32>>::decode(...)` 로 고르면 됩니다. 표준 라이브러리에서 `Iterator` 는 연관 타입 `Item` 을, `From<T>` 는 제네릭 인자를 씁니다. 5편의 `Records` 가 내놓는 것은 하나라서 `type Item` 이었고, 한 타입이 `From<io::Error>` 와 `From<ParseError>` 를 모두 구현하는 것(4편)은 제네릭 인자라 가능했습니다.

## 남의 타입에 남의 트레이트는 붙일 수 없습니다

`Vec<u16>` 를 `{}` 로 찍고 싶어 `Display` 를 구현하면 막힙니다.

```console
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> src/main.rs:3:1
  |
3 | impl fmt::Display for Vec<u16> {
  | ^^^^^^^^^^^^^^^^^^^^^^--------
  |                       |
  |                       `Vec` is not defined in the current crate
...
  = note: define and implement a trait or new type instead
```

고아 규칙(orphan rule)입니다. 트레이트와 타입 중 하나는 내 크레이트 것이어야 합니다. 두 크레이트가 같은 조합을 각자 구현해 충돌하는 일을 막습니다. 컴파일러가 권하는 대로 새 타입으로 감쌉니다.

```rust
struct Tags(Vec<u16>);

impl fmt::Display for Tags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} tags", self.0.len())
    }
}
```

`Tags(vec![66, 67])` 가 `2 tags` 로 찍혔습니다.

## 면접에서 이렇게 묻는다

**Q1. 정적 디스패치와 동적 디스패치는 어떻게 다른가요?**
정적은 제네릭이 타입마다 복사본으로 펼쳐져 직접 호출되고 인라인됩니다. 동적은 `dyn` 이 vtable 을 거쳐 간접 호출합니다. 실측에서 32타입 제네릭은 심볼 32개·392,608 B, `dyn` 은 1개·359,648 B 였고, 호출은 28ms 대 67ms(9편)였습니다.

**Q2. 모노모피제이션의 비용은?**
타입마다 함수 복사본이 생겨 실행 파일과 컴파일 시간이 늘어납니다. 실측에서 복사본 하나가 1KB 꼴을 더했습니다. 타입이 많고 호출이 뜨겁지 않은 곳은 `dyn` 이 낫습니다.

**Q3. 연관 타입과 제네릭 인자는 언제 고르나요?**
구현 타입마다 결과 타입이 하나로 정해지면 연관 타입(`Iterator::Item`), 한 타입이 여러 결과를 가질 수 있으면 제네릭 인자(`From<T>`) 입니다. 연관 타입에 두 번 구현하면 E0119, 제네릭 인자는 호출할 때 E0283 처럼 타입을 적어야 할 수 있습니다.

**Q4. dyn 호환성(객체 안전성) 규칙은?**
vtable 을 만들 수 있어야 합니다. `Self` 를 값으로 돌려주거나 제네릭 메서드가 있으면 E0038 입니다. 그런 메서드에 `where Self: Sized` 를 붙여 vtable 에서 빼면 나머지로 `dyn` 을 씁니다.

**Q5. 고아 규칙이란?**
트레이트나 타입 중 하나는 지금 크레이트에 있어야 구현할 수 있다는 규칙입니다(E0117). 크레이트 사이의 구현 충돌을 막습니다. 새 타입(newtype)으로 감싸 피합니다.

## 연습

**연습 1.** E0038 이 난 `Parser` 를 고쳐 `Vec<Box<dyn Parser>>` 에 담으세요. 고친 뒤 `dyn Parser` 로는 무엇을 부를 수 없나요?

<details>
<summary>풀이 보기</summary>

`boxed_clone` 과 `with_option` 에 `where Self: Sized` 를 붙입니다(본문 코드). 이 두 메서드는 `dyn Parser` 로는 부를 수 없고 `Txt` 같은 구체 타입으로만 부릅니다. 복제가 `dyn` 으로도 필요하면 `fn box_clone(&self) -> Box<dyn Parser>` 처럼 크기가 정해진 `Box` 를 돌려주는 메서드를 따로 둡니다.

</details>

**연습 2.** `Vec<u16>` 를 `{}` 로 찍을 수 있게 하세요.

<details>
<summary>풀이 보기</summary>

본문의 `Tags` 입니다. 감싼 값은 `self.0` 으로 꺼냅니다. 감싼 타입에서 `Vec` 의 메서드를 그대로 쓰고 싶으면 `Deref<Target = Vec<u16>>` 을 구현할 수 있습니다. 다만 Rust API 가이드라인(C-DEREF)은 `Deref` 를 스마트 포인터에만 구현하라고 권합니다. 필요한 메서드만 감싸 여는 편이 명확합니다.

</details>

**연습 3.** 5편의 `Records` 가 `Iterator<Item = Result<Record, ParseError>>` 대신 `Iterator<Record>` 처럼 제네릭 인자였다면 무엇이 불편했을까요?

<details>
<summary>풀이 보기</summary>

한 타입이 여러 `Item` 으로 구현될 수 있으니 `for rec in records(&data)` 마다 어떤 `Item` 인지 정해야 해서 타입 표기가 늘어납니다. `map`·`filter` 같은 어댑터도 `Item` 이 하나로 정해져야 체인을 이어 갈 수 있습니다. 이터레이터의 출력은 타입마다 하나라서 연관 타입이 맞습니다.

</details>

## 한 단계 더 — 제네릭과 dyn 사이의 다리

실무에서는 둘을 섞습니다. 바깥 API 는 제네릭으로 받아 쓰기 편하게 하고, 안쪽의 큰 몸통은 `dyn` 으로 하나만 둡니다.

```rust
pub fn summarize<B: Block>(items: &[B]) -> String {
    let refs: Vec<&dyn Block> = items.iter().map(|b| b as &dyn Block).collect();
    summarize_dyn(&refs)
}
```

제네릭 쪽에는 짧은 변환만 남아 복사본이 커지지 않고, 무거운 몸통은 하나입니다. 블록 둘을 넘기자 `blocks=2 chars=7` 로 같은 요약이 나왔습니다. 이 패턴이 실제로 크기를 얼마나 줄이는지는 이번에 재지 않았습니다. 판단 기준은 몸통의 크기와 타입의 수, 그리고 그 호출이 뜨거운 반복 안에 있는지입니다.

## 정리

제네릭은 타입마다 복사본을 만들어 빠르게 부르고, `dyn` 은 vtable 하나로 작게 부릅니다. 트레이트를 설계할 때는 결과가 하나인지(연관 타입), `dyn` 으로 쓸지(호환성), 누구의 타입인지(고아 규칙)를 정합니다.

17편에서는 동시성을 다룹니다. 이 연재에서 여러 번 나온 `Send` 와 `Sync` 가 무엇을 약속하는지, 스레드와 채널, 범위 스레드로 데이터 경쟁 없는 코드를 쓰는 법을 봅니다.
