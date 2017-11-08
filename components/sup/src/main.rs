// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

extern crate habitat_common as common;
#[macro_use]
extern crate habitat_core as hcore;
extern crate habitat_launcher_client as launcher_client;
#[macro_use]
extern crate habitat_sup as sup;
extern crate log;
extern crate env_logger;
extern crate ansi_term;
extern crate libc;
extern crate clap;
extern crate time;
extern crate url;

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::io::{self, Write};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::process;
use std::str::FromStr;

use ansi_term::Colour::{Red, Yellow};
use clap::{App, ArgMatches};
use common::command::package::install::InstallSource;
use common::ui::UI;
use hcore::channel;
use hcore::crypto::{self, default_cache_key_path, SymKey};
#[cfg(windows)]
use hcore::crypto::dpapi::encrypt;
use hcore::env as henv;
use hcore::fs;
use hcore::package::PackageIdent;
use hcore::package::install::PackageInstall;
use hcore::package::metadata::{BindMapping, PackageType};
use hcore::service::{ApplicationEnvironment, ServiceGroup};
use hcore::url::{bldr_url_from_env, default_bldr_url};
use launcher_client::{LauncherCli, ERR_NO_RETRY_EXCODE, OK_NO_RETRY_EXCODE};

use sup::config::{GossipListenAddr, GOSSIP_DEFAULT_PORT};
use sup::error::{Error, Result, SupError};
use sup::feat;
use sup::command;
use sup::http_gateway;
use sup::manager::{Manager, ManagerConfig};
use sup::manager::service::{DesiredState, ServiceBind, Topology, UpdateStrategy};
use sup::manager::service::{CompositeSpec, ServiceSpec, StartStyle};
use sup::util;

/// Our output key
static LOGKEY: &'static str = "MN";

static RING_ENVVAR: &'static str = "HAB_RING";
static RING_KEY_ENVVAR: &'static str = "HAB_RING_KEY";

fn main() {
    if let Err(err) = start() {
        println!("{}", err);
        match err {
            SupError { err: Error::ProcessLocked(_), .. } => process::exit(ERR_NO_RETRY_EXCODE),
            SupError { err: Error::Departed, .. } => {
                process::exit(ERR_NO_RETRY_EXCODE);
            }
            _ => process::exit(1),
        }
    }
}

fn boot() -> Option<LauncherCli> {
    env_logger::init().unwrap();
    enable_features_from_env();
    if !crypto::init() {
        println!("Crypto initialization failed!");
        process::exit(1);
    }
    match launcher_client::env_pipe() {
        Some(pipe) => {
            match LauncherCli::connect(pipe) {
                Ok(launcher) => Some(launcher),
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }
        None => None,
    }
}

fn start() -> Result<()> {
    let launcher = boot();
    let app_matches = match cli().get_matches_safe() {
        Ok(matches) => matches,
        Err(err) => {
            let out = io::stdout();
            writeln!(&mut out.lock(), "{}", err.message).expect("Error writing Error to stdout");
            process::exit(ERR_NO_RETRY_EXCODE);
        }
    };
    match app_matches.subcommand() {
        ("bash", Some(m)) => sub_bash(m),
        ("config", Some(m)) => sub_config(m),
        ("load", Some(m)) => sub_load(m),
        ("run", Some(m)) => {
            let launcher = launcher.ok_or(sup_error!(Error::NoLauncher))?;
            sub_run(m, launcher)
        }
        ("sh", Some(m)) => sub_sh(m),
        ("start", Some(m)) => {
            let launcher = launcher.ok_or(sup_error!(Error::NoLauncher))?;
            sub_start(m, launcher)
        }
        ("status", Some(m)) => sub_status(m),
        ("stop", Some(m)) => sub_stop(m),
        ("term", Some(m)) => sub_term(m),
        ("unload", Some(m)) => sub_unload(m),
        _ => unreachable!(),
    }
}

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
fn cli<'a, 'b>() -> App<'a, 'b> {
    sup::cli::get("hab-sup")
}

fn sub_bash(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }

    command::shell::bash()
}

fn sub_config(m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    common::command::package::config::start(&ident, "/")?;
    Ok(())
}

fn sub_load(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;
    let install_source = install_source_from_input(m)?;

    // TODO (CM): should load be able to download new artifacts if
    // you're re-loading with --force?
    // If we've already got a spec for this thing, we don't want to
    // inadvertently download a new version

    match existing_specs_for_ident(&cfg, install_source.as_ref().clone())? {
        None => {
            // We don't have any record of this thing; let's set it
            // up!
            //
            // This will install the latest version from Builder
            let installed = util::pkg::install(
                &mut UI::default(),
                &bldr_url(m),
                &install_source,
                &channel(m),
            )?;

            let original_ident = install_source.as_ref();
            let mut specs = generate_new_specs_from_package(original_ident, &installed, m)?;

            for spec in specs.iter_mut() {
                // "load" == persistent services, by definition
                spec.start_style = StartStyle::Persistent;
                Manager::save_spec_for(&cfg, spec)?;
                outputln!("The {} service was successfully loaded", spec.ident);
            }

            // Only saves a composite spec if it's, well, a composite
            if let Ok(composite_spec) =
                CompositeSpec::from_package_install(&original_ident, &installed)
            {
                Manager::save_composite_spec_for(&cfg, &composite_spec)?;
                outputln!(
                    "The {} composite was successfully loaded",
                    composite_spec.ident()
                );
            }
            Ok(())
        }
        Some(spec) => {
            // We've seen this service / composite before. Thus `load`
            // basically acts as a way to edit spec files on the
            // command line. As a result, we a) check that you
            // *really* meant to change an existing spec, and b) DO
            // NOT download a potentially new version of the package
            // in question

            if !m.is_present("FORCE") {
                // TODO (CM): make this error reflect composites
                return Err(sup_error!(Error::ServiceLoaded(spec.ident().clone())));
            }

            match spec {
                Spec::Service(mut service_spec) => {
                    service_spec.ident = install_source.as_ref().clone();
                    update_spec_from_input(&mut service_spec, m)?;
                    service_spec.start_style = StartStyle::Persistent;

                    // Only install if we don't have something
                    // locally; otherwise you could potentially
                    // upgrade each time you load.
                    //
                    // Also make sure you're pulling from where you're
                    // supposed to be pulling from!
                    install_package_if_not_present(
                        &install_source,
                        &service_spec.bldr_url,
                        &service_spec.channel,
                    )?;

                    Manager::save_spec_for(&cfg, &service_spec)?;
                    outputln!("The {} service was successfully loaded", service_spec.ident);
                    Ok(())
                }
                Spec::Composite(composite_spec, mut existing_service_specs) => {
                    if install_source.as_ref() == composite_spec.ident() {
                        let composite_package =
                            match util::pkg::installed(composite_spec.package_ident()) {
                                Some(package) => package,
                                // TODO (CM): this should be a proper error
                                None => unreachable!(), 
                            };

                        update_composite_service_specs(
                            &mut existing_service_specs,
                            &composite_package,
                            m,
                        )?;

                        for service_spec in existing_service_specs.iter() {
                            Manager::save_spec_for(&cfg, service_spec)?;
                            outputln!("The {} service was successfully loaded", service_spec.ident);
                        }
                        outputln!(
                            "The {} composite was successfully loaded",
                            composite_spec.ident()
                        );
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

                        let installed_package = install_package_if_not_present(
                            &install_source,
                            // This (updating from the command-line
                            // args) is a difference from
                            // force-loading a spec, because
                            // composites don't auto-update themselves
                            // like services can.
                            &bldr_url(m),
                            &channel(m),
                        )?;

                        // Generate new specs from the new composite package and
                        // CLI inputs
                        let new_service_specs = generate_new_specs_from_package(
                            install_source.as_ref(),
                            &installed_package,
                            m,
                        )?;

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
                                let file = Manager::spec_path_for(&cfg, spec);
                                outputln!("Unloading {:?}", file);
                                std::fs::remove_file(&file).map_err(|err| {
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
                            Manager::save_spec_for(&cfg, spec)?;
                            outputln!("The {} service was successfully loaded", spec.ident);
                        }

                        // Generate and save the new spec
                        let new_composite_spec = CompositeSpec::from_package_install(
                            install_source.as_ref(),
                            &installed_package,
                        )?;
                        Manager::save_composite_spec_for(&cfg, &new_composite_spec)?;
                        outputln!(
                            "The {} composite was successfully loaded",
                            new_composite_spec.ident()
                        );
                    }
                    Ok(())
                }
            }
        }
    }
}

fn sub_unload(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }

    let cfg = mgrcfg_from_matches(m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    // Gather up the paths to all the spec files we care about. This
    // includes all service specs as well as any composite spec.
    let spec_paths = match existing_specs_for_ident(&cfg, ident)? {
        Some(Spec::Service(spec)) => vec![Manager::spec_path_for(&cfg, &spec)],
        Some(Spec::Composite(composite_spec, specs)) => {
            let mut paths = Vec::with_capacity(specs.len() + 1);
            for spec in specs.iter() {
                paths.push(Manager::spec_path_for(&cfg, spec));
            }
            paths.push(Manager::composite_path_for(&cfg, &composite_spec));
            paths
        }
        None => vec![],
    };

    for file in spec_paths {
        outputln!("Unloading {:?}", file);
        std::fs::remove_file(&file).map_err(|err| {
            sup_error!(Error::ServiceSpecFileIO(file, err))
        })?;
    }

    Ok(())
}

fn sub_run(m: &ArgMatches, launcher: LauncherCli) -> Result<()> {
    let cfg = mgrcfg_from_matches(m)?;
    let mut manager = Manager::load(cfg, launcher)?;
    manager.run()
}

fn sub_sh(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }

    command::shell::sh()
}

fn sub_start(m: &ArgMatches, launcher: LauncherCli) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }

    let cfg = mgrcfg_from_matches(m)?;

    if !fs::am_i_root() {
        let mut ui = UI::default();
        ui.warn(
            "Running the Habitat Supervisor with root or superuser privileges is recommended",
        )?;
        ui.br()?;
    }

    let install_source = install_source_from_input(m)?;
    let original_ident: &PackageIdent = install_source.as_ref();

    // NOTE: As coded, if you try to start a service from a hart file,
    // but you already have a spec for that service (regardless of
    // version), you're not going to ever install your hart file, and
    // the spec isn't going to be updated to point to that exact
    // version.

    let updated_specs = match existing_specs_for_ident(&cfg, original_ident.clone())? {
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
            // We don't have any specs for this thing yet, so we'll
            // need to make some. If we don't already have installed
            // software that satisfies the given identifier, then
            // we'll install the latest thing that will
            // suffice. Otherwise, we'll just use what we find in the
            // local cache of software.
            let installed_package =
                install_package_if_not_present(&install_source, &bldr_url(m), &channel(m))?;
            let new_specs =
                generate_new_specs_from_package(&original_ident, &installed_package, m)?;

            // Saving the composite spec here, because we currently
            // need the PackageInstall to create it! It'll only create
            // a composite spec if the package is itself a composite.
            if let Ok(composite_spec) =
                CompositeSpec::from_package_install(&original_ident, &installed_package)
            {
                Manager::save_composite_spec_for(&cfg, &composite_spec)?;
            }

            new_specs
        }
    };

    let specs_changed = updated_specs.len() > 0;

    for spec in updated_specs.iter() {
        Manager::save_spec_for(&cfg, spec)?;
    }

    if Manager::is_running(&cfg)? {
        if specs_changed {
            outputln!(
                "Supervisor starting {}. See the Supervisor output for more details.",
                original_ident
            );
            Ok(())
        } else {
            // TODO (CM): somehow, this doesn't actually seem to be
            // exiting with a non-zero exit code
            process::exit(OK_NO_RETRY_EXCODE);
        }
    } else {
        let mut manager = Manager::load(cfg, launcher)?;
        manager.run()
    }
}

fn sub_status(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;
    if !Manager::is_running(&cfg)? {
        println!("The Supervisor is not running.");
        process::exit(3);
    }

    // Note that PKG_IDENT is NOT required here
    match m.value_of("PKG_IDENT") {
        Some(pkg) => {
            let ident = PackageIdent::from_str(pkg)?;
            let specs = match existing_specs_for_ident(&cfg, ident)? {
                Some(Spec::Service(spec)) => vec![spec],
                Some(Spec::Composite(_, specs)) => specs,
                None => {
                    println!("{} is not currently loaded.", pkg);
                    process::exit(2);
                }
            };

            for spec in specs {
                let status = Manager::service_status(&cfg, &spec.ident)?;
                outputln!("{}", status);
            }
        }
        None => {
            let statuses = Manager::status(&cfg)?;
            if statuses.is_empty() {
                println!("No services loaded.");
                return Ok(());
            }
            for status in statuses {
                println!("{}", status);
            }
        }
    }
    Ok(())
}

fn sub_stop(m: &ArgMatches) -> Result<()> {
    if m.is_present("VERBOSE") {
        hcore::output::set_verbose(true);
    }
    if m.is_present("NO_COLOR") {
        hcore::output::set_no_color(true);
    }
    let cfg = mgrcfg_from_matches(m)?;

    // PKG_IDENT is required, so unwrap() is safe
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let mut specs = match existing_specs_for_ident(&cfg, ident)? {
        Some(Spec::Service(spec)) => vec![spec],
        Some(Spec::Composite(_, specs)) => specs,
        None => vec![],
    };

    for spec in specs.iter_mut() {
        spec.desired_state = DesiredState::Down;
        Manager::save_spec_for(&cfg, &spec)?;
    }

    Ok(())
}

fn sub_term(m: &ArgMatches) -> Result<()> {
    let cfg = mgrcfg_from_matches(m)?;
    match Manager::term(&cfg) {
        Err(SupError { err: Error::ProcessLockIO(_, _), .. }) => {
            println!("Supervisor not started.");
            Ok(())
        }
        result => result,
    }
}

// Internal Implementation Details
////////////////////////////////////////////////////////////////////////

/// Helper enum to abstract over spec type.
///
/// Currently needed only here. Don't bother moving anywhere because
/// ServiceSpecs AND CompositeSpecs will be going away soon anyway.
enum Spec {
    Service(ServiceSpec),
    Composite(CompositeSpec, Vec<ServiceSpec>),
}

impl Spec {
    /// We need to get at the identifier of a spec, regardless of
    /// which kind it is.
    fn ident(&self) -> &PackageIdent {
        match self {
            &Spec::Composite(ref s, _) => s.ident(),
            &Spec::Service(ref s) => s.ident.as_ref(),
        }
    }
}

/// Given a `PackageIdent`, return current specs if they exist. If
/// the package is a standalone service, only that spec will be
/// returned, but if it is a composite, the composite spec as well as
/// the specs for all the services in the composite will be returned.
fn existing_specs_for_ident(cfg: &ManagerConfig, ident: PackageIdent) -> Result<Option<Spec>> {
    let default_spec = ServiceSpec::default_for(ident.clone());
    let spec_file = Manager::spec_path_for(cfg, &default_spec);

    // Try it as a service first
    if let Ok(spec) = ServiceSpec::from_file(&spec_file) {
        Ok(Some(Spec::Service(spec)))
    } else {
        // Try it as a composite next
        let composite_spec_file = Manager::composite_path_by_ident(&cfg, &ident);
        match CompositeSpec::from_file(composite_spec_file) {
            Ok(composite_spec) => {
                let fs_root_path = Path::new(&*fs::FS_ROOT_PATH);
                let package =
                    PackageInstall::load(composite_spec.package_ident(), Some(fs_root_path))?;
                let mut specs = vec![];

                let services = package.pkg_services()?;
                for service in services {
                    let spec = ServiceSpec::from_file(Manager::spec_path_for(
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

fn mgrcfg_from_matches(m: &ArgMatches) -> Result<ManagerConfig> {
    let mut cfg = ManagerConfig::default();

    cfg.auto_update = m.is_present("AUTO_UPDATE");
    cfg.update_url = bldr_url(m);
    cfg.update_channel = channel(m);
    if let Some(addr_str) = m.value_of("LISTEN_GOSSIP") {
        cfg.gossip_listen = GossipListenAddr::from_str(addr_str)?;
    }
    if let Some(addr_str) = m.value_of("LISTEN_HTTP") {
        cfg.http_listen = http_gateway::ListenAddr::from_str(addr_str)?;
    }
    if let Some(name_str) = m.value_of("NAME") {
        cfg.name = Some(String::from(name_str));
        outputln!("");
        outputln!(
            "{} Running more than one Habitat Supervisor is not recommended for most",
            Red.bold().paint("CAUTION:".to_string())
        );
        outputln!(
            "{} users in most use cases. Using one Supervisor per host for multiple",
            Red.bold().paint("CAUTION:".to_string())
        );
        outputln!(
            "{} services in one ring will yield much better performance.",
            Red.bold().paint("CAUTION:".to_string())
        );
        outputln!("");
        outputln!(
            "{} If you know what you're doing, carry on!",
            Red.bold().paint("CAUTION:".to_string())
        );
        outputln!("");
    }
    cfg.organization = m.value_of("ORGANIZATION").map(|org| org.to_string());
    cfg.gossip_permanent = m.is_present("PERMANENT_PEER");
    // TODO fn: Clean this up--using a for loop doesn't feel good however an iterator was
    // causing a lot of developer/compiler type confusion
    let mut gossip_peers: Vec<SocketAddr> = Vec::new();
    if let Some(peers) = m.values_of("PEER") {
        for peer in peers {
            let peer_addr = if peer.find(':').is_some() {
                peer.to_string()
            } else {
                format!("{}:{}", peer, GOSSIP_DEFAULT_PORT)
            };
            let addrs: Vec<SocketAddr> = match peer_addr.to_socket_addrs() {
                Ok(addrs) => addrs.collect(),
                Err(e) => {
                    outputln!("Failed to resolve peer: {}", peer_addr);
                    return Err(sup_error!(Error::NameLookup(e)));
                }
            };
            let addr: SocketAddr = addrs[0];
            gossip_peers.push(addr);
        }
    }
    cfg.gossip_peers = gossip_peers;
    if let Some(watch_peer_file) = m.value_of("PEER_WATCH_FILE") {
        cfg.watch_peer_file = Some(String::from(watch_peer_file));
    }
    let ring = match m.value_of("RING") {
        Some(val) => Some(SymKey::get_latest_pair_for(
            &val,
            &default_cache_key_path(None),
        )?),
        None => {
            match henv::var(RING_KEY_ENVVAR) {
                Ok(val) => {
                    let (key, _) =
                        SymKey::write_file_from_str(&val, &default_cache_key_path(None))?;
                    Some(key)
                }
                Err(_) => {
                    match henv::var(RING_ENVVAR) {
                        Ok(val) => {
                            Some(SymKey::get_latest_pair_for(
                                &val,
                                &default_cache_key_path(None),
                            )?)
                        }
                        Err(_) => None,
                    }
                }
            }
        }
    };
    if let Some(ring) = ring {
        cfg.ring = Some(ring.name_with_rev());
    }
    if let Some(events) = m.value_of("EVENTS") {
        cfg.eventsrv_group = ServiceGroup::from_str(events).ok();
    }
    Ok(cfg)
}

// Various CLI Parsing Functions
////////////////////////////////////////////////////////////////////////

/// Resolve a Builder URL. Taken from CLI args, the environment, or
/// (failing those) a default value.
fn bldr_url(m: &ArgMatches) -> String {
    match bldr_url_from_input(m) {
        Some(url) => url.to_string(),
        None => default_bldr_url(),
    }
}

/// A Builder URL, but *only* if the user specified it via CLI args or
/// the environment
fn bldr_url_from_input(m: &ArgMatches) -> Option<String> {
    m.value_of("BLDR_URL")
        .and_then(|u| Some(u.to_string()))
        .or_else(|| bldr_url_from_env())
}

/// Resolve a channel. Taken from CLI args, or (failing that), a
/// default value.
fn channel(matches: &ArgMatches) -> String {
    channel_from_input(matches).unwrap_or(channel::default())
}

/// A channel name, but *only* if the user specified via CLI args.
fn channel_from_input(m: &ArgMatches) -> Option<String> {
    m.value_of("CHANNEL").and_then(|c| Some(c.to_string()))
}

fn install_source_from_input(m: &ArgMatches) -> Result<InstallSource> {
    // PKG_IDENT_OR_ARTIFACT is required in subcommands that use it,
    // so unwrap() is safe here.
    let ident_or_artifact = m.value_of("PKG_IDENT_OR_ARTIFACT").unwrap();
    let install_source: InstallSource = ident_or_artifact.parse()?;
    Ok(install_source)
}

// ServiceSpec Modification Functions
////////////////////////////////////////////////////////////////////////

/// If the user supplied a --group option, set it on the
/// spec. Otherwise, we inherit the default value in the ServiceSpec,
/// which is "default".
fn set_group_from_input(spec: &mut ServiceSpec, m: &ArgMatches) {
    if let Some(g) = m.value_of("GROUP") {
        spec.group = g.to_string();
    }
}

/// If the user provides both --application and --environment options,
/// parse and set the value on the spec. Otherwise, we inherit the
/// default value of the ServiceSpec, which is None
fn set_app_env_from_input(spec: &mut ServiceSpec, m: &ArgMatches) -> Result<()> {
    if let (Some(app), Some(env)) = (m.value_of("APPLICATION"), m.value_of("ENVIRONMENT")) {
        spec.application_environment = Some(ApplicationEnvironment::new(
            app.to_string(),
            env.to_string(),
        )?);
    }
    Ok(())
}

/// Set a spec's Builder URL from CLI / environment variables, falling back
/// to a default value.
fn set_bldr_url(spec: &mut ServiceSpec, m: &ArgMatches) {
    spec.bldr_url = bldr_url(m);
}

/// Set a Builder URL only if specified by the user as a CLI argument
/// or an environment variable.
fn set_bldr_url_from_input(spec: &mut ServiceSpec, m: &ArgMatches) {
    if let Some(url) = bldr_url_from_input(m) {
        spec.bldr_url = url
    }
}

/// Set a channel only if specified by the user as a CLI argument.
fn set_channel_from_input(spec: &mut ServiceSpec, m: &ArgMatches) {
    if let Some(channel) = channel_from_input(m) {
        spec.channel = channel
    }
}

/// Set a spec's channel from CLI values, falling back
/// to a default value.
fn set_channel(spec: &mut ServiceSpec, m: &ArgMatches) {
    spec.channel = channel(m);
}

/// Set a topology value only if specified by the user as a CLI
/// argument.
fn set_topology_from_input(spec: &mut ServiceSpec, m: &ArgMatches) {
    if let Some(t) = m.value_of("TOPOLOGY") {
        // unwrap() is safe, because the input is validated by
        // `valid_topology`
        spec.topology = Topology::from_str(t).unwrap();
    }
}

/// Set an update strategy only if specified by the user as a CLI
/// argument.
fn set_strategy_from_input(spec: &mut ServiceSpec, m: &ArgMatches) {
    if let Some(s) = m.value_of("STRATEGY") {
        // unwrap() is safe, because the input is validated by `valid_update_strategy`
        spec.update_strategy = UpdateStrategy::from_str(s).unwrap();
    }
}

/// Set bind values if given on the command line.
///
/// NOTE: At the moment, binds for composite services should NOT be
/// set using this, as we do not have a mechanism to distinguish
/// between the different services within the composite.
fn set_binds_from_input(spec: &mut ServiceSpec, m: &ArgMatches) -> Result<()> {
    if let Some(bind_strs) = m.values_of("BIND") {
        let mut binds = Vec::new();
        for bind_str in bind_strs {
            binds.push(ServiceBind::from_str(bind_str)?);
        }
        spec.binds = binds;
    }
    Ok(())
}

/// When loading a composite, the services within it may require
/// additional binds that cannot be satisfied by the other services
/// within the composite.
///
/// In this case, we modify the existing bind syntax to allow a user
/// to specify which service within the composite is to receive the
/// bind (when you're loading a single service, this is understood to
/// the be that exact service).
///
/// This alternative syntax is "service_name:bind_name:group"
///
/// Since the CLI option may contain multiple values, and since they
/// could each be for different services within the composite, we
/// construct a map of service name to a vector of ServiceBinds and
/// return that for subsequent reconciliation with the binds from the
/// composite.
// TODO (CM): consider making a new type for this return value
// TODO (CM): Consolidate this with non-composite bind processing;
// don't want composite binds showing up in non-composite services and vice-versa
fn composite_binds_from_input(m: &ArgMatches) -> Result<HashMap<String, Vec<ServiceBind>>> {
    let mut map = HashMap::new();

    if let Some(bind_strs) = m.values_of("BIND") {
        for bind_str in bind_strs {
            let parts: Vec<&str> = bind_str.splitn(3, ':').collect();
            if parts.len() == 3 {
                // It's a composite bind
                let service_name = parts[0];
                let bind = format!("{}:{}", parts[1], parts[2]);
                let mut binds = map.entry(service_name.to_string()).or_insert(vec![]);
                binds.push(ServiceBind::from_str(&bind)?);
            } else {
                // You supplied a 2-part (i.e., standalone service)
                // bind when trying to set up a composite!
                return Err(sup_error!(
                    Error::InvalidCompositeBinding(bind_str.to_string())
                ));
            }
        }
    }

    Ok(map)
}

/// Set a custom config directory if given on the command line.
///
/// NOTE: At the moment, this should not be used for composite
/// services, as we do not have a mechanism to distinguish between the
/// different services within the composite.
fn set_config_from_input(spec: &mut ServiceSpec, m: &ArgMatches) -> Result<()> {
    if let Some(ref config_from) = m.value_of("CONFIG_DIR") {
        spec.config_from = Some(PathBuf::from(config_from));
        outputln!("");
        outputln!(
            "{} Setting '{}' should only be used in development, not production!",
            Red.bold().paint("WARNING:".to_string()),
            Yellow.bold().paint(
                format!("--config-from {}", config_from),
            )
        );
        outputln!("");
    }
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_password_from_input(spec: &mut ServiceSpec, m: &ArgMatches) -> Result<()> {
    if let Some(password) = m.value_of("PASSWORD") {
        spec.svc_encrypted_password = Some(encrypt(password.to_string())?);
    }
    Ok(())
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn set_password_from_input(_: &mut ServiceSpec, _: &ArgMatches) -> Result<()> {
    Ok(())
}

// ServiceSpec Generation Functions
////////////////////////////////////////////////////////////////////////
//
// While ServiceSpec has an implementation of the Default trait, we
// want to be sure that specs created in this module unquestionably
// conform to the defaults that our CLI lays out.
//
// Similarly, when ever we update existing specs (e.g., hab svc load
// --force) we must take care that we only change values that the user
// has given explicitly, and not override using default values.
//
// To that end, we have a function that create a "default ServiceSpec"
// as far as this module is concerned, which is to be used when
// creating *new* specs, and another that merges an *existing* spec
// with only command-line arguments.
//
////////////////////////////////////////////////////////////////////////

// Only use this for standalone services!
fn new_service_spec(ident: PackageIdent, m: &ArgMatches) -> Result<ServiceSpec> {
    let mut spec = ServiceSpec::default_for(ident);

    set_bldr_url(&mut spec, m);
    set_channel(&mut spec, m);

    set_app_env_from_input(&mut spec, m)?;
    set_group_from_input(&mut spec, m);
    set_strategy_from_input(&mut spec, m);
    set_topology_from_input(&mut spec, m);
    set_binds_from_input(&mut spec, m)?;
    set_config_from_input(&mut spec, m)?;
    set_password_from_input(&mut spec, m)?;
    Ok(spec)
}

fn update_spec_from_input(mut spec: &mut ServiceSpec, m: &ArgMatches) -> Result<()> {
    // The Builder URL and channel have default values; we only want to
    // change them if the user specified something!
    set_bldr_url_from_input(&mut spec, m);
    set_channel_from_input(&mut spec, m);

    set_app_env_from_input(&mut spec, m)?;
    set_group_from_input(&mut spec, m);
    set_strategy_from_input(&mut spec, m);
    set_topology_from_input(&mut spec, m);

    // TODO (CM): Remove these for composite-member specs
    set_binds_from_input(&mut spec, m)?;
    set_config_from_input(&mut spec, m)?;
    set_password_from_input(&mut spec, m)?;

    Ok(())
}

/// All specs in a composite currently share a lot of the same
/// information. Here, we create a "base spec" that we can clone and
/// further customize for each individual service as needed.
fn base_composite_service_spec(composite_name: &str, m: &ArgMatches) -> Result<ServiceSpec> {
    let mut spec = ServiceSpec::default();

    // All the composite's services are in the same composite,
    // tautologically enough!
    spec.composite = Some(composite_name.to_string());

    // All services will pull from the same channel in the same
    // Builder instance
    set_bldr_url(&mut spec, m);
    set_channel(&mut spec, m);

    // All services will be in the same group and app/env. Binds among
    // the composite's services are generated based on this
    // assumption.
    //
    // (We do not set binds here, though, because that requires
    // specialized, service-specific handling.)
    set_app_env_from_input(&mut spec, m)?;
    set_group_from_input(&mut spec, m);

    // For now, all a composite's services will also share the same
    // update strategy and topology, though we may want to revisit
    // this in the future (particularly for topology).
    set_strategy_from_input(&mut spec, m);
    set_topology_from_input(&mut spec, m);

    // TODO (CM): Not dealing with service passwords for now, since
    // that's a Windows-only feature, and we don't currently build
    // Windows composites yet. And we don't have a nice way target
    // them on a per-service basis.

    // TODO (CM): Not setting the dev-mode service config_from value
    // because we don't currently have a nice way to target them on a
    // per-service basis.

    Ok(spec)
}

/// Generate the binds for a composite's service, taking into account
/// both the values laid out in composite definition and any CLI value
/// the user may have specified. This allows the user to override a
/// composite-defined bind, but also (perhaps more usefully) to
/// declare binds for services within the composite that are not
/// themselves *satisfied* by other members of the composite.
///
/// The final list of bind mappings is generated and then set in the
/// `ServiceSpec`. Any binds that may have been present in the spec
/// before are completely ignored.
///
/// # Parameters
///
/// * bind_map: output of package.bind_map()
/// * cli_binds: per-service overrides given on the CLI
fn set_composite_binds(
    spec: &mut ServiceSpec,
    bind_map: &HashMap<PackageIdent, Vec<BindMapping>>,
    cli_binds: &mut HashMap<String, Vec<ServiceBind>>,
) -> Result<()> {

    // We'll be layering bind specifications from the composite
    // with any additional ones from the CLI. We'll store them here,
    // keyed to the bind name
    let mut final_binds: HashMap<String, ServiceBind> = HashMap::new();

    // First, generate the binds from the composite
    if let Some(bind_mappings) = bind_map.get(&spec.ident) {
        // Turn each BindMapping into a ServiceBind

        // NOTE: We are explicitly NOT generating binds that include
        // "organization". This is a feature that never quite found
        // its footing, and will likely be removed / greatly
        // overhauled Real Soon Now (TM) (as of September 2017).
        //
        // As it exists right now, "organization" is a supervisor-wide
        // setting, and thus is only available for `hab sup run` and
        // `hab svc start`. We don't have a way from `hab svc load` to
        // access the organization setting of an active supervisor,
        // and so we can't generate binds that include organizations.
        for bind_mapping in bind_mappings.iter() {
            let group = ServiceGroup::new(
                spec.application_environment.as_ref(),
                &bind_mapping.satisfying_service.name,
                &spec.group,
                None, // <-- organization
            )?;
            let bind = ServiceBind {
                name: bind_mapping.bind_name.clone(),
                service_group: group,
            };
            final_binds.insert(bind.name.clone(), bind);
        }
    }

    // If anything was overridden or added on the CLI, layer that on
    // now as well. These will take precedence over anything in the
    // composite itself.
    //
    // Note that it consumes the values from cli_binds
    if let Entry::Occupied(b) = cli_binds.entry(spec.ident.name.clone()) {
        let binds = b.remove();
        for bind in binds {
            final_binds.insert(bind.name.clone(), bind);
        }
    }

    // Now take all the ServiceBinds we've collected.
    spec.binds = final_binds.drain().map(|(_, v)| v).collect();
    Ok(())
}

fn enable_features_from_env() {
    let features = vec![(feat::List, "LIST")];

    for feature in &features {
        match henv::var(format!("HAB_FEAT_{}", feature.1)) {
            Ok(ref val) if ["true", "TRUE"].contains(&val.as_str()) => {
                feat::enable(feature.0);
                outputln!("Enabling feature: {:?}", feature.0);
            }
            _ => {}
        }
    }

    if feat::is_enabled(feat::List) {
        outputln!("Listing feature flags environment variables:");
        for feature in &features {
            outputln!("     * {:?}: HAB_FEAT_{}=true", feature.0, feature.1);
        }
        outputln!("The Supervisor will start now, enjoy!");
    }
}

/// Given an InstallSource, install a new package only if an existing
/// one that can satisfy the package identifier is not already
/// present.
///
/// Return the PackageInstall corresponding to the package that was
/// installed, or was pre-existing.
fn install_package_if_not_present(
    install_source: &InstallSource,
    bldr_url: &str,
    channel: &str,
) -> Result<PackageInstall> {
    match util::pkg::installed(install_source.as_ref()) {
        Some(package) => Ok(package),
        None => {
            outputln!("Missing package for {}", install_source.as_ref());
            util::pkg::install(&mut UI::default(), bldr_url, install_source, channel)
        }
    }
}

/// Given an installed package, generate a spec (or specs, in the case
/// of composite packages!) from it and the arguments passed in on the
/// command line.
fn generate_new_specs_from_package(
    original_ident: &PackageIdent,
    package: &PackageInstall,
    m: &ArgMatches,
) -> Result<Vec<ServiceSpec>> {
    let specs = match package.pkg_type()? {
        PackageType::Standalone => {
            let spec = new_service_spec(original_ident.clone(), m)?;
            vec![spec]
        }
        PackageType::Composite => {
            let composite_name = &package.ident().name;

            // All the service specs will be customized copies of
            // this.
            let base_spec = base_composite_service_spec(composite_name, m)?;

            let bind_map = package.bind_map()?;
            let mut cli_composite_binds = composite_binds_from_input(m)?;

            let services = package.pkg_services()?;
            let mut specs: Vec<ServiceSpec> = Vec::with_capacity(services.len());
            for service in services {
                // Customize each service's spec as appropriate
                let mut spec = base_spec.clone();
                spec.ident = service;
                set_composite_binds(&mut spec, &bind_map, &mut cli_composite_binds)?;
                specs.push(spec);
            }
            specs
        }
    };
    Ok(specs)
}

fn update_composite_service_specs(
    spec: &mut Vec<ServiceSpec>,
    package: &PackageInstall,
    m: &ArgMatches,
) -> Result<()> {
    let bind_map = package.bind_map()?;
    // TODO (CM): maybe not mutable?
    let mut cli_composite_binds = composite_binds_from_input(m)?;

    let update_binds = m.values_of("BIND").is_some();

    for spec in spec.iter_mut() {
        // The Builder URL and channel have default values; we only want to
        // change them if the user specified something!
        set_bldr_url_from_input(spec, m);
        set_channel_from_input(spec, m);

        set_app_env_from_input(spec, m)?;
        set_group_from_input(spec, m);
        set_strategy_from_input(spec, m);
        set_topology_from_input(spec, m);

        // No setting of config or password either; see notes in
        // `base_composite_service_spec` for more.

        // Just as with standalone services, we don't do anything to
        // the binds unless you've specified new ones on the CLI. For
        // composites, such binds can be thought of as binds for the
        // overall composite.
        if update_binds {
            set_composite_binds(spec, &bind_map, &mut cli_composite_binds)?;
        }
    }
    Ok(())
}
