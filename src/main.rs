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

extern crate bldr;
extern crate rustc_serialize;
extern crate docopt;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate ansi_term;
extern crate libc;
use docopt::Docopt;
use std::process;
use ansi_term::Colour::{Red, Green, Yellow};
use libc::funcs::posix88::unistd::execvp;
use std::ffi::CString;
use std::ptr;

use bldr::config::{Command, Config};
use bldr::sidecar;
use bldr::error::{BldrResult, BldrError};
use bldr::command::*;

#[allow(dead_code)]
static VERSION: &'static str = "0.0.1";

#[allow(dead_code)]
static USAGE: &'static str = "
Usage: bldr install <package> -u <url>
       bldr start <package> [--group=<group>] [--topology=<topology>] [--watch=<watch>...]
       bldr sh
       bldr bash
       bldr key -u <url>

Options:
    -g, --group=<group>        The service group; shared config and topology [default: default]
    -t, --topology=<topology>  Specify a service topology [default: standalone]
    -u, --url=<url>            Use a specific url for fetching a file
    -w, --watch=<watch>        One or more service groups to watch for updates
";

#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_install: bool,
    cmd_start: bool,
    cmd_key: bool,
    cmd_sh: bool,
    cmd_bash: bool,
    arg_package: String,
    flag_url: String,
    flag_topology: String,
    flag_group: String,
    flag_watch: Vec<String>
}

fn config_from_args(args: &Args, command: Command) -> Config {
    let mut config = Config::new();
    config.set_command(command);
    config.set_package(args.arg_package.clone());
    config.set_url(args.flag_url.clone());
    config.set_topology(args.flag_topology.clone());
    config.set_group(args.flag_group.clone());
    config.set_watch(args.flag_watch.clone());
    config
}

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    debug!("Docopt Args: {:?}", args);
    let result = match args {
        Args{cmd_install: true, ..} => {
            let config = config_from_args(&args, Command::Install);
            install(&config)
        },
        Args{cmd_start: true, ..} => {
            let config = config_from_args(&args, Command::Start);
            start(&config)
        },
        Args{cmd_key: true, ..} => {
            let config = config_from_args(&args, Command::Key);
            key(&config)
        },
        Args{cmd_sh: true, ..} => {
            let config = config_from_args(&args, Command::Shell);
            shell(&config)
        },
        Args{cmd_bash: true, ..} => {
            let config = config_from_args(&args, Command::Shell);
            shell(&config)
        },
        _ => Err(BldrError::CommandNotImplemented),
    };

    match result {
        Ok(_) => {},
        Err(e) => exit_with(e, 1)
    }
}

#[allow(dead_code)]
fn banner() {
    println!("{} version {}", Green.bold().paint("bldr"), VERSION);
}

#[allow(dead_code)]
fn shell(_config: &Config) -> BldrResult<()> {
    banner();
    let shell_arg = try!(CString::new("sh"));
    let mut argv = [ shell_arg.as_ptr(), ptr::null() ];
    unsafe {
        execvp(shell_arg.as_ptr(), argv.as_mut_ptr());
    }
    // Yeah, you don't know any better.. but we aren't coming back from
    // what happens next.
    Ok(())
}

#[allow(dead_code)]
fn install(config: &Config) -> BldrResult<()> {
    banner();
    println!("Installing {}", Yellow.bold().paint(config.package()));
    let pkg_file = try!(install::from_url(config.package(), config.url()));
    try!(install::verify(config.package(), &pkg_file));
    try!(install::unpack(config.package(), &pkg_file));
    Ok(())
}

#[allow(dead_code)]
fn start(config: &Config) -> BldrResult<()> {
    banner();
    println!("Starting {}", Yellow.bold().paint(config.package()));
    try!(sidecar::run(config.package()));
    try!(start::package(config));
    println!("Finished with {}", Yellow.bold().paint(config.package()));
    Ok(())
}

#[allow(dead_code)]
fn key(config: &Config) -> BldrResult<()> {
    banner();
    println!("Installing key {}", Yellow.bold().paint(config.url()));
    try!(key::install(config.url()));
    Ok(())
}

#[allow(dead_code)]
fn exit_with(e: BldrError, code: i32) {
    println!("{}", Red.bold().paint(&format!("{}", e)));
    process::exit(code)
}
