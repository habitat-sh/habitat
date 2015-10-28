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
//! The documentation for bldr-build is generated automatically from the script itself, [and can be
//! found here](bldr-build/bldr-build.html). You can find it in the source tree at
//! `packages/bldr-build`.
//!
//! # bldr
//!
//! Bldr is primarily utilized through the `bldr` command; it can also be used from within Rust as
//! a library. This documentation covers both uses; it explains how things are used from the
//! command line in close proximity to the documentation of the library itself. A few useful
//! starting points:
//!
//! * [The bldr Command Line Reference](command)
//! * [The bldr Sidecar; http interface to promises](sidecar)
//! * [The bldr Repo; http based package repository](repo)
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
extern crate wonder;

pub mod error;
pub mod command;
pub mod util;
pub mod pkg;
pub mod discovery;
pub mod topology;
pub mod state_machine;
pub mod sidecar;
pub mod health_check;
pub mod config;
pub mod repo;
