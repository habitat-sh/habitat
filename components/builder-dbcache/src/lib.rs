// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

extern crate habitat_builder_protocol as protocol;
#[macro_use]
extern crate log;
extern crate protobuf;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rustc_serialize;
extern crate time;

pub mod data_store;
pub mod error;

pub use self::data_store::{ConnectionPool, Bucket, BasicSet, ExpiringSet, InstaSet, IndexSet};
pub use self::error::{Error, Result};
