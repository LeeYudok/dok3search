# 소유권

모든 변수 바인딩은 유효한 “범위(스코프)“를 가지며, 범위 밖에서 변수 사용하면 에러가 발생합니다:

컴파일 오류 예제

``` rust
struct Point(i32, i32);

fn main() {
    {
        let p = Point(3, 4);
        println!("x: {}", p.0);
    }
    println!("y: {}", p.1);
}
```

We say that the variable *owns* the value. Every Rust value has precisely one owner at all times.

At the end of the scope, the variable is *dropped* and the data is freed. A destructor can run here to free up resources.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 가비지 컬렉션 구현에 익숙한 학생은 가비지 컬렉터가 연결 가능한 모든 메모리를 찾기 위해 ‘루트’ 세트로 시작한다는 사실을 알 수 있을 것입니다. Rust의 ‘단일 소유자’ 원칙도 이와 유사합니다.
