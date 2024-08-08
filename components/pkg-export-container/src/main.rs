use habitat_common::ui::{UIWriter,
                         UI};
use habitat_pkg_export_container::cli_driver;

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    if let Err(e) = cli_driver(&mut ui).await {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}
