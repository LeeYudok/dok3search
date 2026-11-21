# 파일시스템 계층

모듈의 내용을 기술하지 않으면, 러스트는 다른 파일에서 그 내용을 읽습니다:

컴파일 오류 예제

``` rust
mod garden;
```

위 코드는 러스트로 하여금 `garden`모듈의 내용을 `src/garden.rs`에서 찾도록 합니다. 비슷하게, `garden::vegetables` 모듈은 `src/garden/vegetables.rs`에서 찾습니다.

`crate(크레이트)`의 루트는 아래 경로 입니다:

- `src/lib.rs` (라이브러리 크레이트)
- `src/main.rs` (바이너리 크레이트)

모듈도 “내부 문서 주석“을 사용하여 문서화할 수 있습니다. 이러한 모듈은 모듈이 포함된 항목(이 경우에는 모듈)을 문서화합니다.

컴파일 오류 예제

``` rust
//! 이 모듈은 정원을 구현합니다.

// 이 모듈에서 타입을 다시 내보냅니다.
pub use garden::Garden;
pub use seeds::SeedPacket;

/// 주어진 씨앗 패킷을 뿌립니다.
pub fn sow(seeds: Vec<SeedPacket>) {
    todo!()
}

/// 준비된 농작물을 정원에서 수확합니다.
pub fn harvest(garden: &mut Garden) {
    todo!()
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> - `module/mod.rs`를 `module.rs`로 바꾼다 하더라도 Rust 2018에서는 하위 모듈을 사용할 수 있습니다.
>
> - `filename.rs`를 `filename/mod.rs`대신 사용할 수 있도록 하는 주된 이유는, `mod.rs`라는 이름을 가진 파일이 많을 경우 IDE에서 이들을 서로 구별하는게 힘들기 때문입니다.
>
> - 폴더를 이용해서 더 깊은 계층 구조를 구현할 수 있습니다. 심지어는 메인 모듈이 파일이더라도요:
>
>   ``` ignore
>   src/
>   ├── main.rs
>   ├── top_module.rs
>   └── top_module/
>       └── sub_module.rs
>   ```
>
> - 러스트가 어디서 모듈들을 찾을지는 컴파일러 디렉티브로 변경 가능합니다:
>
>   ``` rust
>   #[path = "some/path.rs"]
>   mod some_module;
>   ```
>
>   이는 Go언어 에서처럼 어떤 모듈의 테스트를 `some_module_test.rs` 같은 파일에 두고 싶은 경우에 유용합니다.
