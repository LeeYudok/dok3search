# 모의 처리

모의 처리의 경우 널리 사용되는 라이브러리인 [Mockall](https://docs.rs/mockall/)이 있습니다. 트레잇을 사용하도록 코드를 리팩터링해야 합니다. 그런 다음 빠르게 모의 처리할 수 있습니다.

``` rust
use std::time::Duration;

#[mockall::automock]
pub trait Pet {
    fn is_hungry(&self, since_last_meal: Duration) -> bool;
}

#[test]
fn test_robot_dog() {
    let mut mock_dog = MockPet::new();
    mock_dog.expect_is_hungry().return_const(true);
    assert_eq!(mock_dog.is_hungry(Duration::from_secs(10)), true);
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - Mockall is the recommended mocking library in Android (AOSP). There are other [mocking libraries available on crates.io](https://crates.io/keywords/mock), in particular in the area of mocking HTTP services. The other mocking libraries work in a similar fashion as Mockall, meaning that they make it easy to get a mock implementation of a given trait.
>
> - 모의 처리는 다소 *논란의 여지가 있습니다*. 모의를 사용하면 테스트를 종속 항목에서 완전히 분리할 수 있습니다. 그 결과 테스트 실행이 더욱 빠르고 안정적으로 이루어집니다. 반면에 모의는 잘못 구성되어 실제 종속 항목이 실행하는 것과 다른 출력을 반환할 수 있습니다.
>
>   가능하면 실제 종속 항목을 사용하는 것이 좋습니다. 예를 들어 많은 데이터베이스에서는 메모리 내 백엔드를 구성할 수 있습니다. 즉, 테스트에서 올바른 동작을 가져올 뿐만 아니라 속도가 빠르고 자동으로 정리됩니다.
>
>   마찬가지로 많은 웹 프레임워크에서도 `localhost`의 임의 포트에 바인딩되는 프로세스 내 서버를 시작할 수 있습니다. 프레임워크를 모의 처리하는 것보다는 항상 이 방법을 사용하는 것이 좋습니다. 실제 환경에서 코드를 테스트하는 데 도움이 됩니다.
>
> - Mockall은 Rust 플레이그라운드의 일부가 아니므로 로컬 환경에서 이 예를 실행해야 합니다. Mockall을 기존 Cargo 프로젝트에 빠르게 추가하려면 `cargo add mockall`을 사용합니다.
>
> - Mockall에는 더 많은 기능이 있습니다. 특히 전달된 인수에 따라 기대치를 설정할 수 있습니다. 여기서는 마지막으로 먹이를 먹고 3시간이 지나면 배고파지는 고양이를 모의하는 데 이를 사용합니다.
>
> ``` rust
> #[test]
> fn test_robot_cat() {
>     let mut mock_cat = MockPet::new();
>     mock_cat
>         .expect_is_hungry()
>         .with(mockall::predicate::gt(Duration::from_secs(3 * 3600)))
>         .return_const(true);
>     mock_cat.expect_is_hungry().return_const(false);
>     assert_eq!(mock_cat.is_hungry(Duration::from_secs(1 * 3600)), false);
>     assert_eq!(mock_cat.is_hungry(Duration::from_secs(5 * 3600)), true);
> }
> ```
>
> - `.times(n)`를 사용하여 모의 메서드가 호출될 수 있는 횟수를 `n`으로 제한할 수 있습니다. 이 조건이 충족되지 않으면 모의 메서드가 삭제될 때 자동으로 패닉 상태가 됩니다.
