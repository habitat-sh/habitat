use crate::env;

/// Default Binlink Dir
#[cfg(target_os = "windows")]
pub const DEFAULT_BINLINK_DIR: &str = "/hab/bin";
#[cfg(target_os = "linux")]
pub const DEFAULT_BINLINK_DIR: &str = "/bin";
#[cfg(target_os = "macos")]
pub const DEFAULT_BINLINK_DIR: &str = "/usr/local/bin";

/// Binlink Dir Environment variable
pub const BINLINK_DIR_ENVVAR: &str = "HAB_BINLINK_DIR";

pub fn default_binlink_dir() -> String {
    match env::var(BINLINK_DIR_ENVVAR) {
        Ok(val) => val,
        Err(_) => DEFAULT_BINLINK_DIR.to_string(),
    }
}
