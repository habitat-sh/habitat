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

use ansi_term::Colour::White;

use pkg::Package;
use discovery::etcd;
use hyper::status::StatusCode;
use util;
use toml;

#[derive(Debug, Clone, Copy)]
enum State {
    Init,
    CreateDataset,
    RestoreDataset,
    DetermineViability,
    BecomeLeader,
    BecomeFollower,
    Leader,
    Follower,
}

#[derive(Debug, Clone)]
pub struct Leader {
    state: State,
    package: Package,
}

impl Leader {
    pub fn new(package: Package) -> Leader {
        Leader{
            state: State::Init,
            package: package,
        }
    }

    pub fn next(&mut self) {
        match self.state {
            State::Init => self.state_init(),
            State::CreateDataset => self.state_create_dataset(),
            State::RestoreDataset => self.state_restore_dataset(),
            State::DetermineViability => self.state_determine_viability(),
            State::BecomeLeader => self.state_become_leader(),
            State::BecomeFollower => self.state_become_follower(),
            State::Leader => self.state_leader(),
            State::Follower => self.state_follower()
        }
    }

    fn status_value(&mut self) -> String {
        let hostname = util::sys::hostname().unwrap_or(String::from("unknown"));
        let ip = util::sys::ip().unwrap_or(String::from("127.0.0.1"));
        let port = self.package.exposes().pop().unwrap_or(String::from("0"));
        format!("hostname = '{}'\nip = '{}'\nport = '{}'", hostname, ip, port)
    }

    fn preamble(&mut self) -> String {
        format!("{}({} {:?})", self.package.name, White.bold().paint("T"), self.state)
    }

    fn state_init(&mut self) {
        println!("   {}: Attempting to initialize the data set", self.preamble());
        let key = format!("{}/topology/leader/init", self.package.name);
        let ip = util::sys::ip().unwrap();
        let port = self.package.exposes().pop().unwrap_or(String::from("0"));
        let status = format!("ip = '{}'\nport = '{}'", ip, port);
        let (statuscode, response) =
            etcd::set(&key, &[("value", &status), ("prevExist", "false")]).unwrap();
        debug!("Response is {:?} {}", statuscode, response);
        match statuscode {
            StatusCode::Created => {
                println!("   {}: Dataset has never been initialized, and we won the race", self.preamble());
                self.state = State::CreateDataset
            },
            StatusCode::PreconditionFailed => {
                // If it aready exists, see if we are the initializer
                let (statuscode, response) =
                    etcd::set(&key, &[("value", &status), ("prevValue", &status)]).unwrap();
                match statuscode {
                    StatusCode::Ok => {
                        println!("   {}: I am the previous initializer, seeing if I can be the leader", self.preamble());
                        self.state = State::DetermineViability
                    },
                    _ => {
                        println!("   {}: Initialization successful on a different instance - becoming a follower", self.preamble());
                        self.state = State::RestoreDataset
                    }
                }
            },
            _ => unreachable!(),
        };
    }

    fn state_create_dataset(&mut self) {
        println!("   {}: Creating the initial dataset", self.preamble());
        self.state = State::BecomeLeader
    }

    fn state_restore_dataset(&mut self) {
        println!("   {}: Determining if we should restore a new dataset", self.preamble());
        self.state = State::BecomeFollower
    }

    fn state_determine_viability(&mut self) {
        println!("   {}: Determining viability as a leader", self.preamble());
        self.state = State::BecomeLeader
    }

    fn state_become_leader(&mut self) {
        println!("   {}: Becoming the leader", self.preamble());
        println!("   {}: Forming the government", self.preamble());
        let govkey = format!("{}/topology/leader/government", self.package.name);
        let (gov_statuscode, gov_response) =
            etcd::set(&govkey, &[("dir", "true"), ("ttl", "30")]).unwrap();
        match gov_statuscode {
            StatusCode::Created => {
                println!("   {}: We have established a new government", self.preamble());
            },
            StatusCode::Ok => {
                println!("   {}: We have updated an existing government", self.preamble());
            },
            StatusCode::Forbidden => {
                println!("   {}: The government already exists", self.preamble());
            },
            e => {
                println!("   {}: I got a {:?}", self.preamble(), e);
                unreachable!()
            },
        };

        let key = format!("{}/topology/leader/government/leader", self.package.name);
        let (statuscode, response) =
            etcd::set(&key, &[("value", &self.status_value()), ("prevExist", "false")]).unwrap();
        debug!("Response is {:?} {}", statuscode, response);
        match statuscode {
            StatusCode::Created => {
                println!("   {}: We are the leader! On to glory!", self.preamble());
            },
            StatusCode::Ok => {
                println!("   {}: We are the leader! The future is amazing!", self.preamble());
            },
            StatusCode::PreconditionFailed => {
                println!("   {}: There is already a leader - checking to see if it's me", self.preamble());
                let (is_it_me_statuscode, is_it_me_response) =
                    etcd::set(&key, &[("value", &self.status_value()), ("prevValue", &self.status_value())]).unwrap();
                debug!("Response is {:?} {}", is_it_me_statuscode, is_it_me_response);
                match is_it_me_statuscode {
                    StatusCode::Created => {
                        println!("   {}: It was me! Sweet!", self.preamble());
                    },
                    StatusCode::Ok => {
                        println!("   {}: It was me! I'm the best!", self.preamble());
                    },
                    e => {
                        println!("   {}: I got a {:?}", self.preamble(), e);
                        println!("   {}: That means I am not the leader :( - becoming a follower", self.preamble());
                        self.state = State::BecomeFollower;
                        return
                    }
                }
            },
            e => {
                println!("   {}: I got a {:?}", self.preamble(), e);
                unreachable!()
            },
        };
        self.state = State::Leader
    }

    fn state_become_follower(&mut self) {
        println!("   {}: Becoming a follower", self.preamble());
        println!("   {}: Creating my entry in the the census", self.preamble());
        let hostname = util::sys::hostname().unwrap();
        let census_key = format!("{}/topology/leader/census/{}", self.package.name, hostname);
        let (census_statuscode, census_response) =
            etcd::set(&census_key, &[("dir", "true"), ("ttl", "30")]).unwrap();
        match census_statuscode {
            StatusCode::Created => {
                println!("   {}: We have added ourselves to the census", self.preamble());
            },
            StatusCode::Ok => {
                println!("   {}: We have updated an existing census entry", self.preamble());
            },
            StatusCode::Forbidden => {
                println!("   {}: Our census entry already exists", self.preamble());
            },
            e => {
                println!("   {}: I got a {:?}", self.preamble(), e);
                unreachable!()
            },
        };

        let key = format!("{}/topology/leader/census/{}/data", self.package.name, hostname);
        let (statuscode, response) =
            etcd::set(&key, &[("value", &self.status_value())]).unwrap();
        debug!("Response is {:?} {}", statuscode, response);
        match statuscode {
            StatusCode::Created => {
                println!("   {}: We are a full fledged citizen!", self.preamble());
            },
            StatusCode::Ok => {
                println!("   {}: We are a full fledged citizen! We have rights!", self.preamble());
            },
            e => {
                println!("   {}: I got a {:?}", self.preamble(), e);
                unreachable!()
            },
        };
        self.state = State::Follower
    }

    fn state_leader(&mut self) {
        let key = format!("{}/topology/leader/government", self.package.name);
        let (statuscode, response) =
            etcd::set(&key, &[("ttl", "30"), ("dir", "true"), ("prevExist", "true")]).unwrap();
        debug!("Response is {:?} {}", statuscode, response);
        match statuscode {
            StatusCode::Created | StatusCode::Ok => debug!("We are still the leader, and have updated the ttl"),
            e => {
                println!("   {}: I got a {:?} trying to update my leader ttl - moving into an election", self.preamble(), e);
                self.state = State::DetermineViability;
                return
            },
        };
        self.state = State::Leader;
    }

    fn state_follower(&mut self) {
        let hostname = util::sys::hostname().unwrap();
        let key = format!("{}/topology/leader/census/{}", self.package.name, hostname);
        let (statuscode, response) =
            etcd::set(&key, &[("ttl", "30"), ("dir", "true"), ("prevExist", "true")]).unwrap();
        debug!("Response is {:?} {}", statuscode, response);
        match statuscode {
            StatusCode::Created | StatusCode::Ok => debug!("We are still a follower, and have updated the ttl"),
            e => {
                println!("   {}: I got a {:?} trying to update my follower ttl - moving into an election", self.preamble(), e);
                self.state = State::DetermineViability;
                return
            },
        };
        self.state = State::Follower;
    }
}
