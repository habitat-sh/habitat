use habitat_pkg_export_tar as export_tar;

use habitat_common::ui::{UIWriter,
                         UI};

use anyhow::Result;

#[tokio::main]
async fn main() {
    let mut ui = UI::default_with_env();
    if let Err(e) = start(&mut ui).await {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

async fn start(ui: &mut UI) -> Result<()> {
    env_logger::init();

    export_tar::cli_driver(ui).await
}
