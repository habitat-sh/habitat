use clap_v4 as clap;

use crate::error::Result as HabResult;
use clap::Args;
use habitat_common::ui::UI;
use crate::{command::studio::enter,
            cli_v4::utils::ExternalCommandArgs};


#[derive(Clone, Debug, Args)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          about = "Commands relating to Habitat Studios",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]

pub(crate) struct StudioOpts {
    #[command(flatten)]
    args: ExternalCommandArgs,
}

impl StudioOpts {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        #[cfg(any(target_os = "macos",
                          any(all(target_os = "linux",
                                  any(target_arch = "x86_64", target_arch = "aarch64")),
                              all(target_os = "windows", target_arch = "x86_64"),)))]

        enter::start(ui, &self.args.args).await
    }
}
