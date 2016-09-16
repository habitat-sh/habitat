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

use topology::{self, standalone, State, Worker};
use state_machine::StateMachine;
use error::{Result, SupError};
use package::Package;
use config::gconfig;
use census::MIN_QUORUM;
use gossip::server;

static LOGKEY: &'static str = "TL";

pub fn run(package: Package) -> Result<()> {
    let mut worker = try!(Worker::new(package, String::from("leader")));
    let mut sm: StateMachine<State, Worker, SupError> = StateMachine::new(State::Init);
    sm.add_dispatch(State::Init, state_init);
    sm.add_dispatch(State::MinimumQuorum, state_minimum_quorum);
    sm.add_dispatch(State::WaitingForQuorum, state_waiting_for_quorum);
    sm.add_dispatch(State::RestoreDataset, state_restore_dataset);
    sm.add_dispatch(State::StartElection, state_start_election);
    sm.add_dispatch(State::Election, state_election);
    sm.add_dispatch(State::CheckForElection, state_check_for_election);
    sm.add_dispatch(State::BecomeLeader, state_become_leader);
    sm.add_dispatch(State::BecomeFollower, state_become_follower);
    sm.add_dispatch(State::Starting, state_starting);
    topology::run_internal(&mut sm, &mut worker)
}

fn state_init(worker: &mut Worker) -> Result<(State, u64)> {
    let cl = worker.census_list.read().unwrap();
    let census = cl.local_census();
    if !census.minimum_quorum() {
        outputln!("{} of {} census entries; waiting for minimum quorum",
                  census.total_population(),
                  MIN_QUORUM,
                  );
        Ok((State::MinimumQuorum, 0))
    } else if !census.has_quorum() {
        outputln!("Waiting for quorum; {} of {} are alive",
                  census.alive_population(),
                  census.total_population());
        worker.return_state = Some(State::Init);
        Ok((State::WaitingForQuorum, 0))
    } else if census.dataset_initialized() {
        Ok((State::RestoreDataset, 0))
    } else {
        Ok((State::CheckForElection, 0))
    }
}

fn state_minimum_quorum(worker: &mut Worker) -> Result<(State, u64)> {
    let cl = worker.census_list.read().unwrap();
    let census = cl.local_census();
    if census.minimum_quorum() {
        outputln!("Minimum quorum met!");
        Ok((State::Init, 0))
    } else {
        debug!("Minimum quorum not met");
        Ok((State::MinimumQuorum, 200))
    }
}

fn state_waiting_for_quorum(worker: &mut Worker) -> Result<(State, u64)> {
    let cl = worker.census_list.read().unwrap();
    let census = cl.local_census();
    if census.has_quorum() {
        outputln!("We have quorum!");
        Ok((worker.return_state.take().unwrap(), 0))
    } else {
        Ok((State::WaitingForQuorum, 200))
    }
}

fn state_restore_dataset(worker: &mut Worker) -> Result<(State, u64)> {
    outputln!("Restoring the dataset from a peer");
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut ce = cl.me_mut();
        ce.data_init(true);
    }
    Ok((State::CheckForElection, 0))
}

fn state_check_for_election(worker: &mut Worker) -> Result<(State, u64)> {
    let (has_quorum, am_leader, am_follower, has_leader) = {
        let cl = worker.census_list.read().unwrap();
        let census = cl.local_census();
        let has_quorum = census.has_quorum();
        let am_leader = census.me().leader;
        let am_follower = census.me().follower;
        let has_leader = census.has_leader();
        (has_quorum, am_leader, am_follower, has_leader)
    };

    if has_quorum {
        if has_leader {
            if am_leader || am_follower {
                // Are you already the leader or a follower? - good on you!
                Ok((State::CheckForElection, 500))
            } else {
                debug!("We have a leader, but I am not a leader or a follower; becoming a \
                        follower");
                // If you aren't, you should be a follower!
                Ok((State::BecomeFollower, 0))
            }
        } else {
            outputln!("I have quorum, but no leader; starting an election!");
            Ok((State::StartElection, 0))
        }
    } else {
        if am_leader {
            outputln!("I have lost quorum, and I am the leader - I must abdicate!");
            {
                let mut cl = worker.census_list.write().unwrap();
                let mut census = cl.local_census_mut();
                census.in_event = true;
                census.no_leaders_allowed();
                let mut me = census.me_mut();
                me.leader(false);
                me.follower(false);
            }
            outputln!("Stopping the service to ensure there is only one master");
            {
                let mut supervisor = worker.supervisor.write().unwrap();
                try!(supervisor.down());
            }
        } else {
            outputln!("I have lost quorum - getting rid of any leader");
            {
                let mut cl = worker.census_list.write().unwrap();
                let mut census = cl.local_census_mut();
                census.in_event = true;
                census.no_leaders_allowed();
                let mut me = census.me_mut();
                me.leader(false);
                me.follower(false);
            }
        }
        Ok((State::CheckForElection, 200))
    }
}

pub fn state_start_election(worker: &mut Worker) -> Result<(State, u64)> {
    outputln!("Starting election");
    let rumor_list = {
        let el = worker.election_list.read().unwrap();
        el.generate_rumor_list_for(worker.package_name.clone(), gconfig().group().to_string())
    };
    server::process_rumors(rumor_list,
                           worker.rumor_list.clone(),
                           worker.member_list.clone(),
                           worker.census_list.clone(),
                           worker.election_list.clone(),
                           worker.gossip_file_list.clone());
    Ok((State::Election, 200))
}

pub fn state_election(worker: &mut Worker) -> Result<(State, u64)> {
    let (alive_population, has_quorum) = {
        let cl = worker.census_list.read().unwrap();
        let census = cl.local_census();
        (census.alive_population(), census.has_quorum())
    };

    if !has_quorum {
        outputln!("Lost quorum during election - should have a state for this")
    }

    let final_rumor_list = {
        let el = worker.election_list.read().unwrap();
        let election = match el.election() {
            Some(election) => election,
            None => {
                debug!("Missing election object; trying again");
                return Ok((State::StartElection, 200));
            }
        };
        if election.finished() {
            if el.member_id == election.leader_id {
                return Ok((State::BecomeLeader, 0));
            } else {
                return Ok((State::BecomeFollower, 0));
            }
        } else {
            if election.should_finish(&el.member_id, alive_population) {
                Some(el.finished_rumor_list_for(worker.package_name.clone(),
                                                gconfig().group().to_string()))
            } else {
                None
            }
        }
    };

    if let Some(rumor_list) = final_rumor_list {
        server::process_rumors(rumor_list,
                               worker.rumor_list.clone(),
                               worker.member_list.clone(),
                               worker.census_list.clone(),
                               worker.election_list.clone(),
                               worker.gossip_file_list.clone());
    }

    Ok((State::Election, 200))
}

pub fn state_become_leader(worker: &mut Worker) -> Result<(State, u64)> {
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut census = cl.local_census_mut();
        census.in_event = false;
        let mut me = census.me_mut();
        me.follower(false);
        me.leader(true);
        if me.election.is_some() {
            me.election(None)
        }
        if me.vote.is_some() {
            me.vote(None)
        }
    }
    Ok((State::Starting, 200))
}

pub fn state_become_follower(worker: &mut Worker) -> Result<(State, u64)> {
    {
        let cl = worker.census_list.read().unwrap();
        let census = cl.local_census();
        if !census.has_leader() {
            debug!("Waiting for the leader");
            return Ok((State::BecomeFollower, 200));
        }
    }
    outputln!("Becoming a follower");
    {
        let mut cl = worker.census_list.write().unwrap();
        {
            let mut ce = cl.me_mut();
            ce.leader(false);
            ce.follower(true);
            if ce.election.is_some() {
                ce.election(None);
            }
            if ce.vote.is_some() {
                ce.vote(None);
            }
        }
        let mut census = cl.local_census_mut();
        census.in_event = false;
    }
    Ok((State::Starting, 200))
}

pub fn state_starting(worker: &mut Worker) -> Result<(State, u64)> {
    let is_running = {
        let supervisor = worker.supervisor.read().unwrap();
        supervisor.pid.is_some()
    };
    if !is_running {
        try!(initialize(worker));
        try!(standalone::state_starting(worker));
    }
    Ok((State::CheckForElection, 200))
}


fn initialize(worker: &mut Worker) -> Result<()> {
    let service_config = worker.service_config.read().unwrap();
    let package = worker.package.read().unwrap();
    match package.initialize(&service_config) {
        Ok(()) => {
            let mut cl = worker.census_list.write().unwrap();
            let mut me = cl.me_mut();
            me.initialized();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
