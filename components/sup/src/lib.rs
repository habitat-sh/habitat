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

extern crate clap;
extern crate cpu_time;
#[cfg(windows)]
extern crate ctrlc;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate notify;
extern crate num_cpus;
#[cfg(unix)]
extern crate palaver;
#[macro_use]
extern crate prometheus;
extern crate prost;
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
macro_rules! sup_error {
    ($p:expr) => {{
        use $crate::error::SupError;
        SupError::new($p)
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
pub mod event;
pub mod http_gateway;
pub mod manager;
mod sup_futures;
mod sys;
#[cfg(test)]
pub mod test_helpers;
pub mod util;

use std::env;

pub const PRODUCT: &str = "hab-sup";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
