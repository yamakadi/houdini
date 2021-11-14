//! `Houdini` allows you to delete your executable while it's running.
//! This is fairly straightforward for unix systems, since the executable is released
//! after getting mapped to the memory. On Windows, we use a method discovered by [@jonasLyk](https://twitter.com/jonasLyk/status/1350401461985955840)
//! to delete the executable.
//!
//! Windows implementation heavily references [@byt3bl33d3r](https://twitter.com/byt3bl33d3r)'s
//! Nim implementation in [OffensiveNim](https://github.com/byt3bl33d3r/OffensiveNim/blob/master/src/self_delete_bin.nim)
//! and in turn LloydLabs' initial [`C` PoC](https://github.com/LloydLabs/delete-self-poc).
//!

#[cfg(any(target_os = "linux", target_os = "macos"))]
mod unix;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use crate::unix::disappear as disappear_impl;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub use crate::unix::errors::HoudiniError;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
use crate::windows::{disappear as disappear_impl, DEFAULT_PLACEHOLDER};

#[cfg(target_os = "windows")]
pub use crate::windows::errors::HoudiniError;

/// Deletes the executable for the current process.
///
/// It uses [`:svcmsrpc`] as the default placeholder while renaming the executable
/// to an alternate data stream (ADS).
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// match houdini::disappear() {
///     Ok(_) => println!("Pulled a Houdini!!"),
///     Err(e) => println!("Nope! => {}", e),
/// };
/// ```
pub fn disappear() -> Result<(), HoudiniError> {
    #[cfg(target_os = "windows")]
    {
        return disappear_impl(DEFAULT_PLACEHOLDER);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        return disappear_impl();
    }
}

/// Deletes the executable for the current process.
///
/// It uses the first 8 bytes of the given placeholder (9 bytes if the placeholder starts with a `:`)
/// while renaming the executable to an alternate data stream (ADS).
///
/// If you don't want to be too adventurous, do not use any digits or symbols in the placeholder.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// match houdini::disappear_with_placeholder("temporary") {
///     Ok(_) => println!("Pulled a Houdini!!"),
///     Err(e) => println!("Nope! => {}", e),
/// };
/// ```
#[cfg(target_os = "windows")]
pub fn disappear_with_placeholder<S: Into<String>>(placeholder: S) -> Result<(), HoudiniError> {
    let mut p: String = placeholder.into();

    if p.starts_with(":") {
        p = format!("{:s<9}", p);
    } else {
        p = format!(":{:s<8}", p);
    }

    let mut placeholder_bytes: [u8; 9] = Default::default();

    placeholder_bytes.copy_from_slice(&p.into_bytes()[..9]);

    return disappear_impl(&placeholder_bytes);
}
