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

use std::thread;
use hyper::status::StatusCode;

use topology::{self, State, Worker};
use state_machine::StateMachine;
use error::{BldrResult, BldrError};
use util;
use pkg::Package;
use discovery::{etcd, DiscoveryWatcher, DiscoveryWriter, DiscoveryResponse, DiscoveryWriteResponse};
use topology::standalone;
use config::Config;

pub fn run(package: Package, config: &Config) -> BldrResult<()> {
    let mut worker = Worker::new(package, String::from("leader"), config);
    let mut sm: StateMachine<State, Worker, BldrError> = StateMachine::new(State::Init);
    sm.add_dispatch(State::Init, state_init);
    sm.add_dispatch(State::CreateDataset, state_create_dataset);
    sm.add_dispatch(State::RestoreDataset, state_restore_dataset);
    sm.add_dispatch(State::DetermineViability, state_determine_viability);
    sm.add_dispatch(State::BecomeLeader, state_become_leader);
    sm.add_dispatch(State::BecomeFollower, state_become_follower);
    sm.add_dispatch(State::Leader, state_leader);
    sm.add_dispatch(State::Follower, state_follower);
    topology::run_internal(&mut sm, &mut worker)
}

fn status_value(status_type: &str, worker: &mut Worker) -> String {
    let hostname = util::sys::hostname().unwrap_or(String::from("unknown"));
    let ip = util::sys::ip().unwrap_or(String::from("127.0.0.1"));
    let package = worker.package.read().unwrap();
    let port = package.exposes().pop().unwrap_or(String::from("0"));

    format!("{}\nhostname = '{}'\nip = '{}'\nport = '{}'\nexpose = [{}]", status_type, hostname, ip, port, package.exposes().iter().fold(String::new(), |acc, p| format!("{}{},", acc, p)))
}

fn state_init(worker: &mut Worker) -> Result<(State, u32), BldrError> {
    try!(standalone::state_init(worker));
    println!("   {}: Attempting to initialize the data set", worker.preamble());
    let key = {
        let package = worker.package.read().unwrap();
        format!("{}/{}/topology/leader/init", package.name, worker.config.group())
    };
    let status = status_value("[topology.init]", worker);
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
            let (statuscode, _response) =
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
    let pkg_lock = worker.package.clone();
    println!("   {}: Becoming the leader", worker.preamble());
    println!("   {}: Forming the government", worker.preamble());
    let govkey = {
        let package = pkg_lock.read().unwrap();
        format!("{}/{}/topology/leader/government", package.name, worker.config.group())
    };
    let (gov_statuscode, _gov_response) =
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

    let key = {
        let package = pkg_lock.read().unwrap();
        format!("{}/{}/topology/leader/government/leader", package.name, worker.config.group())
    };
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

    write_census(worker);

    {
        let mut package = pkg_lock.write().unwrap();
        try!(package.write_toml_string("102_role.toml", &format!("topology-leader = true")));

        if worker.configuration_thread.is_none() {
            try!(package.write_default_data());
            try!(package.write_environment_data());
            try!(package.write_sys_data());
            try!(package.write_bldr_data());
        }
    }

    worker.discovery.stop();
    worker.discovery.clear();

    let (key, ckey, census_key, gvmt_key) = {
        let hostname = util::sys::hostname().unwrap();
        let package = pkg_lock.read().unwrap();
        let key = format!("{}/{}/topology/leader/government/leader", package.name, worker.config.group());
        let ckey = format!("{}/{}/config", package.name, worker.config.group());
        let census_key = format!("{}/{}/topology/leader/census/{}", package.name, worker.config.group(), hostname);
        let gvmt_key = format!("{}/{}/topology/leader/government", package.name, worker.config.group());
        (key, ckey, census_key, gvmt_key)
    };
    let pkg_arc1 = worker.package.clone();
    let pkg_arc2 = worker.package.clone();
    let pkg_arc3 = worker.package.clone();
    let pkg_arc4 = worker.package.clone();
    let watcher = DiscoveryWatcher::new(pkg_arc1, key, String::from("101_leader.toml"), 1, true, false);
    let cwatcher = DiscoveryWatcher::new(pkg_arc2, ckey, String::from("100_discovery.toml"), 1, true, false);
    let census_writer = DiscoveryWriter::new(pkg_arc3, census_key, None, Some(30));
    let gvmt_writer = DiscoveryWriter::new(pkg_arc4, gvmt_key, None, Some(30));
    worker.discovery.watch(watcher);
    worker.discovery.watch(cwatcher);
    worker.discovery.write(census_writer);
    worker.discovery.write(gvmt_writer);

    Ok((State::Leader, 0))
}

fn write_census(worker: &mut Worker)  {
    println!("   {}: Creating my entry in the the census", worker.preamble());
    let hostname = util::sys::hostname().unwrap();
    let pkg_lock = worker.package.clone();
    let package = pkg_lock.read().unwrap();
    let census_key = format!("{}/{}/topology/leader/census/{}", package.name, worker.config.group(), hostname);
    let (census_statuscode, _census_response) =
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

    let key = format!("{}/{}/topology/leader/census/{}/data", package.name, worker.config.group(), hostname);
    let (statuscode, response) =
        etcd::set(&key, &[("value", &status_value("[[topology.census]]", worker))]).unwrap();
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
}

fn state_become_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Becoming a follower", worker.preamble());

    write_census(worker);

    let pkg_lock = worker.package.clone();
    let mut package = pkg_lock.write().unwrap();
    try!(package.write_toml_string("102_role.toml", &format!("topology-follower = true")));

    if worker.configuration_thread.is_none() {
        try!(package.write_default_data());
        try!(package.write_environment_data());
        try!(package.write_sys_data());
        try!(package.write_bldr_data());
    }

    worker.discovery.stop();
    worker.discovery.clear();

    let hostname = util::sys::hostname().unwrap();
    let pkg_arc1 = worker.package.clone();
    let key = format!("{}/{}/topology/leader/government/leader", package.name, worker.config.group());
    let watcher = DiscoveryWatcher::new(pkg_arc1, key, String::from("101_leader.toml"), 1, true, false);
    worker.discovery.watch(watcher);
    let pkg_arc2 = worker.package.clone();
    let ckey = format!("{}/{}/config", package.name, worker.config.group());
    let cwatcher = DiscoveryWatcher::new(pkg_arc2, ckey, String::from("100_discovery.toml"), 1, true, false);
    worker.discovery.watch(cwatcher);
    let pkg_arc3 = worker.package.clone();
    let census_key = format!("{}/{}/topology/leader/census/{}", package.name, worker.config.group(), hostname);
    let census_writer = DiscoveryWriter::new(pkg_arc3, census_key, None, Some(30));
    worker.discovery.write(census_writer);

    Ok((State::Follower, 0))
}

fn state_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let pkg_lock = worker.package.clone();
    loop {
        let package = pkg_lock.read().unwrap();
        match worker.discovery.write_status(&format!("{}/{}/topology/leader/government", package.name, worker.config.group())) {
            Some(leader) => {
                match leader {
                    &DiscoveryWriteResponse{status: StatusCode::Created, ..} => break,
                    &DiscoveryWriteResponse{status: StatusCode::Ok, ..} => break,
                    _ => {
                        println!("Determining my viability because I got an error updating my own governments ttl");
                        return Ok((State::DetermineViability, 0));
                    }
                }
            },
            None => { return Ok((State::Leader, 0)) }
        }
    }

    if worker.supervisor_thread.is_none() {
        let package = pkg_lock.read().unwrap();
        try!(package.configure());
        try!(standalone::state_starting(worker));
        let watch_package = worker.package.clone();
        let configuration_thread = try!(thread::Builder::new().name(String::from("configuration")).spawn(move || -> BldrResult<()> {
            let package = watch_package.read().unwrap();
            try!(package.watch_configuration());
            Ok(())
        }));
        worker.configuration_thread = Some(configuration_thread);
    }

    Ok((State::Leader, 0))
}

fn state_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let pkg_lock = worker.package.clone();
    loop {
        let package = pkg_lock.read().unwrap();
        match worker.discovery.status(&format!("{}/{}/topology/leader/government/leader", package.name, worker.config.group())) {
            Some(leader) => {
                if let &DiscoveryResponse{value: None, ..} = leader {
                    println!("   {}: Determining my viability as a candidate because the leader has left", worker.preamble());
                    debug!("Discovery state: {:?}", worker.discovery);
                    return Ok((State::DetermineViability, 0));
                } else {
                    break;
                }
            },
            None => {
                return Ok((State::Follower, 0));
            }
        }
    }
    if worker.supervisor_thread.is_none() {
        let package = pkg_lock.read().unwrap();
        try!(package.configure());
        try!(standalone::state_starting(worker));
        let watch_package = worker.package.clone();
        let configuration_thread = try!(thread::Builder::new().name(String::from("configuration")).spawn(move || -> BldrResult<()> {
            let package = watch_package.read().unwrap();
            try!(package.watch_configuration());
            Ok(())
        }));
        worker.configuration_thread = Some(configuration_thread);
    }
    Ok((State::Follower, 0))
}
