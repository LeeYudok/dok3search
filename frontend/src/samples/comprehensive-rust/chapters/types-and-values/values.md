# 값

다음은 몇 가지 기본 내장 타입과 각 타입의 리터럴 값 문법입니다.

|  | 타입 | 리터럴 값 |
|----|----|----|
| 부호있는 정수 | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123_i64` |
| 부호없는 정수 | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10_u16` |
| 부동소수 | `f32`, `f64` | `3.14`, `-10.0e20`, `2_f32` |
| 유니코드 문자 | `char` | `'a'`, `'α'`, `'∞'` |
| 불리언 | `bool` | `true`, `false` |

각 타입의 크기는 다음과 같습니다:

- `iN`, `uN`, `fN`은 모두 \_N_비트 입니다.
- `isize` 와 `usize` 는 포인터와 같은 크기입니다,
- `char` 32 비트 입니다,
- `bool`은 8 비트 입니다.

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> 위에 표시되지 않은 몇 가지 문법이 있습니다:
>
> - All underscores in numbers can be left out, they are for legibility only. So `1_000` can be written as `1000` (or `10_00`), and `123_i64` can be written as `123i64`.
