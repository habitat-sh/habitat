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
use std::sync::{Arc, RwLock};

use wonder;
use wonder::actor::{GenServer, InitResult, HandleResult, StopReason, ActorSender};

use error::BldrError;
use health_check;
use package::Package;
use service_config::ServiceConfig;

const GET_HEALTH: &'static str = "/health";
const GET_CONFIG: &'static str = "/config";
const GET_STATUS: &'static str = "/status";
const LISTEN_ADDR: &'static str = "0.0.0.0:9631";

pub type SidecarActor = wonder::actor::Actor<SidecarMessage>;

pub struct Sidecar;

pub struct SidecarState {
    /// The package this sidecar is helping out
    pub package: Arc<RwLock<Package>>,
    /// The configuration of the supervised service
    pub config: Arc<RwLock<ServiceConfig>>,
}

#[derive(Debug)]
pub enum SidecarMessage {
    Ok,
    Stop,
}

impl SidecarState {
    pub fn new(package: Arc<RwLock<Package>>, config: Arc<RwLock<ServiceConfig>>) -> Self {
        SidecarState {
            package: package,
            config: config,
        }
    }
}

impl Sidecar {
    /// Start the sidecar.
    pub fn start(package: Arc<RwLock<Package>>,
                 config: Arc<RwLock<ServiceConfig>>)
                 -> SidecarActor {
        let state = SidecarState::new(package, config);
        wonder::actor::Builder::new(Sidecar).name("sidecar".to_string()).start(state).unwrap()
    }
}

impl GenServer for Sidecar {
    type T = SidecarMessage;
    type S = SidecarState;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, _state: &mut Self::S) -> InitResult<Self::E> {
        Ok(Some(0))
    }

    fn handle_timeout(&self,
                      _tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        let mut router = Router::new();
        let package_1 = state.package.clone();
        let package_2 = state.package.clone();
        let package_3 = state.package.clone();
        let config_1 = state.config.clone();

        router.get(GET_CONFIG, move |r: &mut Request| config(&package_1, r));
        router.get(GET_STATUS, move |r: &mut Request| status(&package_2, r));
        router.get(GET_HEALTH,
                   move |r: &mut Request| health(&package_3, &config_1, r));

        match Iron::new(router).http(LISTEN_ADDR) {
            Ok(_) => HandleResult::NoReply(None),
            Err(_) =>
                HandleResult::Stop(StopReason::Fatal("couldn't start router".to_string()), None),
        }
    }
}

/// The /health callback.
///
/// Returns the current running configuration.
///
/// # Failures
///
/// * Fails if the configuration cannot be found.
fn config(lock: &Arc<RwLock<Package>>, _req: &mut Request) -> IronResult<Response> {
    let package = lock.read().unwrap();
    let last_config = try!(package.last_config());
    Ok(Response::with((status::Ok, last_config)))
}

/// The /status callback.
///
/// Returns the current status from the supervisors perspective.
///
/// # Failures
///
/// * Fails if the supervisor cannot return the status.
fn status(lock: &Arc<RwLock<Package>>, _req: &mut Request) -> IronResult<Response> {
    let package = lock.read().unwrap();
    let output = try!(package.status());
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
fn health(package_lock: &Arc<RwLock<Package>>,
          config_lock: &Arc<RwLock<ServiceConfig>>,
          _req: &mut Request)
          -> IronResult<Response> {
    let result = {
        let package = package_lock.read().unwrap();
        let config = config_lock.read().unwrap();
        try!(package.health_check(&config))
    };

    match result.status {
        health_check::Status::Ok | health_check::Status::Warning => {
            Ok(Response::with((status::Ok, format!("{}", result))))
        }
        health_check::Status::Critical => {
            Ok(Response::with((status::ServiceUnavailable, format!("{}", result))))
        }
        health_check::Status::Unknown => {
            Ok(Response::with((status::InternalServerError, format!("{}", result))))
        }
    }
}

/// Translates BldrErrors into IronErrors
impl From<BldrError> for IronError {
    fn from(err: BldrError) -> IronError {
        IronError {
            error: Box::new(err),
            response: Response::with((status::InternalServerError, "Internal bldr error")),
        }
    }
}
