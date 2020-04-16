#[macro_use]
extern crate log;

use env_logger;
use habitat_common::ui::{UIWriter,
                         UI};
use habitat_pkg_export_docker::{cli,
                                export_for_cli_matches,
                                Result};

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    if let Err(e) = start(&mut ui).await {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

async fn start(ui: &mut UI) -> Result<()> {
    let cli = cli();
    let m = cli.get_matches();
    debug!("clap cli args: {:?}", m);
    export_for_cli_matches(ui, &m).await.map(|_| ())
}
