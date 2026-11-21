# 트레잇(Trait)

트레잇은 타입을 추상화 하는데 사용됩니다. 인터페이스와 비슷합니다:

``` rust
trait Pet {
    /// Return a sentence from this pet.
    fn talk(&self) -> String;

    /// Print a string to the terminal greeting this pet.
    fn greet(&self);
}
```

> **발표자 노트**
>
> This slide and its sub-slides should take about 15 minutes.
>
> - 트레잇은 해당 트레잇을 구현하기 위해 타입이 가져야 하는 여러 메서드를 정의합니다.
>
> - In the “Generics” segment, next, we will see how to build functionality that is generic over all types implementing a trait.
