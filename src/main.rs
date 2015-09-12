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

/// The version number
#[allow(dead_code)]
static VERSION: &'static str = "0.0.1";

/// The [docopts](http://burntsushi.net/rustdoc/docopt/index.html) usage
/// string. Determines what options are accepted.
#[allow(dead_code)]
static USAGE: &'static str = "
Usage: bldr install <package> -u <url> [-d <deriv>] [-v <version>] [-r <release>]
       bldr start <package> [--group=<group>] [--topology=<topology>] [--watch=<watch>...]
       bldr sh
       bldr bash
       bldr repo [-p <path>]
       bldr upload <package> -u <url> [-d <deriv>] [-v <version>] [-r <release>]
       bldr key <key> [-u <url>]
       bldr key-upload <key> -u <url>
       bldr config <package>

Options:
    -d, --deriv=<deriv>        A package derivative
    -g, --group=<group>        The service group; shared config and topology [default: default]
    -r, --release=<release>    A package release
    -t, --topology=<topology>  Specify a service topology [default: standalone]
    -p, --path=<path>          The path to use for a repository [default: /opt/bldr/srvc/bldr/data]
    -u, --url=<url>            Use the specified package repository url
    -v, --version=<version>    A package version
    -w, --watch=<watch>        One or more service groups to watch for updates
";

/// The struct that docopts renders options
/// into.
#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_install: bool,
    cmd_start: bool,
    cmd_key: bool,
    cmd_sh: bool,
    cmd_bash: bool,
    cmd_repo: bool,
    cmd_upload: bool,
    cmd_key_upload: bool,
    cmd_config: bool,
    arg_package: String,
    arg_key: String,
    flag_path: String,
    flag_deriv: String,
    flag_version: String,
    flag_release: String,
    flag_url: String,
    flag_topology: String,
    flag_group: String,
    flag_watch: Vec<String>
}

/// Creates a [Config](config/struct.Config.html) from the [Args](/Args)
/// struct.
fn config_from_args(args: &Args, command: Command) -> Config {
    let mut config = Config::new();
    config.set_command(command);
    config.set_package(args.arg_package.clone());
    config.set_url(args.flag_url.clone());
    config.set_topology(args.flag_topology.clone());
    config.set_group(args.flag_group.clone());
    config.set_watch(args.flag_watch.clone());
    config.set_path(args.flag_path.clone());
    config.set_version(args.flag_version.clone());
    config.set_deriv(args.flag_deriv.clone());
    config.set_release(args.flag_release.clone());
    config.set_key(args.arg_key.clone());
    config
}

/// The primary loop for bldr.
///
/// * Set up the logger
/// * Pull in the arguments from the Command Line, push through Docopts
/// * Dispatch to a function that handles that action called
/// * Exit cleanly, or if we return an `Error`, call `exit_with(E, 1)`
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
        Args{cmd_key_upload: true, ..} => {
            let config = config_from_args(&args, Command::KeyUpload);
            key_upload(&config)
        },
        Args{cmd_sh: true, ..} => {
            let config = config_from_args(&args, Command::Shell);
            shell(&config)
        },
        Args{cmd_bash: true, ..} => {
            let config = config_from_args(&args, Command::Shell);
            shell(&config)
        },
        Args{cmd_repo: true, ..} => {
            let config = config_from_args(&args, Command::Repo);
            repo(&config)
        },
        Args{cmd_upload: true, ..} => {
            let config = config_from_args(&args, Command::Upload);
            upload(&config)
        },
        Args{cmd_config: true, ..} => {
            let config = config_from_args(&args, Command::Configuration);
            configure(&config)
        },
        _ => Err(BldrError::CommandNotImplemented),
    };

    match result {
        Ok(_) => {},
        Err(e) => exit_with(e, 1)
    }
}

/// Print the banner
#[allow(dead_code)]
fn banner() {
    println!("{} version {}", Green.bold().paint("bldr"), VERSION);
}

/// Start a shell
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

/// Show the configuration options for a service
#[allow(dead_code)]
fn configure(config: &Config) -> BldrResult<()> {
    banner();
    println!("Displaying config for {}", Yellow.bold().paint(config.package()));
    println!("");
    try!(configure::display(config));
    Ok(())
}

/// Install a package
#[allow(dead_code)]
fn install(config: &Config) -> BldrResult<()> {
    banner();
    println!("Installing {}", Yellow.bold().paint(config.package()));
    let pkg_file = try!(install::from_url(config.package(), config.url()));
    try!(install::verify(config.package(), &pkg_file));
    try!(install::unpack(config.package(), &pkg_file));
    Ok(())
}

/// Start a service
#[allow(dead_code)]
fn start(config: &Config) -> BldrResult<()> {
    banner();
    println!("Starting {}", Yellow.bold().paint(config.package()));
    try!(sidecar::run(config.package()));
    try!(start::package(config));
    println!("Finished with {}", Yellow.bold().paint(config.package()));
    Ok(())
}

/// Run a package repo
#[allow(dead_code)]
fn repo(config: &Config) -> BldrResult<()> {
    banner();
    println!("Starting Bldr Repository at {}", Yellow.bold().paint(config.path()));
    try!(repo::start(&config));
    println!("Finished with {}", Yellow.bold().paint(config.package()));
    Ok(())
}

/// Upload a package
#[allow(dead_code)]
fn upload(config: &Config) -> BldrResult<()> {
    banner();
    println!("Upload Bldr Package {}", Yellow.bold().paint(config.package()));
    try!(upload::package(&config));
    println!("Finished with {}", Yellow.bold().paint(config.package()));
    Ok(())
}

/// Download/install a key
#[allow(dead_code)]
fn key(config: &Config) -> BldrResult<()> {
    banner();
    println!("Installing key {}", Yellow.bold().paint(config.key()));
    try!(key::install(&config));
    Ok(())
}

/// Upload a key
#[allow(dead_code)]
fn key_upload(config: &Config) -> BldrResult<()> {
    banner();
    println!("Upload Bldr key {}", Yellow.bold().paint(config.key()));
    try!(key_upload::key(&config));
    println!("Finished with {}", Yellow.bold().paint(config.key()));
    Ok(())
}

/// Exit with an error message and the right status code
#[allow(dead_code)]
fn exit_with(e: BldrError, code: i32) {
    println!("{}", Red.bold().paint(&format!("{}", e)));
    process::exit(code)
}
