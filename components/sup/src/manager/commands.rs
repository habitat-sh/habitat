//! All the code for responding to Supervisor commands

use crate::{ctl_gateway::CtlRequest,
            error::Error,
            manager::{ManagerState,
                      action::{ActionSender,
                               SupervisorAction},
                      service::{DesiredState,
                                spec::ServiceSpec}},
            util};
use habitat_butterfly as butterfly;
use habitat_common::{command::package::install::InstallSource,
                     outputln,
                     ui::UIWriter};
use habitat_core::{package::{Identifiable,
                             PackageIdent,
                             PackageTarget},
                   service::ServiceGroup};
use habitat_sup_protocol::{self as protocol,
                           net::{self,
                                 ErrCode,
                                 NetResult}};
use std::{convert::TryFrom,
          str,
          sync::atomic::Ordering};

static LOGKEY: &str = "CMD";

/// # Locking (see locking.md)
/// * `ManagerServices::inner` (read)
pub fn service_cfg_msr(mgr: &ManagerState,
                       req: &mut CtlRequest,
                       opts: protocol::ctl::SvcGetDefaultCfg)
                       -> NetResult<()> {
    let ident: PackageIdent = opts.ident.ok_or_else(err_update_client)?.into();
    let mut msg = protocol::types::ServiceCfg { format:
                                                    Some(protocol::types::service_cfg::Format::Toml
                                                         as i32),
                                                default: None, };
    for service in mgr.services.lock_msr().running_services() {
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
                     .and_then(|f| protocol::types::service_cfg::Format::try_from(f).ok())
                     .unwrap_or_default();
    if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
        return Err(net::err(ErrCode::EntityTooLarge, "Configuration too large."));
    }
    if format != protocol::types::service_cfg::Format::Toml {
        return Err(net::err(ErrCode::NotSupported,
                            format!("Configuration format {} not available.",
                                    format)));
    }
    let cfg_str =
        str::from_utf8(&cfg).map_err(|e| {
                                net::err(ErrCode::BadPayload,
                                         format!("Unable to decode configuration to string, {}", e))
                            })?;
    let _new_cfg: toml::value::Table = toml::from_str(cfg_str).map_err(|e| {
                                                                  net::err(ErrCode::BadPayload,
                                            format!("Unable to decode configuration as {}, {}",
                                                    format, e))
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

pub async fn service_load(mgr: &ManagerState,
                          req: &mut CtlRequest,
                          opts: protocol::ctl::SvcLoad)
                          -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or_else(err_update_client)?.into();
    let source = InstallSource::Ident(ident.clone(), PackageTarget::active_target());
    let spec = if let Some(spec) = mgr.cfg.spec_for_ident(source.as_ref()) {
        // We've seen this service before. Thus `load` acts as a way to edit spec files from the
        // command line. As a result, we check that you *really* meant to change an existing spec.
        if !opts.force.unwrap_or(false) {
            return Err(net::err(ErrCode::Conflict,
                                format!("Service already loaded. Unload '{}' \
                                         and try again, or load with the \
                                         --force flag to reload and restart the \
                                         service.",
                                        ident)));
        }
        spec.merge_svc_load(opts)?
    } else {
        ServiceSpec::try_from(opts)?
    };

    let package = util::pkg::satisfy_or_install(req, &source, &spec.bldr_url, &spec.channel).await?;
    spec.validate(&package)?;
    mgr.cfg.save_spec_for(&spec)?;

    req.info(format!("The {} service was successfully loaded", spec.ident))?;
    req.reply_complete(net::ok());
    Ok(())
}

pub fn service_update(mgr: &ManagerState,
                      req: &mut CtlRequest,
                      opts: protocol::ctl::SvcUpdate,
                      action_sender: &ActionSender)
                      -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or_else(err_update_client)?.into();
    if let Some(mut service_spec) = mgr.cfg.spec_for_ident(&ident) {
        service_spec.merge_svc_update(opts);
        let action = SupervisorAction::UpdateService { service_spec };
        send_action(action, action_sender)?;

        req.info(format!("Updating {}", ident))?;
        req.reply_complete(net::ok());
        Ok(())
    } else {
        Err(net::err(ErrCode::Internal, Error::ServiceNotLoaded(ident)))
    }
}

pub fn service_unload(mgr: &ManagerState,
                      req: &mut CtlRequest,
                      opts: protocol::ctl::SvcUnload,
                      action_sender: &ActionSender)
                      -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or_else(err_update_client)?.into();
    if let Some(service_spec) = mgr.cfg.spec_for_ident(&ident) {
        let shutdown_input = opts.into();
        let action = SupervisorAction::UnloadService { service_spec,
                                                       shutdown_input };
        send_action(action, action_sender)?;

        // JW TODO: Change this to unloaded from unloading when the Supervisor waits for
        // the work to complete.
        req.info(format!("Unloading {}", ident))?;
        req.reply_complete(net::ok());
        Ok(())
    } else {
        Err(net::err(ErrCode::Internal, Error::ServiceNotLoaded(ident)))
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
                let shutdown_input = opts.into();
                let action = SupervisorAction::StopService { service_spec,
                                                             shutdown_input };
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

#[allow(clippy::needless_pass_by_value)]
pub fn supervisor_restart(mgr: &ManagerState,
                          _req: &mut CtlRequest,
                          _opts: protocol::ctl::SupRestart)
                          -> NetResult<()> {
    mgr.should_restart.store(true, Ordering::Relaxed);
    Ok(())
}

/// # Locking (see locking.md)
/// * `GatewayState::inner` (read)
pub fn service_status_gsr(mgr: &ManagerState,
                          req: &mut CtlRequest,
                          opts: protocol::ctl::SvcStatus)
                          -> NetResult<()> {
    if let Some(ident) = opts.ident {
        let service_status = mgr.gateway_state
                                .lock_gsr()
                                .services_data()
                                .iter()
                                .find_map(|service| {
                                    if service.pkg.ident.satisfies(&ident) {
                                        Some(protocol::types::ServiceStatus::from(service))
                                    } else {
                                        None
                                    }
                                });
        match service_status {
            Some(service_status) => {
                req.reply_complete(service_status);
                Ok(())
            }
            None => Err(net::err(ErrCode::NotFound, format!("Service not loaded, {}", ident))),
        }
    } else {
        // We're not dealing with a single service, but with all of them.
        // We serialize service data into messages completely before sending them over the network
        // to minimize locking of the gateway state.
        let service_statuses: Vec<_> = mgr.gateway_state
                                          .lock_gsr()
                                          .services_data()
                                          .iter()
                                          .map(protocol::types::ServiceStatus::from)
                                          .collect();
        if service_statuses.is_empty() {
            req.reply_complete(net::ok());
        } else {
            let mut list = service_statuses.into_iter().peekable();
            while let Some(service_status) = list.next() {
                if list.peek().is_some() {
                    req.reply_partial(service_status);
                } else {
                    req.reply_complete(service_status);
                }
            }
        }
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////
// Private helper functions
fn err_update_client() -> net::NetErr { net::err(ErrCode::UpdateClient, "client out of date") }

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
