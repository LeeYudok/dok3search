# 메서드

러스트에서 선언된 타입에 대해 `impl`블록에 함수를 선언하여 메서드를 연결 할 수 있습니다:

``` rust
#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    // 수신자 없음, 정적 메서드입니다.
    fn new(name: &str) -> Self {
        Self { name: String::from(name), laps: Vec::new() }
    }

    // self에 대한 독점적 빌림 읽기/쓰기 액세스입니다.
    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    // self에 대한 공유 및 읽기 전용 빌림 액세스입니다.
    fn print_laps(&self) {
        println!("랩 타임 {}회, {} 경기:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("{idx}랩: {lap}초");
        }
    }

    // self의 독점적 소유권
    fn finish(self) {
        let total: i32 = self.laps.iter().sum();
        println!("{} 레이스 종료, 총 랩 시간: {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("모나코 그랑프리");
    race.add_lap(70);
    race.add_lap(68);
    race.print_laps();
    race.add_lap(71);
    race.print_laps();
    race.finish();
    // race.add_lap(42);
}
```

The `self` arguments specify the “receiver” - the object the method acts on. There are several common receivers for a method:

- `&self`: 호출자로부터 공유가능한 불변 참조 방식으로 객체를 빌려옴을 나타냅니다. 객체는 메소드 호출 뒤에도 사용될 수 있습니다.
- `&mut self`: 호출자로부터 유일한 가변 참조 방식으로 객체를 빌려옴을 나타냅니다. 객체는 메소드 호출 뒤에도 사용될 수 있습니다.
- `self`: 호출자로부터 객체의 소유권을 가져오고 객체는 호출자로부터 메소드로 이동됩니다. 메소드가 객체를 소유하게 되며 따라서 명시적으로 소유권을 다른 곳으로 전달하지 않는다면 메서드 종료와 함께 객체는 drop(해제)됩니다.
- `mut self`: same as above, but the method can mutate the object.
- 리시버 없음: 구조체의 정적 메서드가 됩니다. 주로 생성자를 만들때 사용하게 되며, 생성자는 흔히 `new`라고 이름붙입니다.

> **발표자 노트**
>
> This slide should take about 8 minutes.
>
> 키 포인트:
>
> - 메서드를 함수와 비교하여 소개하는 것도 도움이 될 수 있습니다.
>   - 메서드는 구조체나 열거형과 같은 타입의 인스턴스에서 호출 되며, 첫번째 매개변수(파라메터)는 인스턴스를 `self`로 표기합니다.
>   - 메서드를 이용하면 receiver 문법을 사용할 수 있고 코드를 좀더 체계적으로 정리할 수 있습니다. 메서드들이 예측 가능한 위치에 모여 있으니 찾기 쉽습니다.
> - 메서드 receiver인 `self` 키워드 사용을 언급해 주시기 바랍니다.
>   - 예제의 경우 `self: &Self`의 줄인 버전임을 알려주고, 구조체의 이름을 직접 사용하면 어떻게 되는지 보여주는 것도 좋습니다.
>   - `impl` 블록 내부에서는 `Self`가 해당 타입의 이름 대용으로 사용될 수 있음을 알려주세요.
>   - 구조체의 필드를 접근할 때 점 표기를 사용하듯이 `self`에 점 표기를 사용하여 개별 필드들을 접근할 수 있습니다.
>   - This might be a good time to demonstrate how the `&self` differs from `self` by trying to run `finish` twice.
>   - `self`를 사용하는 이같은 변형들 외에도 `Box<Self>`와 같이 리시버 타입으로 허용되는 [특별한 래퍼 타입](https://doc.rust-lang.org/reference/special-types-and-traits.html)이 있습니다.
