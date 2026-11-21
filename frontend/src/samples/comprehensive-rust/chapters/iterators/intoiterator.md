# `IntoIterator`

The `Iterator` trait tells you how to *iterate* once you have created an iterator. The related trait [`IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html) defines how to create an iterator for a type. It is used automatically by the `for` loop.

``` rust
struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter { grid: self, i: 0, j: 0 }
    }
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn main() {
    let grid = Grid { x_coords: vec![3, 5, 7, 9], y_coords: vec![10, 20, 30, 40] };
    for (x, y) in grid {
        println!("point = {x}, {y}");
    }
}
```

> **발표자 노트**
>
> This slide should take about 5 minutes.
>
> Click through to the docs for `IntoIterator`. Every implementation of `IntoIterator` must declare two types:
>
> - `Item`: the type to iterate over, such as `i8`,
> - `IntoIter`: `into_iter` 메서드에서 반환되는 `Iterator`타입.
>
> `IntoIter`에는 `Item`이 연결되어 있음을 주목하세요. `IntoIter` 반복자는 `Item` 타입의 데이터를 가리켜야 합니다. 즉, 반복자는 `Option<Item>`을 리턴합니다
>
> 이 예는 x 및 y 좌표의 모든 조합을 순회합니다.
>
> `main`에서 그리드를 두 번 반복해 보세요. 왜 실패하나요? `IntoIterator::into_iter`는 ’self’의 소유권을 가져옵니다.
>
> `&Grid`에 `IntoIterator`를 구현하고 `GridIter`에 `Grid` 참조를 저장하여 이 문제를 해결하세요.
>
> 표준 라이브러리 타입에서 동일한 문제가 발생할 수 있습니다. `for e in some_vector`는 `some_vector`의 소유권을 가져와 해당 벡터에서 소유한 요소를 반복합니다. `some_vector`의 요소에 대한 참조를 반복하려면 대신 `for e in &some_vector`를 사용하세요.
