#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use self::unix::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

use std::fmt;

#[derive(Debug)]
pub enum ShutdownMethod {
    AlreadyExited,
    GracefulTermination,
    Killed,
}

impl fmt::Display for ShutdownMethod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match self {
            ShutdownMethod::AlreadyExited => "Service already exited",
            ShutdownMethod::GracefulTermination => "Service gracefully terminated",
            ShutdownMethod::Killed => "Service killed",
        };
        write!(f, "{}", msg)
    }
}
