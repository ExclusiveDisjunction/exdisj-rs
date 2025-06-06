use std::iter::once;
use std::ffi::{OsStr, OsString};


pub struct Utf16String {
    data: Vec<u16>
}
impl From<String> for Utf16String {
    fn from(value: String) -> Self {
        Self {
            data: value.encode_utf16().chain(once(0)).collect()
        }
    }
}
impl From<&str> for Utf16String {
    fn from(value: &str) -> Self {
        Self {
            data: value.encode_utf16().chain(once(0)).collect()
        }
    }
}
impl From<OsString> for Utf16String {
    fn from(value: OsString) -> Self {
        Self::from(value.as_os_str())
    }
}
#[cfg(windows)]
impl From<&OsStr> for Utf16String {
    fn from(value: &OsStr) -> Self {
        use std::os::windows::ffi::OsStrExt;

        let data: Vec<u16> = value.encode_wide().chain(once(0)).collect();

        Self {
            data
        }
    }
}
#[cfg(not(windows))]
impl From<&OsStr> for Utf16String {
    fn from(value: &OsStr) -> Self {
        let utf8 = String::from_utf8_lossy(s.as_bytes());
        Self::from(utf8)
    }
}
impl TryInto<String> for Utf16String {
    type Error = std::string::FromUtf16Error;
    fn try_into(self) -> Result<String, Self::Error> {
        String::from_utf16(&self.data)
    }
}
impl Utf16String {
    pub fn get_ptr(&self) -> *const u16 {
        self.data.as_ptr()
    }
}