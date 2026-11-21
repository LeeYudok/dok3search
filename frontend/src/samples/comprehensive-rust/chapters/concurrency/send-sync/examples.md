# 예제

## `Send + Sync`

여러분이 다루게 될 대부분의 타입은 `Send + Sync`입니다:

- `i8`, `f32`, `bool`, `char`, `&str`, …
- `(T1, T2)`, `[T; N]`, `&[T]`, `struct { x: T }`, …
- `String`, `Option<T>`, `Vec<T>`, `Box<T>`, …
- `Arc<T>`: 참조 카운트 조작을 아토믹 하기 때문에 스레드 안전함.
- `Mutex<T>`: 값을 접근하기 위해 뮤택스를 잠궈야 하기 때문에 스레드 안전함.
- `AtomicBool`, `AtomicU8`, …: 값을 접근할 때 특별한 아토믹 명령어들을 사용합니다.

제네릭 타입은 일반적으로 타입 파라메터가 `Send + Sync`이면 `Send + Sync` 입니다.

## `Send + !Sync`

아래 타입들은 다른 스레드로 이동될 수 있지만 내부적으로 값이 변경될 수 있기 때문에 스레드 안전하지 않습니다:

- `mpsc::Sender<T>`
- `mpsc::Receiver<T>`
- `Cell<T>`
- `RefCell<T>`

## `!Send + Sync`

아래 타입들은 스레드 안전하지만 다른 스레드로 이동될 수 없습니다:

- `MutexGuard<T: Sync>`: Uses OS level primitives which must be deallocated on the thread which created them.

## `!Send + !Sync`

아래 타입들은 스레드 안전하지도 않고 다른 스레드로 이동될 수도 없습니다:

- `Rc<T>`: `Rc<T>` 는 아토믹하지 않은 방식으로 참조 카운트를 조작하는 `RcBox<T>`를 참조합니다.
- `*const T`, `*mut T`: 러스트는 포인터가 스레드 안전하지 않다고 가정합니다.
