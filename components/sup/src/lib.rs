//! Habitat helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it on the website](https://www.habitat.sh/).
//!
//! Habitat contains two main components:
//!
//! * `hab-plan-build`, takes a plan ('plan.sh'), a description of how to build a piece of software,
//!   written in [bash](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html), which produces an atomic
//!   package.
//! * `hab-sup`, a run-time executable that knows how to download, install, serve, and manage
//!   services defined in packages.
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

pub mod census;
pub mod command;
pub mod ctl_gateway;
pub mod error;
pub mod event;
pub mod http_gateway;
pub mod lock_file;
pub mod logger; // must be pub if used in the `hab-sup` binary
pub mod manager;
mod sys;
pub mod util;

#[cfg(test)]
pub mod test_helpers;

use std::env;

pub const PRODUCT: &str = "hab-sup";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
