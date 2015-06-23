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

use std::thread;
use hyper::status::StatusCode;
use toml;

use topology::{self, State, Worker};
use state_machine::StateMachine;
use error::{BldrResult, BldrError};
use util;
use pkg::{self, Package};
use discovery::etcd;
use topology::standalone;

pub fn run(package: Package) -> BldrResult<()> {
    let mut worker = Worker::new(package, String::from("leader"));
    let mut sm: StateMachine<State, Worker, BldrError> = StateMachine::new(State::Init);
    sm.add_dispatch(State::Init, state_init);
    sm.add_dispatch(State::CreateDataset, state_create_dataset);
    sm.add_dispatch(State::RestoreDataset, state_restore_dataset);
    sm.add_dispatch(State::DetermineViability, state_determine_viability);
    sm.add_dispatch(State::BecomeLeader, state_become_leader);
    sm.add_dispatch(State::BecomeFollower, state_become_follower);
    sm.add_dispatch(State::Leader, state_leader);
    sm.add_dispatch(State::Follower, state_follower);
    topology::set_signal_handlers();
    topology::run_internal(&mut sm, &mut worker)
}

fn status_value(status_type: &str, worker: &mut Worker) -> String {
    let hostname = util::sys::hostname().unwrap_or(String::from("unknown"));
    let ip = util::sys::ip().unwrap_or(String::from("127.0.0.1"));
    let port = worker.package.exposes().pop().unwrap_or(String::from("0"));
    format!("{}\nhostname = '{}'\nip = '{}'\nport = '{}'", status_type, hostname, ip, port)
}

fn state_init(worker: &mut Worker) -> Result<(State, u32), BldrError> {
    try!(standalone::state_init(worker));
    println!("   {}: Attempting to initialize the data set", worker.preamble());
    let key = format!("{}/topology/leader/init", worker.package.name);
    let ip = util::sys::ip().unwrap();
    let port = worker.package.exposes().pop().unwrap_or(String::from("0"));
    let status = format!("ip = '{}'\nport = '{}'", ip, port);
    let (statuscode, response) =
        etcd::set(&key, &[("value", &status), ("prevExist", "false")]).unwrap();
    debug!("Response is {:?} {}", statuscode, response);
    match statuscode {
        StatusCode::Created => {
            println!("   {}: Dataset has never been initialized, and we won the race", worker.preamble());
            return Ok((State::CreateDataset, 0))
        },
        StatusCode::PreconditionFailed => {
            // If it aready exists, see if we are the initializer
            let (statuscode, response) =
                etcd::set(&key, &[("value", &status), ("prevValue", &status)]).unwrap();
            match statuscode {
                StatusCode::Ok => {
                    println!("   {}: I am the previous initializer, seeing if I can be the leader", worker.preamble());
                    return Ok((State::DetermineViability, 0))
                },
                _ => {
                    println!("   {}: Initialization successful on a different instance - becoming a follower", worker.preamble());
                    return Ok((State::RestoreDataset, 0))
                }
            }
        },
        _ => unreachable!(),
    };
}

fn state_create_dataset(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Creating the initial dataset", worker.preamble());
    Ok((State::BecomeLeader, 0))
}

fn state_restore_dataset(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Determining if we should restore a new dataset", worker.preamble());
    Ok((State::BecomeFollower, 0))
}

fn state_determine_viability(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Determining viability as a leader", worker.preamble());
    Ok((State::BecomeLeader, 0))
}

fn state_become_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Becoming the leader", worker.preamble());
    println!("   {}: Forming the government", worker.preamble());
    let govkey = format!("{}/topology/leader/government", worker.package.name);
    let (gov_statuscode, gov_response) =
        etcd::set(&govkey, &[("dir", "true"), ("ttl", "30")]).unwrap();
    match gov_statuscode {
        StatusCode::Created => {
            println!("   {}: We have established a new government", worker.preamble());
        },
        StatusCode::Ok => {
            println!("   {}: We have updated an existing government", worker.preamble());
        },
        StatusCode::Forbidden => {
            println!("   {}: The government already exists", worker.preamble());
        },
        e => {
            println!("   {}: I got a {:?}", worker.preamble(), e);
            unreachable!()
        },
    };

    let key = format!("{}/topology/leader/government/leader", worker.package.name);
    let (statuscode, response) =
        etcd::set(&key, &[("value", &status_value("[topology.leader]", worker)), ("prevExist", "false")]).unwrap();
    debug!("Response is {:?} {}", statuscode, response);
    match statuscode {
        StatusCode::Created => {
            println!("   {}: We are the leader! On to glory!", worker.preamble());
        },
        StatusCode::Ok => {
            println!("   {}: We are the leader! The future is amazing!", worker.preamble());
        },
        StatusCode::PreconditionFailed => {
            println!("   {}: There is already a leader - checking to see if it's me", worker.preamble());
            let (is_it_me_statuscode, is_it_me_response) =
                etcd::set(&key, &[("value", &status_value("[topology.leader]", worker)), ("prevValue", &status_value("[topology.leader]", worker))]).unwrap();
            debug!("Response is {:?} {}", is_it_me_statuscode, is_it_me_response);
            match is_it_me_statuscode {
                StatusCode::Created => {
                    println!("   {}: It was me! Sweet!", worker.preamble());
                },
                StatusCode::Ok => {
                    println!("   {}: It was me! I'm the best!", worker.preamble());
                },
                e => {
                    println!("   {}: I got a {:?}", worker.preamble(), e);
                    println!("   {}: That means I am not the leader :( - becoming a follower", worker.preamble());
                    return Ok((State::BecomeFollower, 0))
                }
            }
        },
        e => {
            println!("   {}: I got a {:?}", worker.preamble(), e);
            unreachable!()
        },
    };
    Ok((State::Leader, 0))
}

fn state_become_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Becoming a follower", worker.preamble());
    println!("   {}: Creating my entry in the the census", worker.preamble());
    let hostname = util::sys::hostname().unwrap();
    let census_key = format!("{}/topology/leader/census/{}", worker.package.name, hostname);
    let (census_statuscode, census_response) =
        etcd::set(&census_key, &[("dir", "true"), ("ttl", "30")]).unwrap();
    match census_statuscode {
        StatusCode::Created => {
            println!("   {}: We have added ourselves to the census", worker.preamble());
        },
        StatusCode::Ok => {
            println!("   {}: We have updated an existing census entry", worker.preamble());
        },
        StatusCode::Forbidden => {
            println!("   {}: Our census entry already exists", worker.preamble());
        },
        e => {
            println!("   {}: I got a {:?}", worker.preamble(), e);
            unreachable!()
        },
    };

    let key = format!("{}/topology/leader/census/{}/data", worker.package.name, hostname);
    let (statuscode, response) =
        etcd::set(&key, &[("value", &status_value("[[topology.follower]]", worker))]).unwrap();
    debug!("Response is {:?} {}", statuscode, response);
    match statuscode {
        StatusCode::Created => {
            println!("   {}: We are a full fledged citizen!", worker.preamble());
        },
        StatusCode::Ok => {
            println!("   {}: We are a full fledged citizen! We have rights!", worker.preamble());
        },
        e => {
            println!("   {}: I got a {:?}", worker.preamble(), e);
            unreachable!()
        },
    };
    Ok((State::Follower, 0))
}

fn state_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let key = format!("{}/topology/leader/government", worker.package.name);
    let (statuscode, response) =
        etcd::set(&key, &[("ttl", "30"), ("dir", "true"), ("prevExist", "true")]).unwrap();
    debug!("Response is {:?} {}", statuscode, response);
    match statuscode {
        StatusCode::Created | StatusCode::Ok => debug!("We are still the leader, and have updated the ttl"),
        e => {
            println!("   {}: I got a {:?} trying to update my leader ttl - moving into an election", worker.preamble(), e);
            return Ok((State::DetermineViability, 0))
        },
    };
    if worker.configuration_thread.is_none() {
        try!(standalone::state_configure(worker));
    }
    if worker.supervisor_thread.is_none() {
        try!(standalone::state_starting(worker));
    }
    Ok((State::Leader, 20000))
}

fn state_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let hostname = util::sys::hostname().unwrap();
    let key = format!("{}/topology/leader/census/{}", worker.package.name, hostname);
    let (statuscode, response) =
        etcd::set(&key, &[("ttl", "30"), ("dir", "true"), ("prevExist", "true")]).unwrap();
    debug!("Response is {:?} {}", statuscode, response);
    match statuscode {
        StatusCode::Created | StatusCode::Ok => debug!("We are still a follower, and have updated the ttl"),
        e => {
            println!("   {}: I got a {:?} trying to update my follower ttl - moving into an election", worker.preamble(), e);
            return Ok((State::DetermineViability, 0))
        },
    };
    try!(worker.package.write_discovery_data("topology/leader/government/leader", "101_leader.toml", false));
    if worker.configuration_thread.is_none() {
        try!(standalone::state_configure(worker));
        let package = try!(pkg::latest(&worker.package.name));
        let config_join = thread::spawn(move || -> BldrResult<()> {
             loop {
                 try!(package.write_discovery_data("topology/leader/government/leader", "101_leader.toml", true));
                 println!("   {}({}): Waiting 30 seconds before reconnecting", package.name, White.bold().paint("D"));
                 thread::sleep_ms(30000);
             }
        });
    }
    if worker.supervisor_thread.is_none() {
        try!(standalone::state_starting(worker));
    }
    Ok((State::Follower, 20000))
}
