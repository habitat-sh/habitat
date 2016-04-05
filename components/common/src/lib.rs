// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate habitat_core as hcore;
extern crate habitat_depot_core as depot_core;
extern crate habitat_depot_client as depot_client;
#[macro_use]
extern crate log;
extern crate openssl;
extern crate rustc_serialize;
extern crate time;

pub use self::error::{Error, Result};

pub mod command;
pub mod config_file;
pub mod error;
