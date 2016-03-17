// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Starts a service from an installed bldr package.
//!
//! Services in bldr support one or more *topologies*, which are state machines that handle the
//! lifecycle of a service; they are members of a *group*, which is a namespace for their
//! configuration and state; and they can *watch* another service group, incorporating that groups
//! configuration and state into their own.
//!
//! # Examples
//!
//! ```bash
//! $ bldr start chef/redis
//! ```
//!
//! Will start the `redis` service in the `default` group, using the `standalone` topology.
//!
//! ```bash
//! $ bldr start chef/redis -g production
//! ```
//!
//! Will do the same, but in the `production` group.
//!
//! ```bash
//! $ bldr start haproxy -w redis.production
//! ```
//!
//! Will start the `haproxy` service, and have it watch the configuration for the `redis`
//! `production` group (note the `.` as the separator.)
//!
//! ```bash
//! $ bldr start chef/redis -t leader
//! ```
//!
//! Will start the `redis` service using the `leader` topology.
//!
//! ```bash
//! $ bldr start chef/redis -t leader -g production -w haproxy.default
//! ```
//!
//! Will start the `redis` service using the `leader` topology in the `production` group, while
//! watching the `haproxy` `default` group's configuration.
//!
//! See the [documentation on topologies](../topology) for a deeper discussion of how they function.
//!

use std::env;

use ansi_term::Colour::Yellow;
use core::fs::PACKAGE_CACHE;
use depot_client;

use error::{BldrResult, ErrorKind};
use config::Config;
use package::Package;
use topology::{self, Topology};
use command::install;

static LOGKEY: &'static str = "CS";

/// Creates a [Package](../../pkg/struct.Package.html), then passes it to the run method of the
/// selected [topology](../../topology).
///
/// # Failures
///
/// * Fails if it cannot find a package with the given name
/// * Fails if the `run` method for the topology fails
/// * Fails if an unknown topology was specified on the command line
pub fn package(config: &Config) -> BldrResult<()> {
    match Package::load(config.package(), None) {
        Ok(package) => {
            if let Some(ref url) = *config.url() {
                outputln!("Checking remote for newer versions...");
                // It is important to pass `config.package()` to `show_package()` instead of the
                // package identifier of the loaded package. This will ensure that if the operator
                // starts a package while specifying a version number, they will only automaticaly
                // receive release updates for the started package.
                //
                // If the operator does not specify a version number they will automatically receive
                // updates for any releases, regardless of version number, for the started  package.
                let latest_pkg: Package = try!(depot_client::show_package(&url, config.package()))
                                              .into();
                if latest_pkg > package {
                    outputln!("Downloading latest version from remote: {}", &latest_pkg);
                    let archive = try!(depot_client::fetch_package(&url,
                                                                   &latest_pkg.into(),
                                                                   PACKAGE_CACHE));
                    try!(archive.verify());
                    try!(archive.unpack());
                } else {
                    outputln!("Already running latest.");
                };
            }
            start_package(package, config)
        }
        Err(_) => {
            outputln!("{} not found in local cache",
                      Yellow.bold().paint(config.package().to_string()));
            match *config.url() {
                Some(ref url) => {
                    outputln!("Searching for {} in remote {}",
                              Yellow.bold().paint(config.package().to_string()),
                              url);
                    let package: Package = try!(install::from_url(url, config.package())).into();
                    start_package(package, config)
                }
                None => Err(bldr_error!(ErrorKind::PackageNotFound(config.package().clone()))),
            }
        }
    }
}

fn start_package(package: Package, config: &Config) -> BldrResult<()> {
    let run_path = try!(package.run_path());
    debug!("Setting the PATH to {}", run_path);
    env::set_var("PATH", &run_path);
    match *config.topology() {
        Topology::Standalone => topology::standalone::run(package, config),
        Topology::Leader => topology::leader::run(package, config),
        Topology::Initializer => topology::initializer::run(package, config),
    }
}
