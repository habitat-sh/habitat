// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
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

//! This is the [SWIM](https://www.cs.cornell.edu/~asdas/research/dsn02-swim.pdf) implementation for Habitat.
//!
//! It implements SWIM+Susp+Inf. It uses Newscast-style "heat" tracking to share membership rumors,
//! while trying to keep UDP packet sizes below 512 bytes. It has the following changes:
//!
//! 1. It uses a single membership rumor with internal logic for applying the rumors state, rather
//!    than sending differential event messages.
//! 1. If an "Alive" membership rumor is recieved with a higher incarnation, it takes precedent
//!    over "Confirmed" membership rumors.
//! 1. Members can be marked "persistent", which means that they will always be taken through the
//!    Probe cycle, regardless of their status. This allows networks to heal from partitions.
//!
//! The library consists of a single SWIM Server, which has three working threads:
//!
//! 1. An inbound thread, handling receipt of SWIM messages.
//! 1. An outbound thread, which handles the Ping->PingReq cycle and protocol timing.
//! 1. An expire thread, which handles timing out suspected members.
//!
//! Start exploring the code base by following the thread of execution in the `server` module.

#[macro_use]
extern crate log;
extern crate protobuf;
extern crate rand;
extern crate time;
extern crate uuid;

#[macro_use]
pub mod trace;
mod error;
pub mod member;
mod message;
pub mod server;
pub mod rumor;
