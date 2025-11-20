use habitat_common::{FeatureFlag,
                     ui::{UI,
                          UIWriter}};

use hab::cli_driver;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    let features = FeatureFlag::from_env(&mut ui);
    if let Err(e) = cli_driver(&mut ui, features).await {
        let exit_code = e.exit_code();
        ui.fatal(e).unwrap();
        std::process::exit(exit_code)
    }
}
