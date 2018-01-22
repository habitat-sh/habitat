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

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate airlock;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process;
use std::result;

use airlock::{FsRoot, FsRootPolicy, Result};
use airlock::command;
use clap::{App, ArgMatches};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

fn main() {
    env_logger::init();
    if let Err(e) = _main() {
        eprintln!("FATAL: {}", e);
        process::exit(1);
    }
}

fn _main() -> Result<()> {
    let app_matches = cli().get_matches();
    debug!("clap cli matches: {:?}", &app_matches);
    match app_matches.subcommand() {
        ("nsrun", Some(m)) => sub_nsrun(m),
        ("run", Some(m)) => sub_run(m),
        ("netns", Some(m)) => {
            match m.subcommand() {
                ("create", Some(m)) => sub_netns_create(m),
                ("createasuser", Some(m)) => sub_netns_createasuser(m),
                ("createinns", Some(m)) => sub_netns_createinns(m),
                ("destroy", Some(m)) => sub_netns_destroy(m),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    }
}

fn sub_netns_create(m: &ArgMatches) -> Result<()> {
    let ns_dir = Path::new(m.value_of("NS_DIR").unwrap());
    let user = m.value_of("USER").unwrap();
    let interface = m.value_of("INTERFACE").unwrap();
    let gateway = m.value_of("GATEWAY").unwrap();

    command::netns::create::run(ns_dir, user, interface, gateway)
}

fn sub_netns_createasuser(m: &ArgMatches) -> Result<()> {
    let ns_dir = Path::new(m.value_of("NS_DIR").unwrap());
    let interface = m.value_of("INTERFACE").unwrap();
    let gateway = m.value_of("GATEWAY").unwrap();
    let ipv4s: Vec<_> = m.values_of("IPV4").unwrap().collect();
    let ipv6s: Vec<_> = m.values_of("IPV6").unwrap().collect();

    command::netns::createasuser::run(ns_dir, interface, gateway, ipv4s, ipv6s)
}

fn sub_netns_createinns(m: &ArgMatches) -> Result<()> {
    let ns_dir = Path::new(m.value_of("NS_DIR").unwrap());
    let interface = m.value_of("INTERFACE").unwrap();
    let gateway = m.value_of("GATEWAY").unwrap();
    let ipv4s: Vec<_> = m.values_of("IPV4").unwrap().collect();
    let ipv6s: Vec<_> = m.values_of("IPV6").unwrap().collect();

    command::netns::createinns::run(ns_dir, interface, gateway, ipv4s, ipv6s)
}

fn sub_netns_destroy(m: &ArgMatches) -> Result<()> {
    let ns_dir = Path::new(m.value_of("NS_DIR").unwrap());

    command::netns::destroy::run(ns_dir)
}

fn sub_nsrun(m: &ArgMatches) -> Result<()> {
    let fs_root = Path::new(m.value_of("FS_ROOT").unwrap());
    let mount_artifacts = m.is_present("MOUNT_ARTIFACT_CACHE");
    let mut args: Vec<&OsStr> = m.values_of_os("CMD").unwrap().collect();
    // cmd arg is required and multiple so must contain a first element
    let cmd = args.remove(0);

    command::nsrun::run(fs_root, cmd, args, mount_artifacts)
}

fn sub_run(m: &ArgMatches) -> Result<()> {
    let mut args: Vec<&OsStr> = m.values_of_os("CMD").unwrap().collect();
    // cmd arg is required and multiple so must contain a first element
    let cmd = args.remove(0);

    let policy = if m.is_present("NO_RM") {
        FsRootPolicy::Persist
    } else {
        FsRootPolicy::Cleanup
    };
    let fs_root = match m.value_of("FS_ROOT") {
        Some(val) => FsRoot::at(PathBuf::from(val), policy)?,
        None => FsRoot::in_tmpdir(policy)?,
    };

    // If a network namespace is to be used, the corresponding user namespace must be used as the
    // kernel checks for this
    let namespaces = match m.value_of("NETNS") {
        Some(netns) => Some((Path::new(m.value_of("USERNS").unwrap()), Path::new(netns))),
        None => None,
    };
    let mount_artifacts = m.is_present("MOUNT_ARTIFACT_CACHE");

    command::run::run(fs_root, cmd, args, namespaces, mount_artifacts)
}

fn cli<'a, 'b>() -> App<'a, 'b> {
    let program_name = {
        let arg0 = env::args().next().map(|p| PathBuf::from(p));
        arg0.as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|p| p.to_str())
            .unwrap()
            .to_string()
    };
    clap_app!((program_name) =>
        (about: "Airlock: your gateway to a Studio")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand nsrun =>
            (@setting Hidden)
            (about: "**Internal** command to run a command inside the created namespace")
            (@setting TrailingVarArg)
            (@arg MOUNT_ARTIFACT_CACHE: --("mount-artifact-cache") -m
                "Mount the user's Habitat artifact cache directory (default: no)")
            (@arg FS_ROOT: +required +takes_value {validate_dir_exists}
                "Path to the rootfs (ex: /tmp/rootfs)")
            (@arg CMD: +required +takes_value +multiple
                "The command and arguments to execute (ex: ls -l /tmp)")
        )
        (@subcommand run =>
            (about: "Run a command in a namespace")
            (@setting TrailingVarArg)
            (@arg FS_ROOT: --("fs-root") -r +takes_value {validate_dir_not_exists}
                "Path to use for the filesystem root (default: randomly generated under TMPDIR)")
            (@arg RM: --rm conflicts_with[NO_RM]
                "Remove the filsystem root on exit (default: yes)")
            (@arg NO_RM: --("no-rm") conflicts_with[RM]
                "Do not remove the filsystem root on exit (default: no)")
            (@arg NETNS: --("use-netns") +takes_value {validate_file_exists} requires[USERNS]
                "Use network namespace (ex: /tmp/airlock-ns/netns)")
            (@arg USERNS: --("use-userns") +takes_value {validate_file_exists} requires[NETNS]
                "Use user namespace (ex: /tmp/airlock-ns/userns)")
            (@arg MOUNT_ARTIFACT_CACHE: --("mount-artifact-cache") -m
                "Mount the user's Habitat artifact cache directory (default: no)")
            (@arg CMD: +required +takes_value +multiple
                "The command and arguments to execute (ex: ls -l /tmp)")
        )
        (@subcommand netns =>
            (about: "Commands relating to network namespaces")
            (@setting ArgRequiredElseHelp)
            (@subcommand create =>
                (about: "Create a network namespace for a user")
                (@arg NS_DIR: --("ns-dir") -d +takes_value +required {validate_dir_not_exists}
                    "Path where the namespace files will be mounted")
                (@arg USER: --user -u +required +takes_value
                    "Username of the user who will own the namespace (ex: jdoe)")
                (@arg INTERFACE: --interface -i +required +takes_value
                    "Network interface which will be assigned to the namespace (ex: eth1)")
                (@arg GATEWAY: --gateway -g +required +takes_value
                    "Network gateway address which will be assigned to the interface (ex: eth1)")
            )
            (@subcommand createasuser =>
                (@setting Hidden)
                (about: "**Internal** Create the network namespace as the non-root user")
                (@arg NS_DIR: --("ns-dir") -d +takes_value +required {validate_dir_not_exists}
                    "Path where the namespace files will be mounted")
                (@arg INTERFACE: --interface -i +required +takes_value
                    "Network interface which will be assigned to the namespace (ex: eth1)")
                (@arg GATEWAY: --gateway -g +required +takes_value
                    "Network gateway address which will be assigned to the interface (ex: eth1)")
                (@arg IPV4: --ipv4addr +required +takes_value +multiple
                    "IPv4 addresses which will be assigned to the newtwork interface \
                    (ex: 192.168.211.134/24)")
                (@arg IPV6: --ipv6addr +takes_value +multiple
                    "IPv6 addresses which will be assigned to the newtwork interface \
                    (ex: fe80::20c:29ff:fef4:ae/64)")
            )
            (@subcommand createinns =>
                (@setting Hidden)
                (about: "**Internal** Setup the network namespace inside the namespace")
                (@arg NS_DIR: --("ns-dir") -d +takes_value +required {validate_dir_exists}
                    "Path where the namespace files will be mounted")
                (@arg INTERFACE: --interface -i +required +takes_value
                    "Network interface which will be assigned to the namespace (ex: eth1)")
                (@arg GATEWAY: --gateway -g +required +takes_value
                    "Network gateway address which will be assigned to the interface (ex: eth1)")
                (@arg IPV4: --ipv4addr +required +takes_value +multiple
                    "IPv4 addresses which will be assigned to the newtwork interface \
                    (ex: 192.168.211.134/24)")
                (@arg IPV6: --ipv6addr +takes_value +multiple
                    "IPv6 addresses which will be assigned to the newtwork interface \
                    (ex: fe80::20c:29ff:fef4:ae/64)")
            )
            (@subcommand destroy =>
                (about: "Destroy a created network namespace")
                (@arg NS_DIR: --("ns-dir") -d +takes_value +required {validate_dir_exists}
                    "Path where the namespace files will be mounted")
            )
        )
    )
}

fn validate_file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("file '{}' cannot be found, must exist", &val))
    }
}

fn validate_dir_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_dir() {
        Ok(())
    } else {
        Err(format!("directory '{}' cannot be found, must exist", &val))
    }
}

fn validate_dir_not_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).exists() {
        Err(format!(
            "directory or file '{}' found, this directory must not exist",
            &val
        ))
    } else {
        Ok(())
    }
}
