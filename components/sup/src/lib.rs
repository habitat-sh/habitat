// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

//! Bldr helps you build, manage, and run applications - on bare metal, in the cloud, and in
//! containers. You can [read more about it, including setup instructions, in the README](README.html).
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

extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate habitat_depot_client as depot_client;
extern crate habitat_depot_core as depot_core;
extern crate handlebars;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate log;
extern crate tempdir;
extern crate rustc_serialize;
extern crate toml;
extern crate ansi_term;
extern crate regex;
extern crate libc;
extern crate url;
extern crate iron;
#[macro_use]
extern crate router;
extern crate time;
extern crate wonder;
extern crate uuid;
extern crate utp;
extern crate rand;
extern crate threadpool;
extern crate urlencoded;
extern crate openssl;
#[macro_use]
extern crate lazy_static;

#[macro_export]
/// Creates a new SupError, embedding the current file name, line number, column, and module path.
macro_rules! sup_error {
    ($p: expr) => {
        {
            use $crate::error::SupError;
            SupError::new($p, LOGKEY, file!(), line!(), column!())
        }
    }
}

#[macro_export]
/// Works the same as the print! macro, but uses our StructuredOutput formatter.
macro_rules! output {
    ($content: expr) => {
        {
            use $crate::output::StructuredOutput;
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
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
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
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
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
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
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
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
            use $crate::PROGRAM_NAME;
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
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
            use $crate::PROGRAM_NAME;
            let content = format!($content, $($arg)*);
            let so = StructuredOutput::new(PROGRAM_NAME.as_str(),
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
pub mod topology;
pub mod state_machine;
pub mod sidecar;
pub mod health_check;
pub mod config;
pub mod service_config;
pub mod census;
pub mod gossip;
pub mod election;
pub mod supervisor;

use std::env;
use std::path::PathBuf;

lazy_static!{
    pub static ref PROGRAM_NAME: String = {
        let arg0 = env::args().next().map(|p| PathBuf::from(p));
        arg0.as_ref().and_then(|p| p.file_stem()).and_then(|p| p.to_str()).unwrap().to_string()
    };
}

#[allow(dead_code)]
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
