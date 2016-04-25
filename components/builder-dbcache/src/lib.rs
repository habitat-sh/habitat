// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

#[macro_use]
extern crate log;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;
extern crate rustc_serialize;
extern crate time;

pub mod data_store;
pub mod error;
pub mod model;

pub use self::data_store::{ConnectionPool, DataRecord, ValueTable, InstaId, IndexTable, RecordTable, Table};
pub use self::model::Model;
pub use self::error::{Error, Result};
