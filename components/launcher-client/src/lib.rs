#[macro_use]
extern crate log;

mod client;
pub mod error;

pub use habitat_launcher_protocol::{ERR_NO_RETRY_EXCODE,
                                    LAUNCHER_LOCK_CLEAN_ENV,
                                    LAUNCHER_PID_ENV,
                                    OK_NO_RETRY_EXCODE};

pub use crate::{client::LauncherCli,
                error::Error};

pub fn env_pipe() -> Option<String> {
    habitat_core::env::var(habitat_launcher_protocol::LAUNCHER_PIPE_ENV).ok()
}
