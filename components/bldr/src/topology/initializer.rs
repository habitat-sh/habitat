// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! This is the building block of complicated topologies which require a leader. It is
//! used when a single member of your cluster should perform additional applications
//! level initialization and/or if the other members of your cluster need to perform
//! additional initialization steps.
//!
//! We guarantee that the leader will perform it's initialization sequence before the
//! followers attempt to run thier initialization sequences.

use config::Config;
use error::{BldrResult, BldrError};
use state_machine::StateMachine;
use topology::{self, standalone, State, Worker};
use package::Package;

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

pub fn state_determine_viability(worker: &mut Worker) -> BldrResult<(State, u64)> {
    outputln!("Determining viability as a leader");
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut ce = cl.me_mut();
        ce.follower(false);
        ce.leader(false);
    }
    {
        let cl = worker.census_list.read().unwrap();
        let census = cl.local_census();
        if census.has_leader() {
            Ok((State::BecomeFollower, 0))
        } else {
            Ok((State::StartElection, 0))
        }
    }
}

pub fn state_start_election(worker: &mut Worker) -> BldrResult<(State, u64)> {
    outputln!("Starting an election");
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut ce = cl.me_mut();
        ce.election = Some(true);
    }
    let candidate = {
        worker.census_list
              .read()
              .unwrap()
              .local_census()
              .determine_vote()
              .candidate_string()
    };
    outputln!("My candidate is {:?}", candidate);
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut ce = cl.me_mut();
        ce.vote(Some(candidate));
    }
    Ok((State::InElection, 0))
}

pub fn state_in_election(worker: &mut Worker) -> BldrResult<(State, u64)> {
    let candidate = {
        worker.census_list
              .read()
              .unwrap()
              .local_census()
              .determine_vote()
              .candidate_string()
    };

    {
        let mut cl = worker.census_list.write().unwrap();
        let mut ce = cl.me_mut();
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

    {
        let cl = worker.census_list.read().unwrap();
        let census = cl.local_census();
        if let Some(leader_ce) = census.get_leader() {
            outputln!("{} has already been elected; becoming a follower",
                      leader_ce.candidate_string());
            return Ok((State::BecomeFollower, 0));
        }

        match census.voting_finished() {
            Some(winner) => {
                let me = census.me();
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
}

pub fn state_become_leader(worker: &mut Worker) -> BldrResult<(State, u64)> {
    let mut cl = worker.census_list.write().unwrap();
    let mut census = cl.local_census_mut();
    if census.has_leader() == false {
        census.me_mut().leader(true);
    }

    if census.has_all_followers() {
        outputln!("Starting my term as leader");
        {
            let mut me = census.me_mut();
            if me.election.is_some() {
                me.election(None)
            }
            if me.vote.is_some() {
                me.vote(None)
            }
        }

        if census.in_event {
            census.in_event = false;
        }
        Ok((State::Leader, 0))
    } else {
        outputln!("Waiting for all my followers to agree");
        Ok((State::BecomeLeader, 0))
    }
}

pub fn state_become_follower(worker: &mut Worker) -> BldrResult<(State, u64)> {
    outputln!("Becoming a follower");
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut ce = cl.me_mut();
        ce.follower(true);
    }

    let cl = worker.census_list.read().unwrap();
    let census = cl.local_census();
    if census.has_leader() {
        outputln!("Becoming a follower for real");
        Ok((State::Follower, 0))
    } else {
        outputln!("Waiting for a leader");
        Ok((State::BecomeFollower, 0))
    }
}

pub fn state_leader(worker: &mut Worker) -> BldrResult<(State, u64)> {
    if worker.supervisor_thread.is_none() {
        try!(initialize(worker));
        try!(standalone::state_starting(worker));
    }

    Ok((State::Leader, 0))
}

pub fn state_follower(worker: &mut Worker) -> BldrResult<(State, u64)> {
    {
        let mut cl = worker.census_list.write().unwrap();
        let mut me = cl.me_mut();
        if me.election.is_some() {
            me.election(None);
        }
        if me.vote.is_some() {
            me.vote(None);
        }
    }

    {
        let mut cl = worker.census_list.write().unwrap();
        let mut census = cl.local_census_mut();
        if census.in_event {
            census.in_event = false;
        }
    }

    let gate = {
        let cl = worker.census_list.read().unwrap();
        let census = cl.local_census();
        if let Some(leader) = census.get_leader() {
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

    if !worker.census_list.read().unwrap().local_census().has_leader() {
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
            let mut cl = worker.census_list.write().unwrap();
            let mut me = cl.me_mut();
            me.initialized();
            Ok(())
        }
        Err(e) => Err(e),
    }
}
