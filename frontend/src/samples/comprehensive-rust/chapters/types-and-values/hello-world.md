# Hello World!

가장 간단한 러스트 프로그램으로써, 고전적인 Hello World 를 작성해 보겠습니다:

``` rust
fn main() {
    println!("Hello 🌍!");
}
```

확인할 수 있는 것들:

- 함수는 `fn`으로 선언합니다.
- C/C++ 와 마찬가지로 중괄호`{}`로 블록을 표시합니다.
- `main` 함수는 프로그램 진입점입니다.
- 러스트는 똑똑한 매크로(hygienic macros) 시스템을 가지고 있습니다. `println!`는 그 예시입니다.
- 러스트의 문자열은 UTF-8로 인코딩되며 이모지와 같은 유니코드 문자를 포함할 수 있습니다.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> This slide tries to make the students comfortable with Rust code. They will see a ton of it over the next four days so we start small with something familiar.
>
> 키 포인트:
>
> - 러스트는 C/C++/Java와 같은 전통적인 다른 언어와 매우 유사합니다. 러스트는 절차적 언어입니다. 정말로 필요한 경우가 아니라면, 러스트는 이미 존재하는 것을 새로 구현하려고 하지 않습니다.
>
> - 러스트는 유니코드 지원과 같은 현대 언어의 특징을 전부 지원합니다.
>
> - Rust uses macros for situations where you want to have a variable number of arguments (no function [overloading](../control-flow-basics/functions.md)).
>
> - 똑똑한 매크로(hygienic macro)는 매크로가 사용되는 스코프에서 의도치 않게 변수를 가로채지 않습니다. 사실 러스트 매크로는 완전히 hygenic하지는 않습니다. [링크](https://veykril.github.io/tlborm/decl-macros/minutiae/hygiene.html)를 참고하세요.
>
> - 러스트는 멀티 패러다임 언어입니다. 예를 들어 강력한 [객체 지향 프로그래밍 기능](https://doc.rust-lang.org/book/ch17-00-oop.html)을 지원하기도 하며, 함수형 언어로 분류되지는 않지만 폭넓은 범위의 [함수형 컨셉](https://doc.rust-lang.org/book/ch13-00-functional-features.html)을 지원합니다.
