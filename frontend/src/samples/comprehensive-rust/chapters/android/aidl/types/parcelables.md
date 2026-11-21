# 변수

Binder for Rust supports sending parcelables directly:

**birthday_service/aidl/com/example/birthdayservice/BirthdayInfo.aidl**:

``` java
package com.example.birthdayservice;

parcelable BirthdayInfo {
    String name;
    int years;
}
```

**birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl**:

``` java
import com.example.birthdayservice.BirthdayInfo;

interface IBirthdayService {
    /** The same thing, but with a parcelable. */
    String wishWithInfo(in BirthdayInfo info);
}
```

**birthday_service/src/client.rs**:

``` rust
fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("BirthdayService에 연결할 수 없습니다.");

    service.wishWithInfo(&BirthdayInfo { name: name.clone(), years })?;
}
```
