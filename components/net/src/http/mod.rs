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

pub mod controller;
pub mod middleware;
pub mod rendering;

use std::error;
use std::fmt;

#[derive(Debug)]
pub enum HttpError {
    Authorization,
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            HttpError::Authorization => format!("not authenticated"),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for HttpError {
    fn description(&self) -> &str {
        match *self {
            HttpError::Authorization => "not authenticated",
        }
    }
}
