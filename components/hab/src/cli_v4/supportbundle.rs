use crate::{cli_v4::clap::Args,
            command::supportbundle::start,
            error::Result as HabResult};
use clap_v4 as clap;
use habitat_common::ui::UI;
use habitat_core::crypto::init;

#[derive(Debug, Clone, Args)]
#[command(name = "supportbundle",
          help_template = "{name} {version} {author-section} \
                           {about-section}\n{usage-heading}\n{usage}\n\n{all-args}\n")]
pub(crate) struct SupportBundleOpts;

impl SupportBundleOpts {
    pub(crate) async fn do_command(&self, ui: &mut UI) -> HabResult<()> {
        init()?;
        start(ui)
    }
}
