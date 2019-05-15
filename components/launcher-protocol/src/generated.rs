use std::fmt;

include!(concat!(env!("OUT_DIR"), "/launcher.error.rs"));
include!(concat!(env!("OUT_DIR"), "/launcher.launcher.rs"));
include!(concat!(env!("OUT_DIR"), "/launcher.net.rs"));
include!(concat!(env!("OUT_DIR"), "/launcher.supervisor.rs"));

impl fmt::Display for ShutdownMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match *self {
            ShutdownMethod::AlreadyExited => "Already Exited",
            ShutdownMethod::GracefulTermination => "Graceful Termination",
            ShutdownMethod::Killed => "Killed",
        };
        write!(f, "{}", printable)
    }
}
