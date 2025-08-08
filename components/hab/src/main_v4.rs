#[cfg(feature = "v4")]
use habitat_common::{ui::{UIWriter,
                          UI},
                     FeatureFlag};

#[cfg(feature = "v4")]
use hab::cli_driver;

#[cfg(feature = "v4")]
pub(crate) async fn main_v4() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    let features = FeatureFlag::from_env(&mut ui);
    if let Err(e) = cli_driver(&mut ui, features).await {
        let exit_code = e.exit_code();
        ui.fatal(e).unwrap();
        std::process::exit(exit_code)
    }
}

// Hack required for now to have this compile when v4 is not enabled
#[cfg(not(feature = "v4"))]
pub(crate) async fn main_v4() {
    unreachable!();
}
