# Result를 이용한 구조화된 오류처리

다음은 표현식 언어의 매우 간단한 파서를 구현합니다. 그러나 패닉을 통해 오류를 처리합니다. 대신 관용적인 오류 처리를 사용하고 오류를 `main`의 반환으로 전파하도록 다시 작성합니다. `thiserror` 및 `anyhow`를 얼마든지 사용하세요.

힌트: 먼저 `parse` 함수에서 오류 처리를 수정하세요. 제대로 작동하면 `Iterator<Item=Result<Token, TokenizerError>>`를 구현하도록 `Tokenizer`를 업데이트하고 파서에서 처리합니다.

``` rust
use std::iter::Peekable;
use std::str::Chars;

/// 산술 연산자입니다.
#[derive(Debug, PartialEq, Clone, Copy)]
enum Op {
    Add,
    Sub,
}

/// 표현식 언어의 토큰입니다.
#[derive(Debug, PartialEq)]
enum Token {
    Number(String),
    Identifier(String),
    Operator(Op),
}

/// 표현식 언어의 표현식입니다.
#[derive(Debug, PartialEq)]
enum Expression {
    /// 변수 참조입니다.
    Var(String),
    /// 리터럴 숫자입니다.
    Number(u32),
    /// 이진 연산입니다.
    Operation(Box<Expression>, Op, Box<Expression>),
}

fn tokenize(input: &str) -> Tokenizer {
    return Tokenizer(input.chars().peekable());
}

struct Tokenizer<'a>(Peekable<Chars<'a>>);

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let c = self.0.next()?;
        match c {
            '0'..='9' => {
                let mut num = String::from(c);
                while let Some(c @ '0'..='9') = self.0.peek() {
                    num.push(*c);
                    self.0.next();
                }
                Some(Token::Number(num))
            }
            'a'..='z' => {
                let mut ident = String::from(c);
                while let Some(c @ ('a'..='z' | '_' | '0'..='9')) = self.0.peek() {
                    ident.push(*c);
                    self.0.next();
                }
                Some(Token::Identifier(ident))
            }
            '+' => Some(Token::Operator(Op::Add)),
            '-' => Some(Token::Operator(Op::Sub)),
            _ => panic!("예기치 않은 문자 {c}"),
        }
    }
}

fn parse(input: &str) -> Expression {
    let mut tokens = tokenize(input);

    fn parse_expr<'a>(tokens: &mut Tokenizer<'a>) -> Expression {
        let Some(tok) = tokens.next() else {
            panic!("예기치 않은 입력 종료");
        };
        let expr = match tok {
            Token::Number(num) => {
                let v = num.parse().expect("잘못된 32비트 정수입니다.'");
                Expression::Number(v)
            }
            Token::Identifier(ident) => Expression::Var(ident),
            Token::Operator(_) => panic!("예기치 않은 토큰 {tok:?}"),
        };
        // 이진 연산이 있는 경우 이를 파싱합니다.
        match tokens.next() {
            None => expr,
            Some(Token::Operator(op)) => Expression::Operation(
                Box::new(expr),
                op,
                Box::new(parse_expr(tokens)),
            ),
            Some(tok) => panic!("예기치 않은 토큰 {tok:?}"),
        }
    }

    parse_expr(&mut tokens)
}

fn main() {
    let expr = parse("10+foo+20-30");
    println!("{expr:?}");
}
```
