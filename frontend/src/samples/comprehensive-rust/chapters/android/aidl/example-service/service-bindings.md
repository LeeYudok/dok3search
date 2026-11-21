# Generated Service API

Binder generates a trait corresponding to the interface definition. trait to talk to the service.

*birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl*:

``` java
/** 생일 서비스 인터페이스입니다. */
interface IBirthdayService {
    /** 생일 축하 메시지를 생성합니다. */
    String wishHappyBirthday(String name, int years);
}
```

*Generated trait*:

``` rust
trait IBirthdayService {
    fn wishHappyBirthday(&self, name: &str, years: i32) -> binder::Result<String>;
}
```

Your service will need to implement this trait, and your client will use this trait to talk to the service.

> **발표자 노트**
>
> - The generated bindings can be found at `out/soong/.intermediates/<path to module>/`.
> - Point out how the generated function signature, specifically the argument and return types, correspond the interface definition.
>   - `String` for an argument results in a different Rust type than `String` as a return type.
