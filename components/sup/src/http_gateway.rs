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

use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use std::thread::{self, JoinHandle};

use hcore::service::ServiceGroup;
use iron::prelude::*;
use iron::status;
use iron::typemap;
use persistent;
use router::Router;
use rustc_serialize::json;

use config::gconfig;
use error::{Result, Error, SupError};
use health_check;
use manager;

static LOGKEY: &'static str = "HG";

#[derive(PartialEq, Eq, Debug)]
pub struct ListenAddr(pub SocketAddr);

impl Default for ListenAddr {
    fn default() -> ListenAddr {
        ListenAddr(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 9631)))
    }
}

impl Deref for ListenAddr {
    type Target = SocketAddr;

    fn deref(&self) -> &SocketAddr {
        &self.0
    }
}

impl DerefMut for ListenAddr {
    fn deref_mut(&mut self) -> &mut SocketAddr {
        &mut self.0
    }
}

impl FromStr for ListenAddr {
    type Err = SupError;

    fn from_str(val: &str) -> Result<Self> {
        match SocketAddr::from_str(val) {
            Ok(addr) => Ok(ListenAddr(addr)),
            Err(_) => {
                match IpAddr::from_str(val) {
                    Ok(ip) => {
                        let mut addr = ListenAddr::default();
                        addr.set_ip(ip);
                        Ok(addr)
                    }
                    Err(_) => Err(sup_error!(Error::IPFailed)),
                }
            }
        }
    }
}

struct ManagerState;

impl typemap::Key for ManagerState {
    type Value = manager::State;
}

pub struct Server(Iron<Chain>);

impl Server {
    pub fn new(manager_state: manager::State) -> Self {
        let router = router!(
            butterfly: get "/butterfly" => butterfly,
            census: get "/census" => census,
            services: get "/services" => services,
            service_config: get "/services/:svc/:group/config" => config,
            service_health: get "/services/:svc/:group/health" => health,
            service_config_org: get "/services/:svc/:group/:org/config" => config,
            service_health_org: get "/services/:svc/:group/:org/health" => health,
        );
        let mut chain = Chain::new(router);
        chain.link(persistent::Read::<ManagerState>::both(manager_state));
        Server(Iron::new(chain))
    }

    pub fn start(self) -> Result<JoinHandle<()>> {
        let handle = try!(thread::Builder::new()
            .name("http-gateway".to_string())
            .spawn(move || {
                self.0
                    .http(*gconfig().http_listen_addr())
                    .expect("unable to start http-gateway thread");
            }));
        Ok(handle)
    }
}

fn butterfly(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    Ok(Response::with((status::Ok, json::encode(&state.butterfly).unwrap())))
}

fn census(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let data = state.census_list.read().unwrap();
    Ok(Response::with((status::Ok, json::encode(&*data).unwrap())))
}

fn config(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let service_group =
        ServiceGroup::new(req.extensions.get::<Router>().unwrap().find("svc").unwrap(),
                          req.extensions.get::<Router>().unwrap().find("group").unwrap(),
                          req.extensions.get::<Router>().unwrap().find("org").map(|v| v.into()));
    let services = state.services.read().unwrap();
    match services.iter().find(|s| s.service_group == service_group) {
        Some(service) => {
            match service.package.last_config() {
                Ok(config) => Ok(Response::with((status::Ok, config))),
                Err(err) => {
                    error!("Couldn't retrieve last config, err={:?}", err);
                    Ok(Response::with(status::ServiceUnavailable))
                }
            }
        }
        None => Ok(Response::with(status::NotFound)),
    }
}

fn health(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let service_group =
        ServiceGroup::new(req.extensions.get::<Router>().unwrap().find("svc").unwrap(),
                          req.extensions.get::<Router>().unwrap().find("group").unwrap(),
                          req.extensions.get::<Router>().unwrap().find("org").map(|v| v.into()));
    let services = state.services.read().unwrap();
    match services.iter().find(|s| s.service_group == service_group) {
        Some(service) => {
            match service.health_check() {
                Ok(result) => Ok(result.into()),
                Err(err) => {
                    error!("Health Check failed, err={:?}", err);
                    Ok(Response::with(status::InternalServerError))
                }
            }
        }
        None => Ok(Response::with(status::NotFound)),
    }
}

fn services(req: &mut Request) -> IronResult<Response> {
    let state = req.get::<persistent::Read<ManagerState>>().unwrap();
    let data = state.services.read().unwrap();
    Ok(Response::with((status::Ok, json::encode(&*data).unwrap())))
}

impl Into<Response> for health_check::CheckResult {
    fn into(self) -> Response {
        let status: status::Status = self.status.into();
        Response::with((status, self.output))
    }
}

impl Into<status::Status> for health_check::Status {
    fn into(self) -> status::Status {
        match self {
            health_check::Status::Ok |
            health_check::Status::Warning => status::Ok,
            health_check::Status::Critical => status::ServiceUnavailable,
            health_check::Status::Unknown => status::InternalServerError,
        }
    }
}
