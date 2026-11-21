# Join

Join 연산은 모든 future가 준비될 때까지 기다린 후, 각 future의 결과값을 담은 컬렉션을 리턴합니다. 이는 자바스크립트의 `Promise.all`이나 파이썬의 `asyncio.gather`와 유사합니다.

컴파일 오류 예제

``` rust
use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;

async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

#[tokio::main]
async fn main() {
    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> =
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:?}", page_sizes_dict);
}
```

> **발표자 노트**
>
> 이 예제를, 로컬 컴퓨터에 만들어 둔 `src/main.rs`에 복사하고 거기에서 실행하세요.
>
> - 서로 다른 타입을 가지는 여러 여러 futures들을 join하고자 할 경우 `std::future::join!`을 사용할 수 있습니다. 이 매크로를 사용하려면 futures가 몇 개나 있을지 컴파일 할 때 알아야 한다는 점을 주의하세요. 이 매크로는 지금은 `futures` 크레이트에 있으며 곧 안정화 되어 `std::future`에 포함될 예정입니다.
>
> - The risk of `join` is that one of the futures may never resolve, this would cause your program to stall.
>
> - `join_all`을 `join!`과 결합하여 http 서비스와 데이터베이스에 대한 모든 요청들을 한꺼번에 진행시킬 수도 있습니다. `futures::join!`을 사용하여 `tokio::time::sleep`을 future에 추가해 보세요. 이건 타임아웃을 구현하는 것이 아님을 주의하세요. 실제로, 타임아웃은 다음 장에서 설명하는 `select!`를 사용해서 구현해야 합니다. 여기서는 `tokio::time::sleep`을 사용한 것은 단순히 `join!`의 동작을 설명하기 위함입니다.
