// Implemenatation of `hab sup depart`

use clap_v4 as clap;

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult};
use clap::Parser;
use habitat_common::ui::UI;

#[cfg(not(target_os = "macos"))]
use habitat_sup_client::{SrvClient,
                         SrvClientError};

#[cfg(not(target_os = "macos"))]
use habitat_sup_protocol as sup_proto;

#[cfg(not(target_os = "macos"))]
use std::io;

#[cfg(not(target_os = "macos"))]
use futures::stream::StreamExt;

#[cfg(not(target_os = "macos"))]
use habitat_common::ui::{Status,
                         UIWriter};

#[derive(Debug, Clone, Parser)]
#[command(arg_required_else_help = true,
    disable_version_flag = true,
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
        let remote = SrvClient::ctl_addr(self.remote_sup.inner())?;
        // let mut ui = ui::ui();
        let msg = sup_proto::ctl::SupDepart { member_id: Some(self.member_id.clone()), };

        ui.begin(format!("Permanently marking {} as departed", &self.member_id))?;
        ui.status(Status::Applying, format!("via peer {}", remote))?;
        let mut response = SrvClient::request(Some(&remote), msg).await?;

        while let Some(message_result) = response.next().await {
            let reply = message_result?;
            match reply.message_id() {
                "NetOk" => (),
                "NetErr" => {
                    let m = reply.parse::<sup_proto::net::NetErr>()
                                .map_err(SrvClientError::Decode)?;
                    return Err(SrvClientError::from(m).into());
                }
                _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
            }
        }
        ui.end("Departure recorded.")?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn do_depart(&self, _ui: &mut UI) -> HabResult<()> { Ok(()) }
}
