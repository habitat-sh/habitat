pub mod docker;
pub mod enter;

pub fn native_studio_support() -> bool { cfg!(target_os = "linux") || cfg!(target_os = "windows") }
