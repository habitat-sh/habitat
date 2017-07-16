// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

extern crate env_logger;
extern crate habitat_launcher as launcher;

use std::env;
use std::process;

use launcher::server;

fn main() {
    env_logger::init().unwrap();
    let args = env::args().skip(1).collect();

    if let Err(err) = server::run(args) {
        println!("{}", err);
        process::exit(1);
    }
}
