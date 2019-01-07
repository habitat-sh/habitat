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

use std::{
    collections::HashSet,
    fmt, fs,
    path::{Path, PathBuf},
    result,
};

use serde_json;
use time::{self, Duration as TimeDuration, Timespec};
use toml;

use crate::butterfly;
use crate::common::{command::package::install::InstallSource, ui::UIWriter};
use crate::ctl_gateway::CtlRequest;
use crate::error::{Error, Result};
use crate::hcore::{
    fs::FS_ROOT_PATH,
    package::metadata::PackageType,
    package::{Identifiable, PackageIdent, PackageInstall, PackageTarget},
    service::ServiceGroup,
};
use crate::manager::{
    service::{
        spec::{IntoServiceSpec, ServiceSpec},
        CompositeSpec, DesiredState, Pkg, ProcessState, Spec,
    },
    ManagerConfig, ManagerState,
};
use crate::protocol::{
    self,
    net::{self, ErrCode, NetResult},
};
use crate::util;

static LOGKEY: &'static str = "CMD";

pub fn service_cfg(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcGetDefaultCfg,
) -> NetResult<()> {
    let ident: PackageIdent = opts.ident.ok_or(err_update_client())?.into();
    let mut msg = protocol::types::ServiceCfg {
        format: Some(protocol::types::service_cfg::Format::Toml as i32),
        default: None,
    };
    for service in mgr
        .services
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
    Err(net::err(
        ErrCode::NotFound,
        format!("Service not loaded, {}", ident),
    ))
}

pub fn service_cfg_validate(
    _mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcValidateCfg,
) -> NetResult<()> {
    let cfg = opts.cfg.ok_or(err_update_client())?;
    let format = opts
        .format
        .and_then(protocol::types::service_cfg::Format::from_i32)
        .unwrap_or_default();
    if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
        return Err(net::err(
            ErrCode::EntityTooLarge,
            "Configuration too large.",
        ));
    }
    if format != protocol::types::service_cfg::Format::Toml {
        return Err(net::err(
            ErrCode::NotSupported,
            format!("Configuration format {} not available.", format),
        ));
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

pub fn service_cfg_set(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcSetCfg,
) -> NetResult<()> {
    let cfg = opts.cfg.ok_or(err_update_client())?;
    let is_encrypted = opts.is_encrypted.unwrap_or(false);
    let version = opts.version.ok_or(err_update_client())?;
    let service_group: ServiceGroup = opts.service_group.ok_or(err_update_client())?.into();
    if cfg.len() > protocol::butterfly::MAX_SVC_CFG_SIZE {
        return Err(net::err(
            ErrCode::EntityTooLarge,
            "Configuration too large.",
        ));
    }
    outputln!(
        "Setting new configuration version {} for {}",
        version,
        service_group,
    );
    let mut client = match butterfly::client::Client::new(
        mgr.cfg.gossip_listen.local_addr(),
        mgr.cfg.ring_key.clone(),
    ) {
        Ok(client) => client,
        Err(err) => {
            outputln!("Failed to connect to own gossip server, {}", err);
            return Err(net::err(ErrCode::Internal, err.to_string()));
        }
    };
    match client.send_service_config(service_group, version, cfg, is_encrypted) {
        Ok(()) => {
            req.reply_complete(net::ok());
            return Ok(());
        }
        Err(e) => return Err(net::err(ErrCode::Internal, e.to_string())),
    }
}

pub fn service_file_put(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcFilePut,
) -> NetResult<()> {
    let content = opts.content.ok_or(err_update_client())?;
    let filename = opts.filename.ok_or(err_update_client())?;
    let is_encrypted = opts.is_encrypted.unwrap_or(false);
    let version = opts.version.ok_or(err_update_client())?;
    let service_group: ServiceGroup = opts.service_group.ok_or(err_update_client())?.into();
    if content.len() > protocol::butterfly::MAX_FILE_PUT_SIZE_BYTES {
        return Err(net::err(ErrCode::EntityTooLarge, "File content too large."));
    }
    outputln!(
        "Receiving new version {} of file {} for {}",
        version,
        filename,
        service_group,
    );
    let mut client = match butterfly::client::Client::new(
        mgr.cfg.gossip_listen.local_addr(),
        mgr.cfg.ring_key.clone(),
    ) {
        Ok(client) => client,
        Err(err) => {
            outputln!("Failed to connect to own gossip server, {}", err);
            return Err(net::err(ErrCode::Internal, err.to_string()));
        }
    };
    match client.send_service_file(service_group, filename, version, content, is_encrypted) {
        Ok(()) => {
            req.reply_complete(net::ok());
            return Ok(());
        }
        Err(e) => return Err(net::err(ErrCode::Internal, e.to_string())),
    }
}

pub fn service_load(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcLoad,
) -> NetResult<()> {
    let ident: PackageIdent = opts.ident.clone().ok_or(err_update_client())?.into();
    let bldr_url = opts
        .bldr_url
        .clone()
        .unwrap_or(protocol::DEFAULT_BLDR_URL.to_string());
    let bldr_channel = opts
        .bldr_channel
        .clone()
        .unwrap_or(protocol::DEFAULT_BLDR_CHANNEL.to_string());
    let force = opts.force.clone().unwrap_or(false);
    let source = InstallSource::Ident(ident.clone(), *PackageTarget::active_target());
    match existing_specs_for_ident(&mgr.cfg, source.as_ref())? {
        None => {
            // We don't have any record of this thing; let's set it up!
            //
            // If a package exists on disk that satisfies the
            // desired package identifier, it will be used;
            // otherwise, we'll install the latest suitable
            // version from the specified Builder channel.
            let installed = util::pkg::satisfy_or_install(req, &source, &bldr_url, &bldr_channel)?;

            let mut specs = generate_new_specs_from_package(&installed, &opts)?;

            for spec in specs.iter_mut() {
                save_spec_for(&mgr.cfg, spec)?;
                req.info(format!(
                    "The {} service was successfully loaded",
                    spec.ident
                ))?;
            }

            // Only saves a composite spec if it's, well, a composite
            if let Ok(composite_spec) =
                CompositeSpec::from_package_install(source.as_ref(), &installed)
            {
                save_composite_spec_for(&mgr.cfg, &composite_spec)?;
                req.info(format!(
                    "The {} composite was successfully loaded",
                    composite_spec.ident()
                ))?;
            }
        }
        Some(spec) => {
            // We've seen this service / composite before. Thus `load`
            // basically acts as a way to edit spec files on the
            // command line. As a result, we a) check that you
            // *really* meant to change an existing spec, and b) DO
            // NOT download a potentially new version of the package
            // in question

            if !force {
                // TODO (CM): make this error reflect composites
                return Err(net::err(
                    ErrCode::Conflict,
                    format!("Service already loaded, unload '{}' and try again", ident),
                ));
            }

            match spec {
                Spec::Service(mut service_spec) => {
                    opts.into_spec(&mut service_spec);

                    // Only install if we don't have something
                    // locally; otherwise you could potentially
                    // upgrade each time you load.
                    //
                    // Also make sure you're pulling from where you're
                    // supposed to be pulling from!
                    util::pkg::satisfy_or_install(
                        req,
                        &source,
                        &service_spec.bldr_url,
                        &service_spec.channel,
                    )?;

                    save_spec_for(&mgr.cfg, &service_spec)?;
                    req.info(format!(
                        "The {} service was successfully loaded",
                        service_spec.ident
                    ))?;
                }
                Spec::Composite(composite_spec, mut existing_service_specs) => {
                    if source.as_ref() == composite_spec.ident() {
                        let mut bind_map =
                            match util::pkg::installed(composite_spec.package_ident()) {
                                Some(package) => package.bind_map()?,
                                // TODO (CM): this should be a proper error
                                None => unreachable!(),
                            };

                        for mut service_spec in existing_service_specs.iter_mut() {
                            opts.update_composite(&mut bind_map, &mut service_spec);
                            save_spec_for(&mgr.cfg, service_spec)?;
                            req.info(format!(
                                "The {} service was successfully loaded",
                                service_spec.ident
                            ))?;
                        }
                        req.info(format!(
                            "The {} composite was successfully loaded",
                            composite_spec.ident()
                        ))?;
                    } else {
                        // It changed!
                        // OK, here's the deal.
                        //
                        // We're going to install a new composite if
                        // we need to in order to satisfy the spec
                        // we've now got. That also means that the
                        // services that are currently running may get
                        // unloaded (because they are no longer in the
                        // composite), and new services may start
                        // (because they were added to the composite).

                        let installed_package = util::pkg::satisfy_or_install(
                            req,
                            &source,
                            // This (updating from the command-line
                            // args) is a difference from
                            // force-loading a spec, because
                            // composites don't auto-update themselves
                            // like services can.
                            &bldr_url,
                            &bldr_channel,
                        )?;

                        // Generate new specs from the new composite package and
                        // CLI inputs
                        let new_service_specs =
                            generate_new_specs_from_package(&installed_package, &opts)?;

                        // Delete any specs that are not in the new
                        // composite
                        let mut old_spec_names = HashSet::new();
                        for s in existing_service_specs.iter() {
                            old_spec_names.insert(s.ident.name.clone());
                        }
                        let mut new_spec_names = HashSet::new();
                        for s in new_service_specs.iter() {
                            new_spec_names.insert(s.ident.name.clone());
                        }

                        let specs_to_delete: HashSet<_> =
                            old_spec_names.difference(&new_spec_names).collect();
                        for spec in existing_service_specs.iter() {
                            if specs_to_delete.contains(&spec.ident.name) {
                                let file = spec_path_for(&mgr.cfg, spec);
                                req.info(format!("Unloading {:?}", file))?;
                                fs::remove_file(&file).map_err(|err| {
                                    sup_error!(Error::ServiceSpecFileIO(file, err))
                                })?;
                            }
                        }
                        // <-- end of deletion

                        // Save all the new specs. If there are
                        // services that exist in both composites,
                        // their service spec files will have the same
                        // name, so they'll be taken care of here (we
                        // don't need to treat them differently)
                        for spec in new_service_specs.iter() {
                            save_spec_for(&mgr.cfg, spec)?;
                            req.info(format!(
                                "The {} service was successfully loaded",
                                spec.ident
                            ))?;
                        }

                        // Generate and save the new spec
                        let new_composite_spec = CompositeSpec::from_package_install(
                            source.as_ref(),
                            &installed_package,
                        )?;
                        save_composite_spec_for(&mgr.cfg, &new_composite_spec)?;
                        req.info(format!(
                            "The {} composite was successfully loaded",
                            new_composite_spec.ident()
                        ))?;
                    }
                }
            }
        }
    }
    req.reply_complete(net::ok());
    Ok(())
}

pub fn service_unload(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcUnload,
) -> NetResult<()> {
    let ident: PackageIdent = opts.ident.ok_or(err_update_client())?.into();
    // Gather up the paths to all the spec files we care about,
    // along with their corresponding idents (we do this to ensure
    // we emit a proper "unloading X" message for each member of a
    // composite).
    //
    // This includes all service specs as well as any composite
    // spec.
    let path_ident_pairs = match existing_specs_for_ident(&mgr.cfg, &ident)? {
        Some(Spec::Service(spec)) => vec![(spec_path_for(&mgr.cfg, &spec), ident)],
        Some(Spec::Composite(composite_spec, specs)) => {
            let mut paths = Vec::with_capacity(specs.len() + 1);
            for spec in specs.iter() {
                paths.push((spec_path_for(&mgr.cfg, spec), spec.ident.clone()));
            }
            paths.push((composite_path_for(&mgr.cfg, &composite_spec), ident));
            paths
        }
        None => vec![],
    };

    for (file, ident) in path_ident_pairs {
        if let Err(err) = fs::remove_file(&file) {
            return Err(net::err(
                ErrCode::Internal,
                format!("{}", sup_error!(Error::ServiceSpecFileIO(file, err))),
            ));
        };
        // JW TODO: Change this to unloaded from unloading when the Supervisor waits for
        // the work to complete.
        req.info(format!("Unloading {}", ident))?;
    }
    req.reply_complete(net::ok());
    Ok(())
}

pub fn service_start(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcStart,
) -> NetResult<()> {
    let ident = opts.ident.ok_or(err_update_client())?.into();
    let updated_specs = match existing_specs_for_ident(&mgr.cfg, &ident)? {
        Some(Spec::Service(mut spec)) => {
            let mut updated_specs = vec![];
            if spec.desired_state == DesiredState::Down {
                spec.desired_state = DesiredState::Up;
                updated_specs.push(spec);
            }
            updated_specs
        }
        Some(Spec::Composite(_, service_specs)) => {
            let mut updated_specs = vec![];
            for mut spec in service_specs {
                if spec.desired_state == DesiredState::Down {
                    spec.desired_state = DesiredState::Up;
                    updated_specs.push(spec);
                }
            }
            updated_specs
        }
        None => {
            return Err(net::err(
                ErrCode::NotFound,
                format!("Service not loaded, {}", &ident),
            ));
        }
    };
    let specs_changed = updated_specs.len() > 0;
    for spec in updated_specs.iter() {
        save_spec_for(&mgr.cfg, spec)?;
    }
    if specs_changed {
        // JW TODO: Change the language of the message below to "started" when we actually
        // synchronously control services from the ctl gateway.
        req.info(format!(
            "Supervisor starting {}. See the Supervisor output for more details.",
            &ident
        ))?;
    }
    req.reply_complete(net::ok());
    Ok(())
}

pub fn service_stop(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcStop,
) -> NetResult<()> {
    let ident: PackageIdent = opts.ident.ok_or(err_update_client())?.into();
    let updated_specs = match existing_specs_for_ident(&mgr.cfg, &ident)? {
        Some(Spec::Service(mut spec)) => {
            let mut updated_specs = vec![];
            if spec.desired_state == DesiredState::Up {
                spec.desired_state = DesiredState::Down;
                updated_specs.push(spec);
            }
            updated_specs
        }
        Some(Spec::Composite(_, service_specs)) => {
            let mut updated_specs = vec![];
            for mut spec in service_specs {
                if spec.desired_state == DesiredState::Up {
                    spec.desired_state = DesiredState::Down;
                    updated_specs.push(spec);
                }
            }
            updated_specs
        }
        None => {
            return Err(net::err(
                ErrCode::NotFound,
                format!("Service not loaded, {}", &ident),
            ));
        }
    };
    let specs_changed = updated_specs.len() > 0;
    for spec in updated_specs.iter() {
        save_spec_for(&mgr.cfg, spec)?;
    }
    if specs_changed {
        // JW TODO: Change the langauge of the message below to "stopped" when we actually
        // synchronously control services from the ctl gateway.
        req.info(format!(
            "Supervisor stopping {}. See the Supervisor output for more details.",
            &ident
        ))?;
    }
    req.reply_complete(net::ok());
    Ok(())
}

pub fn supervisor_depart(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SupDepart,
) -> NetResult<()> {
    let member_id = opts.member_id.ok_or(err_update_client())?;
    let mut client = match butterfly::client::Client::new(
        mgr.cfg.gossip_listen.local_addr(),
        mgr.cfg.ring_key.clone(),
    ) {
        Ok(client) => client,
        Err(err) => {
            outputln!("Failed to connect to own gossip server, {}", err);
            return Err(net::err(ErrCode::Internal, err.to_string()));
        }
    };
    outputln!("Attempting to depart member: {}", member_id);
    match client.send_departure(member_id) {
        Ok(()) => {
            req.reply_complete(net::ok());
            Ok(())
        }
        Err(e) => Err(net::err(ErrCode::Internal, e.to_string())),
    }
}

pub fn service_status(
    mgr: &ManagerState,
    req: &mut CtlRequest,
    opts: protocol::ctl::SvcStatus,
) -> NetResult<()> {
    let services_data = &mgr
        .gateway_state
        .read()
        .expect("GatewayState lock is poisoned")
        .services_data;
    let statuses: Vec<ServiceStatus> = serde_json::from_str(&services_data)
        .map_err(|e| sup_error!(Error::ServiceDeserializationError(e)))?;

    if let Some(ident) = opts.ident {
        for status in statuses {
            if status.pkg.ident.satisfies(&ident) {
                let msg: protocol::types::ServiceStatus = status.into();
                req.reply_complete(msg);
                return Ok(());
            }
        }
        return Err(net::err(
            ErrCode::NotFound,
            format!("Service not loaded, {}", ident),
        ));
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

fn composites_path<T>(state_path: T) -> PathBuf
where
    T: AsRef<Path>,
{
    state_path.as_ref().join("composites")
}

fn err_update_client() -> net::NetErr {
    net::err(ErrCode::UpdateClient, "client out of date")
}

fn composite_path_for(cfg: &ManagerConfig, spec: &CompositeSpec) -> PathBuf {
    composites_path(cfg.sup_root()).join(spec.file_name())
}

fn save_composite_spec_for(cfg: &ManagerConfig, spec: &CompositeSpec) -> Result<()> {
    spec.to_file(composite_path_for(cfg, spec))
}

fn spec_path_for(cfg: &ManagerConfig, spec: &ServiceSpec) -> PathBuf {
    cfg.sup_root().join("specs").join(spec.file_name())
}

fn save_spec_for(cfg: &ManagerConfig, spec: &ServiceSpec) -> Result<()> {
    spec.to_file(spec_path_for(cfg, spec))
}

fn composite_path_by_ident(cfg: &ManagerConfig, ident: &PackageIdent) -> PathBuf {
    let mut p = composites_path(cfg.sup_root()).join(&ident.name);
    p.set_extension("spec");
    p
}

/// Given an installed package, generate a spec (or specs, in the case
/// of composite packages!) from it and the arguments passed in on the
/// command line.
fn generate_new_specs_from_package(
    package: &PackageInstall,
    opts: &protocol::ctl::SvcLoad,
) -> Result<Vec<ServiceSpec>> {
    let specs = match package.pkg_type()? {
        PackageType::Standalone => {
            let mut spec = ServiceSpec::default();
            opts.into_spec(&mut spec);
            vec![spec]
        }
        PackageType::Composite => opts.into_composite_spec(
            package.ident().name.clone(),
            package.pkg_services()?,
            package.bind_map()?,
        ),
    };
    Ok(specs)
}

/// Given a `PackageIdent`, return current specs if they exist. If
/// the package is a standalone service, only that spec will be
/// returned, but if it is a composite, the composite spec as well as
/// the specs for all the services in the composite will be returned.
fn existing_specs_for_ident(cfg: &ManagerConfig, ident: &PackageIdent) -> Result<Option<Spec>> {
    let default_spec = ServiceSpec::default_for(ident.clone());
    let spec_file = spec_path_for(cfg, &default_spec);

    // Try it as a service first
    if let Ok(spec) = ServiceSpec::from_file(&spec_file) {
        Ok(Some(Spec::Service(spec)))
    } else {
        // Try it as a composite next
        let composite_spec_file = composite_path_by_ident(&cfg, ident);
        match CompositeSpec::from_file(composite_spec_file) {
            Ok(composite_spec) => {
                let fs_root_path = Path::new(&*FS_ROOT_PATH);
                let package =
                    PackageInstall::load(composite_spec.package_ident(), Some(fs_root_path))?;
                let mut specs = vec![];

                let services = package.pkg_services()?;
                for service in services {
                    let spec = ServiceSpec::from_file(spec_path_for(
                        cfg,
                        &ServiceSpec::default_for(service),
                    ))?;
                    specs.push(spec);
                }

                Ok(Some(Spec::Composite(composite_spec, specs)))
            }
            // Looks like we have no specs for this thing at all
            Err(_) => Ok(None),
        }
    }
}

#[derive(Deserialize)]
struct ServiceStatus {
    pkg: Pkg,
    process: ProcessStatus,
    service_group: ServiceGroup,
    composite: Option<String>,
    desired_state: DesiredState,
}

impl fmt::Display for ServiceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}), {}, group:{}",
            self.pkg.ident,
            self.composite.as_ref().unwrap_or(&"standalone".to_string()),
            self.process,
            self.service_group,
        )
    }
}

impl From<ServiceStatus> for protocol::types::ServiceStatus {
    fn from(other: ServiceStatus) -> Self {
        let mut proto = protocol::types::ServiceStatus::default();
        proto.ident = other.pkg.ident.into();
        proto.process = Some(other.process.into());
        proto.service_group = other.service_group.into();
        if let Some(composite) = other.composite {
            proto.composite = Some(composite);
        }
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
            Some(pid) => write!(
                f,
                "state:{}, time:{}, pid:{}",
                self.state, self.elapsed, pid
            ),
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
where
    D: serde::Deserializer<'de>,
{
    struct FromTimespec;

    impl<'de> serde::de::Visitor<'de> for FromTimespec {
        type Value = TimeDuration;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("a i64 integer")
        }

        fn visit_u64<R>(self, value: u64) -> result::Result<TimeDuration, R>
        where
            R: serde::de::Error,
        {
            let tspec = Timespec {
                sec: (value as i64),
                nsec: 0,
            };
            Ok(time::get_time() - tspec)
        }
    }

    d.deserialize_u64(FromTimespec)
}
