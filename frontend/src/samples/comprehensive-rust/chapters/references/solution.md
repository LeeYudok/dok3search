# 해답

``` rust
/// 지정된 벡터의 크기를 계산합니다.
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }
    mag_squared.sqrt()
}

/// 방향 변경 없이 벡터 크기를 1.0으로 변경합니다.
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}

fn main() {
    println!("단위 벡터의 크기: {}", magnitude(&[0.0, 1.0, 0.0]));

    let mut v = [1.0, 2.0, 9.0];
    println!("{v:?} 크기: {}", magnitude(&v));
    normalize(&mut v);
    println!("정규화 후 {v:?}의 크기: {}", magnitude(&v));
}
```
