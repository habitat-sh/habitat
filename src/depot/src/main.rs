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

extern crate ansi_term;
#[macro_use]
extern crate bldr;
extern crate docopt;
extern crate env_logger;
extern crate lmdb_rs;
#[macro_use]
extern crate log;
extern crate rustc_serialize;

pub mod error;

use std::process;

use docopt::Docopt;

use error::{DepotResult, DepotError, ErrorKind};

#[allow(dead_code)]
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[allow(dead_code)]
#[cfg_attr(rustfmt, rustfmt_skip)]
static USAGE: &'static str = "
Usage: bldr-depot [-p <path>] [--port=<port>] [-vn]

Options::
    -p, --path=<path>   The path to use for a repository [default: /opt/bldr/srvc/bldr/data]
    -v, --verbose       Verbose output; shows line numbers
    -n, --no-color      Turn ANSI color off :(
";

/// Our output key
static LOGKEY: &'static str = "DMN";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_path: Option<String>,
    flag_port: Option<u16>,
    flag_verbose: bool,
    flag_no_color: bool,
}

fn main() {
    env_logger::init().unwrap();

    let args: Args = Docopt::new(USAGE)
                         .and_then(|d| d.decode())
                         .unwrap_or_else(|e| e.exit());
    debug!("Docopt Args: {:?}", args);
    match start(args) {
        Ok(_) => (),
        Err(e) => exit_with(e, 1),
    }
}

fn start(_args: Args) -> DepotResult<()> {
    Err(bldr_error!(ErrorKind::NotImplemented))
}

/// Exit with an error message and the right status code
#[allow(dead_code)]
fn exit_with(e: DepotError, code: i32) {
    println!("{}", e);
    process::exit(code)
}
