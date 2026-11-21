# 원시 포인터 역참조(따라가기)

포인터를 만드는 것은 안전합니다. 하지만 역참조(따라가기)할 경우 `unsafe`가 필요합니다:

``` rust
fn main() {
    let mut s = String::from("조심하세요!");

    let r1 = &mut s as *mut String;
    let r2 = r1 as *const String;

    // Safe because r1 and r2 were obtained from references and so are
    // guaranteed to be non-null and properly aligned, the objects underlying
    // the references from which they were obtained are live throughout the
    // whole unsafe block, and they are not accessed either through the
    // references or concurrently through any other pointers.
    unsafe {
        println!("r1은 {}입니다.", *r1);
        *r1 = String::from("이런");
        println!("r2는 {}입니다.", *r2);
    }

    // 안전하지 않음. 이렇게 하지 마세요.
    /*
    let r3: &String = unsafe { &*r1 };
    drop(s);
    println!("r3 is: {}", *r3);
    */
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> 모든 `unsafe` 블록에 대해 왜 그 코드가 안전한지에 대한 설명을 주석으로 다는 것은 좋은 습관입니다(사실 안드로이드의 러스트 스타일 가이드에서는 이게 필수입니다).
>
> 포인터 역참조를 할 경우, 포인터가 [*유효*](https://doc.rust-lang.org/std/ptr/index.html#safety)해야 합니다. 예를 들어:
>
> - 포인터는 null이면 안됩니다.
> - 포인터는 따라가기가 가능해야 합니다 (객체의 어느 한 부분을 가리키고 있어야 합니다).
> - 이미 반환된 객체를 가리키면 안됩니다.
> - 같은 위치에 대해 동시적인 접근이 있으면 안됩니다.
> - 참조를 캐스팅 해서 포인터를 만들었다면, 그 참조가 가리키는 객체는 살아 있어야 하며, 그 객체의 메모리를 접근하는 참조가 하나도 없어야 합니다.
>
> 대부분의 경우 포인터는 align되어 있어야 합니다.
>
> The “NOT SAFE” section gives an example of a common kind of UB bug: `*r1` has the `'static` lifetime, so `r3` has type `&'static String`, and thus outlives `s`. Creating a reference from a pointer requires *great care*.
