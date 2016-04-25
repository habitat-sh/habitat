// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

extern crate habitat_builder_dbcache as dbcache;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_net as hnet;
extern crate hyper;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rustc_serialize;
extern crate time;
#[macro_use]
extern crate zmq;

pub mod config;
pub mod data_store;
pub mod error;
pub mod oauth;
pub mod server;

pub use self::config::Config;
pub use self::error::{Error, Result};
