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

extern crate habitat_builder_protocol as protocol;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate protobuf;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rustc_serialize;
extern crate time;

pub mod config;
pub mod data_store;
pub mod error;

pub use self::data_store::{ConnectionPool, Bucket, BasicSet, ExpiringSet, InstaSet, IndexSet};
pub use self::error::{Error, Result};
