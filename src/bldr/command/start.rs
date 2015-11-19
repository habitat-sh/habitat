//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

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

use ansi_term::Colour::Yellow;

use super::super::PACKAGE_CACHE;
use error::{BldrResult, BldrError};
use config::Config;
use pkg::Package;
use topology::{self, Topology};
use command::install;
use repo;

/// Creates a [Package](../../pkg/struct.Package.html), then passes it to the run method of the
/// selected [topology](../../topology).
///
/// # Failures
///
/// * Fails if it cannot find a package with the given name
/// * Fails if the `run` method for the topology fails
/// * Fails if an unknown topology was specified on the command line
pub fn package(config: &Config) -> BldrResult<()> {
    match Package::latest(config.deriv(), config.package(), None) {
        Ok(mut package) => {
            if let Some(ref url) = *config.url() {
                println!("Checking remote for newer versions...");
                let latest_pkg = try!(repo::client::show_package_latest(&url, &package));
                if latest_pkg > package {
                    println!("Downloading latest version from remote: {}", &latest_pkg);
                    let pkg_file = try!(repo::client::fetch_package(&url,
                                                                    &latest_pkg,
                                                                    PACKAGE_CACHE));
                    try!(install::verify(config.package(), &pkg_file));
                    try!(install::unpack(config.package(), &pkg_file));
                    package = try!(Package::from_path(&pkg_file));
                } else {
                    println!("Already running latest.");
                };
            }
            start_package(package, config)
        }
        Err(_) => {
            println!("{} not found in local cache",
                     Yellow.bold().paint(config.package()));
            match *config.url() {
                Some(ref url) => {
                    println!("Searching for {} in remote {}",
                             Yellow.bold().paint(config.package()),
                             url);
                    let pkg_file = try!(install::latest_from_url(config.deriv(),
                                                                 config.package(),
                                                                 url));
                    try!(install::verify(&config.package(), &pkg_file));
                    try!(install::unpack(&config.package(), &pkg_file));
                    let package = try!(Package::from_path(&pkg_file));
                    start_package(package, config)
                }
                None => {
                    Err(BldrError::PackageNotFound(config.deriv().to_string(),
                                                   config.package().to_string()))
                }
            }
        }
    }
}

fn start_package(package: Package, config: &Config) -> BldrResult<()> {
    match *config.topology() {
        Topology::Standalone => topology::standalone::run(package, config),
        Topology::Leader => topology::leader::run(package, config),
        Topology::Initializer => topology::initializer::run(package, config),
    }
}
