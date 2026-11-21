# Select

Select 연산은 여러 future들 모두에 대해서 준비될 때 까지 기다리다가, 그 중 어떤 한 future가 최초로 준비 상태가 되면 해당 future의 결과값을 리턴합니다. 이것은 자바스크립트에서의 `Promise.race`와 비슷합니다. 파이썬에서라면 `asyncio.wait(task_set, return_when=asyncio.FIRST_COMPLETED)`가 하는 동작과 비슷합니다.

Similar to a match statement, the body of `select!` has a number of arms, each of the form `pattern = future => statement`. When a `future` is ready, its return value is destructured by the `pattern`. The `statement` is then run with the resulting variables. The `statement` result becomes the result of the `select!` macro.

컴파일 오류 예제

``` rust
use tokio::sync::mpsc::{self, Receiver};
use tokio::time::{sleep, Duration};

#[derive(Debug, PartialEq)]
enum Animal {
    Cat { name: String },
    Dog { name: String },
}

async fn first_animal_to_finish_race(
    mut cat_rcv: Receiver<String>,
    mut dog_rcv: Receiver<String>,
) -> Option<Animal> {
    tokio::select! {
        cat_name = cat_rcv.recv() => Some(Animal::Cat { name: cat_name? }),
        dog_name = dog_rcv.recv() => Some(Animal::Dog { name: dog_name? })
    }
}

#[tokio::main]
async fn main() {
    let (cat_sender, cat_receiver) = mpsc::channel(32);
    let (dog_sender, dog_receiver) = mpsc::channel(32);
    tokio::spawn(async move {
        sleep(Duration::from_millis(500)).await;
        cat_sender.send(String::from("펠릭스")).await.expect("고양이를 보내지 못했습니다.");
    });
    tokio::spawn(async move {
        sleep(Duration::from_millis(50)).await;
        dog_sender.send(String::from("렉스")).await.expect("개를 보내지 못했습니다.");
    });

    let winner = first_animal_to_finish_race(cat_receiver, dog_receiver)
        .await
        .expect("우승자를 수신하지 못했습니다.");

    println!("우승자: {winner:?}");
}
```

> **발표자 노트**
>
> - In this example, we have a race between a cat and a dog. `first_animal_to_finish_race` listens to both channels and will pick whichever arrives first. Since the dog takes 50ms, it wins against the cat that take 500ms.
>
> - You can use `oneshot` channels in this example as the channels are supposed to receive only one `send`.
>
> - Try adding a deadline to the race, demonstrating selecting different sorts of futures.
>
> - Note that `select!` drops unmatched branches, which cancels their futures. It is easiest to use when every execution of `select!` creates new futures.
>
>   - 대안은 future 자체 대신 `&mut future`를 전달하는 것입니다. 하지만 이렇게 하면 문제가 발생할 수 있습니다(Pinning을 다룰 때 자세히 설명할 예정임).
