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

use topology::{self, initializer, State, Worker};
use state_machine::StateMachine;
use error::{BldrResult, BldrError};
use pkg::Package;
use config::Config;

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
    sm.add_dispatch(State::InitializingLeader,
                    initializer::state_initializing_leader);
    sm.add_dispatch(State::InitializingFollower,
                    initializer::state_initializing_follower);
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
    println!("   {}: Restoring the dataset from a peer",
             worker.preamble());
    let ce = try!(worker.census.me_mut());
    ce.data_init(Some(true));
    Ok((State::DetermineViability, 0))
}
