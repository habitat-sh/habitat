// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! Bldr helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it, including setup instructions, in the README](README.html).
//!
//! Bldr contains two main components:
//!
//! * `bldr-build`, takes a plan ('plan.sh'), a description of how to build a piece of software, written
//! in [bash](http://tldp.org/HOWTO/Bash-Prog-Intro-HOWTO.html), which produces an atomic package.
//! * `bldr`, a run-time executable that knows how to download, install, serve, and manage services
//! defined in packages.
//!
//! # bldr-build
//!
//! The documentation for bldr-build is generated automatically from the script itself, [and can be
//! found here](bldr-build/bldr-build.html). You can find it in the source tree at
//! `plans/bldr-build`.
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
//! * [The bldr Depot; http based package repository](depot)
//!

extern crate bldr_core as core;
extern crate bldr_depot_client as depot_client;
extern crate bldr_depot_core as depot_core;
extern crate bincode;
#[macro_use]
extern crate bitflags;
extern crate crypto;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate tempdir;
extern crate mustache;
extern crate rustc_serialize;
extern crate toml;
extern crate ansi_term;
extern crate gpgme;
#[macro_use]
extern crate lazy_static;
extern crate libarchive;
extern crate regex;
extern crate libc;
extern crate url;
extern crate fnv;
extern crate iron;
extern crate lmdb_sys;
#[macro_use]
extern crate router;
extern crate time;
extern crate wonder;
extern crate uuid;
extern crate utp;
extern crate rpassword;
extern crate rand;
extern crate threadpool;
extern crate urlencoded;
extern crate openssl;
extern crate walkdir;

#[macro_export]
/// Creates a new BldrError, embedding the current file name, line number, column, and module path.
macro_rules! bldr_error {
    ($p: expr) => {
        {
            use $crate::error::BldrError;
            BldrError::new($p, LOGKEY, file!(), line!(), column!())
        }
    }
}

#[macro_export]
/// Works the same as the print! macro, but uses our StructuredOutput formatter.
macro_rules! output {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new("bldr",
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            print!("{}", so);
        }
    };
    (preamble $preamble: expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new($preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            print!("{}", so);
        }
    };
    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new("bldr",
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            print!("{}", so);
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new($preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            print!("{}", so);
        }
    };
}

#[macro_export]
/// Works the same as println!, but uses our structured output formatter.
macro_rules! outputln {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new("bldr",
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            println!("{}", so);
        }
    };
    (preamble $preamble:expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new($preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            println!("{}", so);
        }
    };
    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new("bldr",
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            println!("{}", so);
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new($preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            println!("{}", so);
        }
    }
}

#[macro_export]
/// Works the same as format!, but uses our structured output formatter.
macro_rules! output_format {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new("bldr",
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            format!("{}", so)
        }
    };
    (preamble $preamble:expr, $content: expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new($preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           $content);
            format!("{}", so)
        }
    };
    (preamble $preamble:expr, logkey $logkey:expr) => {
        {
            use $crate::output::StructuredOutput;
            let so = StructuredOutput::new($preamble,
                                           $logkey,
                                           line!(),
                                           file!(),
                                           column!(),
                                           "");
            format!("{}", so)
        }
    };

    ($content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new("bldr",
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            format!("{}", so)
        }
    };
    (preamble $preamble: expr, $content: expr, $($arg:tt)*) => {
        {
            use $crate::output::StructuredOutput;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new($preamble,
                                           LOGKEY,
                                           line!(),
                                           file!(),
                                           column!(),
                                           &content);
            format!("{}", so)
        }
    }
}

pub mod output;
pub mod error;
pub mod command;
pub mod util;
pub mod package;
pub mod discovery;
pub mod topology;
pub mod state_machine;
pub mod sidecar;
pub mod health_check;
pub mod config;
pub mod user_config;
pub mod service_config;
pub mod census;
pub mod gossip;
pub mod election;

#[allow(dead_code)]
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
