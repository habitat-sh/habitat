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
use butterfly::rumor::service::Service as ServiceRumor;
use butterfly::server::timing::Timing;
use hcore::crypto::{default_cache_key_path, SymKey};
use hcore::service::ServiceGroup;
use time::{SteadyTime, Duration as TimeDuration};

use self::service_updater::ServiceUpdater;
use error::{Error, Result};
use config::{gconfig, UpdateStrategy, Topology};
use manager::service::Service;
use manager::census::{CensusUpdate, CensusList, CensusEntry};
use manager::signals::SignalEvent;
use package::Package;
use http_gateway;
use util;

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

pub struct Manager {
    state: State,
    updater: ServiceUpdater,
}

impl Manager {
    pub fn new() -> Result<Manager> {
        let gossip_addr: SocketAddr = try!(gconfig().gossip_listen().parse());

        let mut member = Member::new();
        member.set_persistent(gconfig().gossip_permanent());
        member.set_swim_port(gossip_addr.port() as i32);
        member.set_gossip_port(gossip_addr.port() as i32);

        let ring_key = match gconfig().ring() {
            &Some(ref ring_with_revision) => {
                outputln!("Joining ring {}", ring_with_revision);
                Some(try!(SymKey::get_pair_for(&ring_with_revision, &default_cache_key_path(None))))
            }
            &None => None,
        };

        let server = try!(butterfly::Server::new(gconfig().gossip_listen(),
                                                 gconfig().gossip_listen(),
                                                 member,
                                                 Trace::default(),
                                                 ring_key,
                                                 None));
        outputln!("Butterfly Member ID {}", server.member_id());
        for peer_addr in gconfig().gossip_peer() {
            let addr: SocketAddr = peer_addr.parse().unwrap();
            let mut peer = Member::new();
            peer.set_address(format!("{}", addr.ip()));
            peer.set_swim_port(addr.port() as i32);
            peer.set_gossip_port(addr.port() as i32);
            server.member_list.add_initial_member(peer);
        }
        Ok(Manager {
            state: State::new(server),
            updater: ServiceUpdater::default(),
        })
    }

    pub fn add_service(&mut self,
                       package: Package,
                       topology: Topology,
                       update_strategy: UpdateStrategy)
                       -> Result<()> {
        let service_group = ServiceGroup::new(package.name.clone(),
                                              gconfig().group().to_string(),
                                              gconfig().organization().clone());
        let hostname = try!(util::sys::hostname());
        let ip = try!(util::sys::ip());
        // TODO: We should do this much earlier, to confirm that the ports we expose are not
        //       bullshit.
        let mut exposes = Vec::new();
        for port in package.exposes().into_iter() {
            let port_num = try!(port.parse::<u32>().map_err(|e| sup_error!(Error::InvalidPort(e))));
            exposes.push(port_num);
        }
        let service_rumor = ServiceRumor::new(self.state.butterfly.member_id(),
                                              service_group.clone(),
                                              hostname,
                                              format!("{}", ip),
                                              exposes);
        self.state.butterfly.insert_service(service_rumor);

        if topology == Topology::Leader || topology == Topology::Initializer {
            // Note - eventually, we need to deal with suitability here. The original implementation
            // didn't have this working either.
            self.state.butterfly.start_election(service_group.clone(), 0, 0);
        }

        let service = try!(Service::new(service_group, package, topology, update_strategy));
        self.updater.add(&service);
        self.state.services.write().expect("Services lock is poisoned!").push(service);
        Ok(())
    }

    pub fn build_census(&mut self, last_update: &CensusUpdate) -> Result<(bool, CensusUpdate)> {
        let update = CensusUpdate::new(self.state.butterfly.service_store.get_update_counter(),
                                       self.state.butterfly.election_store.get_update_counter(),
                                       self.state.butterfly.member_list.get_update_counter());

        if &update != last_update {
            let mut cl = CensusList::new();
            debug!("Updating census from butterfly data");
            self.state.butterfly.service_store.with_keys(|(_service_group, rumors)| {
                for (_member_id, service) in rumors.iter() {
                    let mut ce = CensusEntry::default();
                    ce.populate_from_service(service);
                    cl.insert(String::from(self.state.butterfly.member_id()), ce);
                }
            });
            self.state.butterfly.election_store.with_keys(|(_service_group, rumors)| {
                // We know you have an election, and this is the only key in the hash
                let election = rumors.get("election").unwrap();
                cl.populate_from_election(election);
            });
            self.state.butterfly.member_list.with_members(|member| {
                cl.populate_from_member(member);
            });
            *self.state.census_list.write().expect("Census list lock is poisoned!") = cl;
            return Ok((true, update));
        }
        Ok((false, update))
    }

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
    pub fn check_for_updated_packages(&mut self) {
        for service in self.state.services.write().expect("Services lock is poisoned!").iter_mut() {
            self.updater.check_for_updated_package(service);
        }
    }

    //  * Start butterfly
    //  Loop {
    //    * Check for incoming signals; forward them; shut down if neccessary
    //    * Check if each service needs its package updated
    //      * Update the package
    //    * Check if the Census needs building from Butterfly, or the package changed
    //      * Loop the services, and reconfigure the service from the Census
    //    * Reap any dead children
    //    * Start or restart the services
    //  }
    //
    pub fn run(&mut self) -> Result<()> {
        // Set the global signal handlers
        signals::init();

        outputln!("Starting butterfly on {}", gconfig().gossip_listen());
        try!(self.state.butterfly.start(Timing::default()));
        debug!("butterfly server started");
        outputln!("Starting http-gateway on {}", gconfig().http_listen_addr());
        try!(http_gateway::Server::new(self.state.clone()).start());
        debug!("http-gateway server started");

        // Watch for updates
        let mut last_census_update = CensusUpdate::new(0, 0, 0);

        'services: loop {
            let next_check = SteadyTime::now() + TimeDuration::milliseconds(1000);
            // Check for incoming signals.
            //
            // This takes signals passed to `hab-sup` and either shuts down all the services, or
            // passes the signals through. This functionality is totally going to need a refactor
            // when we get all the way to a single-sup-per-kernel model, since passing all random
            // signals through to all services is most certainly not what you want.
            //
            // This function returns true if we are supposed to shut the system down, false if we
            // can keep going.
            if self.check_for_incoming_signals() {
                outputln!("Habitat thanks you - shutting down!");
                return Ok(());
            }

            // Check for updated packages; this updates the Service to point to the new service
            // struct, and then marks it for restarting.
            self.check_for_updated_packages();

            // Check if any elections need restarting.
            self.state.butterfly.restart_elections();

            // Try and build the census from the gossip data, updating the last_census_update with
            // the resulting checkpoints. The census is our representation of the data produced
            // by Butterfly.
            let (census_updated, ncu) = try!(self.build_census(&last_census_update));
            if census_updated {
                last_census_update = ncu;
            }

            for service in self.state
                .services
                .write()
                .expect("Services lock is poisoned!")
                .iter_mut() {

                // Write out any files we received via butterfly
                let mut service_files_updated = false;
                for (incarnation, filename, body) in self.state
                    .butterfly
                    .service_files_for(&service.service_group_str(),
                                       &service.current_service_files)
                    .into_iter() {
                    let result = service.write_butterfly_service_file(filename, incarnation, body);
                    if service_files_updated == false && result == true {
                        service_files_updated = true;
                    }
                }
                if service_files_updated {
                    service.file_updated();
                }

                // Write out any service configuration we received via butterfly
                let mut service_config_updated = false;
                if let Some((incarnation, config)) = self.state
                    .butterfly
                    .service_config_for(&service.service_group_str(),
                                        service.service_config_incarnation) {
                    service_config_updated = service.write_butterfly_service_config(config);
                    service.service_config_incarnation = Some(incarnation);
                }

                // Reconfigure if neccessary
                if census_updated || service_config_updated {
                    service.reconfigure(&self.state
                        .census_list
                        .read()
                        .expect("Census list lock is poisoned!"));
                }

                // If this service has not been initialized, do so now.
                service.initialize();

                // Reap dead children
                let _ = service.check_process();

                // Start or restart the service
                if service.initialized && (service.needs_restart || service.is_down()) {
                    match service.restart(&self.state
                        .census_list
                        .read()
                        .expect("Census list lock is poisoned!")) {
                        Ok(()) => {}
                        Err(e) => outputln!("Cannot restart service: {}", e),
                    }
                }
            }

            let time_to_wait = next_check - SteadyTime::now();
            thread::sleep(Duration::from_millis(time_to_wait.num_milliseconds() as u64));
        }
    }
}
