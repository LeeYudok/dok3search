# 안전하지 않은 러스트

러스트로 작성된 프로그램은 크게 두 부분으로 나뉩니다:

- **안전한 러스트:** 메모리 관련 오류 발생 불가능, 정의되지 않은 동작 없음.
- **안전하지 않은 러스트:** 특별한 조건을 만족하지 않은채로 사용되면 정의되지 않은 동작을 유발할 수 있음.

We saw mostly safe Rust in this course, but it’s important to know what Unsafe Rust is.

보통, 안전하지 않은 러스트 코드는 크기가 작으며, 독립적으로 존재합니다. 그리고 코드가 왜 잘 작동하는지에 대해 세밀하게 문서화가 되어 있습니다. 그리고, 많은 경우 안전한 러스트 코드를 통해서 추상화를 시킨 후 사용하게 됩니다.

안전하지 않은 러스트를 이용하면 다음과 같은 다섯 가지 것들이 가능해 집니다:

- 원시 포인터 역참조(따라가기)
- 정적 가변변수 접근 및 수정.
- `union` 필드 접근.
- `extern` 함수를 포함한 `unsafe` 함수 호출.
- `unsafe` 트레잇 구현.

위 기능들에 대해 간략히 살펴보겠습니다. 자세한 내용은 [러스트 프로그래밍 언어, 19.1절](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)과 [Rustonomicon](https://doc.rust-lang.org/nomicon/)를 참조하세요.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> Unsafe Rust does not mean the code is incorrect. It means that developers have turned off some compiler safety features and have to write correct code by themselves. It means the compiler no longer enforces Rust’s memory-safety rules.
