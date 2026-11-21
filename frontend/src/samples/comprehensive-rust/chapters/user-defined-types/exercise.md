# 연습문제: 엘리베이터 이벤트

엘리베이터 제어 시스템의 이벤트를 나타내는 데이터 구조를 만들어 보겠습니다. 다양한 이벤트를 구성하기 위해 타입과 함수를 정의하는 것은 개발자의 몫입니다. 해당 타입을 `{:?}` 문법을 통해 출력할 수 있도록 `#[derive(Debug)]`를 사용합시다.

이 연습에서는 `main` 함수가 에러없이 동작하도록 데이터 구조만 만들면 됩니다. 이 과정의 다음 부분에서는 이러한 구조에서 데이터를 가져오는 방법을 다룹니다.

패닉 발생 예제

``` rust
#[derive(Debug)]
/// 컨트롤러가 반응해야 하는 엘리베이터 시스템의 이벤트입니다.
enum Event {
    // TODO: 필요한 변형들을 추가하세요.
}

/// 이동 방향입니다.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// 엘리베이터가 지정된 층에 도착했습니다.
fn car_arrived(floor: i32) -> Event {
    todo!()
}

/// 엘리베이터 문이 열렸습니다.
fn car_door_opened() -> Event {
    todo!()
}

/// 엘리베이터 문이 닫혔습니다.
fn car_door_closed() -> Event {
    todo!()
}

/// 지정된 층의 엘리베이터 로비에서 방향 버튼을 눌렀습니다.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    todo!()
}

/// 엘리베이터에서 층 버튼을 눌렀습니다.
fn car_floor_button_pressed(floor: i32) -> Event {
    todo!()
}

fn main() {
    println!(
        "1층 승객이 위쪽 버튼을 눌렀습니다. {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("엘리베이터가 1층에 도착했습니다: {:?}", car_arrived(0));
    println!("엘리베이터 문이 열렸습니다. {:?}", car_door_opened());
    println!(
        "승객이 3층 버튼을 눌렀습니다. {:?}",
        car_floor_button_pressed(3)
    );
    println!("엘리베이터 문이 닫혔습니다: {:?}", car_door_closed());
    println!("엘리베이터가 3층에 도착했습니다. {:?}", car_arrived(3));
}
```
