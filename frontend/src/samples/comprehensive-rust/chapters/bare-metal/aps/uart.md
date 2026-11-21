# UART 드라이버 작성

QEMU의 ‘virt’ 보드에는 [PL011](https://developer.arm.com/documentation/ddi0183/g) UART가 있으므로 이를 위한 드라이버를 작성해 보겠습니다.

``` rust
const FLAG_REGISTER_OFFSET: usize = 0x18;
const FR_BUSY: u8 = 1 << 3;
const FR_TXFF: u8 = 1 << 5;

/// PL011 UART의 최소 드라이버입니다.
#[derive(Debug)]
pub struct Uart {
    base_address: *mut u8,
}

impl Uart {
    /// 지정된 기본 주소에 PL011 기기에 대한 UART 드라이버의 새 인스턴스를
 /// 생성합니다.
 ///
 /// # 안전
 ///
 /// 지정된 기본 주소는 PL011 기기의
 /// MMIO 제어 레지스터 8개를 가리켜야 하며,
 /// 이는 프로세스의 주소 공간에 기기 메모리로
 /// 매핑되어야 하며 다른 별칭은 없어야 합니다.
    pub unsafe fn new(base_address: *mut u8) -> Self {
        Self { base_address }
    }

    /// UART에 단일 바이트를 씁니다.
    pub fn write_byte(&self, byte: u8) {
        // TX 버퍼에 공간이 확보될 때까지 기다립니다.
        while self.read_flag_register() & FR_TXFF != 0 {}

        // 기본 주소가 적절하게 매핑된 PL011 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe {
            // TX 버퍼에 씁니다.
            self.base_address.write_volatile(byte);
        }

        // UART가 더 이상 사용 중이 아닐 때까지 기다립니다.
        while self.read_flag_register() & FR_BUSY != 0 {}
    }

    fn read_flag_register(&self) -> u8 {
        // 기본 주소가 적절하게 매핑된 PL011 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe { self.base_address.add(FLAG_REGISTER_OFFSET).read_volatile() }
    }
}
```

> **발표자 노트**
>
> - `Uart::new`는 안전하지 않지만(usafe), 그 외 다른 메서드들은 안전한(safe) 점에 주목하세요.다른 메서드들이 안전할 수 있는 이유는, `Uart::new`의 안전 요구사항(즉, 지정된 UART의 드라이버 인스턴스가 하나만 있으며 주소 공간에 별칭을 지정하는 다른 항목이 없음) 이 만족되기만 하면 `write_byte`와 같은 함수를 안전하게 호출하는데 있어서 필요한 모든 전제조건이 만족되기 때문입니다.
> - 반대 방법으로도 실행할 수 있지만(`new`를 안전하게 만들고 `write_byte`를 안전하지 않게 만듦) 이는 `write_byte`를 호출하는 모든 위치에서 안전성에 관해 추론해야 하므로 사용 편의성이 훨씬 떨어집니다.
> - 이는 안전하지 않은 코드의 안전한 래퍼를 작성하는 일반적인 패턴입니다. 안전에 관한 증명 부담을 여러 많은 위치에서 소수의 위치로 옮기는 것입니다.
