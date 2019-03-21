pub use self::imp::*;

#[cfg(unix)]
#[path = "unix/mod.rs"]
mod imp;

#[cfg(windows)]
#[path = "windows/mod.rs"]
mod imp;
