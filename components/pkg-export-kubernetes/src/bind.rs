// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use clap::ArgMatches;
use failure::SyncFailure;

use habitat_sup::manager::service::ServiceBind;

use export_docker::Result;

/// Helper function to parse CLI arguments into `ServiceBind` instances.
pub fn parse_bind_args(matches: &ArgMatches) -> Result<Vec<ServiceBind>> {
    let mut binds = Vec::new();

    if let Some(bind_args) = matches.values_of("BIND") {
        for arg in bind_args {
            let b = arg.parse::<ServiceBind>().map_err(SyncFailure::new)?;

            binds.push(b);
        }
    };

    Ok(binds)
}
