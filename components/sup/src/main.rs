// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

#[macro_use]
extern crate habitat_sup as sup;
extern crate habitat_core as hcore;
extern crate rustc_serialize;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ansi_term;
extern crate libc;
#[macro_use]
extern crate clap;

use std::process;
use std::result;
use std::str::FromStr;

use ansi_term::Colour::Yellow;
use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use hcore::env as henv;
use hcore::fs;
use hcore::crypto::{default_cache_key_path, SymKey};
use hcore::crypto::init as crypto_init;
use hcore::package::PackageIdent;
use hcore::url::{DEFAULT_DEPOT_URL, DEPOT_URL_ENVVAR};

use sup::config::{Command, Config, UpdateStrategy};
use sup::error::{Error, Result, SupError};
use sup::command::*;
use sup::topology::Topology;
use sup::util::parse_ip_port_with_defaults;
use sup::util::sys::ip;

/// Our output key
static LOGKEY: &'static str = "MN";

/// The version number
const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

/// CLI defaults
static DEFAULT_GROUP: &'static str = "default";

static DEFAULT_HTTP_LISTEN_IP: &'static str = "0.0.0.0";
static DEFAULT_HTTP_LISTEN_PORT: u16 = 9631;

const DEFAULT_GOSSIP_LISTEN_PORT: u16 = 9634;

static RING_ENVVAR: &'static str = "HAB_RING";
static RING_KEY_ENVVAR: &'static str = "HAB_RING_KEY";

/// Creates a [Config](config/struct.Config.html) from global args
/// and subcommand args.
fn config_from_args(args: &ArgMatches, subcommand: &str, sub_args: &ArgMatches) -> Result<Config> {
    let mut config = Config::new();
    let command = try!(Command::from_str(subcommand));
    config.set_command(command);
    if let Some(ref strategy) = sub_args.value_of("strategy") {
        config.set_update_strategy(UpdateStrategy::from_str(strategy));
    }
    if let Some(ref archive) = sub_args.value_of("archive") {
        config.set_archive(archive.to_string());
    }
    if let Some(ref package) = sub_args.value_of("package") {
        let ident = try!(PackageIdent::from_str(package));
        config.set_package(ident);
    }
    if let Some(key) = sub_args.value_of("key") {
        config.set_key(key.to_string());
    }
    if let Some(email) = sub_args.value_of("email") {
        config.set_email(email.to_string());
    }
    if let Some(user) = sub_args.value_of("user") {
        config.set_user_key(user.to_string());
    }
    if let Some(service) = sub_args.value_of("service") {
        config.set_service_key(service.to_string());
    }
    if let Some(infile) = sub_args.value_of("infile") {
        config.set_infile(infile.to_string());
    }
    if let Some(outfile) = sub_args.value_of("outfile") {
        config.set_outfile(outfile.to_string());
    }
    if let Some(topology) = sub_args.value_of("topology") {
        match topology.as_ref() {
            "standalone" => {
                config.set_topology(Topology::Standalone);
            }
            "leader" => {
                config.set_topology(Topology::Leader);
            }
            "initializer" => {
                config.set_topology(Topology::Initializer);
            }
            t => return Err(sup_error!(Error::UnknownTopology(String::from(t)))),
        }
    }
    if sub_args.value_of("expire-days").is_some() {
        let ed = value_t!(sub_args.value_of("expire-days"), u16).unwrap_or_else(|e| e.exit());
        config.set_expire_days(ed);
    }
    let env_or_default = henv::var(DEPOT_URL_ENVVAR).unwrap_or(DEFAULT_DEPOT_URL.to_string());
    let url = sub_args.value_of("url").unwrap_or(&env_or_default);
    config.set_url(url.to_string());
    config.set_group(sub_args.value_of("group").unwrap_or(DEFAULT_GROUP).to_string());
    let bindings = match sub_args.values_of("bind") {
        Some(bind) => bind.map(|s| s.to_string()).collect(),
        None => vec![],
    };
    config.set_bind(bindings);
    config.set_path(sub_args.value_of("path")
        .unwrap_or(fs::svc_path(sup::PROGRAM_NAME.as_str())
            .join("data")
            .to_string_lossy()
            .as_ref())
        .to_string());

    let default_gossip_ip = try!(ip());
    let (gossip_ip, gossip_port) = try!(parse_ip_port_with_defaults(
                                        sub_args.value_of("listen-peer"),
                                        &default_gossip_ip,
                                        DEFAULT_GOSSIP_LISTEN_PORT));

    debug!("Gossip IP = {}", &gossip_ip);
    debug!("Gossip port = {}", &gossip_port);
    config.set_gossip_listen_ip(gossip_ip);
    config.set_gossip_listen_port(gossip_port);

    let (sidecar_ip, sidecar_port) = try!(parse_ip_port_with_defaults(
                                            sub_args.value_of("listen-http"),
                                            DEFAULT_HTTP_LISTEN_IP,
                                            DEFAULT_HTTP_LISTEN_PORT));

    debug!("HTTP IP = {}", &sidecar_ip);
    debug!("HTTP port = {}", &sidecar_port);

    config.set_http_listen_ip(sidecar_ip);
    config.set_http_listen_port(sidecar_port);

    let gossip_peers = match sub_args.values_of("peer") {
        Some(gp) => gp.map(|s| s.to_string()).collect(),
        None => vec![],
    };
    config.set_gossip_peer(gossip_peers);
    if sub_args.value_of("permanent-peer").is_some() {
        config.set_gossip_permanent(true);
    }
    if let Some(sg) = sub_args.value_of("service-group") {
        config.set_service_group(sg.to_string());
    }
    if let Some(fp) = sub_args.value_of("file-path") {
        config.set_file_path(fp.to_string());
    }
    config.set_version_number(value_t!(sub_args, "version-number", u64).unwrap_or(0));
    let ring = match sub_args.value_of("ring") {
        Some(val) => Some(try!(SymKey::get_latest_pair_for(&val, &default_cache_key_path(None)))),
        None => {
            match henv::var(RING_KEY_ENVVAR) {
                Ok(val) => {
                    let (key, _) = try!(SymKey::write_file_from_str(&val,
                                                                    &default_cache_key_path(None)));
                    Some(key)
                }
                Err(_) => {
                    match henv::var(RING_ENVVAR) {
                        Ok(val) => {
                            Some(try!(SymKey::get_latest_pair_for(&val,
                                                                  &default_cache_key_path(None))))
                        }
                        Err(_) => None,
                    }
                }
            }
        }
    };
    if let Some(ring) = ring {
        config.set_ring(ring.name_with_rev());
    }
    if args.value_of("verbose").is_some() {
        sup::output::set_verbose(true);
    }
    if args.value_of("no-color").is_some() {
        sup::output::set_no_color(true);
    }

    if let Some(org) = sub_args.value_of("organization") {
        config.set_organization(org.to_string());
    }
    debug!("Config:\n{:?}", config);
    Ok(config)
}

type Handler = fn(&Config) -> result::Result<(), sup::error::SupError>;

/// The entrypoint for the Supervisor.
///
/// * Set up the logger
/// * Pull in the arguments from the Command Line, push through clap
/// * Dispatch to a function that handles that action called
/// * Exit cleanly, or if we return an `Error`, call `exit_with(E, 1)`
#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();
    crypto_init();

    let arg_url = || {
        Arg::with_name("url")
            .short("u")
            .long("url")
            .takes_value(true)
            .help("Use the specified package depot url")
    };
    let arg_group = || {
        Arg::with_name("group")
            .long("group")
            .takes_value(true)
            .help("The service group; shared config and topology [default: default].")
    };

    let arg_org = || {
        Arg::with_name("organization")
            .long("org")
            .takes_value(true)
            .help("The organization that a service is part of")
    };

    let arg_strategy = || {
        Arg::with_name("strategy")
            .long("strategy")
            .short("s")
            .takes_value(true)
            .possible_values(&["none", "at-once"])
            .help("The update strategy; [default: none].")
    };

    let sub_start = SubCommand::with_name("start")
        .about("Start a Habitat-supervised service from a package")
        .aliases(&["st", "sta", "star"])
        .arg(Arg::with_name("package")
            .index(1)
            .required(true)
            .help("Name of package to start"))
        .arg(arg_url())
        .arg(arg_group())
        .arg(arg_org())
        .arg(arg_strategy())
        .arg(Arg::with_name("topology")
            .short("t")
            .long("topology")
            .value_name("topology")
            .help("Service topology"))
        .arg(Arg::with_name("bind")
            .long("bind")
            .value_name("bind")
            .multiple(true)
            .help("One or more service groups to bind to a configuration"))
        .arg(Arg::with_name("ring")
            .short("r")
            .long("ring")
            .value_name("ring")
            .help("Ring key name"))
        .arg(Arg::with_name("peer")
            .long("peer")
            .value_name("ip:port")
            .multiple(true)
            .help("The listen address of an initial peer"))
        .arg(Arg::with_name("listen-peer")
            .long("listen-peer")
            .value_name("ip:port")
            .help("The listen address [default: ip_with_default_route:9634]"))
        .arg(Arg::with_name("listen-http")
            .long("listen-http")
            .value_name("ip:port")
            .help("The HTTP API listen address [default: 0.0.0.0:9631]"))
        .arg(Arg::with_name("permanent-peer")
            .short("I")
            .long("permanent-peer")
            .help("If this service is a permanent peer"));
    let sub_bash = SubCommand::with_name("bash")
        .about("Start an interactive shell (bash)")
        .aliases(&["b", "ba", "bas"]);
    let sub_sh = SubCommand::with_name("sh").about("Start an interactive shell (sh)");
    let sub_config = SubCommand::with_name("config")
        .about("Print the default.toml for a given package")
        .aliases(&["c", "co", "con", "conf", "confi"])
        .arg(Arg::with_name("package")
            .index(1)
            .required(true)
            .help("Name of package"));
    let args = App::new(sup::PROGRAM_NAME.as_str())
        .version(VERSION)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::with_name("verbose")
            .short("v")
            .global(true)
            .help("Verbose output; shows line numbers"))
        .arg(Arg::with_name("no-color")
            .long("no-color")
            .global(true)
            .help("Turn ANSI color off :("))
        .subcommand(sub_start)
        .subcommand(sub_bash)
        .subcommand(sub_sh)
        .subcommand(sub_config);
    let matches = args.get_matches();

    debug!("clap matches {:?}", matches);

    let subcommand_name = matches.subcommand_name().unwrap();
    let subcommand_matches = matches.subcommand_matches(subcommand_name).unwrap();

    let config = match config_from_args(&matches, subcommand_name, &subcommand_matches) {
        Ok(config) => config,
        Err(e) => return exit_with(e, 1),
    };

    let result = match config.command() {
        Command::ShellBash => shell_bash(&config),
        Command::ShellSh => shell_sh(&config),
        Command::Config => configure(&config),
        Command::Start => start(&config),
    };

    match result {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with(e, 1),
    }
}

/// Exit with an error message and the right status code
#[allow(dead_code)]
fn exit_with(e: SupError, code: i32) {
    println!("{}", e.to_string());
    process::exit(code)
}

/// Start a sh shell
#[allow(dead_code)]
fn shell_sh(_config: &Config) -> Result<()> {
    shell::sh()
}

/// Start a bash shell
#[allow(dead_code)]
fn shell_bash(_config: &Config) -> Result<()> {
    shell::bash()
}

/// Show the configuration options for a service
#[allow(dead_code)]
fn configure(config: &Config) -> Result<()> {
    try!(configure::display(config));
    Ok(())
}

/// Start a service
#[allow(dead_code)]
fn start(config: &Config) -> Result<()> {
    outputln!("Starting {}",
              Yellow.bold().paint(config.package().to_string()));
    try!(start::package(config));
    outputln!("Finished with {}",
              Yellow.bold().paint(config.package().to_string()));
    Ok(())
}
