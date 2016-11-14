// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

//! The Habitat Director is a supervisor for a group of `hab-sup` processes.
//! It loads packages to start from it's `config.toml` file.  The director
//! will automatically restart child process upon failure detection. Each
//! child service runs in it's own `hab-sup` process. The director can be
//! run inside of a `hab-sup` instance as well.
//!
//! ### Components
//! - `Task`
//! 	- manages a hab-sup as a child process
//! 	- tracks a single child process PID
//! 	- generates CLI arguments for `hab-sup start`
//! 	- creates a PID file for it's child process
//! 	- starts a thread to read stdout from the child process
//!
//! - `Controller`
//! 	- A controller "has" and supervises many Tasks (children)
//! 	- calculates gossip and http port #'s for all children before starting.
//! 	- runs in a tight loop to see if children are down and start/restarts them.
//! 	- catches OS signals
//!
//! - `ExecContext`
//! 	- A task "execution context".  The `ExecContext` is used to
//! 	    decouple service root directory and path to a supervisor executable.
//! 	    Decoupling these values into a struct allows us to easily test
//! 	    `Tasks` + `Controllers`.
//!
//! - `ExecParams`
//!     - Config values for a `Task` that the `Controller` calculates during
//!       startup. `ExecParams` currently includes:
//!          - gossip_listen
//!          - http_listen
//!          - Option<peer_ip_port>
//!
//! - `ServiceDef`
//! 	- A combination of `PackageIdent`, `ServiceGroup`, and CLI args. These
//! 	values are loaded from the config file and are set by the user, as
//! 	opposed to `ExecContext` values which are set by the `Controller`.
//! 	- Examples:
//! 		- 	`core.redis.somegroup.someorg` corresponds to the
//! 		`core/redis` `PackageIdent`, and the
//! 		`redis.somegroup@someorg` `ServiceGroup`.
//! 		-  `core.redis.somegroup` corresponds to the
//! 		`core/redis` `PackageIdent`, and the `redis.somegroup` `ServiceGroup`
//! 		(org-less).
//!
//! ```
//! ┌──────────┐
//! │  hab-sup │
//! └──────────┘
//!       │
//!       │
//!       │    ┌───────────────┐
//!       │    │               │
//!       └───▶│   Controller  │────┐    ┌────────────┐   ┌────────┐    ┌──────────┐
//!            │               │    ├───▶│ ExecParams │──▶│  Task  │───▶│  hab-sup │
//!            └───────────────┘    │    └────────────┘   └────────┘    └──────────┘
//!              ┌─────────────┐    │    ┌────────────┐   ┌────────┐    ┌──────────┐
//!              │ ExecContext │────┼───▶│ ExecParams │──▶│  Task  │───▶│  hab-sup │
//!              └─────────────┘    │    └────────────┘   └────────┘    └──────────┘
//!                                 │    ┌────────────┐   ┌────────┐    ┌──────────┐
//!                                 └───▶│ ExecParams │──▶│  Task  │───▶│  hab-sup │
//!                                      └────────────┘   └────────┘    └──────────┘
//! ```
//! ### Config file format
//!
//! `ServiceDef`s are parsed from the `config.toml` file upon startup.
//!
//! All services must be described as children of the `services` toml table.
//! Note, when toml is rendered, the values for `services` will be
//! located under `cfg.services.*`.
//!
//! Each service definition is a `.` separated list of values as a TOML table name.
//!
//! `[services.<origin>.<name>.<group>.<organization>]`
//!
//! or rendered by `hab-sup`:
//!
//! `[cfg.services.<origin>.<name>.<group>.<organization>]`
//!
//! A service definition can additionally specify a `start` key/value under
//! the service table definition:
//!
//! ```
//! # Start core/redis with --group somegroup and --org someorg
//! # Additionally, pass in --permanent-peer to the start CLI
//! [cfg.services.core.redis.somegroup.someorg]
//! start = "--permanent-peer"
//!
//! [cfg.services.core.rngd.foo.someorg]
//! start = "--permanent-peer --foo=bar"
//!
//! Environment variables can be specified for each service in a TOML
//! table. Only TOML string values are supported as environment variables.
//!
//! ```
//! [cfg.services.core.someservice.somegroup.someorg]
//! start = "--permanent-peer"
//! [cfg.services.core.someservice.somegroup.someorg.env]
//! JAVA_HOME="/hab/pkgs/core/jdk/foo"
//! CLASSPATH="/hab/pkgs/core/bar/jars"
//!
//! ```
//! ### Signal handling
//!
//! - If started from bash: when hab-director receives SIGINT or SIGTERM,
//! the director exits. Child processes will have already been sent the signal
//! from bash because they are in the same session group, and will die as well.
//! - TODO: If NOT started from bash: when hab-director receives SIGINT or SIGTERM,
//! signal behavior is undefined and signals are NOT forwarded to child tasks.
//! - When hab-director receives any other signal (that doesn't
//! kill *this* process), they are re-sent to each `Task` in the same
//! order that services are defined in `config.toml`.
//!
//! ### PID files
//!
//! - for each `Task` created, a pid file is created in the `hab-director`
//! service directory. For example, for `core.redis.somegroup.someorg`, we'll
//! have a `/hab/svc/hab-director/core-redis-somegroup-someorg.pid` file
//! created. Creation/removal of these files is handled automatically by
//! the director.
//!
//! ### Ring behavior + port assignment
//!
//! - The first task that is created will attempt to join
//! `sys.gossip_ip`:`sys.gossip_port` by specifying a `--peer` to hab-sup.
//! This only occurs if the director is running under a supervisor.
//! - Each subsequent task that is created uses the previous tasks IP:port as
//! a value for --peer.
//! - Gossip port numbers are assigned starting with FIRST_GOSSIP_PORT (9000)
//! - HTTP port numbers are assigned starting with FIRST_HTTP_PORT (8000)
//! - If the hab-sup that's running the director is assigned ports other than
//! the defaults (9634, 9631), there is a possibility that they could conflict
//! with the automatically assigned port numbers of the child tasks.
//! - The diagram below shows a hab-sup process running the director with it's
//! default IP + port (changeable by the user). Each task that's started is
//! assigned a new consecutive gossip + http IP.
//!
//!                 ┌────────────────────────────┐
//!                 │     hab-sup (Director)     │
//!              ┌─▶│       Gossip = 9634        │ * default ports
//!              │  │       HTTP = 9631          │
//!              │  └────────────────────────────┘
//!              │
//!         Peer │
//!              │  ┌────────────────────────────┐
//!              │  │Task 0                      │
//!              └──│FIRST_GOSSIP_PORT (9000)    │◀─┐
//!                 │FIRST_HTTP_PORT (8000)      │  │
//!                 └────────────────────────────┘  │
//!                                                 │
//!                                                 │ Peer
//!                 ┌────────────────────────────┐  │
//!                 │Task 1                      │  │
//!              ┌─▶│FIRST_GOSSIP_PORT+1 (9001)  │──┘
//!              │  │FIRST_HTTP_PORT+1 (8001)    │
//!              │  └────────────────────────────┘
//!              │
//!         Peer │
//!              │  ┌────────────────────────────┐
//!              │  │Task 2                      │
//!              └──│FIRST_GOSSIP_PORT+2 (9002)  │
//!                 │FIRST_HTTP_PORT+2 (8002)    │
//!                 └────────────────────────────┘
//!

#[macro_use]
extern crate habitat_director as director;
extern crate habitat_core as hcore;
extern crate habitat_common as hcommon;
#[macro_use]
extern crate habitat_sup as hsup;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::process;

use hcommon::ui::UI;
use hcore::config::ConfigFile;
use hcore::fs::am_i_root;

use director::config::Config;
use director::controller::Controller;
use director::task::ExecContext;
use director::error::{Error, Result};

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
const CFG_DEFAULT_PATH: &'static str = "/hab/svc/hab-director/config.toml";

static LOGKEY: &'static str = "DIR";

fn main() {
    env_logger::init().unwrap();
    let matches = app().get_matches();
    debug!("CLI matches: {:?}", matches);
    let config = match config_from_args(&matches) {
        Ok(result) => result,
        Err(e) => return exit_with(e, 1),
    };
    match dispatch(config, &matches) {
        Ok(_) => std::process::exit(0),
        Err(e) => exit_with(e, 1),
    }
}

fn app<'a, 'b>() -> clap::App<'a, 'b> {
    clap_app!(Director =>
        (version: VERSION)
        (about: "Launch and supervise multiple Habitat services")
        (@setting VersionlessSubcommands)
        (@setting SubcommandRequiredElseHelp)
        (@subcommand start =>
            (about: "Run a Habitat director")
            (@arg config: -c --config +takes_value +global
             "Path to configuration file. [default: config.toml]")
        )
    )
}

fn config_from_args(matches: &clap::ArgMatches) -> Result<Config> {
    let cmd = matches.subcommand_name().unwrap();
    let args = matches.subcommand_matches(cmd).unwrap();

    let config = match args.value_of("config") {
        Some(cfg_path) => try!(Config::from_file(cfg_path)),
        None => Config::from_file(CFG_DEFAULT_PATH).unwrap_or(Config::default()),
    };
    Ok(config)
}

fn dispatch(config: Config, matches: &clap::ArgMatches) -> Result<()> {
    match matches.subcommand_name() {
        Some("start") => start(config),
        Some(cmd) => {
            debug!("Dispatch failed, no match for command: {:?}", cmd);
            Ok(())
        }
        None => Ok(()),
    }
}

fn start(config: Config) -> Result<()> {
    let mut ui = UI::default();
    if !am_i_root() {
        try!(ui.warn("Running the Habitat Supervisor requires root or administrator privileges. \
                      Please retry this command as a super user or use a privilege-granting \
                      facility such as sudo."));
        try!(ui.br());
        return Err(Error::RootRequired);
    }

    outputln!("Starting Controller");
    let ec = ExecContext::default();
    let mut controller = Controller::new(config, ec);
    try!(controller.start());
    Ok(())
}

fn exit_with(err: Error, code: i32) {
    println!("{}", err);
    process::exit(code)
}
