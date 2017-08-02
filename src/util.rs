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

use std::io::Read;

use hyper::client::Response;
use serde;
use serde_json;

use error::{Error, Result};

pub fn decoded_response<T>(mut response: Response) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let mut encoded = String::new();
    response.read_to_string(&mut encoded).map_err(Error::IO)?;
    debug!("Body: {:?}", encoded);
    let thing = serde_json::from_str(&encoded).map_err(Error::Json)?;
    Ok(thing)
}
