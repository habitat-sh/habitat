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

#[macro_use]
extern crate bitflags;

#[cfg(target_os = "linux")]
extern crate caps;
extern crate clap;
extern crate cpu_time;
#[cfg(windows)]
extern crate ctrlc;
#[macro_use]
extern crate features;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate notify;
extern crate num_cpus;
#[cfg(unix)]
extern crate proc_self;
#[macro_use]
extern crate prometheus;
extern crate prost;
extern crate protobuf;
extern crate rand;
extern crate regex;
extern crate rustls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[allow(unused_imports)]
#[macro_use]
extern crate serde_json;
extern crate time as time_crate;

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
pub mod http_gateway;
pub mod manager;
mod sys;
#[cfg(test)]
pub mod test_helpers;
pub mod util;

use std::env;

/// List enables printing out the list features which can be dynamically enabled
/// TestExit enables triggering an abrupt exit to simulate failures
/// TestBootFail exits with a fatal error before even calling boot()
/// Search for feat::is_enabled(feat::FeatureName) to learn more
features! {
    pub mod feat {
        const List          = 0b0000_0001,
        const TestExit      = 0b0000_0010,
        const TestBootFail  = 0b0000_0100,
        const RedactHTTP    = 0b0000_1000,
        const IgnoreSignals = 0b0001_0000,
        const InstallHook   = 0b0010_0000
    }
}

pub const PRODUCT: &str = "hab-sup";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
