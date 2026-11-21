# 모듈

`impl`블록은 해당 타입의 함수들에 대한 네임스페이스를 제공합니다.

마찬가지로, `mod`는 타입과 함수들에 대해 네임스페이스를 제공합니다:

``` rust
mod foo {
    pub fn do_something() {
        println!("foo 모듈 내부");
    }
}

mod bar {
    pub fn do_something() {
        println!("bar 모듈 내부");
    }
}

fn main() {
    foo::do_something();
    bar::do_something();
}
```

> **발표자 노트**
>
> This slide should take about 3 minutes.
>
> - 패키지는 기능을 제공하며 하나의 대표 `Cargo.toml` 파일을 포함합니다. 패키지를 구성하는 크레이트들을 빌드하는 방법이 이 파일에 기술됩니다.
> - 크레이트는 모듈의 트리입니다. 바이너리 크레이트는 실행파일로 빌드되고, 라이브러리 크레이트는 라이브러리로 빌드됩니다.
> - 모듈은 코드를 조직화하고 스코프를 정의하는 단위입니다.
