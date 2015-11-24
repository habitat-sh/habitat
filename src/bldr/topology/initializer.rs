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
// This is the building block of complicated topologies which require a leader. It is
// used when a single member of your cluster should perform additional applications
// level initialization and/or if the other members of your cluster need to perform
// additional initialization steps.
//
// We guarantee that the leader will perform it's initialization sequence before the
// followers attempt to run thier initialization sequences.

use config::Config;
use error::{BldrResult, BldrError};
use state_machine::StateMachine;
use topology::{self, standalone, State, Worker};
use pkg::Package;

static LOGKEY: &'static str = "TI";

enum InitGate {
    NoLeader,
    Waiting,
    Done,
}

pub fn run(package: Package, config: &Config) -> BldrResult<()> {
    let mut worker = try!(Worker::new(package, String::from("initializer"), config));
    let mut sm: StateMachine<State, Worker, BldrError> =
        StateMachine::new(State::DetermineViability);
    sm.add_dispatch(State::DetermineViability, state_determine_viability);
    sm.add_dispatch(State::StartElection, state_start_election);
    sm.add_dispatch(State::InElection, state_in_election);
    sm.add_dispatch(State::BecomeLeader, state_become_leader);
    sm.add_dispatch(State::BecomeFollower, state_become_follower);
    sm.add_dispatch(State::Leader, state_leader);
    sm.add_dispatch(State::Follower, state_follower);
    topology::run_internal(&mut sm, &mut worker)
}

pub fn state_determine_viability(worker: &mut Worker) -> BldrResult<(State, u32)> {
    outputln!("Determining viability as a leader");
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

pub fn state_start_election(worker: &mut Worker) -> BldrResult<(State, u32)> {
    outputln!("Starting an election");
    {
        let mut ce = try!(worker.census.me_mut());
        ce.election = Some(true);
    }
    let candidate = worker.census.determine_vote().candidate_string();
    outputln!("My candidate is {:?}", candidate);
    let mut ce = try!(worker.census.me_mut());
    ce.vote(Some(candidate));
    Ok((State::InElection, 0))
}

pub fn state_in_election(worker: &mut Worker) -> BldrResult<(State, u32)> {
    let candidate = worker.census.determine_vote().candidate_string();
    {
        let mut ce = try!(worker.census.me_mut());
        match ce.vote {
            Some(ref c) if *c == candidate => {}
            Some(_) => {
                outputln!("Switching my vote to {}", candidate);
                ce.vote(Some(candidate));
            }
            None => {
                outputln!("Switching my vote to {}", candidate);
                ce.vote(Some(candidate));
            }
        }
    }

    if let Some(leader_ce) = worker.census.get_leader() {
        outputln!("{} has already been elected; becoming a follower",
                  leader_ce.candidate_string());
        return Ok((State::BecomeFollower, 0));
    }

    match worker.census.voting_finished() {
        Some(winner) => {
            let me = try!(worker.census.me());
            if me == winner {
                outputln!("The votes are in! I won! I will serve with humility.");
                Ok((State::BecomeLeader, 0))
            } else {
                outputln!("The votes are in! I lost! I will serve with grace.");
                Ok((State::BecomeFollower, 0))
            }
        }
        None => Ok((State::InElection, 10)),
    }
}

pub fn state_become_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    if worker.census.has_leader() == false {
        {
            let mut ce = try!(worker.census.me_mut());
            ce.leader = Some(true);
        }
    }

    if worker.census.has_all_followers() {
        outputln!("Starting my term as leader");
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
        Ok((State::Leader, 0))
    } else {
        outputln!("Waiting for all my followers to agree");
        Ok((State::BecomeLeader, 0))
    }
}

pub fn state_become_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
    outputln!("Becoming a follower");
    {
        let mut ce = try!(worker.census.me_mut());
        if ce.follower.is_none() {
            ce.follower(Some(true));
        }
    }

    if worker.census.has_leader() {
        outputln!("Becoming a follower for real");
        Ok((State::Follower, 0))
    } else {
        outputln!("Waiting for a leader");
        Ok((State::BecomeFollower, 0))
    }
}

pub fn state_leader(worker: &mut Worker) -> BldrResult<(State, u32)> {
    if worker.supervisor_thread.is_none() {
        try!(initialize(worker));
        try!(standalone::state_starting(worker));
    }

    Ok((State::Leader, 0))
}

pub fn state_follower(worker: &mut Worker) -> BldrResult<(State, u32)> {
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

    let gate = {
        if let Some(leader) = worker.census.get_leader() {
            if leader.initialized {
                InitGate::Done
            } else {
                InitGate::Waiting
            }
        } else {
            InitGate::NoLeader
        }
    };

    match gate {
        InitGate::Done => {}
        InitGate::Waiting => return Ok((State::Follower, 0)),
        InitGate::NoLeader => return Ok((State::DetermineViability, 0)),
    }

    if worker.supervisor_thread.is_none() {
        try!(initialize(worker));
        try!(standalone::state_starting(worker));
    }

    if !worker.census.has_leader() {
        Ok((State::DetermineViability, 0))
    } else {
        Ok((State::Follower, 0))
    }
}

fn initialize(worker: &mut Worker) -> BldrResult<()> {
    let service_config = worker.service_config.read().unwrap();
    let package = worker.package.read().unwrap();
    match package.initialize(&service_config) {
        Ok(()) => {
            let mut me = try!(worker.census.me_mut());
            me.initialized();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
