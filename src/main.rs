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

use docopt::Docopt;
use bldr::error::{BldrResult, BldrError};
use bldr::command::*;
use std::process;
use ansi_term::Colour::{Red, Green, Yellow};
use std::thread;
use bldr::pkg;

#[allow(dead_code)]
static VERSION: &'static str = "0.0.1";

#[allow(dead_code)]
static USAGE: &'static str = "
Usage: bldr install <package> -u <url>
       bldr config <package> [--wait]
       bldr start <package>
       bldr key -u <url>

Options:
    -u, --url=<url>       Use a specific url for fetching the package
    -w, --wait            Wait for new configuration data
";

#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_install: bool,
    cmd_config: bool,
    cmd_start: bool,
    cmd_key: bool,
    arg_package: String,
    flag_url: String,
    flag_wait: bool,
}

#[allow(dead_code)]
fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    debug!("Docopt Args: {:?}", args);
    let result = match args {
        Args {
            cmd_install: true,
            arg_package: package,
            flag_url: url,
            ..
        } => install(&package, &url),
        Args {
            cmd_config: true,
            arg_package: package,
            flag_wait: wait,
            ..
        } => config(&package, wait),
        Args {
            cmd_start: true,
            arg_package: package,
            ..
        } => start(&package),
        Args {
            cmd_key: true,
            flag_url: url,
            ..
        } => key(&url),
        _ => Err(BldrError::CommandNotImplemented)
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
fn install(package: &str, url: &str) -> BldrResult<()> {
    banner();
    println!("Installing {}", Yellow.bold().paint(package));
    let pkg_file = try!(install::from_url(package, &url));
    try!(install::verify(package, &pkg_file));
    try!(install::unpack(package, &pkg_file));
    Ok(())
}

#[allow(dead_code)]
fn config(package: &str, wait: bool) -> BldrResult<()> {
    match wait {
        true => {
            let pkg = try!(pkg::latest(package));
            loop {
                println!("   {}: Waiting for configuration changes", package);
                match pkg.config_data(wait) {
                    Ok(_) => {},
                    Err(e) => {
                        println!("   {}: Had an error reconfiguring - {:?}", package, e)
                    }
                }
                thread::sleep_ms(1000);
            }
        },
        false => {
            banner();
            println!("Configuring {}", Yellow.bold().paint(package));
            try!(config::package(package, wait));
        }
    }
    Ok(())
}

#[allow(dead_code)]
fn start(package: &str) -> BldrResult<()> {
    banner();
    println!("Starting {}", Yellow.bold().paint(package));
    try!(config::package(package, false));
    try!(start::package(package));
    println!("Finished with {}", Yellow.bold().paint(package));
    Ok(())
}

#[allow(dead_code)]
fn key(url: &str) -> BldrResult<()> {
    banner();
    println!("Installing key {}", Yellow.bold().paint(url));
    try!(key::install(url));
    Ok(())
}

#[allow(dead_code)]
fn exit_with(e: BldrError, code: i32) {
    println!("{}", Red.bold().paint(&format!("{}", e)));
    process::exit(code)
}
