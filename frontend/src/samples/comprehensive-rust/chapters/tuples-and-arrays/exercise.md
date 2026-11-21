# 연습문제: 중첩 배열

배열에는 다음과 같은 다른 배열이 포함될 수 있습니다.

``` rust
#![allow(unused)]
fn main() {
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
}
```

What is the type of this variable?

Use an array such as the above to write a function `transpose` which will transpose a matrix (turn rows into columns):

![2 · 5 · 8 · 4 · 6 · 7⎤ · 8⎥ · 9⎦ · transpose · == · ⎛ · ⎡ · 1 · ⎜ · ⎢ · ⎝ · ⎣ · 7 · 3⎤ · ⎞ · 6⎥ · ⎟ · ⎠ · 3](../../assets/tuples-and-arrays/exercise-diagram-1.svg)

두 함수 모두 행렬의 크기는 3 x 3 으로 하드코딩 합니다.

아래 코드를 <https://play.rust-lang.org/>에 복사해서 구현하시면 됩니다:

패닉 발생 예제

``` rust
// TODO: 구현이 완료되면 이를 삭제합니다.
#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    unimplemented!()
}

#[test]
fn test_transpose() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];
    let transposed = transpose(matrix);
    assert_eq!(
        transposed,
        [
            [101, 201, 301], //
            [102, 202, 302],
            [103, 203, 303],
        ]
    );
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- 주석으로 rustfmt가 줄바꿈을 추가합니다.
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("행렬: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("전치행렬: {:#?}", transposed);
}
```
