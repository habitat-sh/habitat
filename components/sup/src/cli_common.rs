// All common functionality
use std::process;

use log::error;

use habitat_core::crypto;
use habitat_launcher_client::LauncherCli;

use habitat_sup::{command,
                  error::{Error,
                          Result},
                  manager::Manager};

pub(crate) fn boot() -> Option<LauncherCli> {
    if crypto::init().is_err() {
        error!("Failed to initialization libsodium, make sure it is available in your runtime \
                environment");
        process::exit(1);
    }
    match habitat_launcher_client::env_pipe() {
        Some(pipe) => {
            match LauncherCli::connect(pipe) {
                Ok(launcher) => Some(launcher),
                Err(err) => {
                    error!("Failed to connect to launcher: {:?}",
                           anyhow::Error::new(err));
                    process::exit(1);
                }
            }
        }
        None => None,
    }
}

#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64"),))]
pub(crate) async fn sub_sh() -> Result<()> { command::shell::sh().await }

pub(crate) fn sub_term() -> Result<()> {
    match Manager::term() {
        Err(e @ Error::LockFileError(..)) => {
            println!("Supervisor not terminated: {}", e);
            Ok(())
        }
        result => result,
    }
}

#[cfg(any(all(target_os = "linux",
              any(target_arch = "x86_64", target_arch = "aarch64")),
          all(target_os = "windows", target_arch = "x86_64"),))]
pub(crate) async fn sub_bash() -> Result<()> { command::shell::bash().await }
