# 서비스 구현

이제 AIDL서비스를 구현할 수 있습니다:

*birthday_service/src/lib.rs*:

``` rust
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::IBirthdayService;
use com_example_birthdayservice::binder;

/// The `IBirthdayService` implementation.
pub struct BirthdayService;

impl binder::Interface for BirthdayService {}

impl IBirthdayService for BirthdayService {
    fn wishHappyBirthday(&self, name: &str, years: i32) -> binder::Result<String> {
        Ok(format!("{name}님의 생일을 축하합니다. {years}주년을 축하합니다."))
    }
}
```

*birthday_service/Android.bp*:

``` javascript
rust_library {
    name: "libbirthdayservice",
    srcs: ["src/lib.rs"],
    crate_name: "birthdayservice",
    rustlibs: [
        "com.example.birthdayservice-rust",
        "libbinder_rs",
    ],
}
```

> **발표자 노트**
>
> - Point out the path to the generated `IBirthdayService` trait, and explain why each of the segments is necessary.
> - TODO: What does the `binder::Interface` trait do? Are there methods to override? Where source?
