# 값 대여

As we saw before, instead of transferring ownership when calling a function, you can let a function *borrow* the value:

``` rust
#[derive(Debug)]
struct Point(i32, i32);

fn add(p1: &Point, p2: &Point) -> Point {
    Point(p1.0 + p2.0, p1.1 + p2.1)
}

fn main() {
    let p1 = Point(3, 4);
    let p2 = Point(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");
}
```

- `add` 함수는 두 `Point` 객체 값을 \_빌려_와서 새로운 `Point` 객체를 반환합니다.
- `p1`과 `p2`의 소유권은 여전히 호출자(`main`함수)에 있습니다.

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> 이 슬라이드는 1일 차의 참조를 다룬 자료를 검토하는 것으로, 약간 확장되어 함수 인수와 반환 값을 포함합니다.
>
> ## 더 살펴보기
>
> 스택에 할당된 값을 리턴하는 것에 대한 참고:
>
> - Demonstrate that the return from `add` is cheap because the compiler can eliminate the copy operation. Change the above code to print stack addresses and run it on the [Playground](https://play.rust-lang.org/?version=stable&mode=release&edition=2021&gist=0cb13be1c05d7e3446686ad9947c4671) or look at the assembly in [Godbolt](https://rust.godbolt.org/). In the “DEBUG” optimization level, the addresses should change, while they stay the same when changing to the “RELEASE” setting:
>
>   ``` rust
>   #[derive(Debug)]
>   struct Point(i32, i32);
>
>   fn add(p1: &Point, p2: &Point) -> Point {
>       let p = Point(p1.0 + p2.0, p1.1 + p2.1);
>       println!("&p.0: {:p}", &p.0);
>       p
>   }
>
>   pub fn main() {
>       let p1 = Point(3, 4);
>       let p2 = Point(10, 20);
>       let p3 = add(&p1, &p2);
>       println!("&p3.0: {:p}", &p3.0);
>       println!("{p1:?} + {p2:?} = {p3:?}");
>   }
>   ```
>
> - 러스트 컴파일러는 반환값 최적화(RVO)를 수행할 수 있습니다.
>
> - C++에서 copy elision은 생성자의 부수효과 가능성이 있어 언어레벨의 정의가 필요하지만 러스트에서는 문제가 되지 않습니다. 만약 RVO가 발생하지 않으면 러스트는 항상 간단하고 효율적인 `memcpy`복사를 수행할 것입니다.
