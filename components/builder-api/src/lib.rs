// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate habitat_core as core;
extern crate habitat_depot as depot;
extern crate habitat_net as hab_net;
extern crate habitat_builder_protocol as protocol;
extern crate iron;
#[macro_use]
extern crate log;
extern crate mount;
extern crate protobuf;
extern crate redis;
#[macro_use]
extern crate router;
extern crate rustc_serialize;
extern crate toml;
extern crate unicase;
extern crate urlencoded;
extern crate zmq;

pub mod config;
pub mod error;
pub mod http;
pub mod broker;
pub mod server;

pub use self::config::Config;
pub use self::error::{Error, Result};
