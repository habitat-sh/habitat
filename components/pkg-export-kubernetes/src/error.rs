// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use crate::hcore;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid bind specification '{}'", _0)]
    InvalidBindSpec(String),
    #[fail(
        display = "Invalid topology '{}'. Possible values: standalone, leader",
        _0
    )]
    InvalidTopology(String),
    #[fail(
        display = "Invalid binding \"{}\", must be of the form <NAME>:<SERVICE_GROUP> where \
                   <NAME> is a service name and <SERVICE_GROUP> is a valid service group",
        _0
    )]
    InvalidBinding(String),
    #[fail(
        display = "Invalid environment variable \"{}\", must be in the form <NAME>=<VALUE>",
        _0
    )]
    InvalidEnvironmentVariable(String),
    #[fail(
        display = "Invalid persistent storage specification \"{}\", must in the form \
                   <SIZE>:<PATH>:<STORAGE_CLASS_NAME> where <SIZE> is a size in bytes (with E, P, \
                   T, G, M, K or Ei, Pi, Ti, Gi, Mi, Ki suffixes, or in 123e5 form, see K8s docs \
                   for more details), <PATH> is an absolute path",
        _0
    )]
    InvalidPersistentStorageSpec(String),
    #[fail(display = "{}", _0)]
    HabitatCore(hcore::Error),
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error { Error::HabitatCore(err) }
}
