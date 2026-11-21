# Updating Client and Service

Update the client and server code to account for the new API.

*birthday_service/src/lib.rs*:

``` rust
impl IBirthdayService for BirthdayService {
    fn wishHappyBirthday(
        &self,
        name: &str,
        years: i32,
        text: &[String],
    ) -> binder::Result<String> {
        let mut msg = format!(
            "{name}님의 생일을 축하합니다. {years}주년을 축하합니다.",
        );

        for line in text {
            msg.push('\n');
            msg.push_str(line);
        }

        Ok(msg)
    }
}
```

*birthday_service/src/client.rs*:

``` rust
let msg = service.wishHappyBirthday(
    &name,
    years,
    &[
        String::from("Habby birfday to yuuuuu"),
        String::from("And also: many more"),
    ],
)?;
```

> **발표자 노트**
>
> - TODO: Move code snippets into project files where they’ll actually be built?
