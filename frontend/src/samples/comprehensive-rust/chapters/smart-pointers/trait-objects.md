# 트레잇 객체

트레잇 객체는 타입이 다른 값(예를 들어 컬렉션에 속한 각 값)들을 가질 수 있습니다:

``` rust
struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("멍멍, 제 이름은 {}입니다.", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("냐옹!")
    }
}

fn main() {
    let pets: Vec<Box<dyn Pet>> = vec![
        Box::new(Cat { lives: 9 }),
        Box::new(Dog { name: String::from("Fido"), age: 5 }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
}
```

`pets`를 할당한 이후의 메모리 레이아웃:

![\<Dog as Pet\>::talk · \<Cat as Pet\>::talk · Stack · Heap · F · i · d · o · ptr · lives · 9 · len · 2 · capacity · data: · name · , · 4, · 4 · age · 5 · vtable · pets: Vec\<dyn Pet\> · data: Cat · Dog · Program text](../../assets/smart-pointers/trait-objects-diagram-1.svg)

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - Types that implement a given trait may be of different sizes. This makes it impossible to have things like `Vec<dyn Pet>` in the example above.
>
> - `dyn Pet`이라고 하면이 타입의 크기는 동적이며 `Pet`을 구현하고 있다고 컴파일러에게 알려주는 것입니다.
>
> - 이 예에서는 `pets`가 스택에 할당되고 벡터 데이터는 힙에 있습니다. 두 벡터 요소는 \_fat 포인터_입니다.
>   - A fat pointer is a double-width pointer. It has two components: a pointer to the actual object and a pointer to the [virtual method table](https://en.wikipedia.org/wiki/Virtual_method_table) (vtable) for the `Pet` implementation of that particular object.
>   - 이름이 Fido인 `Dog`의 데이터는 `name` 및 `age` 필드입니다. `Cat`에는 `lives` 필드가 있습니다.
>
> - 아래 코드의 결과와 비교해보세요:
>
>   ``` rust
>   println!("{} {}", std::mem::size_of::<Dog>(), std::mem::size_of::<Cat>());
>   println!("{} {}", std::mem::size_of::<&Dog>(), std::mem::size_of::<&Cat>());
>   println!("{}", std::mem::size_of::<&dyn Pet>());
>   println!("{}", std::mem::size_of::<Box<dyn Pet>>());
>   ```
