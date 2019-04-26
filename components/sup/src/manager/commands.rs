// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! All the code for responding to Supervisor commands

use crate::{ctl_gateway::CtlRequest,
            error::Error,
            manager::{action::{ActionSender,
                               SupervisorAction},
                      service::{spec::{IntoServiceSpec,
                                       ServiceSpec},
                                DesiredState,
                                ProcessState},
                      ManagerState},
            util};
use habitat_butterfly as butterfly;
use habitat_common::{command::package::install::InstallSource,
                     outputln,
                     templating::package::Pkg,
                     ui::UIWriter};
use habitat_core::{package::{Identifiable,
                             PackageIdent,
                             PackageTarget},
                   service::ServiceGroup,
                   ChannelIdent};
use habitat_sup_protocol::{self as protocol,
                           net::{self,
                                 ErrCode,
                                 NetResult}};
use serde_json;
use std::{fmt,
          result};
use time::{self,
           Duration as TimeDuration,
           Timespec};
use toml;

static LOGKEY: &'static str = "CMD";

pub fn service_cfg(mgr: &ManagerState,
                   req: &mut CtlRequest,
                   opts: protocol::ctl::SvcGetDefaultCfg)
                   -> NetResult<()> {
    let ident: PackageIdent = opts.ident.ok_or_else(err_update_client)?.into();
    let mut msg = protocol::types::ServiceCfg { format:
                                                    Some(protocol::types::service_cfg::Format::Toml
                                                         as i32),
                                                default: None, };
    for service in mgr.services
                      .read()
                      .expect("Services lock is poisoned")
                      .values()
    {
        if service.pkg.ident.satisfies(&ident) {
            if let Some(ref cfg) = service.cfg.default {
                msg.default =
                    Some(toml::to_string_pretty(&toml::value::Value::Table(cfg.clone())).unwrap());
                req.reply_complete(msg);
            }
            return Ok(());
        }
    }
    Err(net::err(ErrCode::NotFound, format!("Service not loaded, {}", ident)))
}

pub fn service_cfg_validate(_mgr: &ManagerState,
                            req: &mut CtlRequest,
                            opts: protocol::ctl::SvcValidateCfg)
                            -> NetResult<()> {
    let cfg = opts.cfg.ok_or_else(err_update_client)?;
    let format = opts.format
                     .and_then(protocol::types::service_cfg::Format::from_i32)
                     .unwrap_or_default();
    if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
        return Err(net::err(ErrCode::EntityTooLarge, "Configuration too large."));
    }
    if format != protocol::types::service_cfg::Format::Toml {
        return Err(net::err(ErrCode::NotSupported,
                            format!("Configuration format {} not available.",
                                    format)));
    }
    let _new_cfg: toml::value::Table = toml::from_slice(&cfg).map_err(|e| {
                                                                 net::err(
            ErrCode::BadPayload,
            format!("Unable to decode configuration as {}, {}", format, e),
        )
                                                             })?;
    req.reply_complete(net::ok());
    Ok(())
    // JW TODO: Hold off on validation until we can validate services which aren't currently
    // loaded in the Supervisor but are known through rumor propagation.
    // let service_group: ServiceGroup = opts.service_group.into();
    // for service in mgr.services.read().unwrap().iter() {
    //     if service.service_group != service_group {
    //         continue;
    //     }
    //     if let Some(interface) = service.cfg.interface() {
    //         match Cfg::validate(interface, &new_cfg) {
    //             None => req.reply_complete(net::ok()),
    //             Some(errors) => {
    //                 for error in errors {
    //                     req.reply_partial(net::err(ErrCode::InvalidPayload, error));
    //                 }
    //                 req.reply_complete(net::ok());
    //             }
    //         }
    //         return Ok(());
    //     } else {
    //         // No interface, this service can't be configured.
    //         return Err(net::err(
    //             ErrCode::NotFound,
    //             "Service has no configurable attributes.",
    //         ));
    //     }
    // }
    // Err(net::err(
    //     ErrCode::NotFound,
    //     format!("Service not loaded, {}", service_group),
    // ))
}

pub fn service_cfg_set(mgr: &ManagerState,
                       req: &mut CtlRequest,
                       opts: protocol::ctl::SvcSetCfg)
                       -> NetResult<()> {
    let cfg = opts.cfg.ok_or_else(err_update_client)?;
    let is_encrypted = opts.is_encrypted.unwrap_or(false);
    let version = opts.version.ok_or_else(err_update_client)?;
    let service_group: ServiceGroup = opts.service_group.ok_or_else(err_update_client)?.into();
    if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
        return Err(net::err(ErrCode::EntityTooLarge, "Configuration too large."));
    }
    outputln!("Setting new configuration version {} for {}",
              version,
              service_group,);
    let mut client =
        match butterfly::client::Client::new(&mgr.cfg.gossip_listen.local_addr().to_string(),
                                             mgr.cfg.ring_key.clone())
        {
            Ok(client) => client,
            Err(err) => {
                outputln!("Failed to connect to own gossip server, {}", err);
                return Err(net::err(ErrCode::Internal, err.to_string()));
            }
        };
    client.send_service_config(service_group, version, &cfg, is_encrypted)
          .map_err(|e| net::err(ErrCode::Internal, e.to_string()))
          .map(|_| {
              req.reply_complete(net::ok());
          })
}

pub fn service_file_put(mgr: &ManagerState,
                        req: &mut CtlRequest,
                        opts: protocol::ctl::SvcFilePut)
                        -> NetResult<()> {
    let content = opts.content.ok_or_else(err_update_client)?;
    let filename = opts.filename.ok_or_else(err_update_client)?;
    let is_encrypted = opts.is_encrypted.unwrap_or(false);
    let version = opts.version.ok_or_else(err_update_client)?;
    let service_group: ServiceGroup = opts.service_group.ok_or_else(err_update_client)?.into();
    if content.len() > protocol::butterfly::MAX_FILE_PUT_SIZE_BYTES {
        return Err(net::err(ErrCode::EntityTooLarge, "File content too large."));
    }
    outputln!("Receiving new version {} of file {} for {}",
              version,
              filename,
              service_group,);
    let mut client =
        match butterfly::client::Client::new(&mgr.cfg.gossip_listen.local_addr().to_string(),
                                             mgr.cfg.ring_key.clone())
        {
            Ok(client) => client,
            Err(err) => {
                outputln!("Failed to connect to own gossip server, {}", err);
                return Err(net::err(ErrCode::Internal, err.to_string()));
            }
        };
    client.send_service_file(service_group, filename, version, &content, is_encrypted)
          .map_err(|e| net::err(ErrCode::Internal, e.to_string()))
          .map(|_| {
              req.reply_complete(net::ok());
          })
}

pub fn service_load(mgr: &ManagerState,
                    req: &mut CtlRequest,
                    opts: &protocol::ctl::SvcLoad)
                    -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or_else(err_update_client)?.into();
    let bldr_url = opts.bldr_url
                       .clone()
                       .unwrap_or_else(|| protocol::DEFAULT_BLDR_URL.to_string());
    let bldr_channel = opts.bldr_channel
                           .clone()
                           .map(ChannelIdent::from)
                           .unwrap_or_default();
    let force = opts.force.unwrap_or(false);
    let source = InstallSource::Ident(ident.clone(), PackageTarget::active_target());
    match mgr.cfg.spec_for_ident(source.as_ref()) {
        None => {
            let mut spec = ServiceSpec::default();
            opts.into_spec(&mut spec);

            // We don't have any record of this thing; let's set it up!
            //
            // If a package exists on disk that satisfies the
            // desired package identifier, it will be used;
            // otherwise, we'll install the latest suitable
            // version from the specified Builder channel.
            util::pkg::satisfy_or_install(req, &source, &bldr_url, &bldr_channel)?;

            mgr.cfg.save_spec_for(&spec)?;
            req.info(format!("The {} service was successfully loaded", spec.ident))?;
        }
        Some(mut spec) => {
            // We've seen this service  before. Thus `load`
            // basically acts as a way to edit spec files on the
            // command line. As a result, we a) check that you
            // *really* meant to change an existing spec, and b) DO
            // NOT download a potentially new version of the package
            // in question

            if !force {
                return Err(net::err(ErrCode::Conflict,
                                    format!("Service already loaded, unload '{}' \
                                             and try again",
                                            ident)));
            }

            opts.into_spec(&mut spec);

            // Only install if we don't have something
            // locally; otherwise you could potentially
            // upgrade each time you load.
            //
            // Also make sure you're pulling from where you're
            // supposed to be pulling from!
            util::pkg::satisfy_or_install(req, &source, &spec.bldr_url, &spec.channel)?;

            mgr.cfg.save_spec_for(&spec)?;
            req.info(format!("The {} service was successfully loaded", spec.ident))?;
        }
    }
    req.reply_complete(net::ok());
    Ok(())
}

pub fn service_unload(mgr: &ManagerState,
                      req: &mut CtlRequest,
                      opts: protocol::ctl::SvcUnload,
                      action_sender: &ActionSender)
                      -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or_else(err_update_client)?.into();
    if let Some(service_spec) = mgr.cfg.spec_for_ident(&ident) {
        let shutdown_spec = opts.into();
        let action = SupervisorAction::UnloadService { service_spec,
                                                       shutdown_spec };
        send_action(action, action_sender)?;

        // JW TODO: Change this to unloaded from unloading when the Supervisor waits for
        // the work to complete.
        req.info(format!("Unloading {}", ident))?;
        req.reply_complete(net::ok());
        Ok(())
    } else {
        Err(net::err(ErrCode::Internal,
                     sup_error!(Error::ServiceNotLoaded(ident))))
    }
}

pub fn service_start(mgr: &ManagerState,
                     req: &mut CtlRequest,
                     opts: protocol::ctl::SvcStart)
                     -> NetResult<()> {
    let ident = opts.ident.ok_or_else(err_update_client)?.into();
    match mgr.cfg.spec_for_ident(&ident) {
        Some(mut spec) => {
            if spec.desired_state == DesiredState::Down {
                spec.desired_state = DesiredState::Up;
                mgr.cfg.save_spec_for(&spec)?;

                // JW TODO: Change the language of the message below to "started" when we actually
                // synchronously control services from the ctl gateway.
                req.info(format!("Supervisor starting {}. See the Supervisor output for more \
                                  details.",
                                 &ident))?;
            }
        }
        None => {
            return Err(net::err(ErrCode::NotFound, format!("Service not loaded, {}", &ident)));
        }
    };
    req.reply_complete(net::ok());
    Ok(())
}

pub fn service_stop(mgr: &ManagerState,
                    req: &mut CtlRequest,
                    opts: protocol::ctl::SvcStop,
                    action_sender: &ActionSender)
                    -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or_else(err_update_client)?.into();
    match mgr.cfg.spec_for_ident(&ident) {
        Some(service_spec) => {
            if service_spec.desired_state == DesiredState::Up {
                let shutdown_spec = opts.into();
                let action = SupervisorAction::StopService { service_spec,
                                                             shutdown_spec };
                send_action(action, action_sender)?;

                // JW TODO: Change the langauge of the message below to "stopped" when we actually
                // synchronously control services from the ctl gateway.
                req.info(format!("Supervisor stopping {}. See the Supervisor output for more \
                                  details.",
                                 &ident))?;
            }
        }
        None => {
            return Err(net::err(ErrCode::NotFound, format!("Service not loaded, {}", &ident)));
        }
    };

    req.reply_complete(net::ok());
    Ok(())
}

pub fn supervisor_depart(mgr: &ManagerState,
                         req: &mut CtlRequest,
                         opts: protocol::ctl::SupDepart)
                         -> NetResult<()> {
    let member_id = opts.member_id.ok_or_else(err_update_client)?;
    let mut client =
        match butterfly::client::Client::new(&mgr.cfg.gossip_listen.local_addr().to_string(),
                                             mgr.cfg.ring_key.clone())
        {
            Ok(client) => client,
            Err(err) => {
                outputln!("Failed to connect to own gossip server, {}", err);
                return Err(net::err(ErrCode::Internal, err.to_string()));
            }
        };
    outputln!("Attempting to depart member: {}", member_id);
    match client.send_departure(&member_id) {
        Ok(()) => {
            req.reply_complete(net::ok());
            Ok(())
        }
        Err(e) => Err(net::err(ErrCode::Internal, e.to_string())),
    }
}

pub fn service_status(mgr: &ManagerState,
                      req: &mut CtlRequest,
                      opts: protocol::ctl::SvcStatus)
                      -> NetResult<()> {
    let services_data = &mgr.gateway_state
                            .read()
                            .expect("GatewayState lock is poisoned")
                            .services_data;
    let statuses: Vec<ServiceStatus> = serde_json::from_str(&services_data).map_err(|e| {
                                           sup_error!(Error::ServiceDeserializationError(e))
                                       })?;

    if let Some(ident) = opts.ident {
        for status in statuses {
            if status.pkg.ident.satisfies(&ident) {
                let msg: protocol::types::ServiceStatus = status.into();
                req.reply_complete(msg);
                return Ok(());
            }
        }
        return Err(net::err(ErrCode::NotFound, format!("Service not loaded, {}", ident)));
    }

    // We're not dealing with a single service, but with all of them.
    if statuses.is_empty() {
        req.reply_complete(net::ok());
    } else {
        let mut list = statuses.into_iter().peekable();
        while let Some(status) = list.next() {
            let msg: protocol::types::ServiceStatus = status.into();
            if list.peek().is_some() {
                req.reply_partial(msg);
            } else {
                req.reply_complete(msg);
            }
        }
    }
    Ok(())
}

////////////////////////////////////////////////////////////////////////
// Private helper functions
fn err_update_client() -> net::NetErr { net::err(ErrCode::UpdateClient, "client out of date") }

#[derive(Deserialize)]
struct ServiceStatus {
    pkg:           Pkg,
    process:       ProcessStatus,
    service_group: ServiceGroup,
    desired_state: DesiredState,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "{}, {}, group:{}",
               self.pkg.ident, self.process, self.service_group,)
    }
}

impl From<ServiceStatus> for protocol::types::ServiceStatus {
    fn from(other: ServiceStatus) -> Self {
        let mut proto = protocol::types::ServiceStatus::default();
        proto.ident = other.pkg.ident.into();
        proto.process = Some(other.process.into());
        proto.service_group = other.service_group.into();
        proto.desired_state = Some(other.desired_state.into());
        proto
    }
}

#[derive(Deserialize)]
struct ProcessStatus {
    #[serde(deserialize_with = "deserialize_time", rename = "state_entered")]
    elapsed: TimeDuration,
    pid: Option<u32>,
    state: ProcessState,
}

impl fmt::Display for ProcessStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.pid {
            Some(pid) => {
                write!(f,
                       "state:{}, time:{}, pid:{}",
                       self.state, self.elapsed, pid)
            }
            None => write!(f, "state:{}, time:{}", self.state, self.elapsed),
        }
    }
}

impl From<ProcessStatus> for protocol::types::ProcessStatus {
    fn from(other: ProcessStatus) -> Self {
        let mut proto = protocol::types::ProcessStatus::default();
        proto.elapsed = Some(other.elapsed.num_seconds());
        proto.state = other.state.into();
        if let Some(pid) = other.pid {
            proto.pid = Some(pid);
        }
        proto
    }
}

fn deserialize_time<'de, D>(d: D) -> result::Result<TimeDuration, D::Error>
    where D: serde::Deserializer<'de>
{
    struct FromTimespec;

    impl<'de> serde::de::Visitor<'de> for FromTimespec {
        type Value = TimeDuration;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a i64 integer")
        }

        fn visit_u64<R>(self, value: u64) -> result::Result<TimeDuration, R>
            where R: serde::de::Error
        {
            let tspec = Timespec { sec:  (value as i64),
                                   nsec: 0, };
            Ok(time::get_time() - tspec)
        }
    }

    d.deserialize_u64(FromTimespec)
}

/// Helper function to ensure that all errors in sending are handled identically.
fn send_action(action: SupervisorAction, sender: &ActionSender) -> NetResult<()> {
    if sender.send(action).is_err() {
        // This would happen if the manager somehow went
        // away... in other words, something has gone *very* wrong.
        Err(net::err(ErrCode::Internal, "Action could not be completed"))
    } else {
        Ok(())
    }
}
