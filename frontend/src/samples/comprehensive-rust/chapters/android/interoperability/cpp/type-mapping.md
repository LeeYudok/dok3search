# 추가 타입

| Rust Type         | C++ Type             |
|-------------------|----------------------|
| `String`          | `rust::String`       |
| `&str`            | `rust::Str`          |
| `CxxString`       | `std::string`        |
| `&[T]`/`&mut [T]` | `rust::Slice`        |
| `Box<T>`          | `rust::Box<T>`       |
| `UniquePtr<T>`    | `std::unique_ptr<T>` |
| `Vec<T>`          | `rust::Vec<T>`       |
| `CxxVector<T>`    | `std::vector<T>`     |

> **발표자 노트**
>
> - 이러한 타입은 공유 구조체의 필드와 extern 함수의 인수 및 반환에서 사용할 수 있습니다.
> - Rust의 `String`은 `std::string`에 직접 매핑되지 않습니다. 여기에는 다음과 같은 몇 가지 이유가 있습니다.
>   - `std::string`은 `String`에 필요한 UTF-8 불변값을 유지하지 않습니다.
>   - 두 타입은 메모리에 서로 다른 레이아웃을 가지고 있으므로 언어 간에 직접 전달될 수 없습니다.
>   - `std::string`에는 Rust의 이동 의미 체계와 일치하지 않는 이동 생성자가 필요하므로 `std::string`을 값으로 Rust에 전달할 수 없습니다.
