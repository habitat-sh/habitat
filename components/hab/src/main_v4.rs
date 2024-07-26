#[cfg(feature = "v4")]
use hab::cli_driver;

#[cfg(feature = "v4")]
pub(crate) async fn main_v4() { cli_driver() }

// Hack required for now to have this compile when v4 is not enabled
#[cfg(not(feature = "v4"))]
pub(crate) async fn main_v4() {
    todo!();
}
