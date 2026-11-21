# Implementing Traits

``` rust
trait Pet {
    fn talk(&self) -> String;

    fn greet(&self) {
        println!("오, 귀여워! 이름이 뭐야? {}", self.talk());
    }
}

struct Dog {
    name: String,
    age: i8,
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("멍멍, 제 이름은 {}입니다.", self.name)
    }
}

fn main() {
    let fido = Dog { name: String::from("Fido"), age: 5 };
    fido.greet();
}
```

> **발표자 노트**
>
> - To implement `Trait` for `Type`, you use an `impl Trait for Type { .. }` block.
>
> - Unlike Go interfaces, just having matching methods is not enough: a `Cat` type with a `talk()` method would not automatically satisfy `Pet` unless it is in an `impl Pet` block.
>
> - Traits may provide default implementations of some methods. Default implementations can rely on all the methods of the trait. In this case, `greet` is provided, and relies on `talk`.
