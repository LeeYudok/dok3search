# 해답

``` rust
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
        let path =
            CString::new(path).map_err(|err| format!("잘못된 경로: {err}"))?;
        // SAFETY: path.as_ptr()은 NULL일 수 없습니다.
        let dir = unsafe { ffi::opendir(path.as_ptr()) };
        if dir.is_null() {
            Err(format!("{:?}을(를) 열 수 없습니다.", path))
        } else {
            Ok(DirectoryIterator { path, dir })
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // NULL 포인터를 다시 얻을 때까지 readdir을 계속 호출합니다.
        // SAFETY: self.dir은 NULL이 아닙니다.
        let dirent = unsafe { ffi::readdir(self.dir) };
        if dirent.is_null() {
            // 디렉터리의 끝에 도달했습니다.
            return None;
        }
        // SAFETY: dirent는 NULL이 아니며 dirent.d_name은 NUL
        // 종료됩니다.
        let d_name = unsafe { CStr::from_ptr((*dirent).d_name.as_ptr()) };
        let os_str = OsStr::from_bytes(d_name.to_bytes());
        Some(os_str.to_owned())
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // 필요에 따라 closedir을 호출합니다.
        if !self.dir.is_null() {
            // SAFETY: self.dir은 NULL이 아닙니다.
            if unsafe { ffi::closedir(self.dir) } != 0 {
                panic!("{:?}을(를) 닫을 수 없습니다.", self.path);
            }
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("파일: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn test_nonexisting_directory() {
        let iter = DirectoryIterator::new("no-such-directory");
        assert!(iter.is_err());
    }

    #[test]
    fn test_empty_directory() -> Result<(), Box<dyn Error>> {
        let tmp = tempfile::TempDir::new()?;
        let iter = DirectoryIterator::new(
            tmp.path().to_str().ok_or("경로에 UTF-8이 아닌 문자가 있음")?,
        )?;
        let mut entries = iter.collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, &[".", ".."]);
        Ok(())
    }

    #[test]
    fn test_nonempty_directory() -> Result<(), Box<dyn Error>> {
        let tmp = tempfile::TempDir::new()?;
        std::fs::write(tmp.path().join("foo.txt"), "Foo 다이어리\n")?;
        std::fs::write(tmp.path().join("bar.png"), "<PNG>\n")?;
        std::fs::write(tmp.path().join("crab.rs"), "//! Crab\n")?;
        let iter = DirectoryIterator::new(
            tmp.path().to_str().ok_or("경로에 UTF-8이 아닌 문자가 있음")?,
        )?;
        let mut entries = iter.collect::<Vec<_>>();
        entries.sort();
        assert_eq!(entries, &[".", "..", "bar.png", "crab.rs", "foo.txt"]);
        Ok(())
    }
}
```
