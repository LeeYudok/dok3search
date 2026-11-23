# Rust 강의 14편 — 스마트 포인터와 Drop

> 주인이 하나라는 규칙으로는 그리기 어려운 구조, 예를 들어 자기를 품는 타입이나 여러 곳이 함께 쓰는 데이터는 어떻게 만들까?

답부터 적습니다. 크기를 정할 수 없는 재귀 타입은 `Box` 로 힙에 두어 크기를 포인터 하나로 고정합니다. 여러 곳이 함께 소유해야 하면 `Rc` 로 주인 수를 세고, 마지막 주인이 놓을 때 해제합니다. `Rc` 끼리 서로를 붙잡으면 수가 0 이 되지 않아 영영 해제되지 않습니다. 그래서 거꾸로 가리키는 쪽은 수를 세지 않는 `Weak` 로 둡니다. 해제는 모두 `Drop` 이 맡습니다. 지역 변수는 선언의 역순, 구조체 필드는 선언 순서로 버려지고, 패닉으로 풀릴 때도 돕니다.

> [그림] 왼쪽 공유 문서에 주인 a 와 b 가 끈으로 이어져 strong 2 꼬리표가 붙어 있고 검수 로봇이 그 꼬리표를 본다. 가운데 x 와 y 가 서로를 강한 끈으로 붙잡아 drop 이 안 불린다. 오른쪽 1장이 1.1절을 품고 1.1절은 점선 Weak 끈으로 부모를 가리켜 둘 다 해제된다

세 장면은 이 글의 실행 결과 그대로입니다. 가운데 순환에서는 drop 메시지가 끝내 찍히지 않았습니다.

Rust 강의 14편입니다. [2편](https://blog.doksam.com/blog/rust-course-02-ownership/)에서 주인은 하나이고 범위가 끝나면 버려진다고 했습니다. 이번 편은 그 규칙을 지키면서 더 복잡한 구조를 만드는 도구를 봅니다. 코드는 2026-11-18 에 rustc 1.100.0 으로 돌려 확인했습니다.

## Box 는 크기를 모르는 재귀 타입을 포인터 하나로 만듭니다

문서 목차는 항목 안에 하위 항목이 있습니다. 그대로 적으면 막힙니다.

```rust
enum Toc {
    Item(String, Toc),
    End,
}
```

```console
error[E0072]: recursive type `Toc` has infinite size
 --> src/main.rs:2:1
  |
2 | enum Toc {
  | ^^^^^^^^
3 |     Item(String, Toc),
  |                  --- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
3 |     Item(String, Box<Toc>),
```

`Toc` 의 크기를 계산하려면 `Toc` 의 크기가 필요해서 끝나지 않습니다. `Box<Toc>` 는 힙 주소 하나라 크기가 정해집니다. 실측에서 `Box<Toc>` 는 8바이트, `Toc` 는 32바이트(`String` 24 + `Box` 8)였습니다. 컴파일러가 `Rc` 나 `&` 도 함께 권하는 것은 셋 다 "다른 곳을 가리키는 고정 크기 값"이기 때문입니다.

`Box` 는 주인이 하나인 힙 값입니다. 4편의 `Box<dyn Parser>` 처럼 크기가 다른 타입을 한 자리에 담을 때도 씁니다.

## Rc 는 주인 수를 세고 마지막 주인이 놓을 때 해제합니다

```rust
    let a = Rc::new(Noisy("공유 문서"));
    let b = Rc::clone(&a);
    println!("  strong = {}", Rc::strong_count(&a));
    drop(a);
    println!("  a 를 버린 뒤 strong = {}", Rc::strong_count(&b));
    drop(b);
    println!("  b 까지 버렸다");
```

```console
  strong = 2
  a 를 버린 뒤 strong = 1
  drop 공유 문서
  b 까지 버렸다
```

`Rc::clone` 은 데이터를 복사하지 않고 수만 하나 올립니다. 100MB 벡터로 쟀더니 `Rc::clone` 은 42나노초(타이머 오차 수준), 깊은 복제는 4.9밀리초였습니다. 2편의 이동과 `clone()` 차이와 같은 크기입니다. `Rc<String>` 자체는 8바이트로, 힙에 놓인 수와 값을 가리키는 포인터 하나입니다.

`Rc` 의 수는 원자 연산이 아닌 보통 정수입니다. 그래서 다른 스레드로 보낼 수 없습니다.

```console
error[E0277]: `Rc<String>` cannot be sent between threads safely
...
  = help: within `{closure@src/main.rs:7:27: 7:34}`, the trait `Send` is not implemented for `Rc<String>`
```

스레드 사이에서는 원자 연산으로 세는 `Arc` 를 씁니다(15·17편). 한 스레드 안이라면 `Rc` 가 원자 연산 비용을 내지 않습니다.

## 강한 순환은 해제되지 않습니다

서로를 `Rc` 로 가리키는 두 노드입니다. 만든 뒤에 연결하려면 내용을 고칠 수 있어야 해서 `RefCell` 을 썼습니다(15편에서 다룹니다).

```rust
struct Strong {
    name: &'static str,
    next: RefCell<Option<Rc<Strong>>>,
}

    {
        let x = Rc::new(Strong { name: "x", next: RefCell::new(None) });
        let y = Rc::new(Strong { name: "y", next: RefCell::new(Some(Rc::clone(&x))) });
        *x.next.borrow_mut() = Some(Rc::clone(&y));
        println!("  x strong = {}, y strong = {}", Rc::strong_count(&x), Rc::strong_count(&y));
    }
    println!("  블록을 나왔다 (drop x, drop y 가 없다)");
```

```console
  x strong = 2, y strong = 2
  블록을 나왔다 (drop x, drop y 가 없다)
```

블록이 끝나며 지역 변수 `x`, `y` 가 수를 하나씩 내렸지만, 서로가 붙잡은 하나씩이 남아 1 에서 멈췄습니다. 두 노드는 프로그램이 끝날 때까지 메모리에 남습니다. Rust 의 안전 보장에 메모리 누수 방지는 들어 있지 않습니다. The Book 15.6절도 `Rc` 와 `RefCell` 로 순환을 만들면 누수가 난다고 적습니다. 컴파일러는 이 코드를 막지 않았습니다.

## 거꾸로 가리키는 쪽은 Weak 로 둡니다

문서 구조의 장·절처럼 부모가 자식을 소유하고 자식이 부모를 알아야 하는 경우입니다. 자식에서 부모로 가는 끈은 `Weak` 로 둡니다.

```rust
struct Section {
    name: &'static str,
    parent: RefCell<Weak<Section>>,
    children: RefCell<Vec<Rc<Section>>>,
}

impl Section {
    fn parent_name(&self) -> Option<&'static str> {
        self.parent.borrow().upgrade().map(|p| p.name)
    }
}
```

`Weak` 는 수를 세지 않습니다. 대신 쓸 때마다 `upgrade()` 로 "아직 살아 있으면 `Rc` 를 달라"고 묻고 `Option` 을 받습니다.

```console
  root strong = 1, weak = 1
  child 의 부모: Some("1장")
  drop 1장
  root 가 사라진 뒤 부모: None
  drop 1.1절
```

`root` 의 강한 수는 1(변수 하나)이고 약한 수가 1(자식의 끈)입니다. 블록이 끝나자 1장이 바로 해제됐고, 살아남은 자식의 `upgrade()` 는 `None` 이었습니다. 설계 규칙으로 줄이면 이렇습니다. 소유하는 방향은 `Rc`, 거슬러 올라가는 방향은 `Weak` 입니다.

## Drop 은 역순으로, 필드는 선언 순서로, 패닉 중에도 돕니다

2편에서 지역 변수는 선언의 역순으로 버려졌습니다. 구조체 필드는 반대로 선언 순서입니다.

```rust
struct Page {
    header: Noisy,
    body: Noisy,
    footer: Noisy,
}
```

`Page` 하나를 만들었다 버리면 `drop header`, `drop body`, `drop footer` 순서로 찍혔습니다. The Rust Reference 의 "Destructors" 절이 이 두 순서를 규칙으로 정해 둡니다.

`Drop` 이 확실히 불린다는 성질을 이용하는 것이 RAII(자원 획득이 곧 초기화)입니다. 자원을 값에 묶어 두면 값이 사라질 때 자원도 풀립니다. 범위를 벗어나면 임시 파일을 지우는 가드를 만들어 패닉 중에 시험했습니다.

```rust
struct TempFile(std::path::PathBuf);

impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
        println!("  임시 파일 지움");
    }
}

    let r = std::panic::catch_unwind(|| {
        let tmp = TempFile::create("rc-ep14-work.bin", b"partial").unwrap();
        let _n = Noisy("임시 버퍼");
        assert!(tmp.0.exists());
        panic!("망가진 입력");
    });
```

```console
thread 'main' (30785264) panicked at src/main.rs:176:9:
망가진 입력
  drop 임시 버퍼
  임시 파일 지움
  catch_unwind 결과 is_err = true, 파일 남음 = false
```

패닉으로 함수가 풀리는 동안에도 지역 변수가 역순으로 버려졌고 임시 파일이 지워졌습니다. 변환 도중 망가진 입력에서 패닉이 나도 찌꺼기 파일이 남지 않습니다. 다만 `panic = "abort"`(6편의 `release-small` 프로필)로 빌드하면 풀지 않고 바로 끝나므로 `Drop` 도 돌지 않습니다. 같은 코드를 그 설정으로 돌리자 `catch_unwind` 결과도 찍히지 않고 임시 파일이 남았습니다.

## 면접에서 이렇게 묻는다

**Q1. `Box` 는 언제 쓰나요?**
재귀 타입처럼 크기를 정할 수 없을 때, `Box<dyn Trait>` 로 크기가 다른 타입을 한 자리에 담을 때, 큰 값을 스택 대신 힙에 둘 때 씁니다. 주인은 하나이고 비용은 할당 한 번입니다.

**Q2. `Rc` 와 `Arc` 는 무엇이 다른가요?**
둘 다 주인 수를 세는 공동 소유입니다. `Rc` 는 보통 정수로 세서 빠르지만 `Send` 가 아니라 스레드를 넘지 못합니다. `Arc` 는 원자 연산으로 세서 스레드 사이에서 씁니다.

**Q3. Rust 에서 메모리 누수가 날 수 있나요?**
납니다. `Rc` 강한 순환, `mem::forget`, `Box::leak` 이 대표입니다. 누수는 메모리 안전 위반이 아니라서 안전한 코드로도 생깁니다. 실험에서 강한 순환은 drop 이 끝내 불리지 않았습니다.

**Q4. 순환 참조는 어떻게 피하나요?**
소유하는 방향은 `Rc`, 거꾸로 가리키는 방향은 `Weak` 로 둡니다. `Weak::upgrade()` 가 `Option<Rc<T>>` 를 돌려줘 이미 해제됐는지 확인합니다.

**Q5. `Drop` 이 불리지 않는 경우는?**
`mem::forget`, 강한 순환, `process::exit`, `panic = "abort"` 로 끝날 때입니다. 그래서 `Drop` 은 정리 수단이지 "반드시 실행된다"는 보장으로 쓰면 안 됩니다.

## 연습

**연습 1.** 자식 절에서 부모 장의 이름을 돌려주는 `parent_name` 을 쓰고, 부모가 사라진 뒤 `None` 이 나오는지 확인하세요.

<details>
<summary>풀이 보기</summary>

본문의 `parent_name` 입니다. `upgrade()` 가 `Option<Rc<Section>>` 을 주므로 `map` 으로 이름만 꺼냅니다. 실행에서 부모가 있을 때 `Some("1장")`, 사라진 뒤 `None` 이었습니다. 반환 타입을 `Option<Rc<Section>>` 으로 하면 호출자가 부모를 붙잡아 수명을 늘릴 수 있으니, 필요한 값만 돌려주는 편이 안전합니다.

</details>

**연습 2.** 범위를 벗어나면 임시 파일을 지우는 가드 `TempFile` 을 쓰고, 패닉 중에도 지워지는지 시험하세요.

<details>
<summary>풀이 보기</summary>

```rust
impl TempFile {
    fn create(name: &str, data: &[u8]) -> std::io::Result<TempFile> {
        let path = std::env::temp_dir().join(name);
        std::fs::write(&path, data)?;
        Ok(TempFile(path))
    }
}
```

`Drop` 은 본문 그대로입니다. `drop` 안에서는 에러를 돌려줄 수 없으니 `let _ =` 로 지우기 실패를 무시했습니다. 실패를 알려야 한다면 따로 `close(self) -> io::Result<()>` 메서드를 두고 `Drop` 은 마지막 안전망으로 남깁니다.

</details>

**연습 3.** 아래에서 찍히는 순서를 먼저 적으세요. `let p = Page { header: Noisy("h"), body: Noisy("b"), footer: Noisy("f") }; let x = Noisy("x");`

<details>
<summary>풀이 보기</summary>

`drop x`, `drop h`, `drop b`, `drop f` 입니다. 지역 변수는 역순이라 나중에 선언한 `x` 가 먼저이고, `p` 를 버릴 때는 필드가 선언 순서로 버려집니다. 본문의 `Page` 실행이 필드 순서를, 2편 실행이 지역 변수 순서를 보여 줍니다.

</details>

## 한 단계 더 — Rc 의 수는 어디에 있을까

`Rc<T>` 가 8바이트인 이유는 수가 포인터 옆이 아니라 힙 쪽에 있기 때문입니다. 표준 라이브러리 문서는 `Rc` 가 힙에 할당한 값에 강한 수와 약한 수를 함께 둔다고 설명합니다. 그래서 `Rc::clone` 은 같은 힙 칸의 수만 고치고, 모든 복제본이 같은 수를 봅니다.

`Weak` 가 남아 있으면 무슨 일이 생길까요. 강한 수가 0 이 되면 값의 `Drop` 이 돌고 값은 버려집니다. 하지만 수가 들어 있는 힙 칸은 약한 수까지 0 이 될 때 풀립니다. 그래야 남은 `Weak` 가 `upgrade()` 할 때 "이미 죽었다"를 확인할 수 있습니다. 위 실험에서 1장의 drop 메시지가 자식보다 먼저 찍힌 것이 앞부분이고, 뒷부분(힙 칸 해제 시점)은 직접 관찰하지 않았습니다. 이 설명은 std 문서의 `Rc`·`Weak` 절에 근거합니다.

## 정리

주인이 하나라는 규칙으로 그리기 어려운 구조는 스마트 포인터로 만듭니다. 크기가 문제면 `Box`, 공동 소유면 `Rc`, 거꾸로 가리키면 `Weak` 입니다. 정리는 `Drop` 에 맡기되, `Drop` 이 안 도는 경우를 알고 씁니다.

15편에서는 이 편에서 미뤄 둔 `RefCell` 을 다룹니다. `&` 로 받고도 고칠 수 있게 하는 내부 가변성, `Cell`·`RefCell`·`Mutex`·`RwLock` 의 차이와 실행 중 빌림 검사의 비용을 봅니다.
