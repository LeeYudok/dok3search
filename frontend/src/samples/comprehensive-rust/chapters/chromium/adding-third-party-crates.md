# 서드 파티 크레이트 추가

Rust 라이브러리는 ’크레이트’라고 하며 [crates.io](https://crates.io)에서 찾을 수 있습니다. Rust 크레이트가 서로 종속되는 것은 *아주 쉽습니다*. 따라서 서로 종속됩니다.

| 속성                     | C++ library | Rust crate           |
|--------------------------|-------------|----------------------|
| Build system             | 1억+        | 일관성: `Cargo.toml` |
| 일반적인 라이브러리 크기 | 큰 편       | 작게                 |
| 모든 종속성들            | 적음        | 1억+                 |

Chromium 엔지니어에게는 다음과 같은 장단점이 있습니다.

- 모든 크레이트는 공통 빌드 시스템을 사용하므로 Chromium에 자동으로 포함할 수 있습니다.
- 그러나 크레이트에는 일반적으로 전이 종속 항목이 있으므로 여러 라이브러리를 가져와야 할 수 있습니다.

다룰 내용은 다음과 같습니다.

- Chromium 소스 코드 트리에 크레이트를 추가하는 방법
- 이를 위해 `gn` 빌드 규칙을 만드는 방법
- 충분한 안전성을 위해 소스 코드를 감사하는 방법

> **발표자 노트**
>
> All of the things in the table on this slide are generalizations, and counter-examples can be found. But in general it's important for students to understand that most Rust code depends on other Rust libraries, because it's easy to do so, and that this has both benefits and costs.
