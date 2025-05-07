// To handle basic commands such as launching a shell or terminating

use clap_v4 as clap;

use clap::Parser;

use crate::error::Result as HabResult;
use habitat_common::ui::UI;

use std::ffi::OsString;

#[cfg(not(target_os = "macos"))]
use crate::command;

/// Gracefully terminate the Habitat Supervisor and all of its running services
#[derive(Debug, Clone, Parser)]
#[command(help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct SupTermCommand {
    #[arg()]
    args: Vec<OsString>,
}

impl SupTermCommand {
    #[cfg(not(target_os = "macos"))]
    pub(super) async fn execute(&self, ui: &mut UI) -> HabResult<()> {
        let mut args = vec!["term".into()];
        args.extend(self.args.clone());
        return command::sup::start(ui, &args).await;
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn execute(&self, _ui: &mut UI) -> HabResult<()> { Ok(()) }
}
