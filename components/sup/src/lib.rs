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

//! Habitat helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it on the website](https://www.habitat.sh/).
//!
//! Habitat contains two main components:
//!
//! * `hab-plan-build`, takes a plan ('plan.sh'), a description of how to build a piece of software,
//! written in [bash](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html), which produces an atomic
//! package.
//! * `hab-sup`, a run-time executable that knows how to download, install, serve, and
//! manage services defined in packages.
//!
//! # hab-plan-build
//!
//! The documentation for hab-plan-build is generated automatically from the script itself, [and
//! can be found here](hab-plan-build/hab-plan-build.html). You can find it in the source tree at
//! `components/plan-build`.
//!
//! # The Supervisor
//!
//! The Supervisor is primarily utilized through the `hab-sup` command; it can also be used from
//! within Rust as a library. This documentation covers both uses; it explains how things are used
//! from the command line in close proximity to the documentation of the library itself. A few
//! useful starting points:
//!
//! * [The Habitat Command Line Reference](command)
//! * [The Habitat Supervisor Sidecar; http interface to promises](sidecar)

use actix;


#[macro_use]
extern crate bitflags;

#[cfg(target_os = "linux")]
extern crate caps;

#[cfg(windows)]
extern crate ctrlc;
#[macro_use]
extern crate features;
#[macro_use]
extern crate futures;
use glob;

use habitat_butterfly as butterfly;
#[cfg_attr(test, macro_use)]
use habitat_common as common;
#[macro_use]
extern crate habitat_core as hcore;
use habitat_api_client as api_client;
use habitat_eventsrv_client as eventsrv_client;
use habitat_launcher_client as launcher_client;
use habitat_sup_protocol as protocol;
use handlebars;
#[macro_use]
extern crate lazy_static;
use libc;
#[macro_use]
extern crate log;
use notify;
use num_cpus;
use prost;



use rustls;
use serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
use serde_transcode;
use serde_yaml;




#[macro_use]
extern crate tokio_core;
use toml;



#[cfg(test)]
extern crate json;

#[macro_export]
/// Creates a new SupError, embedding the current file name, line number, column, and module path.
macro_rules! sup_error {
    ($p:expr) => {{
        use $crate::error::SupError;
        SupError::new($p, LOGKEY, file!(), line!(), column!())
    }};
}

#[cfg(test)]
#[macro_use]
pub mod cli_test_helpers;
pub mod census;
pub mod cli;
pub mod command;
pub mod config;
pub mod ctl_gateway;
pub mod error;
pub mod fs;
pub mod http_gateway;
pub mod manager;
mod sys;
pub mod templating;
#[cfg(test)]
pub mod test_helpers;
pub mod util;

use std::env;
use std::path::PathBuf;

lazy_static! {
    pub static ref PROGRAM_NAME: String = {
        let arg0 = env::args().next().map(|p| PathBuf::from(p));
        arg0.as_ref()
            .and_then(|p| p.file_stem())
            .and_then(|p| p.to_str())
            .unwrap()
            .to_string()
    };
}

/// List enables printing out the list features which can be dynamically enabled
/// TestExit enables triggering an abrupt exit to simulate failures
/// TestBootFail exits with a fatal error before even calling boot()
/// Search for feat::is_enabled(feat::FeatureName) to learn more
features! {
    pub mod feat {
        const List          = 0b00000001,
        const TestExit      = 0b00000010,
        const TestBootFail  = 0b00000100,
        const RedactHTTP    = 0b00001000,
        const IgnoreSignals = 0b00010000
    }
}

pub const PRODUCT: &'static str = "hab-sup";
pub const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

#[derive(Copy, Clone)]
pub enum ShutdownReason {
    Departed,
    LauncherStopping,
    PkgUpdating,
    Signal,
    SvcStopCmd,
}
