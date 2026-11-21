# FFI래퍼

Rust has great support for calling functions through a *foreign function interface* (FFI). We will use this to build a safe wrapper for the `libc` functions you would use from C to read the names of files in a directory.

아래 리눅스 메뉴얼 문서들을 참조하시기 바랍니다:

- [`opendir(3)`](https://man7.org/linux/man-pages/man3/opendir.3.html)
- [`readdir(3)`](https://man7.org/linux/man-pages/man3/readdir.3.html)
- [`closedir(3)`](https://man7.org/linux/man-pages/man3/closedir.3.html)

아마 [`std::ffi`](https://doc.rust-lang.org/std/ffi/)모듈을 참조할 필요가 있을 것입니다. 거기에는 이번 예제를 수행하는데 필요한 다양한 종류의 문자열 타입들이 소개되어 있습니다:

| 타입 | 인코딩 | 사용 |
|----|----|----|
| [`str`](https://doc.rust-lang.org/std/primitive.str.html)과 [`String`](https://doc.rust-lang.org/std/string/struct.String.html) | UTF-8 | 러스트에서의 문자열 처리 |
| [`CStr`](https://doc.rust-lang.org/std/ffi/struct.CStr.html)과 [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html) | 널(NUL)로 끝남 | C함수와 연동하기 |
| [`OsStr`](https://doc.rust-lang.org/std/ffi/struct.OsStr.html)와 [`OsString`](https://doc.rust-lang.org/std/ffi/struct.OsString.html) | OS가 정의함 | OS와 연동하기 위한 문자열 |

이 타입들 간의 변환은 다음과 같습니다:

- `&str`에서 `CString`으로의 변환: 맨 마지막의 `\0` 문자를 저장하기 위한 공간을 할당해야 합니다,
- `CString`에서 `*const i8`로의 변환: C함수를 호출하기 위해서는 포인터가 필요합니다,
- `*const i8`에서 `&CStr`로의 변환: 주어진 바이트 시퀀스가 `\0`로 끝나는지 확인하고 싶은 경우,
- `&CStr` to `&[u8]`: a slice of bytes is the universal interface for “some unknown data”,
- `&[u8]`에서 `&OsStr`로의 변환: `&OsStr`는 `OsString`으로 가기 위한 중간 단계 입니다. [`OsStrExt`](https://doc.rust-lang.org/std/os/unix/ffi/trait.OsStrExt.html)를 사용해서 `OsStr`를 생성하세요,
- `&OsStr`에서 `OsString`으로의 변환: `&OsStr`이 가리키고 있는 데이터를 복사함으로써, 이 데이터를 리턴하고, `readdir`함수를 호출할 때 사용할 수 있게 해 줍니다.

[Nomicon](https://doc.rust-lang.org/nomicon/ffi.html)에 FFI와 관련한 아주 유용한 챕터가 있습니다.

아래 코드를 <https://play.rust-lang.org/>에 복사하고 빠진 함수와 메서드를 채워봅니다:

패닉 발생 예제

``` rust
// TODO: 구현이 완료되면 이를 삭제합니다.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "매크로"))]
    use std::os::raw::{c_long, c_uchar, c_ulong, c_ushort};

    // 불투명 타입입니다. https://doc.rust-lang.org/nomicon/ffi.html을 참고하세요.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // readdir(3)의 Linux man 페이지에 따른 레이아웃입니다.
    // 여기서 ino_t 및 off_t는
    // /usr/include/x86_64-linux-gnu/{sys/types.h, bits/typesizes.h}의 정의에 따라 확인됩니다.
    #[cfg(not(target_os = "매크로"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_ulong,
        pub d_off: c_long,
        pub d_reclen: c_ushort,
        pub d_type: c_uchar,
        pub d_name: [c_char; 256],
    }

    // dir(5)의 macOS man 페이지에 따른 레이아웃입니다.
    #[cfg(all(target_os = "매크로"))]
    #[repr(C)]
    pub struct dirent {
        pub d_fileno: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;

        #[cfg(not(all(target_os = "매크로", target_arch = "x86_64")))]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        // https://github.com/rust-lang/libc/issues/414 및
 // stat(2)에 관한 macOS man 페이지의 _DARWIN_FEATURE_64_BIT_INODE 섹션을 참고하세요.
 //
 // ' 이 업데이트가 제공되기 전에 존재했던 플랫폼은'
 // Intel 및 PowerPC의 macOS (iOS/wearOS 등이 아님)를 의미합니다.
        #[cfg(all(target_os = "매크로", target_arch = "x86_64"))]
        #[link_name = "readdir$INODE64"]
        pub fn readdir(s: *mut DIR) -> *const dirent;

        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // opendir을 호출하고 제대로 작동하면 Ok 값을 반환하고
        // 그렇지 않으면 메시지와 함께 Err을 반환합니다.
        unimplemented!()
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // NULL 포인터를 다시 가져올 때까지 readdir을 계속 호출합니다.
        unimplemented!()
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // 필요에 따라 closedir을 호출합니다.
        unimplemented!()
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("파일: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}
```
