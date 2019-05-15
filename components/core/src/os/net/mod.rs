#[allow(unused_variables)]
#[cfg(windows)]
#[path = "windows.rs"]
mod imp;

#[cfg(not(windows))]
#[path = "unix.rs"]
mod imp;

pub use self::imp::*;
