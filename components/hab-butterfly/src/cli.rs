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

use clap::App;

pub fn get() -> App<'static, 'static> {
    clap_app!(hab_butterfly =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: super::VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
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
    )
}
