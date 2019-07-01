#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::uname;

#[cfg(not(windows))]
pub mod linux;
#[cfg(not(windows))]
pub use self::linux::uname;

#[derive(Debug)]
pub struct Uname {
    pub sys_name:  String,
    pub node_name: String,
    pub release:   String,
    pub version:   String,
    pub machine:   String,
}
