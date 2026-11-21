# 컴파일러 린트 및 Clippy

Rust 컴파일러는 유용한 내장 린트뿐 아니라 멋진 오류 메시지를 생성합니다. [Clippy](https://doc.rust-lang.org/clippy/)는 프로젝트별로 사용 설정할 수 있는 그룹으로 구성된 더 많은 린트를 제공합니다.

패닉 발생 예제

``` rust
#[deny(clippy::cast_possible_truncation)]
fn main() {
    let x = 3;
    while (x < 70000) {
        x *= 2;
    }
    println!("X는 u16에 맞지 않을까요? {}", x as u16);
}
```

> **발표자 노트**
>
> This slide should take about 3 minutes.
>
> 코드 샘플을 실행하고 오류 메시지를 확인합니다. 여기에도 린트가 표시되지만 코드가 컴파일되고 나면 표시되지 않습니다. 플레이그라운드 사이트로 전환하여 이러한 린트를 표시합니다.
>
> 린트를 해결한 후 플레이그라운드 사이트에서 `clippy`를 실행하여 clippy 경고를 표시합니다. Clippy는 린트에 관한 광범위한 문서를 보유하고 있으며 항상 새로운 린트(default-deny 린트 포함)를 추가합니다.
>
> `help: ...`가 포함된 오류나 경고는 `cargo fix` 또는 편집기를 통해 수정할 수 있습니다.
