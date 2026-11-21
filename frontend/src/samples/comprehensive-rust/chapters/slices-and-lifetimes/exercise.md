# 연습문제: Protobuf 파싱

이 연습에서는 [protobuf 바이너리 인코딩](https://protobuf.dev/programming-guides/encoding/)용 파서를 빌드합니다. 생각보다 간단합니다. 이는 데이터 슬라이스를 전달하는 일반적인 파싱 패턴을 보여줍니다. 기본 데이터 자체는 복사되지 않습니다.

protobuf 메시지를 완전히 파싱하려면 필드 번호로 색인이 생성된 필드의 타입을 알아야 합니다. 이는 일반적으로 `proto` 파일에 제공됩니다. 이 연습에서는 이러한 정보를 각 필드에 대해 호출되는 함수의 `match` 문으로 인코딩합니다.

다음 proto를 사용합니다.

``` proto
message PhoneNumber {
  optional string number = 1;
  optional string type = 2;
}

message Person {
  optional string name = 1;
  optional int32 id = 2;
  repeated PhoneNumber phones = 3;
}
```

proto 메시지는 일련의 필드로 차례로 인코딩됩니다. 각각은 값이 뒤에 오는 ’태그’로 구현됩니다. 태그에는 필드 번호(예: `Person` 메시지의 `id` 필드에 대한 `2`) 및 바이트 스트림에서 페이로드를 결정하는 방법을 정의하는 와이어 타입이 포함됩니다.

태그를 포함한 정수는 VARINT라는 가변 길이 인코딩으로 표시됩니다. 다행히 `parse_varint`는 아래에 정의되어 있습니다. 또한 제공된 코드는 콜백을 정의하여 `Person` 및 `PhoneNumber` 필드를 처리하고 메시지를 이러한 콜백에 대한 일련의 호출로 파싱합니다.

이제 `parse_field` 함수를 구현하고 `Person`과 `PhoneNumber` 구조체애 대해 `ProtoMessage` 트레잇만 구현하면 됩니다.

컴파일 오류 예제

``` rust
use std::convert::TryFrom;
use thiserror::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("잘못된 varint")]
    InvalidVarint,
    #[error("잘못된 wire-type")]
    InvalidWireType,
    #[error("예상치 못한 EOF")]
    UnexpectedEOF,
    #[error("잘못된 길이")]
    InvalidSize(#[from] std::num::TryFromIntError),
    #[error("예상치 못한 wire-type")]
    UnexpectedWireType,
    #[error("잘못된 문자열(UTF-8 아님)")]
    InvalidString,
}

/// 와이어에 표시된 와이어 타입입니다.
enum WireType {
    /// Varint WireType은 값이 단일 VARINT임을 나타냅니다.
    Varint,
    //I64,  -- not needed for this exercise
    /// The Len WireType indicates that the value is a length represented as a
    /// VARINT followed by exactly that number of bytes.
    Len,
    /// The I32 WireType indicates that the value is precisely 4 bytes in
    /// little-endian order containing a 32-bit signed integer.
    I32,
}

#[derive(Debug)]
/// 와이어 타입에 따라 타입이 지정된 필드 값입니다.
enum FieldValue<'a> {
    Varint(u64),
    //I64(i64),  -- 이 연습에서는 필요하지 않습니다.
    Len(&'a [u8]),
    I32(i32),
}

#[derive(Debug)]
/// 필드 번호 및 값을 포함하는 필드입니다.
struct Field<'a> {
    field_num: u64,
    value: FieldValue<'a>,
}

trait ProtoMessage<'a>: Default + 'a {
    fn add_field(&mut self, field: Field<'a>) -> Result<(), Error>;
}

impl TryFrom<u64> for WireType {
    type Error = Error;

    fn try_from(value: u64) -> Result<WireType, Error> {
        Ok(match value {
            0 => WireType::Varint,
            //1 => WireType::I64,  -- 이 연습에서는 필요하지 않습니다.
            2 => WireType::Len,
            5 => WireType::I32,
            _ => return Err(Error::InvalidWireType),
        })
    }
}

impl<'a> FieldValue<'a> {
    fn as_string(&self) -> Result<&'a str, Error> {
        let FieldValue::Len(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        std::str::from_utf8(data).map_err(|_| Error::InvalidString)
    }

    fn as_bytes(&self) -> Result<&'a [u8], Error> {
        let FieldValue::Len(data) = self else {
            return Err(Error::UnexpectedWireType);
        };
        Ok(data)
    }

    fn as_u64(&self) -> Result<u64, Error> {
        let FieldValue::Varint(value) = self else {
            return Err(Error::UnexpectedWireType);
        };
        Ok(*value)
    }
}

/// VARINT를 파싱하여 파싱된 값과 나머지 바이트를 반환합니다.
fn parse_varint(data: &[u8]) -> Result<(u64, &[u8]), Error> {
    for i in 0..7 {
        let Some(b) = data.get(i) else {
            return Err(Error::InvalidVarint);
        };
        if b & 0x80 == 0 {
            // 이는 VARINT의 마지막 바이트이므로
            // u64로 변환하여 반환합니다.
            let mut value = 0u64;
            for b in data[..=i].iter().rev() {
                value = (value << 7) | (b & 0x7f) as u64;
            }
            return Ok((value, &data[i + 1..]));
        }
    }

    // 7바이트를 초과하면 유효하지 않습니다.
    Err(Error::InvalidVarint)
}

/// 태그를 필드 번호와 WireType으로 변환합니다.
fn unpack_tag(tag: u64) -> Result<(u64, WireType), Error> {
    let field_num = tag >> 3;
    let wire_type = WireType::try_from(tag & 0x7)?;
    Ok((field_num, wire_type))
}


/// 필드를 파싱하여 나머지 바이트를 반환합니다.
fn parse_field(data: &[u8]) -> Result<(Field, &[u8]), Error> {
    let (tag, remainder) = parse_varint(data)?;
    let (field_num, wire_type) = unpack_tag(tag)?;
    let (fieldvalue, remainder) = match wire_type {
        _ => todo!("Based on the wire type, build a Field, consuming as many bytes as necessary.")
    };
    todo!("Return the field, and any un-consumed bytes.")
}

/// 주어진 데이터에서 메시지를 파싱하여 메시지의 각 필드에 대해 `T::add_field`를 호출합니다.
///
/// 전체 입력이 사용됩니다.
fn parse_message<'a, T: ProtoMessage<'a>>(mut data: &'a [u8]) -> Result<T, Error> {
    let mut result = T::default();
    while !data.is_empty() {
        let parsed = parse_field(data)?;
        result.add_field(parsed.0)?;
        data = parsed.1;
    }
    Ok(result)
}

#[derive(Debug, Default)]
struct PhoneNumber<'a> {
    number: &'a str,
    type_: &'a str,
}

#[derive(Debug, Default)]
struct Person<'a> {
    name: &'a str,
    id: u64,
    phone: Vec<PhoneNumber<'a>>,
}

// TODO: Implement ProtoMessage for Person and PhoneNumber.

fn main() {
    let person: Person = parse_message(&[
        0x0a, 0x07, 0x6d, 0x61, 0x78, 0x77, 0x65, 0x6c, 0x6c, 0x10, 0x2a, 0x1a,
        0x16, 0x0a, 0x0e, 0x2b, 0x31, 0x32, 0x30, 0x32, 0x2d, 0x35, 0x35, 0x35,
        0x2d, 0x31, 0x32, 0x31, 0x32, 0x12, 0x04, 0x68, 0x6f, 0x6d, 0x65, 0x1a,
        0x18, 0x0a, 0x0e, 0x2b, 0x31, 0x38, 0x30, 0x30, 0x2d, 0x38, 0x36, 0x37,
        0x2d, 0x35, 0x33, 0x30, 0x38, 0x12, 0x06, 0x6d, 0x6f, 0x62, 0x69, 0x6c,
        0x65,
    ])
    .unwrap();
    println!("{:#?}", person);
}
```
