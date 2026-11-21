# Clone

값을 *복사해야* 하는 경우도 있습니다. `Clone` 트레잇을 사용하면 됩니다.

``` rust
#[derive(Default)]
struct Backends {
    hostnames: Vec<String>,
    weights: Vec<f64>,
}

impl Backends {
    fn set_hostnames(&mut self, hostnames: &Vec<String>) {
        self.hostnames = hostnames.clone();
        self.weights = hostnames.iter().map(|_| 1.0).collect();
    }
}
```

> **발표자 노트**
>
> This slide should take about 2 minutes.
>
> `Clone`의 개념은 힙 할당이 발생하는 위치를 쉽게 알아내는 것입니다. `.clone()` 및 `Vec::new` 또는 `Box::new`와 같은 다른 코드도 찾아봅니다.
>
> 빌림 검사기로 문제 해결 방법을 ’클론’하고 나중에 다시 방문하여 해당 클론을 최적화하려고 시도하는 경우가 많습니다.
