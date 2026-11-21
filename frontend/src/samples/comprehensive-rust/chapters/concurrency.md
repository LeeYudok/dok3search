# Welcome to Concurrency in Rust

러스트는 동시성 지원이 막강합니다. 운영체제 레벨의 스레드를 사용하며, 뮤택스와 채널도 지원합니다.

러스트의 타입 시스템은 프로그램에 동시성 버그가 있을 경우, 컴파일 시에 에러가 발생하도록 해 줍니다. 컴파일러를 이용해서 프로그램이 수행시에 정확히 동작함을 미리 보장해 주기 때문에, 사람들은 이를 종종 *겁없는 동시성* 이라고 합니다.

> **발표자 노트**
>
> - Rust lets us access OS concurrency toolkit: threads, sync. primitives, etc.
> - The type system gives us safety for concurrency without any special features.
> - The same tools that help with “concurrent” access in a single thread (e.g., a called function that might mutate an argument or save references to it to read later) save us from multi-threading issues.
