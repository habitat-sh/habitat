// Implemenatation of `hab sup restart`

use clap_v4 as clap;

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult};
use clap::Parser;

#[cfg(not(target_os = "macos"))]
use habitat_sup_client::{SrvClient,
                         SrvClientError};

#[cfg(not(target_os = "macos"))]
use std::io;

#[cfg(not(target_os = "macos"))]
use habitat_common::ui::{self,
                         UIWriter};

#[cfg(not(target_os = "macos"))]
use futures::stream::StreamExt;

#[cfg(not(target_os = "macos"))]
use habitat_sup_protocol as sup_proto;

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct SupRestartOptions {
    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl SupRestartOptions {
    #[cfg(not(target_os = "macos"))]
    pub(super) async fn do_restart(&self) -> HabResult<()> {
        let mut ui = ui::ui();
        let msg = sup_proto::ctl::SupRestart::default();

        ui.begin(format!("Restarting supervisor {}", self.remote_sup.inner()))?;
        let mut response = SrvClient::request(self.remote_sup.inner(), msg).await?;
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
        ui.end("Restart recorded.")?;
        Ok(())
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn do_restart(&self) -> HabResult<()> { Ok(()) }
}
