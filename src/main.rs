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

#[macro_use]
extern crate bldr;
extern crate rustc_serialize;
extern crate docopt;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate ansi_term;
extern crate libc;
use docopt::Docopt;
use std::process;
use ansi_term::Colour::Yellow;
use libc::funcs::posix88::unistd::execvp;
use std::ffi::CString;
use std::ptr;

use bldr::config::{Command, Config};
use bldr::error::{BldrResult, BldrError, ErrorKind};
use bldr::command::*;
use bldr::topology::Topology;

/// Our output key
static LOGKEY: &'static str = "MN";

/// The version number
#[allow(dead_code)]
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

/// The [docopts](http://burntsushi.net/rustdoc/docopt/index.html) usage
/// string. Determines what options are accepted.
#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
static USAGE: &'static str = "
Usage: bldr install <package> -u <url> [-vn]
       bldr start <package> [-u <url>] [--group=<group>] [--topology=<topology>] [--watch=<watch>...] [-vn]
       bldr sh [-v -n]
       bldr bash [-v -n]
       bldr repo [-p <path>] [--port=<port>] [-vn]
       bldr upload <package> -u <url> [-vn]
       bldr key <key> [-u <url>] [-vn]
       bldr key-upload <key> -u <url> [-vn]
       bldr config <package> [-vn]

Options::
    -g, --group=<group>        The service group; shared config and topology [default: default]
    -t, --topology=<topology>  Specify a service topology [default: standalone]
    -p, --path=<path>          The path to use for a repository [default: /opt/bldr/srvc/bldr/data]
    -u, --url=<url>            Use the specified package repository url
    -w, --watch=<watch>        One or more service groups to watch for updates
    -v, --verbose              Verbose output; shows line numbers
    -n, --no-color             Turn ANSI color off :(
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
    arg_package: Option<String>,
    arg_key: Option<String>,
    flag_path: String,
    flag_port: Option<u16>,
    flag_url: Option<String>,
    flag_topology: Option<String>,
    flag_group: String,
    flag_watch: Vec<String>,
    flag_verbose: bool,
    flag_no_color: bool,
}

/// Creates a [Config](config/struct.Config.html) from the [Args](/Args)
/// struct.
fn config_from_args(args: &Args, command: Command) -> BldrResult<Config> {
    let mut config = Config::new();
    config.set_command(command);
    if let Some(ref package) = args.arg_package {
        let (deriv, name, version, release) = try!(split_package_arg(package));
        config.set_deriv(deriv);
        config.set_package(name);
        if let Some(ver) = version {
            config.set_version(ver);
        }
        if let Some(rel) = release {
            config.set_release(rel);
        }
    }
    if let Some(ref arg_key) = args.arg_key {
        config.set_key(arg_key.clone());
    }
    if let Some(ref topology) = args.flag_topology {
        match topology.as_ref() {
            "standalone" => {
                config.set_topology(Topology::Standalone);
            }
            "leader" => {
                config.set_topology(Topology::Leader);
            }
            "initializer" => {
                config.set_topology(Topology::Initializer);
            }
            t => return Err(bldr_error!(ErrorKind::UnknownTopology(String::from(t)))),
        }
    }
    if let Some(port) = args.flag_port {
        config.set_port(port);
    }
    if let Some(ref url) = args.flag_url {
        config.set_url(url.clone());
    }
    config.set_group(args.flag_group.clone());
    config.set_watch(args.flag_watch.clone());
    config.set_path(args.flag_path.clone());
    if args.flag_verbose {
        bldr::output::set_verbose(true);
    }
    if args.flag_no_color {
        bldr::output::set_no_color(true);
    }
    Ok(config)
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
            match config_from_args(&args, Command::Install) {
                Ok(config) => install(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_start: true, ..} => {
            match config_from_args(&args, Command::Start) {
                Ok(config) => start(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_key: true, ..} => {
            match config_from_args(&args, Command::Key) {
                Ok(config) => key(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_key_upload: true, ..} => {
            match config_from_args(&args, Command::KeyUpload) {
                Ok(config) => key_upload(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_sh: true, ..} => {
            match config_from_args(&args, Command::Shell) {
                Ok(config) => shell(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_bash: true, ..} => {
            match config_from_args(&args, Command::Shell) {
                Ok(config) => shell(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_repo: true, ..} => {
            match config_from_args(&args, Command::Repo) {
                Ok(config) => repo(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_upload: true, ..} => {
            match config_from_args(&args, Command::Upload) {
                Ok(config) => upload(&config),
                Err(e) => Err(e),
            }
        }
        Args{cmd_config: true, ..} => {
            match config_from_args(&args, Command::Configuration) {
                Ok(config) => configure(&config),
                Err(e) => Err(e),
            }
        }
        _ => Err(bldr_error!(ErrorKind::CommandNotImplemented)),
    };

    match result {
        Ok(_) => {}
        Err(e) => exit_with(e, 1),
    }
}

/// Start a shell
#[allow(dead_code)]
fn shell(_config: &Config) -> BldrResult<()> {
    outputln!("Starting your shell; enjoy!");
    let shell_arg = try!(CString::new("sh"));
    let mut argv = [shell_arg.as_ptr(), ptr::null()];
    // Yeah, you don't know any better.. but we aren't coming back from
    // what happens next.
    unsafe {
        execvp(shell_arg.as_ptr(), argv.as_mut_ptr());
    }
    Ok(())
}

/// Show the configuration options for a service
#[allow(dead_code)]
fn configure(config: &Config) -> BldrResult<()> {
    try!(configure::display(config));
    Ok(())
}

/// Install a package
#[allow(dead_code)]
fn install(config: &Config) -> BldrResult<()> {
    outputln!("Installing {}", Yellow.bold().paint(&config.package_id()));
    try!(install::from_url(&config.url().as_ref().unwrap(),
                           config.deriv(),
                           config.package(),
                           config.version().clone(),
                           config.release().clone()));
    Ok(())
}

/// Start a service
#[allow(dead_code)]
fn start(config: &Config) -> BldrResult<()> {
    outputln!("Starting {}", Yellow.bold().paint(&config.package_id()));
    try!(start::package(config));
    outputln!("Finished with {}",
              Yellow.bold().paint(&config.package_id()));
    Ok(())
}

/// Run a package repo
#[allow(dead_code)]
fn repo(config: &Config) -> BldrResult<()> {
    outputln!("Starting Bldr Repository at {}",
              Yellow.bold().paint(config.path()));
    try!(repo::start(&config));
    outputln!("Finished with {}",
              Yellow.bold().paint(&config.package_id()));
    Ok(())
}

/// Upload a package
#[allow(dead_code)]
fn upload(config: &Config) -> BldrResult<()> {
    outputln!("Upload Bldr Package {}",
              Yellow.bold().paint(config.package()));
    try!(upload::package(&config));
    outputln!("Finished with {}",
              Yellow.bold().paint(&config.package_id()));
    Ok(())
}

/// Download/install a key
#[allow(dead_code)]
fn key(config: &Config) -> BldrResult<()> {
    outputln!("Installing key {}", Yellow.bold().paint(config.key()));
    try!(key::install(&config));
    Ok(())
}

/// Upload a key
#[allow(dead_code)]
fn key_upload(config: &Config) -> BldrResult<()> {
    outputln!("Upload Bldr key {}", Yellow.bold().paint(config.key()));
    try!(key_upload::key(&config));
    outputln!("Finished with {}", Yellow.bold().paint(config.key()));
    Ok(())
}

/// Exit with an error message and the right status code
#[allow(dead_code)]
fn exit_with(e: BldrError, code: i32) {
    println!("{}", e);
    process::exit(code)
}

fn split_package_arg(arg: &str) -> BldrResult<(String, String, Option<String>, Option<String>)> {
    let items: Vec<&str> = arg.split("/").collect();
    match items.len() {
        2 => Ok((items[0].to_string(), items[1].to_string(), None, None)),
        3 => Ok((items[0].to_string(),
                 items[1].to_string(),
                 Some(items[2].to_string()),
                 None)),
        4 => Ok((items[0].to_string(),
                 items[1].to_string(),
                 Some(items[2].to_string()),
                 Some(items[3].to_string()))),
        _ => Err(bldr_error!(ErrorKind::InvalidPackageIdent(arg.to_string()))),
    }
}
