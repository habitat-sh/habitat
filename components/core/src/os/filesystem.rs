#[allow(unused_variables)]
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::symlink;

#[cfg(not(windows))]
mod linux;

#[cfg(not(windows))]
pub use self::linux::symlink;
