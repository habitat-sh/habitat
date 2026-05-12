//! Consolidate logic for interacting with the Supervisor's control
//! gateway.

use crate::error::Result;
use futures::stream::StreamExt;
use habitat_common as common;
use habitat_common::{types::ResolvedListenCtlAddr,
                     ui::{UI,
                          UIWriter}};
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol as sup_proto;
use habitat_sup_protocol::codec::SrvMessage;
use std::{fmt,
          fs,
          io,
          path::Path,
          result,
          str::FromStr,
          time::{Duration,
                 Instant}};
use termcolor::{self,
                Color,
                ColorSpec};

const SUPERVISOR_STARTUP_WAIT: Duration = Duration::from_secs(30);
const SUPERVISOR_STARTUP_RETRY_INTERVAL: Duration = Duration::from_millis(500);
const RECENT_LOCK_FILE_THRESHOLD: Duration = Duration::from_secs(60);
const SUPERVISOR_LOCK_FILE: &str = "LOCK";

/// Connect to the Supervisor's control gateway, send a single
/// message, and process the reply.
///
/// Unfortunately not all control gateway-interacting functions use
/// this logic yet.
pub async fn send(remote_sup_addr: &ResolvedListenCtlAddr,
                  msg: impl Into<SrvMessage> + fmt::Debug)
                  -> Result<()> {
    let mut response = SrvClient::request(remote_sup_addr, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_ctl_reply(&reply)?;
    }
    Ok(())
}

/// Send a request to a local Supervisor that may still be starting.
///
/// The Supervisor writes its lock file before the control gateway starts
/// listening. During that short window, a `hab svc load` issued immediately
/// after `hab sup run` can otherwise fail with a generic connection-refused
/// error even though the Supervisor is actively coming up.
pub async fn send_waiting_for_startup(remote_sup_addr: &ResolvedListenCtlAddr,
                                      msg: impl Clone + Into<SrvMessage> + fmt::Debug)
                                      -> Result<()> {
    let start = Instant::now();
    let mut warned = false;

    loop {
        match send(remote_sup_addr, msg.clone()).await {
            Err(crate::error::Error::CtlClient(SrvClientError::ConnectionRefused))
                if should_wait_for_local_supervisor_startup(remote_sup_addr, start.elapsed()) =>
            {
                if !warned {
                    let mut ui = UI::default_with_env();
                    let _ = ui.warn("The local Supervisor is not accepting commands yet; \
                                     waiting for it to finish starting.");
                    warned = true;
                }
                tokio::time::sleep(SUPERVISOR_STARTUP_RETRY_INTERVAL).await;
            }
            result => return result,
        }
    }
}

fn should_wait_for_local_supervisor_startup(remote_sup_addr: &ResolvedListenCtlAddr,
                                            elapsed: Duration)
                                            -> bool {
    should_wait_for_local_supervisor_startup_with_lock_file(remote_sup_addr,
                                                            elapsed,
                                                            supervisor_lock_file_path())
}

fn should_wait_for_local_supervisor_startup_with_lock_file<P>(remote_sup_addr:
                                                                  &ResolvedListenCtlAddr,
                                                              elapsed: Duration,
                                                              lock_file_path: P)
                                                              -> bool
    where P: AsRef<Path>
{
    elapsed < SUPERVISOR_STARTUP_WAIT
    && remote_sup_addr.addr().ip().is_loopback()
    && has_recent_supervisor_lock_file(lock_file_path, RECENT_LOCK_FILE_THRESHOLD)
}

fn supervisor_lock_file_path() -> std::path::PathBuf {
    sup_proto::sup_root(None).join(SUPERVISOR_LOCK_FILE)
}

fn has_recent_supervisor_lock_file<P>(path: P, threshold: Duration) -> bool
    where P: AsRef<Path>
{
    fs::metadata(path).and_then(|metadata| metadata.modified())
                      .and_then(|modified| modified.elapsed().map_err(io::Error::other))
                      .is_ok_and(|age| age <= threshold)
}

////////////////////////////////////////////////////////////////////////

fn handle_ctl_reply(reply: &SrvMessage) -> result::Result<(), SrvClientError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs::File,
              time::Duration};
    use tempfile::TempDir;

    #[test]
    fn recent_supervisor_lock_file_is_detected() {
        let dir = TempDir::new().expect("tempdir");
        let lock_file = dir.path().join("LOCK");

        assert!(!has_recent_supervisor_lock_file(&lock_file, Duration::from_secs(60)));

        File::create(&lock_file).expect("create lock file");

        assert!(has_recent_supervisor_lock_file(&lock_file, Duration::from_secs(60)));
        assert!(!has_recent_supervisor_lock_file(&lock_file, Duration::ZERO));
    }

    #[test]
    fn startup_wait_is_only_for_local_addresses_during_the_wait_window() {
        let dir = TempDir::new().expect("tempdir");
        let lock_file = dir.path().join("LOCK");
        File::create(&lock_file).expect("create lock file");
        let local: ResolvedListenCtlAddr = "127.0.0.1:9632".parse().expect("local address");
        let remote: ResolvedListenCtlAddr = "192.0.2.1:9632".parse().expect("remote address");

        assert!(should_wait_for_local_supervisor_startup_with_lock_file(&local,
                                                                        Duration::from_secs(1),
                                                                        &lock_file));
        assert!(!should_wait_for_local_supervisor_startup_with_lock_file(&remote,
                                                                         Duration::from_secs(1),
                                                                         &lock_file));
        assert!(!should_wait_for_local_supervisor_startup_with_lock_file(&local,
                                                                         SUPERVISOR_STARTUP_WAIT,
                                                                         &lock_file));
    }
}
