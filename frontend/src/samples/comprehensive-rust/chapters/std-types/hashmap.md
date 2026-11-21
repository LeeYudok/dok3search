# `HashMap`

HashDoS 공격으로부터 보호되는 표준 해시 맵입니다:

``` rust
use std::collections::HashMap;

fn main() {
    let mut page_counts = HashMap::new();
    page_counts.insert("허클베리 핀의 모험".to_string(), 207);
    page_counts.insert("그림 동화".to_string(), 751);
    page_counts.insert("오만과 편견".to_string(), 303);

    if !page_counts.contains_key("레 미제라블") {
        println!(
            "{}의 책은 알고 있지만 레 미제라블은 알지 못합니다.",
            page_counts.len()
        );
    }

    for book in ["오만과 편견", "이상한 나라의 앨리스"] {
        match page_counts.get(book) {
            Some(count) => println!("{book}: {count}페이지"),
            None => println!("{book}을(를) 알 수 없습니다."),
        }
    }

    // 값을 찾을 수 없는 경우 .entry() 메서드를 사용하여 값을 삽입합니다.
    for book in ["오만과 편견", "이상한 나라의 앨리스"] {
        let page_count: &mut i32 = page_counts.entry(book.to_string()).or_insert(0);
        *page_count += 1;
    }

    println!("{page_counts:#?}");
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - `HashMap`은 prelude에 정의되어 있지 않기 때문에 명시적으로 추가해줘야 합니다.
>
> - 아래 코드를 테스트해보세요. 첫 문장에서는 해시맵에 책이 있는지 검사하여, 없으면 디폴트 값을 반환합니다. 두번 째 문장에서는 해시맵에 해당 책이 없는 경우, 지정한 값을 해시맵에 추가한 뒤 그 값을 반환합니다.
>
>   ``` rust
>   let pc1 = page_counts
>       .get("해리 포터와 마법사의 돌")
>       .unwrap_or(&336);
>   let pc2 = page_counts
>       .entry("헝거게임".to_string())
>       .or_insert(374);
>   ```
>
> - 안타깝지만 `hashmap!`같은 매크로가 없습니다.
>
>   - 러스트 1.56부터는 `HashMap`이 [`From<[(K, V); N]>`](https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html#impl-From%3C%5B(K,+V);+N%5D%3E-for-HashMap%3CK,+V,+RandomState%3E)을 구현하기 때문에 배열 리터럴을 이용하여 쉽게 해시맵을 초기화할 수 있습니다:
>
>     ``` rust
>     let page_counts = HashMap::from([
>       ("해리 포터와 마법사의 돌".to_string(), 336),
>       ("헝거게임".to_string(), 374),
>     ]);
>     ```
>
> - 키-값 쌍에 대한 `Iterator`로 해시맵을 만들 수도 있습니다.
>
> - 예제 코드에서는 편의상 해시맵의 키로 `&str`를 사용하지 않았습니다. 물론 컬렉션에 참조를 사용할 수도 있습니다. 다만 참조를 사용하게 되면 빌림 검사기 때문에 복잡해 질 수 있습니다.
>
>   - 예제 코드에서 `to_string()`을 없애도 컴파일에 문제가 없는지 확인해보세요. 어떤 문제에 부딪힐까요?
>
> - 해시맵의 몇 몇 메서드는 해시맵 내부의 특별한 타입(예를 들어 `std::collections::hash_map::Keys`)들을 리턴합니다. 이러한 타입들은 Rust 문서에서도 검색할 수 있습니다. 수강생들에게 이 타입들에 대한 문서를 보여주고, 이 문서에 `keys` 메서드로의 역 링크가 있음을 알려주세요.
