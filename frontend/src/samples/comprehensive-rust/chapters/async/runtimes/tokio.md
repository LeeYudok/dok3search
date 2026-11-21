# Tokio

Tokio provides:

- 비동기 코드 실행을 위한 멀티스레드 런타임
- 표준 라이브러리의 비동기 버전
- 대규모 라이브러리 생태계

컴파일 오류 예제

``` rust
use tokio::time;

async fn count_to(count: i32) {
    for i in 1..=count {
        println!("작업 개수: {i}개!");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}

#[tokio::main]
async fn main() {
    tokio::spawn(count_to(10));

    for i in 1..5 {
        println!("기본 작업: {i}");
        time::sleep(time::Duration::from_millis(5)).await;
    }
}
```

> **발표자 노트**
>
> - 이제 `tokio::main` 매크로를 사용하면 `main`을 비동기로 만들 수 있습니다.
>
> - `spawn` 함수는 동시 실행되는 새로운 “작업“을 만듭니다.
>
> - 참고: `spawn`은 `Future`를 인자로 받습니다. 때문에 `count_to`에 `.await`를 호출하지 않는 점을 주목하세요.
>
> **심화 학습:**
>
> - `count_to`가 10에 도달하지 않는 경우가 많은데 그 이유는 무엇일까요? 이는 비동기적인 취소를 보여주는 예입니다. `tokio::spawn`이 리턴하는 것은 완료될 때까지 기다리도록 대기하는데 사용되는 핸들입니다.
>
> - `tokio::spawn` 대신 `count_to(10).await`를 사용해 보세요.
>
> - `tokio::spawn`에서 반환된 작업을 `await` 해 보세요.
