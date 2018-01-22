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

use std::path::Path;
use std::result;

use clap::App;
use hcore::service::ServiceGroup;

pub fn get() -> App<'static, 'static> {
    clap_app!(hab_butterfly =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: super::VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand config =>
            (about: "Commands relating to Habitat runtime config")
            (aliases: &["co", "con", "conf", "confi"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_config_apply().aliases(&["a", "ap", "app", "appl"]))
        )
        (@subcommand depart =>
            (about: "Manage the departure status of a butterfly member")
            (aliases: &["d", "de", "dep", "depa", "depart"])
            (@setting ArgRequiredElseHelp)
            (@arg MEMBER_ID: +required +takes_value "The member ID to depart")
            (@arg PEER: -p --peer +takes_value
             "A comma-delimited list of one or more Habitat Supervisor peers \
                    (default: 127.0.0.1:9638)")
            (@arg RING: -r --ring +takes_value
             "Ring key name, which will encrypt communication messages")
        )
        (@subcommand file =>
            (about: "Commands relating to Habitat files")
            (aliases: &["f", "fi", "fil"])
            (@setting ArgRequiredElseHelp)
            (@subcommand upload =>
                (about: "Upload a file to the Supervisor ring.")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg SERVICE_GROUP: +required +takes_value {valid_service_group}
                    "Target service group (ex: redis.default)")
                (@arg VERSION_NUMBER: +required
                    "A version number (positive integer) for this configuration (ex: 42)")
                (@arg FILE: +required {file_exists} "Path to local file on disk")
                (@arg ORG: --org +takes_value "Name of service organization")
                (@arg USER: -u --user +takes_value "Name of the user key")
                (@arg PEER: -p --peer +takes_value
                    "A comma-delimited list of one or more Habitat Supervisor peers \
                    (default: 127.0.0.1:9638)")
                (@arg RING: -r --ring +takes_value
                    "Ring key name, which will encrypt communication messages")
            )
        )
    )
}

fn sub_config_apply() -> App<'static, 'static> {
    clap_app!(@subcommand apply =>
        (about: "Applies a configuration to a group of Habitat Supervisors")
        (@arg PEER: -p --peer +takes_value
            "A comma-delimited list of one or more Habitat Supervisor peers \
            (default: 127.0.0.1:9638)")
        (@arg RING: -r --ring +takes_value
            "Ring key name, which will encrypt communication messages")
        (@arg SERVICE_GROUP: +required {valid_service_group}
            "Target service group (ex: redis.default)")
        (@arg VERSION_NUMBER: +required
            "A version number (positive integer) for this configuration (ex: 42)")
        (@arg FILE: {file_exists_or_stdin}
            "Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)")
        (@arg ORG: --org +takes_value "Name of service organization to use for encryption")
        (@arg USER: -u --user +takes_value "Name of a user key to use for encryption")
    )
}

fn file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("File: '{}' cannot be found", &val))
    }
}

fn file_exists_or_stdin(val: String) -> result::Result<(), String> {
    if val == "-" { Ok(()) } else { file_exists(val) }
}

fn valid_service_group(val: String) -> result::Result<(), String> {
    match ServiceGroup::validate(&val) {
        Ok(()) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}
