// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate fnv;
extern crate habitat_core as hab_core;
#[macro_use]
extern crate lazy_static;
extern crate protobuf;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate time;

pub mod error;
pub mod jobsrv;
pub mod message;
pub mod net;
pub mod routesrv;
pub mod search;
pub mod sessionsrv;
pub mod sharding;
pub mod originsrv;

pub use self::error::{ProtocolError, ProtocolResult};
pub use self::message::{Message, Protocol, Persistable, Routable, RouteKey};
pub use self::sharding::{ShardId, SHARD_COUNT, InstaId};
