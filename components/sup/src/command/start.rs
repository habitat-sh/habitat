// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Starts a service from an installed Habitat package.
//!
//! Services run by the Supervisor support one or more *topologies*, which are state machines that
//! handle the lifecycle of a service; they are members of a *group*, which is a namespace for
//! their configuration and state; and they can *watch* another service group, incorporating that
//! groups configuration and state into their own.
//!
//! # Examples
//!
//! ```bash
//! $ hab-sup start acme/redis
//! ```
//!
//! Will start the `redis` service in the `default` group, using the `standalone` topology.
//!
//! ```bash
//! $ hab-sup start acme/redis -g production
//! ```
//!
//! Will do the same, but in the `production` group.
//!
//! ```bash
//! $ hab-sup start haproxy -w redis.production
//! ```
//!
//! Will start the `haproxy` service, and have it watch the configuration for the `redis`
//! `production` group (note the `.` as the separator.)
//!
//! ```bash
//! $ hab-sup start acme/redis -t leader
//! ```
//!
//! Will start the `redis` service using the `leader` topology.
//!
//! ```bash
//! $ hab-sup start acme/redis -t leader -g production -w haproxy.default
//! ```
//!
//! Will start the `redis` service using the `leader` topology in the `production` group, while
//! watching the `haproxy` `default` group's configuration.
//!
//! See the [documentation on topologies](../topology) for a deeper discussion of how they function.
//!

use std::env;
use std::path::Path;

use ansi_term::Colour::Yellow;
use common::command::ProgressBar;
use common::command::package::install;
use depot_client;
use hcore::crypto::default_cache_key_path;
use hcore::fs::{cache_artifact_path, FS_ROOT_PATH};

use error::{Error, Result};
use config::{Config, UpdateStrategy};
use package::Package;
use topology::{self, Topology};

static LOGKEY: &'static str = "CS";

/// Creates a [Package](../../pkg/struct.Package.html), then passes it to the run method of the
/// selected [topology](../../topology).
///
/// # Failures
///
/// * Fails if it cannot find a package with the given name
/// * Fails if the `run` method for the topology fails
/// * Fails if an unknown topology was specified on the command line
pub fn package(config: &Config) -> Result<()> {
    match Package::load(config.package(), None) {
        Ok(package) => {
            let update_strategy = config.update_strategy();
            match update_strategy {
                UpdateStrategy::None => {}
                _ => {
                    if let &Some(ref url) = config.url() {
                        outputln!("Checking remote for newer versions...");
                        // It is important to pass `config.package()` to `show_package()` instead of the
                        // package identifier of the loaded package. This will ensure that if the operator
                        // starts a package while specifying a version number, they will only automaticaly
                        // receive release updates for the started package.
                        //
                        // If the operator does not specify a version number they will automatically receive
                        // updates for any releases, regardless of version number, for the started  package.
                        let latest_pkg_data = try!(depot_client::show_package(&url,
                                                                              config.package()));
                        let latest_ident = latest_pkg_data.ident.as_ref();
                        if latest_ident > package.ident() {
                            outputln!("Downloading latest version from remote: {}", latest_ident);
                            let mut progress = ProgressBar::default();
                            let archive = try!(depot_client::fetch_package(&url,
                                                                 latest_ident,
                                                                 &cache_artifact_path(None),
                                                                 Some(&mut progress)));
                            try!(archive.verify(&default_cache_key_path(None)));
                            try!(archive.unpack(None));
                        } else {
                            outputln!("Already running latest.");
                        };
                    }
                }
            }
            start_package(package, config)
        }
        Err(_) => {
            outputln!("{} is not installed",
                      Yellow.bold().paint(config.package().to_string()));
            match *config.url() {
                Some(ref url) => {
                    outputln!("Searching for {} in remote {}",
                              Yellow.bold().paint(config.package().to_string()),
                              url);
                    let new_pkg_data = try!(install::from_url(url,
                                                              config.package(),
                                                              Path::new(FS_ROOT_PATH),
                                                              &cache_artifact_path(None),
                                                              &default_cache_key_path(None)));
                    let package = try!(Package::load(new_pkg_data.ident.as_ref(), None));
                    start_package(package, config)
                }
                None => Err(sup_error!(Error::PackageNotFound(config.package().clone()))),
            }
        }
    }
}

fn start_package(package: Package, config: &Config) -> Result<()> {
    let run_path = try!(package.run_path());
    debug!("Setting the PATH to {}", run_path);
    env::set_var("PATH", &run_path);
    match *config.topology() {
        Topology::Standalone => topology::standalone::run(package, config),
        Topology::Leader => topology::leader::run(package, config),
        Topology::Initializer => topology::initializer::run(package, config),
    }
}
