pub mod docker;
pub mod enter;
pub mod native;

pub fn docker_studio_support() -> bool { cfg!(target_os = "linux") || cfg!(target_os = "windows") }
