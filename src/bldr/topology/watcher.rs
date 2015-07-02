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

// use ansi_term::Colour::White;
// 
// use std::thread;
// use std::sync::mpsc::channel;
// 
// use hyper::status::StatusCode;
// use toml;
// 
// use topology::{self, State, Worker};
// use state_machine::StateMachine;
// use error::{BldrResult, BldrError};
// use util;
// use pkg::{self, Package};
// use discovery::etcd;
// use topology::standalone;
// 
// pub fn run(package: Package) -> BldrResult<()> {
//     let mut worker = Worker::new(package, String::from("watcher"));
//     let mut sm: StateMachine<State, Worker, BldrError> = StateMachine::new(State::Init);
//     sm.add_dispatch(State::Init, standalone::state_init);
//     sm.add_dispatch(State::Configure, state_configure);
//     sm.add_dispatch(State::Starting, standalone::state_starting);
//     sm.add_dispatch(State::Running, standalone::state_running);
//     sm.add_dispatch(State::Finished, standalone::state_finished);
//     topology::set_signal_handlers();
//     topology::run_internal(&mut sm, &mut worker)
// }
// 
// pub fn state_configure(worker: &mut Worker) -> Result<(State, u32), BldrError> {
//     try!(standalone::state_configure);
// 
//     let mut package = try!(pkg::latest(&worker.package.name));
//     let config_join = thread::spawn(move || -> BldrResult<()> {
//         loop {
//             try!(package.write_discovery_data("topology/leader/government/leader", "101_leader.toml", false));
//             println!("   {}({}): Waiting 30 seconds before reconnecting", package.name, White.bold().paint("D"));
//             thread::sleep_ms(30000);
//         }
//     });
// 
//     Ok((State::Starting, 0))
// }
// 
