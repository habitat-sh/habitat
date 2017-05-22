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

use aws_sdk_rust;
use db;
use extern_url;
use hab_core;
use hab_net;
use postgres;
use protobuf;
use r2d2;
use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;
use std::result;
use zmq;

#[derive(Debug)]
pub enum Error {
    BadPort(String),
    CaughtPanic(String, String),
    Db(db::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransaction(postgres::error::Error),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    HabitatCore(hab_core::Error),
    InvalidUrl,
    IO(io::Error),
    JobCreate(postgres::error::Error),
    JobGet(postgres::error::Error),
    JobLogArchive(u64, aws_sdk_rust::aws::errors::s3::S3Error),
    JobLogRetrieval(u64, aws_sdk_rust::aws::errors::s3::S3Error),
    JobMarkArchived(postgres::error::Error),
    JobPending(postgres::error::Error),
    JobReset(postgres::error::Error),
    JobSetLogUrl(postgres::error::Error),
    JobSetState(postgres::error::Error),
    LogDirDoesNotExist(PathBuf, io::Error),
    LogDirIsNotDir(PathBuf),
    LogDirNotWritable(PathBuf),
    NetError(hab_net::Error),
    ProjectJobsGet(postgres::error::Error),
    Protobuf(protobuf::ProtobufError),
    UnknownVCS,
    UnknownJobState,
    Zmq(zmq::Error),
}


pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::BadPort(ref e) => format!("{} is an invalid port. Valid range 1-65535.", e),
            Error::CaughtPanic(ref msg, ref source) => {
                format!("Caught a panic: {}. {}", msg, source)
            }
            Error::Db(ref e) => format!("{}", e),
            Error::DbPoolTimeout(ref e) => {
                format!("Timeout getting connection from the database pool, {}", e)
            }
            Error::DbTransaction(ref e) => format!("Database transaction error, {}", e),
            Error::DbTransactionStart(ref e) => {
                format!("Failed to start database transaction, {}", e)
            }
            Error::DbTransactionCommit(ref e) => {
                format!("Failed to commit database transaction, {}", e)
            }
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::InvalidUrl => format!("Bad URL!"),
            Error::IO(ref e) => format!("{}", e),
            Error::JobCreate(ref e) => format!("Database error creating a new job, {}", e),
            Error::JobGet(ref e) => format!("Database error getting job data, {}", e),
            Error::JobLogArchive(job_id, ref e) => {
                format!("Log archiving error for job {}, {}", job_id, e)
            }
            Error::JobLogRetrieval(job_id, ref e) => {
                format!("Log retrieval error for job {}, {}", job_id, e)
            }
            Error::JobMarkArchived(ref e) => {
                format!("Database error marking job as archived, {}", e)
            }
            Error::JobPending(ref e) => format!("Database error getting pending jobs, {}", e),
            Error::JobReset(ref e) => format!("Database error reseting jobs, {}", e),
            Error::JobSetLogUrl(ref e) => format!("Database error setting job log URL, {}", e),
            Error::JobSetState(ref e) => format!("Database error setting job state, {}", e),
            Error::LogDirDoesNotExist(ref path, ref e) => {
                format!("Build log directory {:?} doesn't exist!: {:?}", path, e)
            }
            Error::LogDirIsNotDir(ref path) => {
                format!("Build log directory {:?} is not a directory!", path)
            }
            Error::LogDirNotWritable(ref path) => {
                format!("Build log directory {:?} is not writable!", path)
            }
            Error::NetError(ref e) => format!("{}", e),
            Error::Protobuf(ref e) => format!("{}", e),
            Error::ProjectJobsGet(ref e) => {
                format!("Database error getting jobs for project, {}", e)
            }
            Error::UnknownVCS => format!("Unknown VCS"),
            Error::UnknownJobState => format!("Unknown Job State"),
            Error::Zmq(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadPort(_) => "Received an invalid port or a number outside of the valid range.",
            Error::CaughtPanic(_, _) => "Caught a panic",
            Error::Db(ref err) => err.description(),
            Error::DbPoolTimeout(ref err) => err.description(),
            Error::DbTransaction(ref err) => err.description(),
            Error::DbTransactionCommit(ref err) => err.description(),
            Error::DbTransactionStart(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::InvalidUrl => "Bad Url!",
            Error::JobCreate(ref err) => err.description(),
            Error::JobGet(ref err) => err.description(),
            Error::JobLogArchive(_, ref err) => err.description(),
            Error::JobLogRetrieval(_, ref err) => err.description(),
            Error::JobMarkArchived(ref err) => err.description(),
            Error::JobPending(ref err) => err.description(),
            Error::JobReset(ref err) => err.description(),
            Error::JobSetLogUrl(ref err) => err.description(),
            Error::JobSetState(ref err) => err.description(),
            Error::LogDirDoesNotExist(_, ref err) => err.description(),
            Error::LogDirIsNotDir(_) => "Build log directory is not a directory",
            Error::LogDirNotWritable(_) => "Build log directory is not writable",
            Error::NetError(ref err) => err.description(),
            Error::ProjectJobsGet(ref err) => err.description(),
            Error::Protobuf(ref err) => err.description(),
            Error::UnknownJobState => "Unknown Job State",
            Error::UnknownVCS => "Unknown VCS",
            Error::Zmq(ref err) => err.description(),
        }
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Error {
        Error::DbPoolTimeout(err)
    }
}

impl From<hab_core::Error> for Error {
    fn from(err: hab_core::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<protobuf::ProtobufError> for Error {
    fn from(err: protobuf::ProtobufError) -> Error {
        Error::Protobuf(err)
    }
}

impl From<zmq::Error> for Error {
    fn from(err: zmq::Error) -> Error {
        Error::Zmq(err)
    }
}

impl From<extern_url::ParseError> for Error {
    fn from(_err: extern_url::ParseError) -> Self {
        Error::InvalidUrl
    }
}
