# 트레잇 상속하기

지원되는 트레잇은 다음과 같이 맞춤 타입에 자동으로 구현할 수 있습니다.

``` rust
#[derive(Debug, Clone, Default)]
struct Player {
    name: String,
    strength: u8,
    hit_points: u8,
}

fn main() {
    let p1 = Player::default(); // 기본 트레잇은 `default` 생성자를 추가합니다.
    let mut p2 = p1.clone(); // Clone 트레잇은 `clone` 메서드를 추가합니다.
    p2.name = String::from("EldurScrollz");
    // Debug 트레잇은 `{:?}` 표현을 사용한 출력을 지원합니다.
    println!("{:?} 대 {:?}", p1, p2);
}
```

> **발표자 노트**
>
> This slide should take about 3 minutes.
>
> 상속은 매크로를 사용하여 구현되며 많은 크레이트가 유용한 상속 매크로를 제공하여 유용한 기능을 추가합니다. 예를 들어 `serde`는 `#[derive(Serialize)]`를 사용하여 구조체의 직렬화 지원을 상속할 수 있습니다.
