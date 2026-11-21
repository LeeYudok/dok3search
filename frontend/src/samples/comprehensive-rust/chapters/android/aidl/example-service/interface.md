# AIDL 인터페이스

AIDL 인터페이스를 이용해서 서비스의 API를 선언합니다:

*birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl*:

``` java
/** 생일 서비스 인터페이스입니다. */
interface IBirthdayService {
    /** 생일 축하 메시지를 생성합니다. */
    String wishHappyBirthday(String name, int years);
}
```

*birthday_service/aidl/Android.bp*:

``` javascript
aidl_interface {
    name: "com.example.birthdayservice",
    srcs: ["com/example/birthdayservice/*.aidl"],
    unstable: true,
    backend: {
        rust: { // Rust는 기본적으로 사용 설정되지 않습니다.
            enabled: true,
        },
    },
}
```

> **발표자 노트**
>
> - Note that the directory structure under the `aidl/` directory needs to match the package name used in the AIDL file, i.e. the package is `com.example.birthdayservice` and the file is at `aidl/com/example/IBirthdayService.aidl`.
