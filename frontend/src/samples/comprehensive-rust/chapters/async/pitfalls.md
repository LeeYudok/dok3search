# async/await에서 주의해야할 함정

Async와 await는 동시 비동기 프로그래밍을 위한 편리하고 효율적인 추상화를 제공합니다. 하지만 Rust의 async/await 모델에도 문제는 있습니다. 이 장에서 몇 가지 예를 살펴보겠습니다.

- [실행자 차단](pitfalls/blocking-executor.md)
- [Pin](pitfalls/pin.md)
- [비동기 트레잇](pitfalls/async-traits.md)
- [취소](pitfalls/cancellation.md)
