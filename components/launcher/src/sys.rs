#[cfg(unix)]
#[path = "sys/unix/service.rs"]
pub mod service;

#[cfg(windows)]
#[path = "sys/windows/service.rs"]
pub mod service;
