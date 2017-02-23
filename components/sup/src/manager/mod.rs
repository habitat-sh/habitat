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

pub mod census;
pub mod service;
pub mod signals;
pub mod service_updater;

use std::net::SocketAddr;
use std::thread;
use std::sync::{Arc, RwLock};
use std::time::Duration;

use butterfly;
use butterfly::member::Member;
use butterfly::trace::Trace;
use butterfly::server::timing::Timing;
use hcore::crypto::{default_cache_key_path, SymKey};
use time::{SteadyTime, Duration as TimeDuration};
use toml;

pub use manager::service::{Service, ServiceConfig, ServiceSpec, UpdateStrategy, Topology};
use self::service_updater::ServiceUpdater;
use error::Result;
use config::GossipListenAddr;
use manager::census::{CensusUpdate, CensusList, CensusEntry};
use manager::signals::SignalEvent;
use http_gateway;

static LOGKEY: &'static str = "MR";

#[derive(Clone)]
pub struct State {
    pub butterfly: butterfly::Server,
    pub census_list: Arc<RwLock<CensusList>>,
    pub services: Arc<RwLock<Vec<Service>>>,
}

impl State {
    pub fn new(butterfly: butterfly::Server) -> Self {
        State {
            butterfly: butterfly,
            census_list: Arc::new(RwLock::new(CensusList::new())),
            services: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

#[derive(Default)]
pub struct ManagerConfig {
    pub gossip_listen: GossipListenAddr,
    pub http_listen: http_gateway::ListenAddr,
    pub gossip_peers: Vec<SocketAddr>,
    pub gossip_permanent: bool,
    pub ring: Option<String>,
}

pub struct Manager {
    state: State,
    updater: ServiceUpdater,
    gossip_listen: GossipListenAddr,
    http_listen: http_gateway::ListenAddr,
}

impl Manager {
    pub fn new(cfg: ManagerConfig) -> Result<Manager> {
        let mut member = Member::new();
        member.set_persistent(cfg.gossip_permanent);
        member.set_swim_port(cfg.gossip_listen.port() as i32);
        member.set_gossip_port(cfg.gossip_listen.port() as i32);

        let ring_key = match cfg.ring {
            Some(ref ring_with_revision) => {
                outputln!("Joining ring {}", ring_with_revision);
                Some(try!(SymKey::get_pair_for(&ring_with_revision, &default_cache_key_path(None))))
            }
            None => None,
        };

        let server = try!(butterfly::Server::new(&cfg.gossip_listen,
                                                 &cfg.gossip_listen,
                                                 member,
                                                 Trace::default(),
                                                 ring_key,
                                                 None));
        outputln!("Butterfly Member ID {}", server.member_id());
        for peer_addr in &cfg.gossip_peers {
            let mut peer = Member::new();
            peer.set_address(format!("{}", peer_addr.ip()));
            peer.set_swim_port(peer_addr.port() as i32);
            peer.set_gossip_port(peer_addr.port() as i32);
            server.member_list.add_initial_member(peer);
        }
        Ok(Manager {
            updater: ServiceUpdater::new(server.clone()),
            state: State::new(server),
            gossip_listen: cfg.gossip_listen,
            http_listen: cfg.http_listen,
        })
    }

    pub fn add_service(&mut self, spec: ServiceSpec) -> Result<()> {
        let service = Service::load(spec, &self.gossip_listen, &self.http_listen)?;
        service.add()?;
        self.state.butterfly.insert_service(service.to_rumor(self.state.butterfly.member_id()));
        if service.topology == Topology::Leader {
            // Note - eventually, we need to deal with suitability here. The original implementation
            // didn't have this working either.
            self.state.butterfly.start_election(service.service_group.clone(), 0, 0);
        }
        self.updater.add(&service);
        self.state.services.write().expect("Services lock is poisoned!").push(service);
        Ok(())
    }

    pub fn remove_service(&self, service: &Service) -> Result<()> {
        // TODO FN: there is more to removing a service, more to follow
        service.remove()?;
        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        signals::init();

        outputln!("Starting butterfly on {}",
                  self.state.butterfly.gossip_addr());
        try!(self.state.butterfly.start(Timing::default()));
        debug!("butterfly server started");
        outputln!("Starting http-gateway on {}", self.http_listen);
        try!(http_gateway::Server::new(self.state.clone(), self.http_listen.clone()).start());
        debug!("http-gateway server started");

        let mut last_census_update = CensusUpdate::default();

        loop {
            let next_check = SteadyTime::now() + TimeDuration::milliseconds(1000);
            if self.check_for_incoming_signals() {
                let mut services = self.state
                    .services
                    .write()
                    .expect("Services lock is poisend!");
                for service in services.drain(..) {
                    self.remove_service(&service)?;
                }
                outputln!("Habitat thanks you - shutting down!");
                return Ok(());
            }
            self.check_for_updated_packages(&mut last_census_update);
            self.restart_elections();
            let (census_updated, ncu) = self.build_census(&last_census_update);
            if census_updated {
                last_census_update = ncu;
            }
            for service in self.state
                .services
                .write()
                .expect("Services lock is poisoned!")
                .iter_mut() {
                service.tick(&self.state.butterfly,
                             &self.state
                                 .census_list
                                 .read()
                                 .expect("Census list lock is poisoned!"),
                             census_updated,
                             &mut last_census_update)
            }
            let time_to_wait = (next_check - SteadyTime::now()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            }
        }
    }

    // Try and build the census from the gossip data, updating the last_census_update with
    // the resulting checkpoints. The census is our representation of the data produced
    // by Butterfly.
    fn build_census(&mut self, last_update: &CensusUpdate) -> (bool, CensusUpdate) {
        let update = CensusUpdate::new(&self.state.butterfly);
        if update != *last_update {
            // JW TODO: We should re-use the already allocated census list and entries instead of
            // recreating entirely new structures. We can, and should, only modify structures which
            // have had their incarnation updated.
            let mut cl = CensusList::new();
            debug!("Updating census from butterfly data");
            self.state
                .butterfly
                .service_store
                .with_keys(|(_group, rumors)| for (_member_id, service) in rumors.iter() {
                    let mut ce = CensusEntry::default();
                    ce.populate_from_service(service);
                    cl.insert(String::from(self.state.butterfly.member_id()), ce);
                });
            self.state.butterfly.election_store.with_keys(|(_service_group, rumors)| {
                // We know you have an election, and this is the only key in the hash
                let election = rumors.get("election").unwrap();
                cl.populate_from_election(election);
            });
            self.state.butterfly.update_store.with_keys(|(_service_group, rumors)| {
                // We know you have an election, and this is the only key in the hash
                let election = rumors.get("election").unwrap();
                cl.populate_from_update_election(election);
            });
            self.state.butterfly.member_list.with_members(|member| {
                cl.populate_from_member(member);
                if let Some(health) = self.state.butterfly.member_list.health_of(member) {
                    cl.populate_from_health(member, health);
                }
            });
            *self.state.census_list.write().expect("Census list lock is poisoned!") = cl;
            return (true, update);
        }
        (false, update)
    }

    // Takes signals passed to `hab-sup` and either shuts down all the services, or
    // passes the signals through. This functionality is totally going to need a refactor
    // when we get all the way to a single-sup-per-kernel model, since passing all random
    // signals through to all services is most certainly not what you want.
    //
    // This function returns true if we are supposed to shut the system down, false if we
    // can keep going.
    fn check_for_incoming_signals(&mut self) -> bool {
        match signals::check_for_signal() {
            Some(SignalEvent::Shutdown) => {
                for service in self.state
                    .services
                    .write()
                    .expect("Services lock is poisoned!")
                    .iter_mut() {
                    outputln!("Shutting down {}", service);
                    service.down()
                        .unwrap_or_else(|err| outputln!("Failed to shutdown {}: {}", service, err));
                }
                true
            }
            Some(SignalEvent::Passthrough(signal_code)) => {
                for service in self.state
                    .services
                    .read()
                    .expect("Services lock is poisoned!")
                    .iter() {
                    outputln!("Forwarding signal {} to {}", signal_code, service);
                    if let Err(e) = service.send_signal(signal_code) {
                        outputln!("Failed to send signal {} to {}: {}",
                                  signal_code,
                                  service,
                                  e);
                    }
                }
                false
            }
            None => false,
        }
    }

    /// Walk each service and check if it has an updated package installed via the Update Strategy.
    /// This updates the Service to point to the new service struct, and then marks it for
    /// restarting.
    ///
    /// The run loop's last updated census is a required parameter on this function to inform the
    /// main loop that we, ourselves, updated the service counter when we updated ourselves.
    fn check_for_updated_packages(&mut self, last_update: &mut CensusUpdate) {
        let member_id = {
            self.state.butterfly.member_id().to_string()
        };
        let census_list = self.state.census_list.read().expect("Census list lock is poisoned!");
        for service in self.state.services.write().expect("Services lock is poisoned!").iter_mut() {
            if self.updater.check_for_updated_package(service, &census_list) {
                let mut rumor = {
                    let list = self.state
                        .butterfly
                        .service_store
                        .list
                        .read()
                        .expect("Rumor store lock poisoned");
                    list.get(&*service.service_group)
                        .and_then(|r| r.get(&member_id))
                        .unwrap()
                        .clone()
                };
                let incarnation = rumor.get_incarnation() + 1;
                rumor.set_pkg(service.package().to_string());
                rumor.set_incarnation(incarnation);
                service.populate(&census_list);
                // TODO FN: the updated toml API returns a `Result` when serializing--we should
                // handle this and not potentially panic
                match service.config.to_exported() {
                    Ok(cfg) => {
                        *rumor.mut_cfg() = toml::ser::to_vec(&cfg)
                            .expect("Can't serialize to TOML bytes")
                    }
                    Err(err) => warn!("Error loading service config after update, err={}", err),
                }
                self.state.butterfly.insert_service(rumor);
                last_update.service_counter += 1;
            }
        }
    }

    /// Check if any elections need restarting.
    fn restart_elections(&mut self) {
        self.state.butterfly.restart_elections();
    }
}
