# 전Bare Metal 오후

## RTC driver

([연습문제로 돌아가기](rtc.md))

*main.rs*:

컴파일 오류 예제

``` rust
#![no_main]
#![no_std]

mod exceptions;
mod logger;
mod pl011;
mod pl031;

use crate::pl031::Rtc;
use arm_gic::gicv3::{IntId, Trigger};
use arm_gic::{irq_enable, wfi};
use chrono::{TimeZone, Utc};
use core::hint::spin_loop;
use crate::pl011::Uart;
use arm_gic::gicv3::GicV3;
use core::panic::PanicInfo;
use log::{error, info, trace, LevelFilter};
use smccc::psci::system_off;
use smccc::Hvc;

/// GICv3의 기본 주소입니다.
const GICD_BASE_ADDRESS: *mut u64 = 0x800_0000 as _;
const GICR_BASE_ADDRESS: *mut u64 = 0x80A_0000 as _;

/// 기본 PL011 UART의 기본 주소입니다.
const PL011_BASE_ADDRESS: *mut u32 = 0x900_0000 as _;

/// PL031 RTC의 기본 주소입니다.
const PL031_BASE_ADDRESS: *mut u32 = 0x901_0000 as _;
/// PL031 RTC에서 사용하는 IRQ입니다.
const PL031_IRQ: IntId = IntId::spi(2);

#[no_mangle]
extern "C" fn main(x0: u64, x1: u64, x2: u64, x3: u64) {
    // `PL011_BASE_ADDRESS`가 PL011 기기의 기본 주소이고
    // 이 주소 범위에 액세스하는 다른 항목이 없으므로 안전합니다.
    let uart = unsafe { Uart::new(PL011_BASE_ADDRESS) };
    logger::init(uart, LevelFilter::Trace).unwrap();

    info!("main({:#x}, {:#x}, {:#x}, {:#x})", x0, x1, x2, x3);

    // `GICD_BASE_ADDRESS` 및 `GICR_BASE_ADDRESS`가 각각 GICv3 배포자 및 재배포자의
 // 기본 주소이이고
    // 이러한 주소 범위에 액세스하는 다른 항목이 없으므로 안전합니다.
    let mut gic = unsafe { GicV3::new(GICD_BASE_ADDRESS, GICR_BASE_ADDRESS) };
    gic.setup();

    // `PL031_BASE_ADDRESS`가 PL031 기기의 기본 주소이고
    // 이 주소 범위에 액세스하는 다른 항목이 없으므로 안전합니다.
    let mut rtc = unsafe { Rtc::new(PL031_BASE_ADDRESS) };
    let timestamp = rtc.read();
    let time = Utc.timestamp_opt(timestamp.into(), 0).unwrap();
    info!("RTC: {time}");

    GicV3::set_priority_mask(0xff);
    gic.set_interrupt_priority(PL031_IRQ, 0x80);
    gic.set_trigger(PL031_IRQ, Trigger::Level);
    irq_enable();
    gic.enable_interrupt(PL031_IRQ, true);

    // 인터럽트 없이 3초간 기다립니다.
    let target = timestamp + 3;
    rtc.set_match(target);
    info!("{}을(를) 기다리는 중", Utc.timestamp_opt(target.into(), 0).unwrap());
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    while !rtc.matched() {
        spin_loop();
    }
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    info!("대기 완료됨");

    // 인터럽트를 위해 3초 더 기다립니다.
    let target = timestamp + 6;
    info!("{}을(를) 기다리는 중", Utc.timestamp_opt(target.into(), 0).unwrap());
    rtc.set_match(target);
    rtc.clear_interrupt();
    rtc.enable_interrupt(true);
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    while !rtc.interrupt_pending() {
        wfi();
    }
    trace!(
        "matched={}, interrupt_pending={}",
        rtc.matched(),
        rtc.interrupt_pending()
    );
    info!("대기 완료됨");

    system_off::<Hvc>().unwrap();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{info}");
    system_off::<Hvc>().unwrap();
    loop {}
}
```

*pl031.rs*:

``` rust
#![allow(unused)]
fn main() {
use core::ptr::{addr_of, addr_of_mut};

#[repr(C, align(4))]
struct Registers {
    /// 데이터 레지스터
    dr: u32,
    /// 일치 레지스터
    mr: u32,
    /// 로드 레지스터
    lr: u32,
    /// 제어 레지스터
    cr: u8,
    _reserved0: [u8; 3],
    /// 인터럽트 마스크 세트 또는 정리 레지스터
    imsc: u8,
    _reserved1: [u8; 3],
    /// 원시 인터럽트 상태
    ris: u8,
    _reserved2: [u8; 3],
    /// 마스킹된 인터럽트 상태
    mis: u8,
    _reserved3: [u8; 3],
    /// 인터럽트 정리 레지스터
    icr: u8,
    _reserved4: [u8; 3],
}

/// PL031 실시간 시계용 드라이버
#[derive(Debug)]
pub struct Rtc {
    registers: *mut Registers,
}

impl Rtc {
    /// 지정된 기본 주소에 PL031 기기에 대한 RTC 드라이버의 새 인스턴스를
 /// 생성합니다.
 ///
 /// # 안전
 ///
 /// 지정된 기본 주소는 PL031 기기의
 /// MMIO 제어 레지스터를 가리켜야 하며,
 /// 이는 프로세스의 주소 공간에 기기 메모리로
 /// 매핑되어야 하며 다른 별칭은 없어야 합니다.
    pub unsafe fn new(base_address: *mut u32) -> Self {
        Self { registers: base_address as *mut Registers }
    }

    /// 현재 RTC 값을 읽습니다.
    pub fn read(&self) -> u32 {
        // self.registers가 적절하게 매핑된 PL031 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe { addr_of!((*self.registers).dr).read_volatile() }
    }

    /// 일치 값을 작성합니다. RTC 값이 이 값과 일치하면 인터럽트가
    /// 생성됩니다(사용 설정된 경우).
    pub fn set_match(&mut self, value: u32) {
        // self.registers가 적절하게 매핑된 PL031 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe { addr_of_mut!((*self.registers).mr).write_volatile(value) }
    }

    /// 인터럽트가 활성화되었는지 여부와 관계없이 일치 레지스터가 RTC 값과 일치하는지 여부를
    /// 반환합니다.
    pub fn matched(&self) -> bool {
        // self.registers가 적절하게 매핑된 PL031 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        let ris = unsafe { addr_of!((*self.registers).ris).read_volatile() };
        (ris & 0x01) != 0
    }

    /// 현재 대기 중인 인터럽트가 있는지 여부를 반환합니다.
    ///
    /// 이는 'matched'가 true를 반환하고 인터럽트가 마스킹된 경우에만
    /// true여야 합니다.
    pub fn interrupt_pending(&self) -> bool {
        // self.registers가 적절하게 매핑된 PL031 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        let ris = unsafe { addr_of!((*self.registers).mis).read_volatile() };
        (ris & 0x01) != 0
    }

    /// 인터럽트 마스크를 설정하거나 지웁니다.
    ///
    /// 마스크가 true인 경우 인터럽트가 사용 설정됩니다. false이면
    /// 인터럽트가 사용 중지됩니다.
    pub fn enable_interrupt(&mut self, mask: bool) {
        let imsc = if mask { 0x01 } else { 0x00 };
        // self.registers가 적절하게 매핑된 PL031 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe { addr_of_mut!((*self.registers).imsc).write_volatile(imsc) }
    }

    /// 대기 중인 인터럽트가 있는 경우 이를 지웁니다.
    pub fn clear_interrupt(&mut self) {
        // self.registers가 적절하게 매핑된 PL031 기기의 제어 레지스터를
        // 가리키고 있으므로 안전합니다.
        unsafe { addr_of_mut!((*self.registers).icr).write_volatile(0x01) }
    }
}

// 모든 컨텍스트에서 액세스할 수 있는 기기 메모리에 대한
// 포인터만 포함하므로 안전합니다.
unsafe impl Send for Rtc {}
}
```
