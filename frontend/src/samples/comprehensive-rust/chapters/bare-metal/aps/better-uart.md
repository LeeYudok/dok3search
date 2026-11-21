# 더 나은 UART 드라이버

PL011에는 실제로 [훨씬 더 많은 레지스터](https://developer.arm.com/documentation/ddi0183/g/programmers-model/summary-of-registers)가 있으며, 이에 액세스할 포인터를 구성하기 위해 오프셋을 추가하면 오류가 발생하기 쉽고 읽기 어렵습니다. 또한 그중 일부는 구조화된 방식으로 액세스할 수 있는 비트 필드입니다.

| 오프셋 | 레지스터 이름 | 너비 |
|--------|---------------|------|
| 0x00   | DR            | 12   |
| 0x04   | RSR           | 4    |
| 0x18   | FR            | 9    |
| 0x20   | ILPR          | 8    |
| 0x24   | IBRD          | 16   |
| 0x28   | FBRD          | 6    |
| 0x2c   | LCR_H         | 8    |
| 0x30   | CR            | 16   |
| 0x34   | IFLS          | 6    |
| 0x38   | IMSC          | 11   |
| 0x3c   | RIS           | 11   |
| 0x40   | MIS           | 11   |
| 0x44   | ICR           | 11   |
| 0x48   | DMACR         | 3    |

> **발표자 노트**
>
> - 간결성을 위해 일부 ID 레지스터는 생략되었습니다.
