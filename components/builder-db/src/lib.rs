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

//! # Builder database design
//!
//! The database is designed to scale vertically, but is set up to allow for
//! horizontal partitioning in the future.
//!
//! The tables are denormalized. Rather than follow the typical 3-4nf style, we
//! accept duplication of data in order to ensure speedy retrieval.
//!
//!   * Joins are strictly forbidden. Prefer arrays comprised of natural keys, or of
//!     hstore/jsonb.
//!   * Database activity is driven by stored procedure; no raw queries in normal
//!     course of events.
//!   * The application itself is responsible for the schema; mirgrations are inline,
//!     and any service that needs it can upgrade the database.
//!   * Roll forward schema changes only. You can add columns, you can migrate data,
//!     but you can't remove columns.
//!   * All tables must include `created_at` and `updated_at`, as they are
//!     automatically managed
//!
//! The `builder-db` module provides the backend for all database access - it provides:
//!
//! * Connection pooling
//! * Migrations
//! * Code for checking the migrations of registered backends
//! * Shared test harness for crates that consume it
//!
//! In individual services, you implement a data_store module, which defines the
//! schema, the procedures, the migrations, and the wrapper functions allowing your
//! service handlers to work.
//!
//! ## A sidebar about stored procedures
//!
//! We drive much of the database access through stored procedures. It's controversial - we get it. Here is why we're doing it:
//!
//! 1. We want extensive test coverage of the database, and its migrations. It's a requirement.
//! 1. We want the service itself to handle migrations at boot. It's automatic.
//! 1. Versioned stored procedures will enforce several of the above rules - the big one being that only append-style migrations are allowed to tables. If you try and build a migration that breaks a query, the migration will fail inside its transaction, with the database being unharmed.
//! 1. Access to the database through the stored procedure calls means we can use the same functions for maintenance as required.
//! 1. We can ensure that a release of the service that manages a database component uses only the functions it was designed for - when the service upgrades, it gets the upgraded functions. Rollbacks will work cleanly.
//! 1. In postgresql, and plpgsql function is automatically prepped by the query parser and analyzer. This means that, in general, they are as fast as a prepared statement, all the time, without having to actually build a prepared statement per-connection.
//!
//! Answers to common questions:
//!
//! 1. Q: Isn't this a lot of ceremony? A: Yes. We think its worth it.
//! 1. Q: I don't know plpgsql. A: That's not a question. It's not hard. You learned Rust to get here :)
//! 1. Q: Won't this impact database performance? A: Probably, but in a positive way. Think about it - 99% of the time, you run the same queries all the time. This is the equivalent of having them prepared in advance for you all the time.
//! 1. Q: But what about those horror stories? A: The horror stories are about encoding your business logic in the database. For example, doing complex transformations on the data, or map reducing it, or all kinds of other crazy business. Both our application and our access patterns mean we likely won't need to do a whole lot of that.
//!

extern crate fnv;
#[macro_use]
extern crate log;
extern crate num_cpus;
extern crate postgres;
extern crate threadpool;
extern crate rand;
extern crate time;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate fallible_iterator;
extern crate habitat_builder_protocol as protocol;
extern crate habitat_net as hab_net;

pub mod config;
pub mod error;
pub mod migration;
pub mod pool;
pub mod async;
pub mod test;
