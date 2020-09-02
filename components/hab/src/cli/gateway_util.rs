//! Consolidate logic for interacting with the Supervisor's control
//! gateway.

use futures::stream::StreamExt;
use habitat_common::{self as common,
                     types::ListenCtlAddr,
                     ui::{UIWriter,
                          UI}};
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol as sup_proto;
use habitat_sup_protocol::{codec::SrvMessage,
                           message::MessageStatic};
use prost::Message;
use std::{fmt,
          io::{self,
               Error as IoError,
               ErrorKind as IoErrorKind},
          str::FromStr};
use termcolor::{self,
                Color,
                ColorSpec};

/// Connect to the Supervisor's control gateway, send a single
/// message, and wait for a single reply of a specific type.
pub async fn send_expect_response<M, R>(remote_sup_addr: &ListenCtlAddr,
                                        msg: M)
                                        -> Result<R, SrvClientError>
    where M: Into<SrvMessage> + fmt::Debug,
          R: Default + MessageStatic + Message
{
    let mut response = SrvClient::request(remote_sup_addr, msg).await?;
    if let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            id if id == R::MESSAGE_ID => reply.parse::<R>().map_err(SrvClientError::Decode),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                Err(m.into())
            }
            id => {
                Err(IoError::new(IoErrorKind::InvalidData,
                                 format!("received unexpected message '{}'", id)).into())
            }
        }
    } else {
        Err(IoError::new(IoErrorKind::UnexpectedEof,
                         format!("communication with the server ended before a '{}' message was \
                                  received",
                                 R::MESSAGE_ID)).into())
    }
}

/// Connect to the Supervisor's control gateway, send a single
/// message, and process replies showing progress in the console.
pub async fn send_with_progress(remote_sup_addr: &ListenCtlAddr,
                                msg: impl Into<SrvMessage> + fmt::Debug)
                                -> Result<(), SrvClientError> {
    let mut response = SrvClient::request(remote_sup_addr, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_reply_with_progress(&reply)?;
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////

fn handle_reply_with_progress(reply: &SrvMessage) -> Result<(), SrvClientError> {
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
