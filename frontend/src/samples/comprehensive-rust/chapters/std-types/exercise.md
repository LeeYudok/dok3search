# 연습문제: 카운터

이 연습에서는 매우 간단한 데이터 구조를 사용하여 제네릭으로 만듭니다. [`std::collections::HashMap`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html)을 사용하여 어떤 값이 표시되었는지, 각각 얼마나 표시되었는지 추적합니다.

`Counter`의 초기 버전은 `u32` 값에만 작동하도록 하드 코딩되어 있습니다. 추적 중인 값 타입에 대해 구조체 및 메서드를 제네릭으로 만듭니다. 그러면 `Counter`가 모든 타입의 값을 추적할 수 있습니다.

일찍 완료한 경우 [`entry`](https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.entry) 메서드를 사용하여, `count` 메서드를 구현하는 데 필요한 해시 조회 횟수를 절반으로 줄여보세요.

컴파일 오류 예제

``` rust
use std::collections::HashMap;

/// Counter는 각 T 타입 값이 표시된 횟수를 계산합니다.
struct Counter {
    values: HashMap<u32, u64>,
}

impl Counter {
    /// 새 Counter를 만듭니다.
    fn new() -> Self {
        Counter {
            values: HashMap::new(),
        }
    }

    /// 지정된 값의 발생 횟수를 셉니다.
    fn count(&mut self, value: u32) {
        if self.values.contains_key(&value) {
            *self.values.get_mut(&value).unwrap() += 1;
        } else {
            self.values.insert(value, 1);
        }
    }

    /// 지정된 값이 표시된 횟수를 반환합니다.
    fn times_seen(&self, value: u32) -> u64 {
        self.values.get(&value).copied().unwrap_or_default()
    }
}

fn main() {
    let mut ctr = Counter::new();
    ctr.count(13);
    ctr.count(14);
    ctr.count(16);
    ctr.count(14);
    ctr.count(14);
    ctr.count(11);

    for i in 10..20 {
        println!("{} 개의 {} 값을 발견했습니다.", ctr.times_seen(i), i);
    }

    let mut strctr = Counter::new();
    strctr.count("사과");
    strctr.count("오렌지");
    strctr.count("사과");
    println!("사과 {}개 받음", strctr.times_seen("사과"));
}
```
