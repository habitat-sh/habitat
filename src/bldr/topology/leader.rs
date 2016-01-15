// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use topology::{self, initializer, State, Worker};
use state_machine::StateMachine;
use error::{BldrResult, BldrError};
use package::Package;
use config::Config;

static LOGKEY: &'static str = "TL";

pub fn run(package: Package, config: &Config) -> BldrResult<()> {
    let mut worker = try!(Worker::new(package, String::from("leader"), config));
    let mut sm: StateMachine<State, Worker, BldrError> = StateMachine::new(State::Init);
    sm.add_dispatch(State::Init, state_init);
    sm.add_dispatch(State::RestoreDataset, state_restore_dataset);
    sm.add_dispatch(State::DetermineViability,
                    initializer::state_determine_viability);
    sm.add_dispatch(State::StartElection, initializer::state_start_election);
    sm.add_dispatch(State::InElection, initializer::state_in_election);
    sm.add_dispatch(State::BecomeLeader, initializer::state_become_leader);
    sm.add_dispatch(State::BecomeFollower, initializer::state_become_follower);
    sm.add_dispatch(State::Leader, initializer::state_leader);
    sm.add_dispatch(State::Follower, initializer::state_follower);
    topology::run_internal(&mut sm, &mut worker)
}

fn state_init(worker: &mut Worker) -> BldrResult<(State, u32)> {
    if worker.census.dataset_initialized() {
        Ok((State::RestoreDataset, 0))
    } else {
        Ok((State::DetermineViability, 0))
    }
}

fn state_restore_dataset(worker: &mut Worker) -> BldrResult<(State, u32)> {
    outputln!("Restoring the dataset from a peer");
    let ce = try!(worker.census.me_mut());
    ce.data_init(Some(true));
    Ok((State::DetermineViability, 0))
}
