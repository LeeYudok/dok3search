# Sending Files

Files can be sent between Binder clients/servers using the `ParcelFileDescriptor` type:

**birthday_service/aidl/com/example/birthdayservice/IBirthdayService.aidl**:

``` java
interface IBirthdayService {
    /** The same thing, but loads info from a file. */
    String wishFromFile(in ParcelFileDescriptor infoFile);
}
```

**birthday_service/src/client.rs**:

``` rust
fn main() {
    binder::ProcessState::start_thread_pool();
    let service = connect().expect("BirthdayService에 연결할 수 없습니다.");

    // Open a file and put the birthday info in it.
    let mut file = File::create("/data/local/tmp/birthday.info").unwrap();
    writeln!(file, "{name}")?;
    writeln!(file, "{years}")?;

    // Create a `ParcelFileDescriptor` from the file and send it.
    let file = ParcelFileDescriptor::new(file);
    service.wishFromFile(&file)?;
}
```

**birthday_service/src/lib.rs**:

``` rust
impl IBirthdayService for BirthdayService {
    fn wishFromFile(
        &self,
        info_file: &ParcelFileDescriptor,
    ) -> binder::Result<String> {
        // Convert the file descriptor to a `File`. `ParcelFileDescriptor` wraps
        // an `OwnedFd`, which can be cloned and then used to create a `File`
        // object.
        let mut info_file = info_file
            .as_ref()
            .try_clone()
            .map(File::from)
            .expect("Invalid file handle");

        let mut contents = String::new();
        info_file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap();
        let years: i32 = lines.next().unwrap().parse().unwrap();

        Ok(format!("{name}님의 생일을 축하합니다. {years}주년을 축하합니다."))
    }
}
```

> **발표자 노트**
>
> - `ParcelFileDescriptor` wraps an `OwnedFd`, and so can be created from a `File` (or any other type that wraps an `OwnedFd`), and can be used to create a new `File` handle on the other side.
> - Other types of file descriptors can be wrapped and sent, e.g. TCP, UDP, and UNIX sockets.
