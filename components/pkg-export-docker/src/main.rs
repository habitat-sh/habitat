// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use env_logger;
use habitat_common as common;

use habitat_pkg_export_docker as export_docker;
#[macro_use]
extern crate log;

use crate::common::ui::{UIWriter, UI};

use crate::export_docker::Result;

fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    let cli = export_docker::cli();
    let m = cli.get_matches();
    debug!("clap cli args: {:?}", m);

    export_docker::export_for_cli_matches(ui, &m).map(|_| ())
}
