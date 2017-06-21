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

use hab_net;

use std::error;
use std::fmt;
use std::result;

use r2d2;
use postgres;

#[derive(Debug)]
pub enum Error {
    AsyncListen(postgres::error::Error),
    AsyncNotification(postgres::error::Error),
    AsyncMalformedChannel(String),
    AsyncMalformedShardId(String),
    AsyncFunctionCheck(postgres::error::Error),
    AsyncFunctionUpdate(postgres::error::Error),
    ConnectionTimeout(r2d2::GetTimeout),
    FunctionCreate(postgres::error::Error),
    FunctionDrop(postgres::error::Error),
    FunctionRun(postgres::error::Error),
    NetError(hab_net::Error),
    Migration(postgres::error::Error),
    MigrationCheck(postgres::error::Error),
    MigrationTable(postgres::error::Error),
    MigrationTracking(postgres::error::Error),
    MigrationLock(postgres::error::Error),
    PostgresConnect(postgres::error::ConnectError),
    SchemaCreate(postgres::error::Error),
    SchemaDrop(postgres::error::Error),
    SchemaSwitch(postgres::error::Error),
    SetSearchPath(postgres::error::Error),
    TransactionCreate(postgres::error::Error),
    TransactionCommit(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::AsyncListen(ref e) => format!("Error setting up async listen, {}", e),
            Error::AsyncNotification(ref e) => format!("Error getting async notification, {}", e),
            Error::AsyncMalformedChannel(ref e) => {
                format!("Notification received, but the channel is malformed, {}", e)
            }
            Error::AsyncMalformedShardId(ref e) => {
                format!(
                    "Notification received, but the channels shard id is malformed, {}",
                    e
                )
            }
            Error::AsyncFunctionCheck(ref e) => {
                format!("Async function database check failed, {}", e)
            }
            Error::AsyncFunctionUpdate(ref e) => {
                format!("Async function database update failed, {}", e)
            }
            Error::ConnectionTimeout(ref e) => format!("Connection timeout, {}", e),
            Error::FunctionCreate(ref e) => format!("Error creating a function: {}", e),
            Error::FunctionDrop(ref e) => format!("Error dropping a function: {}", e),
            Error::FunctionRun(ref e) => format!("Error running a function: {}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::Migration(ref e) => format!("Error executing migration: {}", e),
            Error::MigrationCheck(ref e) => format!("Error checking if a migration has run: {}", e),
            Error::MigrationTable(ref e) => {
                format!("Error creating migration tracking table: {}", e)
            }
            Error::MigrationTracking(ref e) => {
                format!("Error updating migration tracking table: {}", e)
            }
            Error::MigrationLock(ref e) => format!("Error getting migration lock: {}", e),
            Error::PostgresConnect(ref e) => format!("Postgres connection error: {}", e),
            Error::SchemaCreate(ref e) => format!("Error creating schema: {}", e),
            Error::SchemaDrop(ref e) => format!("Error dropping schema: {}", e),
            Error::SchemaSwitch(ref e) => format!("Error switching schema: {}", e),
            Error::SetSearchPath(ref e) => format!("Error setting local search path: {}", e),
            Error::TransactionCreate(ref e) => format!("Error creating transaction: {}", e),
            Error::TransactionCommit(ref e) => format!("Error committing transaction: {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AsyncListen(ref e) => e.description(),
            Error::AsyncNotification(ref e) => e.description(),
            Error::AsyncMalformedChannel(_) => "Error parsing a channel string",
            Error::AsyncMalformedShardId(_) => "Error parsing a channel strings shard id",
            Error::AsyncFunctionCheck(ref e) => e.description(),
            Error::AsyncFunctionUpdate(ref e) => e.description(),
            Error::ConnectionTimeout(ref e) => e.description(),
            Error::FunctionCreate(_) => "Error creating database function",
            Error::FunctionDrop(_) => "Error dropping database function",
            Error::FunctionRun(_) => "Error running a database function",
            Error::NetError(ref err) => err.description(),
            Error::Migration(_) => "Error executing migration",
            Error::MigrationCheck(_) => "Error checking if a migration has run",
            Error::MigrationTable(_) => "Error creat2ing migration tracking table",
            Error::MigrationTracking(_) => "Error updating migration tracking table",
            Error::MigrationLock(_) => "Error getting migration lock",
            Error::PostgresConnect(ref e) => e.description(),
            Error::SchemaCreate(_) => "Error creating a schema",
            Error::SchemaDrop(_) => "Error dropping a schema",
            Error::SchemaSwitch(_) => "Error switching schema",
            Error::SetSearchPath(_) => "Error setting local search path",
            Error::TransactionCreate(_) => "Error creating a transaction",
            Error::TransactionCommit(_) => "Error committing a transaction",
        }
    }
}

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Self {
        Error::ConnectionTimeout(err)
    }
}

impl From<postgres::error::ConnectError> for Error {
    fn from(err: postgres::error::ConnectError) -> Self {
        Error::PostgresConnect(err)
    }
}
