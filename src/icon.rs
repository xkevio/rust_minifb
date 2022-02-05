#[cfg(target_os = "linux")]
use std::convert::TryFrom;
use std::ffi::CString;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{ffi::OsStr, str::FromStr};
#[cfg(any(target_os = "windows"))]
use std::os::windows::prelude::OsStrExt;

///
/// Represents a window icon
///
/// Different under Windows, Linux and MacOS
///
/// **Windows / MacOS**: Icon can be created from a relative path string
///
/// **Linux / X11:** Icon can be created from an ARGB buffer
///
///
#[derive(Clone, Copy, Debug)]
pub enum Icon {
    Path(*const u16, usize),
    MacTest(*const i8, usize),
    Buffer(*const u64, u32),
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl FromStr for Icon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Path to icon cannot be empty!");
        }

        #[cfg(target_os = "windows")]
        let v: Vec<u16> = OsStr::new(s)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        // #[cfg(target_os = "windows")]
        // let v: Vec<u16> = s.encode_utf16().collect();

        #[cfg(target_os = "windows")]
        return Ok(Icon::Path(v.as_ptr(), v.len()));
        #[cfg(target_os = "macos")] {
            let string = CString::new(s).unwrap();
            return Ok(Icon::MacTest(string.as_ptr(), s.len()));
        }
    }
}

#[cfg(target_os = "linux")]
impl TryFrom<&[u64]> for Icon {
    type Error = &'static str;

    fn try_from(value: &[u64]) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err("ARGB buffer cannot be empty!");
        }

        Ok(Icon::Buffer(value.as_ptr(), value.len() as u32))
    }
}
