# `Drop` 트레잇

[`Drop`](https://doc.rust-lang.org/std/ops/trait.Drop.html)트레잇을 구현하면, 그 값이 스코프 밖으로 나갈 때 실행될 코드를 작성할 수 있습니다:

``` rust
struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("{} 삭제 중", self.name);
    }
}

fn main() {
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
            println!("B 블록에서 나가기");
        }
        println!("A 블록에서 나가기");
    }
    drop(a);
    println!("main에서 나가기");
}
```

> **발표자 노트**
>
> This slide should take about 8 minutes.
>
> - `std::mem::drop`은 `std::ops::Drop::drop`과 같지 않습니다.
> - 값이 범위를 벗어나면 자동으로 삭제됩니다.
> - 값이 삭제될 때 `std::ops::Drop`을 구현하면 `Drop::drop` 구현이 호출됩니다.
> - 그러면 `Drop` 구현 여부와 관계없이 해당 필드도 모두 삭제됩니다.
> - `std::mem::drop`은 값을 사용하는 빈 함수입니다. 중요한 점은 값의 소유권을 가지므로 범위 끝에서 삭제된다는 점입니다. 따라서 범위를 벗어날 때보다 빨리 값을 명시적으로 삭제할 수 있는 편리한 방법입니다.
>   - 이는 `drop`에서 잠금 해제, 파일 닫기 등의 작업을 실행하는 객체에 유용할 수 있습니다.
>
> 논의점:
>
> - `Drop::drop`은 왜 `self`를 인자로 받지 않습니까?
>   - 짧은 대답: 만약 그렇게 된다면 `std::mem::drop`이 블록의 끝에서 호출되고, 다시 `Drop::drop`을 호출하게 되어, 스택 오버플로가 발생합니다!
> - `drop(a)`를 `a.drop()`로 변경해 보시기 바랍니다.
