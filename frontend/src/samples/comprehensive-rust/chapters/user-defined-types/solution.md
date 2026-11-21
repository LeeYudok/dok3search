# 해답

``` rust
#[derive(Debug)]
/// 컨트롤러가 반응해야 하는 엘리베이터 시스템의 이벤트입니다.
enum Event {
    /// 버튼을 눌렀습니다.
    ButtonPressed(Button),

    /// 엘리베이터가 지정된 층에 도착했습니다.
    CarArrived(Floor),

    /// 엘리베이터 문이 열렸습니다.
    CarDoorOpened,

    /// 엘리베이터 문이 닫혔습니다.
    CarDoorClosed,
}

/// 층은 정수로 표시됩니다.
type Floor = i32;

/// 이동 방향입니다.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// 사용자가 액세스할 수 있는 버튼입니다.
#[derive(Debug)]
enum Button {
    /// 특정 층의 엘리베이터 로비에 있는 버튼입니다.
    LobbyCall(Direction, Floor),

    /// 엘리베이터 내부의 층 버튼입니다.
    CarFloor(Floor),
}

/// 엘리베이터가 지정된 층에 도착했습니다.
fn car_arrived(floor: i32) -> Event {
    Event::CarArrived(floor)
}

/// 엘리베이터 문이 열렸습니다.
fn car_door_opened() -> Event {
    Event::CarDoorOpened
}

/// 엘리베이터 문이 닫혔습니다.
fn car_door_closed() -> Event {
    Event::CarDoorClosed
}

/// 지정된 층의 엘리베이터 로비에서 방향 버튼을 눌렀습니다.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::ButtonPressed(Button::LobbyCall(dir, floor))
}

/// 엘리베이터에서 층 버튼을 눌렀습니다.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::ButtonPressed(Button::CarFloor(floor))
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
