# Rust 강의 20편 — 매크로: macro_rules! 와 derive

> 매크로는 언제 만들고, 만든 매크로가 무엇을 찍어 내는지는 어떻게 확인할까?

답부터 적습니다. 함수와 제네릭으로 줄일 수 없는 반복, 예를 들어 "형식 목록이 늘 때마다 `enum` 변형과 `match` 갈래를 함께 늘리는 일"에 씁니다. 패턴으로 충분하면 `macro_rules!`(선언적 매크로), 구조체의 모양을 읽어 코드를 만들어야 하면 derive 같은 절차적 매크로입니다. 찍어 낸 결과는 nightly 의 `-Zunpretty=expanded` 로 펼쳐 봅니다. 값은 둘입니다. 읽는 사람이 펼친 모양을 머릿속으로 그려야 하고, 절차적 매크로는 `syn`·`quote` 를 빌드하느라 빈 빌드에 1.7초 안팎을 더했습니다.

> [그림] 왼쪽 카드의 formats! 호출이 가운데 macro_rules! 와 derive(Describe) 틀을 지나 오른쪽 종이에 enum Format, fn name, impl DocInfo 코드로 찍혀 나온다. 검수 로봇이 -Zunpretty=expanded 로 펼친 종이를 본다. 오른쪽 아래에 선언적과 절차적 매크로의 차이가 적혀 있다

오른쪽 종이의 코드는 이 글의 매크로를 실제로 펼친 결과를 줄인 것입니다.

Rust 강의 20편입니다. 이 연재는 처음부터 매크로를 써 왔습니다. `println!`, `vec!`, `#[derive(Debug)]`, [4편](https://blog.doksam.com/blog/rust-course-04-types/)의 `thiserror`, [16편](https://blog.doksam.com/blog/rust-course-16-traits-deep/)의 `blocks!` 입니다. 이번에는 직접 만듭니다. 코드는 2026-11-18 에 rustc 1.100.0 으로, 펼치기는 nightly 1.102.0 으로 돌렸습니다. 절차적 매크로는 `syn` 3.0.10, `quote` 1.0.47 을 씁니다.

## macro_rules! 는 패턴을 맞춰 코드를 찍습니다

4편에서 형식을 하나 더하면 `enum` 과 `name()` 의 `match` 를 함께 고쳐야 했습니다. 목록 하나에서 둘을 찍어 내면 빠뜨릴 일이 없습니다.

```rust
macro_rules! formats {
    ($($variant:ident => $name:literal),+ $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum Format {
            $($variant),+
        }

        impl Format {
            fn name(&self) -> &'static str {
                match self {
                    $(Format::$variant => $name),+
                }
            }

            const ALL: &'static [Format] = &[$(Format::$variant),+];
        }
    };
}

formats! {
    Pdf => "PDF",
    Hwp5 => "HWP 5.0",
    Hwpx => "HWPX",
    Docx => "DOCX",
}
```

`$variant:ident` 는 이름 하나, `$name:literal` 은 리터럴 하나를 받습니다. `$(...),+` 는 쉼표로 구분된 반복이고, 찍어 낼 때 `$(...),+` 로 같은 횟수만큼 펼칩니다. 실행하면 `Pdf → PDF` 부터 `Docx → DOCX` 까지 네 줄이 나왔습니다.

nightly 로 펼친 결과의 일부입니다.

```console
$ cargo +nightly rustc -p demo -- -Zunpretty=expanded
enum Format { Pdf, Hwp5, Hwpx, Docx, }
fn name(&self) -> &'static str {
        match self {
            Format::Pdf =>

                "PDF",
            Format::Hwp5 => "HWP 5.0",
            Format::Hwpx => "HWPX",
            Format::Docx => "DOCX",
        }
    }
```

손으로 쓴 4편의 코드와 같은 모양입니다. 4편의 E0004 처럼 완전성 검사도 그대로 받습니다.

패턴에 맞지 않게 부르면 매크로가 기대한 토큰을 알려 줍니다.

```console
error: no rules expected `"PDF"`
 --> src/main.rs:8:9
  |
1 | macro_rules! formats {
  | -------------------- when calling this macro
...
8 |     Pdf "PDF",
  |         ^^^^^ no rules expected this token in macro call
  |
note: while trying to match `=>`
```

## 매크로 안의 이름은 바깥 이름과 섞이지 않습니다

```rust
macro_rules! double {
    ($e:expr) => {{
        let x = $e;
        x * 2
    }};
}

    let x = 10;
    println!("  double!(x + 1) = {}, 바깥 x = {x}", double!(x + 1));
```

출력은 `double!(x + 1) = 22, 바깥 x = 10` 이었습니다. 매크로 안에서 만든 `x` 와 부른 쪽의 `x` 가 같은 이름인데도 다른 변수로 처리됐습니다. 이것을 위생성(hygiene)이라 합니다. C 의 전처리기 매크로는 글자를 그대로 바꿔 넣어 이런 충돌이 납니다. `$e:expr` 이 식 하나를 통째로 받는 것도 다릅니다. `x + 1` 이 `(x + 1) * 2` 처럼 묶인 채 들어가 연산 순서가 깨지지 않습니다.

## derive 매크로는 구조체의 모양을 읽어 코드를 만듭니다

구조체의 이름과 필드 이름을 돌려주는 `#[derive(Describe)]` 를 만들었습니다. 절차적 매크로는 별도 크레이트여야 합니다. `Cargo.toml` 에 `proc-macro = true` 를 적습니다.

```rust
#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    let fields = match &input.data {
        Data::Struct(s) => match &s.fields {
            Fields::Named(named) => named
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().to_string())
                .collect::<Vec<_>>(),
            _ => Vec::new(),
        },
        _ => {
            return syn::Error::new_spanned(name, "Describe 는 구조체에만 붙일 수 있다")
                .to_compile_error()
                .into();
        }
    };
    let type_name = name.to_string();
    quote! {
        impl #name {
            pub fn describe() -> (&'static str, &'static [&'static str]) {
                (#type_name, &[#(#fields),*])
            }
        }
    }
    .into()
}
```

`syn` 이 토큰을 구문 나무로 읽고, `quote!` 가 새 토큰을 만듭니다. `#name` 은 값을 끼워 넣고 `#(#fields),*` 는 반복입니다. `DocInfo` 에 붙이자 펼친 결과는 이렇습니다.

```rust
impl DocInfo {
    pub fn describe() -> (&'static str, &'static [&'static str]) {
        ("DocInfo", &["title", "author", "pages"])
    }
}
```

실행 결과는 `DocInfo: ["title", "author", "pages"]` 였습니다. 필드 이름이 컴파일할 때 문자열로 박혔으니 실행 비용이 없습니다. `thiserror` 의 `#[error("...")]` 도 같은 방식으로 `Display` 구현을 찍어 냅니다. 4편 "한 단계 더"에서 손으로 쓴 구현과 같은 일입니다.

`enum` 에 붙이면 매크로가 만든 오류가 그 자리를 짚습니다.

```console
error: Describe 는 구조체에만 붙일 수 있다
 --> src/main.rs:4:6
  |
4 | enum Kind {
  |      ^^^^
```

`syn::Error::new_spanned` 가 오류 위치를 사용자 코드의 `Kind` 에 붙였습니다. 매크로 안에서 `panic!` 으로 끝내면 위치 정보가 흐려집니다.

## 매크로의 값은 읽기와 빌드 시간입니다

절차적 매크로를 쓰는 데모를 빈 폴더에서 두 번 빌드하니 1.81초, 1.66초였습니다. 대부분은 `syn`·`quote`·`proc-macro2` 빌드입니다. 4편에서 매크로 없는 같은 규모 크레이트는 0.16~0.18초였습니다. 처음 한 번만 들고 이후에는 캐시되지만, CI 처럼 매번 빈 상태에서 빌드하는 곳에서는 매번 듭니다.

| 고를 때 | 쓰는 것 |
|---|---|
| 함수로 된다 | 함수 |
| 타입만 다르고 몸통이 같다 | 제네릭(16편) |
| 목록에서 여러 곳의 코드를 함께 찍어야 한다 | `macro_rules!` |
| 구조체·`enum` 의 모양을 읽어 구현을 만들어야 한다 | derive 매크로 |

## 면접에서 이렇게 묻는다

**Q1. 선언적 매크로와 절차적 매크로는 어떻게 다른가요?**
`macro_rules!` 는 토큰 패턴을 맞춰 정해진 틀로 찍습니다. 절차적 매크로는 토큰을 받아 Rust 코드로 처리해 새 토큰을 돌려주는 함수이고, 별도 크레이트(`proc-macro = true`)가 필요합니다. derive, 속성, 함수 모양 세 가지가 있습니다.

**Q2. 매크로 위생성이란?**
매크로 안에서 만든 이름이 부른 쪽의 이름과 충돌하지 않는 성질입니다. 실험에서 매크로 안의 `x` 와 바깥 `x` 가 다른 변수였습니다. C 전처리기와 다른 점입니다.

**Q3. 매크로가 무엇을 만드는지 어떻게 보나요?**
nightly 의 `-Zunpretty=expanded` 로 펼칩니다. `cargo expand` 같은 도구도 이 기능을 씁니다. 매크로를 리뷰할 때는 펼친 결과를 함께 봅니다.

**Q4. 매크로의 단점은?**
읽기 어렵고, 오류가 펼친 코드 기준으로 나올 수 있고, 절차적 매크로는 빌드 시간을 늘립니다. 실측에서 빈 빌드 1.7초 안팎이 더해졌습니다. 함수나 제네릭으로 되면 그쪽을 씁니다.

**Q5. derive 매크로에서 잘못 쓴 경우를 어떻게 알리나요?**
`syn::Error::new_spanned(위치, 메시지).to_compile_error()` 를 돌려줍니다. 사용자 코드의 정확한 위치에 컴파일 오류가 붙습니다. `panic!` 은 위치가 흐립니다.

## 연습

**연습 1.** `formats!` 에 확장자도 받게 해 `fn ext(&self) -> &'static str` 를 함께 찍으세요.

<details>
<summary>풀이 보기</summary>

```rust
macro_rules! formats2 {
    ($($variant:ident => $name:literal, $ext:literal);+ $(;)?) => {
        #[derive(Debug, Clone, Copy)]
        enum Format2 { $($variant),+ }
        impl Format2 {
            fn name(&self) -> &'static str { match self { $(Format2::$variant => $name),+ } }
            fn ext(&self) -> &'static str { match self { $(Format2::$variant => $ext),+ } }
        }
    };
}

formats2! {
    Pdf => "PDF", "pdf";
    Hwpx => "HWPX", "hwpx";
}
```

`Format2::Hwpx.ext()` 가 `"hwpx"` 를 돌려줬습니다. 항목 구분을 `;` 로 바꿔 한 항목 안의 쉼표와 섞이지 않게 했습니다.

</details>

**연습 2.** `Describe` 를 튜플 구조체(`struct Pair(u8, u8);`)에 붙이면 무엇이 나올까요?

<details>
<summary>풀이 보기</summary>

`Fields::Named` 가 아니라 `_ => Vec::new()` 갈래로 가서 필드 목록이 빈 `("Pair", [])` 이 됩니다. 컴파일 오류는 아닙니다. 이름 없는 필드를 `"0"`, `"1"` 로 돌려주려면 `Fields::Unnamed` 갈래에서 `(0..n).map(|i| i.to_string())` 을 만들면 됩니다. 매크로가 받을 수 있는 모든 모양을 정하는 것도 설계입니다.

</details>

## 한 단계 더 — 매크로는 컴파일러의 어느 단계에서 도나

매크로는 이름 해석과 타입 검사보다 먼저, 토큰 단계에서 펼쳐집니다. 그래서 `macro_rules!` 는 타입을 모릅니다. `$e:expr` 이 `u32` 인지 `String` 인지는 펼친 뒤 타입 검사에서 정해지고, 틀리면 펼친 코드 위치를 기준으로 오류가 납니다. derive 매크로도 구조체의 토큰만 봅니다. 필드 타입이 어떤 트레이트를 구현하는지는 모르고, 그런 조건은 찍어 낸 코드에 `where` 절로 남겨 컴파일러가 나중에 검사하게 합니다. `#[derive(Clone)]` 이 제네릭 구조체에 `T: Clone` 조건을 붙이는 것이 그 예입니다. 이 단계 설명은 The Rust Reference 의 "Macros" 와 "Procedural Macros" 장에 근거했습니다.

## 정리

매크로는 함수와 제네릭으로 줄일 수 없는 반복을 찍어 내는 틀입니다. 패턴이면 `macro_rules!`, 모양을 읽어야 하면 derive 입니다. 만든 매크로는 펼친 결과로 리뷰하고, 빌드 시간이라는 값을 따져 씁니다.

21편에서는 성능을 잽니다. criterion 으로 벤치마크를 만들고, 이 연재에서 손으로 쟀던 비교 몇 개를 다시 통계적으로 확인합니다.
