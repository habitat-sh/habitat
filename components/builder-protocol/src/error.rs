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

use std::error;
use std::fmt;
use std::result;

#[derive(Debug)]
pub enum ProtocolError {
    BadSearchEntity(String),
    BadSearchKey(String),
}

pub type ProtocolResult<T> = result::Result<T, ProtocolError>;

impl fmt::Display for ProtocolError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            ProtocolError::BadSearchEntity(ref e) => {
                format!("Search not implemented for entity: {}", e)
            }
            ProtocolError::BadSearchKey(ref e) => {
                format!("Search not implemented for entity with key: {}", e)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for ProtocolError {
    fn description(&self) -> &str {
        match *self {
            ProtocolError::BadSearchEntity(_) => "Search not implemented for entity.",
            ProtocolError::BadSearchKey(_) => "Entity not indexed by the given key.",
        }
    }
}
