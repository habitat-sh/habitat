// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

#[macro_use]
extern crate bitflags;
extern crate habitat_core as hab_core;
extern crate protobuf;
extern crate redis;
extern crate rustc_serialize;
extern crate time;

pub mod depotsrv;
pub mod jobsrv;
pub mod net;
pub mod routesrv;
pub mod sessionsrv;
pub mod sharding;
pub mod vault;
mod message;

pub use self::message::{Message, Persistable, Routable, RouteKey};
pub use self::sharding::InstaId;
