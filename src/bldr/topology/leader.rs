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

use topology::{self, State, Worker};
use state_machine::StateMachine;
use error::{BldrResult, BldrError};
use pkg::Package;
use topology::standalone;
use config::Config;

pub fn run(package: Package, config: &Config) -> BldrResult<()> {
    let mut worker = try!(Worker::new(package, String::from("leader"), config));
    let mut sm: StateMachine<State, Worker, BldrError> = StateMachine::new(State::Init);
    sm.add_dispatch(State::Init, state_init);
    sm.add_dispatch(State::RestoreDataset, state_restore_dataset);
    sm.add_dispatch(State::DetermineViability, state_determine_viability);
    sm.add_dispatch(State::StartElection, state_start_election);
    sm.add_dispatch(State::InElection, state_in_election);
    sm.add_dispatch(State::BecomeLeader, state_become_leader);
    sm.add_dispatch(State::BecomeFollower, state_become_follower);
    sm.add_dispatch(State::Leader, state_leader);
    sm.add_dispatch(State::Follower, state_follower);
    topology::run_internal(&mut sm, &mut worker)
}

fn state_init(worker: &mut Worker) -> Result<(State, u32), BldrError> {
    if worker.census.dataset_initialized() {
        Ok((State::RestoreDataset, 0))
    } else {
        Ok((State::DetermineViability, 0))
    }
}

fn state_restore_dataset(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Restoring the dataset from a peer", worker.preamble());
    let ce = try!(worker.census.me_mut());
    ce.data_init(Some(true));
    Ok((State::DetermineViability, 0))
}

fn state_determine_viability(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Determining viability as a leader", worker.preamble());
    worker.census.in_event = true;
    {
        let mut ce = try!(worker.census.me_mut());
        ce.follower(None);
        ce.leader(None);
    }
    if worker.census.has_leader() {
        Ok((State::BecomeFollower, 0))
    } else {
        Ok((State::StartElection, 0))
    }
}

fn state_start_election(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Starting an election", worker.preamble());
    {
        let mut ce = try!(worker.census.me_mut());
        ce.election = Some(true);
    }
    let candidate = worker.census.determine_vote().candidate_string();
    println!("   {}: My candidate is {:?}", worker.preamble(), candidate);
    let mut ce = try!(worker.census.me_mut());
    ce.vote(Some(candidate));
    Ok((State::InElection, 0))
}

fn state_in_election(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let preamble = worker.preamble();
    let candidate = worker.census.determine_vote().candidate_string();
    {
        let mut ce = try!(worker.census.me_mut());
        match ce.vote {
            Some(ref c) if *c == candidate => {},
            Some(_) => {
                println!("   {}: Switching my vote to {}", preamble, candidate);
                ce.vote(Some(candidate));
            },
            None => {
                println!("   {}: Switching my vote to {}", preamble, candidate);
                ce.vote(Some(candidate));
            }
        }
    }

    if let Some(leader_ce) = worker.census.get_leader() {
        println!("   {}: {} has already been elected; becoming a follower", preamble, leader_ce.candidate_string());
        return Ok((State::BecomeFollower, 0));
    }

    match worker.census.voting_finished() {
        Some(winner) => {
            let me = try!(worker.census.me());
            if me == winner {
                println!("   {}: The votes are in! I won! I will serve with humility.", worker.preamble());
                Ok((State::BecomeLeader, 0))
            } else {
                println!("   {}: The votes are in! I lost! I will serve with grace.", worker.preamble());
                Ok((State::BecomeFollower, 0))
            }
        },
        None => Ok((State::InElection, 10))
    }
}

fn state_become_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    if worker.census.has_leader() == false {
        {
            let mut ce = try!(worker.census.me_mut());
            ce.leader = Some(true);
        }
    }

    if worker.census.has_all_followers() {
        println!("   {}: Starting my term as leader", worker.preamble());
        Ok((State::Leader, 0))
    } else {
        println!("   {}: Waiting for all my followers to agree", worker.preamble());
        Ok((State::BecomeLeader, 0))
    }
}

fn state_become_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    println!("   {}: Becoming a follower", worker.preamble());
    {
        let mut ce = try!(worker.census.me_mut());
        if ce.follower.is_none() {
            ce.follower(Some(true));
        }
    }

    if worker.census.has_leader() {
        println!("   {}: Becoming a follower for real", worker.preamble());
        Ok((State::Follower, 0))
    } else {
        println!("   {}: Waiting for a leader", worker.preamble());
        Ok((State::BecomeFollower, 0))
    }
}

fn state_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    {
        let me = try!(worker.census.me_mut());
        if me.election.is_some() {
            me.election(None)
        }
        if me.vote.is_some() {
            me.vote(None)
        }
    }

    if worker.census.in_event {
        worker.census.in_event = false;
    }

    if worker.supervisor_thread.is_none() {
        try!(standalone::state_starting(worker));
    }

    Ok((State::Leader, 0))
}

fn state_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    {
        let me = try!(worker.census.me_mut());
        if me.election.is_some() {
            me.election(None);
        }
        if me.vote.is_some() {
            me.vote(None);
        }
    }

    if worker.census.in_event {
        worker.census.in_event = false;
    }

    if worker.supervisor_thread.is_none() {
        try!(standalone::state_starting(worker));
    }

    if ! worker.census.has_leader() {
        Ok((State::DetermineViability, 0))
    } else {
        Ok((State::Follower, 0))
    }
}
