//! Bldr helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it, including setup instructions, in the README](README.html).
//!
//! Bldr contains two main components:
//!
//! * `bldr-build`, written in [bash](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html), which
//! takes a description of how to build a piece of software (a `Bldrfile`) and produces an atomic
//! package.
//! * `bldr`, a run-time executable that knows how to download, install, serve, and manage services
//! defined in packages.
//!
//! # bldr-build
//!
//!
#[macro_use] extern crate hyper;
#[macro_use] extern crate log;
extern crate tempdir;
extern crate mustache;
extern crate rustc_serialize;
extern crate toml;
extern crate ansi_term;
extern crate regex;
extern crate libc;
extern crate url;
extern crate inotify;
extern crate fnv;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate time;

/// All the Errors we expect
pub mod error;
/// All the Commands you can run from the CLI.
pub mod command;
/// Utility functions; gpg, http, permissions and system info
pub mod util;
/// The representation of a bldr package
pub mod pkg;
/// Interaction with service discovery; etcd
pub mod discovery;
/// Service topologies are implemented here
pub mod topology;
/// A generic state machine implementation, used by the topologies
pub mod state_machine;
/// The HTTP sidecar, for exposing promises
pub mod sidecar;
/// Our nagios compliant health check implementation, exposed through the sidecar
pub mod health_check;
/// Configuration - currently only from the CLI options
pub mod config;
/// The HTTP package repository
pub mod repo;

