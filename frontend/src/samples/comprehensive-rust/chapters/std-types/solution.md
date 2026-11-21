# 해답

``` rust
use std::collections::HashMap;
use std::hash::Hash;

/// Counter는 각 T 타입 값이 표시된 횟수를 계산합니다.
struct Counter<T: Eq + Hash> {
    values: HashMap<T, u64>,
}

impl<T: Eq + Hash> Counter<T> {
    /// 새 Counter를 만듭니다.
    fn new() -> Self {
        Counter { values: HashMap::new() }
    }

    /// 지정된 값의 발생 횟수를 셉니다.
    fn count(&mut self, value: T) {
        *self.values.entry(value).or_default() += 1;
    }

    /// 지정된 값이 표시된 횟수를 반환합니다.
    fn times_seen(&self, value: T) -> u64 {
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
