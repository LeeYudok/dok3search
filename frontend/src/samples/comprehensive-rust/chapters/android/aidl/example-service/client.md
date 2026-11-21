# AIDL 클라이언트

마지막으로, 아까 추가한 서비스에 대한 클라이언트를 러스트로 만들겠습니다.

*birthday_service/src/client.rs*:

``` rust
use com_example_birthdayservice::aidl::com::example::birthdayservice::IBirthdayService::IBirthdayService;
use com_example_birthdayservice::binder;

const SERVICE_IDENTIFIER: &str = "birthdayservice";

/// 생일 서비스를 호출합니다.
fn main() -> Result<(), Box<dyn Error>> {
    let name = std::env::args().nth(1).unwrap_or_else(|| String::from("Bob"));
    let years = std::env::args()
        .nth(2)
        .and_then(|arg| arg.parse::<i32>().ok())
        .unwrap_or(42);

    binder::ProcessState::start_thread_pool();
    let service = binder::get_interface::<dyn IBirthdayService>(SERVICE_IDENTIFIER)
        .map_err(|_| "BirthdayService에 연결할 수 없습니다.")?;

    // Call the service.
    let msg = service.wishHappyBirthday(&name, years)?;
    println!("{msg}");
}
```

*birthday_service/Android.bp*:

``` javascript
rust_binary {
    name: "birthday_client",
    crate_name: "birthday_client",
    srcs: ["src/client.rs"],
    rustlibs: [
        "com.example.birthdayservice-rust",
        "libbinder_rs",
    ],
    prefer_rlib: true, // To avoid dynamic link error.
}
```

클라이언트는 `libbirthdayservice`에 의존하지 않음에 주목하세요.

빌드하고, 가상 디바이스로 넣고, 실행합니다:

``` shell
m birthday_client
adb push "$ANDROID_PRODUCT_OUT/system/bin/birthday_client" /data/local/tmp
adb shell /data/local/tmp/birthday_client Charlie 60
```

``` text
Happy Birthday Charlie, congratulations with the 60 years!
```

> **발표자 노트**
>
> - `Strong<dyn IBirthdayService>` is the trait object representing the service that the client has connected to.
>   - `Strong` is a custom smart pointer type for Binder. It handles both an in-process ref count for the service trait object, and the global Binder ref count that tracks how many processes have a reference to the object.
>   - Note that the trait object that the client uses to talk to the service uses the exact same trait that the server implements. For a given Binder interface, there is a single Rust trait generated that both client and server use.
> - Use the same service identifier used when registering the service. This should ideally be defined in a common crate that both the client and server can depend on.
