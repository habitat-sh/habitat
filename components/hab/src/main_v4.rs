#[cfg(feature = "v4")]
use habitat_common::ui::{UIWriter,
                         UI};

#[cfg(feature = "v4")]
use hab::cli_driver;

#[cfg(feature = "v4")]
pub(crate) async fn main_v4() {
    let mut ui = UI::default_with_env();
    if let Err(e) = cli_driver(&mut ui).await {
        let exit_code = e.exit_code();
        ui.fatal(e).unwrap();
        std::process::exit(exit_code)
    }
}

// Hack required for now to have this compile when v4 is not enabled
#[cfg(not(feature = "v4"))]
pub(crate) async fn main_v4() {
    todo!();
}
