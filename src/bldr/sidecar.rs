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

//! The http sidecar for bldr services. Provides an interface to verifying and validating
//! promises.
//!
//! Supports:
//!
//! * /config: Returns the current configuration of the service
//! * /health: Returns the current health of the service
//! * /status: Returns the current status of the service, from the supervisors point of view

use iron::prelude::*;
use iron::status;
use router::Router;
use std::sync::Arc;
use std::thread;

use error::{BldrError, BldrResult};

use pkg::{self, Package, Signal};
use health_check;

/// The sidecar state
struct Sidecar {
    /// The package this sidecar is helping out
    pub package: Package,
}

impl Sidecar {
    /// Returns a new sidecar.
    ///
    /// # Failures
    ///
    /// * If the package cannot be found
    fn new(pkg: &str) -> BldrResult<Arc<Sidecar>> {
        let package = try!(pkg::latest(pkg, None));
        Ok(Arc::new(Sidecar{package: package}))
    }
}

/// The /health callback.
///
/// Returns the current running configuration.
///
/// # Failures
///
/// * Fails if the configuration cannot be found.
fn config(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let last_config = try!(sidecar.package.last_config());
    Ok(Response::with((status::Ok, last_config)))
}

/// The /status callback.
///
/// Returns the current status from the supervisors perspective.
///
/// # Failures
///
/// * Fails if the supervisor cannot return the status.
fn status(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let output = try!(sidecar.package.signal(Signal::Status));
    Ok(Response::with((status::Ok, output)))
}

/// The /health callback.
///
/// Runs the packages health_check, if one is defined. Returns the status, and outputs both the
/// status and config.
///
/// # Failures
///
/// * If the health_check cannot be run.
fn health(sidecar: &Sidecar, _req: &mut Request) -> IronResult<Response> {
    let result = try!(sidecar.package.health_check());

    match result.status {
        health_check::Status::Ok | health_check::Status::Warning => {
            Ok(Response::with((status::Ok, format!("{}", result))))
        },
        health_check::Status::Critical => {
            Ok(Response::with((status::ServiceUnavailable, format!("{}", result))))
        },
        health_check::Status::Unknown => {
            Ok(Response::with((status::InternalServerError, format!("{}", result))))
        },
    }
}

/// Start the sidecar.
///
/// # Failures
///
/// * If the thread cannot be spawned
pub fn run(pkg: &str) -> BldrResult<()> {
    let pkg_name = String::from(pkg);
    try!(thread::Builder::new().name(String::from("sidecar")).spawn(move || -> BldrResult<()> {
        // The sidecar is in an Arc. The clones are
        // creating instances to share, and when they all go away, we'll
        // reap the instance. Turns out they won't really ever go away,
        // but you do what you need to :)
        let sidecar = try!(Sidecar::new(&pkg_name));
        let sidecar2 = sidecar.clone();
        let sidecar3 = sidecar.clone();

        let mut router = Router::new();

        router.get("/config", move |r: &mut Request| config(&sidecar, r));
        router.get("/status", move |r: &mut Request| status(&sidecar2, r));
        router.get("/health", move |r: &mut Request| health(&sidecar3, r));

        Iron::new(router).http("0.0.0.0:9631").unwrap();
        Ok(())
    }));
    Ok(())
}

/// Translates BldrErrors into IronErrors
impl From<BldrError> for IronError {
    fn from(err: BldrError) -> IronError {
        IronError{error: Box::new(err), response: Response::with((status::InternalServerError, "Internal bldr error"))}
    }
}

