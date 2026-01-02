use clap_v4 as clap;

use std::{convert::Into,
          io::Write,
          str::FromStr};

use clap::Parser;
use futures::stream::StreamExt;
use tabwriter::TabWriter;

use habitat_common::cli::clap_validators::HabPkgIdentValueParser;
use habitat_core::package::PackageIdent;
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol::{codec::SrvMessage,
                           net::NetErr,
                           types::{DesiredState,
                                   ProcessState,
                                   ServiceStatus}};

use crate::{cli_v4::utils::RemoteSup,
            error::Result as HabResult};

lazy_static::lazy_static! {
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

/// Query the status of Habitat services
#[derive(Clone, Debug, Parser)]
#[command(author = "\nThe Habitat Maintainers <humans@habitat.sh>",
          help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct StatusCommand {
    /// A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    #[arg(name = "PKG_IDENT", value_parser = HabPkgIdentValueParser::simple())]
    pkg_ident: Option<PackageIdent>,

    #[command(flatten)]
    remote_sup: RemoteSup,
}

impl StatusCommand {
    pub(crate) async fn do_command(&self) -> HabResult<()> {
        let msg =
            habitat_sup_protocol::ctl::SvcStatus { ident: self.pkg_ident.clone().map(Into::into), };

        let mut out = TabWriter::new(std::io::stdout());
        let mut response = SrvClient::request(self.remote_sup.inner(), msg).await?;
        // Ensure there is at least one result from the server otherwise produce an error
        match response.next().await {
            Some(message_result) => {
                let reply = message_result?;
                print_svc_status(&mut out, &reply, true)?;
            }
            _ => {
                return Err(SrvClientError::from(std::io::Error::from(std::io::ErrorKind::UnexpectedEof)).into());
            }
        }
        while let Some(message_result) = response.next().await {
            let reply = message_result?;
            print_svc_status(&mut out, &reply, false)?;
        }
        out.flush()?;
        Ok(())
    }
}

fn print_svc_status<T>(out: &mut T,
                       reply: &SrvMessage,
                       print_header: bool)
                       -> Result<(), SrvClientError>
    where T: Write
{
    let status = match reply.message_id() {
        "ServiceStatus" => {
            reply.parse::<ServiceStatus>()
                 .map_err(SrvClientError::Decode)?
        }
        "NetOk" => {
            println!("No services loaded.");
            return Ok(());
        }
        "NetErr" => {
            let err = reply.parse::<NetErr>().map_err(SrvClientError::Decode)?;
            return Err(SrvClientError::from(err));
        }
        _ => {
            log::warn!("Unexpected status message, {:?}", reply);
            return Ok(());
        }
    };
    let svc_desired_state = status.desired_state
                                  .map_or_else(|| "<none>".to_string(), |s| s.to_string());
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
