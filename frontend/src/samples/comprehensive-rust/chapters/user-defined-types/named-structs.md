# 구조체

C/C++ 와 마찬가지로 러스트는 커스텀 구조체를 지원합니다:

``` rust
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{}은(는) {}세입니다.", person.name, person.age);
}

fn main() {
    let mut peter = Person { name: String::from("피터"), age: 27 };
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    let name = String::from("에이버리");
    let age = 39;
    let avery = Person { name, age };
    describe(&avery);

    let jackie = Person { name: String::from("재키"), ..avery };
    describe(&jackie);
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> 키 포인트:
>
> - 구조체는 C/C++ 와 유사합니다.
>   - C++ 와 같지만 C와는 달리 타입을 정의하기 위해 ’typedef’가 필요하지 않습니다.
>   - C++ 와 달리 구조체 간 상속은 없습니다.
> - This may be a good time to let people know there are different types of structs.
>   - Zero-sized structs (e.g. `struct Foo;`) might be used when implementing a trait on some type but don’t have any data that you want to store in the value itself.
>   - 다음 슬라이드에서는 필드 이름이 덜 중요할 때 사용할 수 있는 튜플 구조체를 소개합니다.
> - If you already have variables with the right names, then you can create the struct using a shorthand.
> - The syntax `..avery` allows us to copy the majority of the fields from the old struct without having to explicitly type it all out. It must always be the last element.
