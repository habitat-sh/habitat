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
//! $ bldr start redis
//! ```
//!
//! Will start the `redis` service in the `default` group, using the `standalone` topology.
//!
//! ```bash
//! $ bldr start redis -g production
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
//! $ bldr start redis -t leader
//! ```
//!
//! Will start the `redis` service using the `leader` topology.
//!
//! ```bash
//! $ bldr start redis -t leader -g production -w haproxy.default
//! ```
//!
//! Will start the `redis` service using the `leader` topology in the `production` group, while
//! watching the `haproxy` `default` group's configuration.
//!
//! See the [documentation on topologies](../topology) for a deeper discussion of how they function.
//!

use error::BldrResult;
use config::Config;
use pkg::Package;
use topology::{self, Topology};

/// Creates a [Package](../../pkg/struct.Package.html), then passes it to the run method of the
/// selected [topology](../../topology).
///
/// # Failures
///
/// * Fails if it cannot find a package with the given name
/// * Fails if the `run` method for the topology fails
/// * Fails if an unknown topology was specified on the command line
pub fn package(config: &Config) -> BldrResult<()> {
    let package = try!(Package::latest(config.deriv(), config.package(), None));
    match *config.topology() {
        Topology::Standalone => try!(topology::standalone::run(package, config)),
        Topology::Leader => try!(topology::leader::run(package, config)),
        Topology::Initializer => try!(topology::initializer::run(package, config)),
    }
    Ok(())
}
