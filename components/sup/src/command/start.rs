// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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
use common::command::package::install;
use common::ui::UI;
use depot_client::Client;
use hcore::fs::{am_i_root, cache_artifact_path, FS_ROOT_PATH};
use hcore::package::PackageIdent;

use {PRODUCT, VERSION};
use error::{Error, Result};
use config::{gconfig, UpdateStrategy};
use package::Package;
use manager::Manager;

static LOGKEY: &'static str = "CS";

/// Creates a [Package](../../pkg/struct.Package.html), then passes it to the run method of the
/// selected [topology](../../topology).
///
/// # Failures
///
/// * Fails if it cannot find a package with the given name
/// * Fails if the `run` method for the topology fails
/// * Fails if an unknown topology was specified on the command line
pub fn package() -> Result<()> {
    let mut ui = UI::default();
    if !am_i_root() {
        try!(ui.warn("Running the Habitat Supervisor requires root or administrator privileges. \
                      Please retry this command as a super user or use a privilege-granting \
                      facility such as sudo."));
        try!(ui.br());
        return Err(sup_error!(Error::RootRequired));
    }

    match Package::load(gconfig().package(), None) {
        Ok(mut package) => {
            let update_strategy = gconfig().update_strategy();
            match update_strategy {
                UpdateStrategy::None => {}
                _ => {
                    let url = gconfig().url();
                    outputln!("Checking Depot for newer versions...");
                    // It is important to pass `gconfig().package()` to `show_package()` instead
                    // of the package identifier of the loaded package. This will ensure that
                    // if the operator starts a package while specifying a version number, they
                    // will only automaticaly receive release updates for the started package.
                    //
                    // If the operator does not specify a version number they will
                    // automatically receive updates for any releases, regardless of version
                    // number, for the started  package.
                    let depot_client = try!(Client::new(url, PRODUCT, VERSION, None));
                    let latest_pkg_data =
                        try!(depot_client.show_package((*gconfig().package()).clone()));
                    let latest_ident: PackageIdent = latest_pkg_data.get_ident().clone().into();
                    if &latest_ident > package.ident() {
                        outputln!("Downloading latest version from Depot: {}", latest_ident);
                        let new_pkg_data = try!(install::start(&mut ui,
                                                               url,
                                                               &latest_ident.to_string(),
                                                               PRODUCT,
                                                               VERSION,
                                                               Path::new(FS_ROOT_PATH),
                                                               &cache_artifact_path(None)));
                        package = try!(Package::load(&new_pkg_data, None));
                    } else {
                        outputln!("Already running latest.");
                    };
                }
            }
            start_package(package)
        }
        Err(_) => {
            outputln!("{} is not installed",
                      Yellow.bold().paint(gconfig().package().to_string()));
            let url = gconfig().url();
            let new_pkg_data = match gconfig().local_artifact() {
                Some(artifact) => {
                    try!(install::start(&mut ui,
                                        url,
                                        &artifact,
                                        PRODUCT,
                                        VERSION,
                                        Path::new(FS_ROOT_PATH),
                                        &cache_artifact_path(None)))
                }
                None => {
                    outputln!("Searching for {} in remote {}",
                              Yellow.bold().paint(gconfig().package().to_string()),
                              url);
                    try!(install::start(&mut ui,
                                        url,
                                        &gconfig().package().to_string(),
                                        PRODUCT,
                                        VERSION,
                                        Path::new(FS_ROOT_PATH),
                                        &cache_artifact_path(None)))
                }
            };
            let package = try!(Package::load(&new_pkg_data, None));
            start_package(package)
        }
    }
}

fn start_package(package: Package) -> Result<()> {
    let run_path = try!(package.run_path());
    debug!("Setting the PATH to {}", run_path);
    env::set_var("PATH", &run_path);

    let mut manager = try!(Manager::new());
    manager.add_service(package, *gconfig().topology(), gconfig().update_strategy());
    manager.run()
}
