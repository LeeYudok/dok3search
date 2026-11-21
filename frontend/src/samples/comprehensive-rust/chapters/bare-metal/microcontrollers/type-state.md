# 타입 상태 패턴

컴파일 오류 예제

``` rust
#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let gpio0 = p0::Parts::new(p.P0);

    let pin: P0_01<Disconnected> = gpio0.p0_01;

    // let gpio0_01_again = gpio0.p0_01; // 오류가 발생하여 이동했습니다.
    let pin_input: P0_01<Input<Floating>> = pin.into_floating_input();
    if pin_input.is_high().unwrap() {
        // ...
    }
    let mut pin_output: P0_01<Output<OpenDrain>> = pin_input
        .into_open_drain_output(OpenDrainConfig::Disconnect0Standard1, Level::Low);
    pin_output.set_high().unwrap();
    // pin_input.is_high(); // 오류가 발생하여 이동했습니다.

    let _pin2: P0_02<Output<OpenDrain>> = gpio0
        .p0_02
        .into_open_drain_output(OpenDrainConfig::Disconnect0Standard1, Level::Low);
    let _pin3: P0_03<Output<PushPull>> =
        gpio0.p0_03.into_push_pull_output(Level::Low);

    loop {}
}
```

> **발표자 노트**
>
> - 핀은 `Copy` 또는 `Clone`을 구현하지 않으므로 각각 하나의 인스턴스만 존재할 수 있습니다. 핀을 포트 구조체 밖으로 이동하면 아무도 사용할 수 없습니다.
> - 핀 구성을 변경하면 이전 핀 인스턴스가 사용되므로 이후에 이전 인스턴스를 계속 사용할 수 없습니다.
> - 값 타입은 현재 상태를 나타냅니다. 예를 들어 이 경우에는 GPIO 핀의 구성 상태입니다. 이렇게 하면 상태 머신이 타입 시스템으로 인코딩되며, 먼저 올바르게 구성한 후 특정 방식으로 핀을 사용하도록 합니다. 잘못된 상태 전환은 컴파일 시간에 포착됩니다.
> - 입력 핀에서 `is_high`를 호출하고 출력 핀에서 `set_high`를 호출할 수 있지만 그 반대로는 안 됩니다.
> - 많은 HAL 크레이트들이 이 패턴을 따릅니다.
