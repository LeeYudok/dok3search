# 연습문제: 수식 계산

Let’s write a simple recursive evaluator for arithmetic expressions.

여기서 `Box` 타입은 스마트 포인터이며 이 과정의 후반부에서 자세히 다룹니다. 테스트에서 볼 수 있듯이 표현식은 `Box::new`를 사용하여 “박스로 표시“할 수 있습니다. 박스로 표시된 표현식을 평가하려면 deref 연산자 (`*`)를 사용하여 “박스 표시를 해제“합니다: `eval(*boxed_expr)`.

일부 표현식은 평가할 수 없으며 오류를 반환합니다. 표준 [`Result<Value, String>`](https://doc.rust-lang.org/std/result/enum.Result.html) 타입은 성공한 값을 나타내거나 (`Ok(Value)`) 메시지와 함께 오류를 나타냅니다 (`Err(String)`). 나중에 이 Result 타입에 대해서 자세히 살펴보겠습니다.

코드를 복사하여 Rust 플레이그라운드에 붙여넣고 `eval` 구현을 시작합니다. 최종 생성물은 테스트를 통과해야 합니다. `todo!()`를 사용하고 테스트를 하나씩 통과하도록 하면 도움이 될 수 있습니다. `#[ignore]`를 사용하여 테스트를 일시적으로 건너뛸 수도 있습니다.

``` none
#[test]
#[ignore]
fn test_value() { .. }
```

일찍 완료한 경우 0으로 나누기나 정수 오버플로가 발생하는 테스트를 작성해 보세요. 패닉 대신 `Result`로 이 문제를 어떻게 처리할 수 있을까요?

``` rust
#![allow(unused)]
fn main() {
/// 두 개의 하위 표현식에서 실행할 연산입니다.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// 트리 형식의 표현식입니다.
#[derive(Debug)]
enum Expression {
    /// 두 개의 하위 표현식에 관한 연산입니다.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// 리터럴 값
    Value(i64),
}

fn eval(e: Expression) -> Result<i64, String> {
    todo!()
}

#[test]
fn test_value() {
    assert_eq!(eval(Expression::Value(19)), Ok(19));
}

#[test]
fn test_sum() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        }),
        Ok(30)
    );
}

#[test]
fn test_recursion() {
    let term1 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Value(10)),
        right: Box::new(Expression::Value(9)),
    };
    let term2 = Expression::Op {
        op: Operation::Mul,
        left: Box::new(Expression::Op {
            op: Operation::Sub,
            left: Box::new(Expression::Value(3)),
            right: Box::new(Expression::Value(4)),
        }),
        right: Box::new(Expression::Value(5)),
    };
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Add,
            left: Box::new(term1),
            right: Box::new(term2),
        }),
        Ok(85)
    );
}

#[test]
fn test_error() {
    assert_eq!(
        eval(Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(99)),
            right: Box::new(Expression::Value(0)),
        }),
        Err(String::from("0으로 나누기"))
    );
}
}
```
