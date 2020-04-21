//! OS-specific implementations of various things.

mod check;

pub(crate) use check::ensure_proper_docker_platform;
