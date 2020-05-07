#[cfg(not(windows))]
use crate::error::Result;

/// Linux Docker daemons currently only run in one mode, so this can
/// be a no-op.
#[cfg(not(windows))]
pub(crate) fn ensure_proper_docker_platform() -> Result<()> { Ok(()) }

// On Windows, however, we have a bit more work to do.
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub(crate) use windows::ensure_proper_docker_platform;
