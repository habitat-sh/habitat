pub mod docker;
pub mod enter;

#[cfg(target_family = "unix")]
pub mod native;

pub fn docker_studio_support() -> bool { cfg!(target_os = "linux") || cfg!(target_os = "windows") }
