// Implemenatation of `hab sup status`

use clap_v4 as clap;

use clap::Parser;

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult};
use habitat_core::package::PackageIdent;

#[cfg(not(target_os = "macos"))]
use habitat_common::types::ResolvedListenCtlAddr;

#[cfg(not(target_os = "macos"))]
use habitat_sup_client::{SrvClient,
                         SrvClientError};

#[cfg(not(target_os = "macos"))]
use habitat_sup_protocol::{self as sup_proto,
                           codec::*,
                           types::*};

#[cfg(not(target_os = "macos"))]
use futures::stream::StreamExt;

#[cfg(not(target_os = "macos"))]
use std::{io::{self,
               Write},
          result,
          str::FromStr};

#[cfg(not(target_os = "macos"))]
use tabwriter::TabWriter;

#[cfg(not(target_os = "macos"))]
use log::warn;

#[cfg(not(target_os = "macos"))]
use habitat_common::ui::{self,
                         UIWriter};

#[cfg(not(target_os = "macos"))]
use lazy_static::lazy_static;

#[cfg(not(target_os = "macos"))]
lazy_static! {
    static ref STATUS_HEADER: Vec<&'static str> = {
        vec!["package",
             "type",
             "desired",
             "state",
             "elapsed (s)",
             "pid",
             "group",]
    };
}

#[derive(Debug, Clone, Parser)]
#[command(disable_version_flag = true,
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct SupStatusOptions {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT")]
    pkg_ident: Option<PackageIdent>,

    // Remote supervisor connection option
    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl SupStatusOptions {
    #[cfg(not(target_os = "macos"))]
    pub(super) async fn do_status(&self) -> HabResult<()> {
        let mut ui = ui::ui();
        ui.warn("'hab sup status' as an alias for 'hab svc status' is deprecated. Please update \
                 your automation and processes accordingly.")?;
        return sub_svc_status(self.pkg_ident.clone(), self.remote_sup.inner()).await;
    }

    #[cfg(target_os = "macos")]
    pub(super) async fn do_status(&self) -> HabResult<()> { Ok(()) }
}

#[cfg(not(target_os = "macos"))]
async fn sub_svc_status(pkg_ident: Option<PackageIdent>,
                        remote_sup: &ResolvedListenCtlAddr)
                        -> HabResult<()> {
    let msg = sup_proto::ctl::SvcStatus { ident: pkg_ident.map(Into::into), };

    let mut out = TabWriter::new(io::stdout());
    let mut response = SrvClient::request(remote_sup, msg).await?;
    // Ensure there is at least one result from the server otherwise produce an error
    if let Some(message_result) = response.next().await {
        let reply = message_result?;
        print_svc_status(&mut out, &reply, true)?;
    } else {
        return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into());
    }
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        print_svc_status(&mut out, &reply, false)?;
    }
    out.flush()?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn print_svc_status<T>(out: &mut T,
                       reply: &SrvMessage,
                       print_header: bool)
                       -> result::Result<(), SrvClientError>
    where T: io::Write
{
    let status = match reply.message_id() {
        "ServiceStatus" => {
            reply.parse::<sup_proto::types::ServiceStatus>()
                 .map_err(SrvClientError::Decode)?
        }
        "NetOk" => {
            println!("No services loaded.");
            return Ok(());
        }
        "NetErr" => {
            let err = reply.parse::<sup_proto::net::NetErr>()
                           .map_err(SrvClientError::Decode)?;
            return Err(SrvClientError::from(err));
        }
        _ => {
            warn!("Unexpected status message, {:?}", reply);
            return Ok(());
        }
    };
    let svc_desired_state = status.desired_state
                                  .map_or("<none>".to_string(), |s| s.to_string());
    let (svc_state, svc_pid, svc_elapsed) = {
        match status.process {
            Some(process) => {
                (process.state.to_string(),
                 process.pid
                        .map_or_else(|| "<none>".to_string(), |p| p.to_string()),
                 process.elapsed.unwrap_or_default().to_string())
            }
            None => {
                (ProcessState::default().to_string(), "<none>".to_string(), "<none>".to_string())
            }
        }
    };
    if print_header {
        writeln!(out, "{}", STATUS_HEADER.join("\t")).unwrap();
    }
    // Composites were removed in 0.75 but people could be
    // depending on the exact format of this output even if they
    // never used composites. We don't want to break their tooling
    // so we hardcode in 'standalone' as it's the only supported
    // package type
    //
    // TODO: Remove this when we have a stable machine-readable alternative
    // that scripts could depend on
    writeln!(out,
             "{}\tstandalone\t{}\t{}\t{}\t{}\t{}",
             status.ident,
             DesiredState::from_str(&svc_desired_state)?,
             ProcessState::from_str(&svc_state)?,
             svc_elapsed,
             svc_pid,
             status.service_group,)?;
    Ok(())
}
