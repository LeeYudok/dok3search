# 드라이버

이제 드라이버에서 새로운 `Registers` 구조체를 사용해 보겠습니다.

컴파일 오류 예제

``` rust
/// PL011 UART용 드라이버
#[derive(Debug)]
pub struct Uart {
    registers: *mut Registers,
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
    pub unsafe fn new(base_address: *mut u32) -> Self {
        Self { registers: base_address as *mut Registers }
    }

    /// UART에 단일 바이트를 씁니다.
    pub fn write_byte(&self, byte: u8) {
        // TX 버퍼에 공간이 확보될 때까지 기다립니다.
        while self.read_flag_register().contains(Flags::TXFF) {}

        // self.registers가 적절하게 매핑된 PL011 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe {
            // TX 버퍼에 씁니다.
            addr_of_mut!((*self.registers).dr).write_volatile(byte.into());
        }

        // UART가 더 이상 사용 중이 아닐 때까지 기다립니다.
        while self.read_flag_register().contains(Flags::BUSY) {}
    }

    /// Reads and returns a pending byte, or `None` if nothing has been
    /// received.
    pub fn read_byte(&self) -> Option<u8> {
        if self.read_flag_register().contains(Flags::RXFE) {
            None
        } else {
            let data = unsafe { addr_of!((*self.registers).dr).read_volatile() };
            // TODO: 비트 8~11에서 오류 상태를 확인합니다.
            Some(data as u8)
        }
    }

    fn read_flag_register(&self) -> Flags {
        // self.registers가 적절하게 매핑된 PL011 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe { addr_of!((*self.registers).fr).read_volatile() }
    }
}
```

> **발표자 노트**
>
> - `addr_of!` / `addr_of_mut!`를 사용하여 중간 참조를 만들지 않고 개별 필드 포인터를 가져오는 것은 불안정할 수 있습니다.
