//! Consolidate logic for interacting with the Supervisor's control
//! gateway.

use crate::{config,
            error::Result};
use futures::stream::StreamExt;
use habitat_common as common;
use habitat_common::{types::ListenCtlAddr,
                     ui::{UIWriter,
                          UI}};
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol as sup_proto;
use habitat_sup_protocol::{codec::SrvMessage,
                           message::MessageStatic};
use prost::Message;
use std::{fmt,
          io,
          result,
          str::FromStr};
use termcolor::{self,
                Color,
                ColorSpec};

/// Connect to the Supervisor's control gateway, send a single
/// message, and wait for a single reply of a specific type.
pub async fn send_expect_response<M, R>(remote_sup_addr: &ListenCtlAddr,
                                        msg: M)
                                        -> Result<Option<R>>
    where M: Into<SrvMessage> + fmt::Debug,
          R: Default + MessageStatic + Message
{
    let cfg = config::load()?;
    let secret_key = config::ctl_secret_key(&cfg)?;

    let mut response = SrvClient::request(remote_sup_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            id if id == R::MESSAGE_ID => {
                Some(reply.parse::<R>().map_err(SrvClientError::Decode)?);
            }
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                return Err(SrvClientError::from(m).into());
            }
            _ => {
                return {
                    Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into())
                }
            }
        }
    }
    Ok(None)
}

/// Connect to the Supervisor's control gateway, send a single
/// message, and process replies showing progress in the console.
pub async fn send_with_progress(remote_sup_addr: &ListenCtlAddr,
                                msg: impl Into<SrvMessage> + fmt::Debug)
                                -> Result<()> {
    let cfg = config::load()?;
    let secret_key = config::ctl_secret_key(&cfg)?;

    let mut response = SrvClient::request(remote_sup_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_reply_with_progress(&reply)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////

fn handle_reply_with_progress(reply: &SrvMessage) -> result::Result<(), SrvClientError> {
    let mut progress_bar = pbr::ProgressBar::<io::Stdout>::new(0);
    progress_bar.set_units(pbr::Units::Bytes);
    progress_bar.show_tick = true;
    progress_bar.message("    ");
    match reply.message_id() {
        "ConsoleLine" => {
            let m = reply.parse::<sup_proto::ctl::ConsoleLine>()
                         .map_err(SrvClientError::Decode)?;
            let mut new_spec = ColorSpec::new();
            let msg_spec = match m.color {
                Some(color) => {
                    new_spec.set_fg(Some(Color::from_str(&color)?))
                            .set_bold(m.bold)
                }
                None => new_spec.set_bold(m.bold),
            };
            common::ui::print(UI::default_with_env().out(), m.line.as_bytes(), msg_spec)?;
        }
        "NetProgress" => {
            let m = reply.parse::<sup_proto::ctl::NetProgress>()
                         .map_err(SrvClientError::Decode)?;
            progress_bar.total = m.total;
            if progress_bar.set(m.position) >= m.total {
                progress_bar.finish();
            }
        }
        "NetErr" => {
            let m = reply.parse::<sup_proto::net::NetErr>()
                         .map_err(SrvClientError::Decode)?;
            return Err(SrvClientError::from(m));
        }
        _ => (),
    }
    Ok(())
}
