// Implementation of `hab sup depart`

use clap_v4 as clap;

#[cfg(not(target_os = "macos"))]
use crate::cli_v4::utils::process_sup_request;

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult};
use clap::Parser;
use habitat_common::ui::UI;

#[cfg(not(target_os = "macos"))]
use habitat_sup_protocol as sup_proto;

#[cfg(not(target_os = "macos"))]
use habitat_common::ui::{Status,
                         UIWriter};

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct SupDepartOptions {
    /// The member-id of the Supervisor to depart
    #[arg(name = "MEMBER_ID")]
    member_id: String,

    /// Remote supervisor connection options
    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl SupDepartOptions {
    #[cfg(not(target_os = "macos"))]
    pub(super) async fn do_depart(&self, ui: &mut UI) -> HabResult<()> {
        let msg = sup_proto::ctl::SupDepart { member_id: Some(self.member_id.clone()), };

        ui.begin(format!("Permanently marking {} as departed", &self.member_id))?;
        ui.status(Status::Applying,
                  format!("via peer {}", self.remote_sup.inner()))?;

        process_sup_request(self.remote_sup.inner(), msg).await?;
        ui.end("Departure recorded.")?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn do_depart(&self, _ui: &mut UI) -> HabResult<()> { Ok(()) }
}
