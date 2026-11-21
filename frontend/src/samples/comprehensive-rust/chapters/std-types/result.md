# Option, Result

`Result`는 `Option`과 유사하지만 작업의 성공 또는 실패를 나타내며, 각각 타입이 다릅니다. 이는 표현식 연습에서 정의된 `Res`와 유사하지만 제네릭: `Result<T, E>`입니다. 여기서 `T`는 `Ok` 변형에 사용되고 `E`는 `Err` 변형에 표시됩니다.

``` rust
use std::fs::File;
use std::io::Read;

fn main() {
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("다이어리: {contents}({bytes}바이트)");
            } else {
                println!("파일 콘텐츠를 읽을 수 없습니다.");
            }
        }
        Err(err) => {
            println!("다이어리를 열 수 없습니다. {err}");
        }
    }
}
```

> **발표자 노트**
>
> This slide should take about 10 minutes.
>
> - `Option`와 마찬가지로, 성공한 경우의 값은 `Result` 내부에 있습니다. 그래서, 개발자는 명시적으로 이를 추출하여야 합니다. 이렇게 함으로써 값을 읽기 전에 오류 발생 여부를 반드시 체크하도록 유도하고 있습니다. 만일 오류가 절대 발생하지 않는 경우라면 `unwrap()`이나 `expect()`를 사용할 수 있으며, 이는 개발자의 의도(*역주*: 오류가 발생할 수 없음)을 명시적으로 나타내는 방법이기도 합니다.
> - `Result` documentation is a recommended read. Not during the course, but it is worth mentioning. It contains a lot of convenience methods and functions that help functional-style programming.
> - `Result`는 오류 처리를 위한 표준 타입입니다. 3일차 과정에서 살펴봅니다.
