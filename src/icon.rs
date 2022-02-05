#[cfg(target_os = "linux")]
use std::convert::TryFrom;
#[cfg(any(target_os = "windows", target_os = "macos"))]
use std::{ffi::OsStr, os::windows::prelude::OsStrExt, str::FromStr};

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
    Buffer(*const u64, u32),
}

#[cfg(any(target_os = "windows", target_os = "macos"))]
impl FromStr for Icon {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err("Path to icon cannot be empty!");
        }

        let v: Vec<u16> = OsStr::new(s)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect();

        Ok(Icon::Path(v.as_ptr(), v.len()))
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
