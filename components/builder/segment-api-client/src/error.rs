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

use std::collections::HashMap;
use std::error;
use std::fmt;
use std::io;

use base64;
use hyper;
use serde_json;

pub type SegmentResult<T> = Result<T, SegmentError>;

#[derive(Debug)]
pub enum SegmentError {
    ApiError(hyper::status::StatusCode, HashMap<String, String>),
    ContentDecode(base64::DecodeError),
    HttpClient(hyper::Error),
    HttpClientParse(hyper::error::ParseError),
    HttpResponse(hyper::status::StatusCode),
    IO(io::Error),
    Serialization(serde_json::Error),
}

impl fmt::Display for SegmentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            SegmentError::ApiError(ref code, ref response) => {
                format!(
                    "Received a non-200 response, status={}, response={:?}",
                    code,
                    response
                )
            }
            SegmentError::ContentDecode(ref e) => format!("{}", e),
            SegmentError::HttpClient(ref e) => format!("{}", e),
            SegmentError::HttpClientParse(ref e) => format!("{}", e),
            SegmentError::HttpResponse(ref e) => format!("{}", e),
            SegmentError::IO(ref e) => format!("{}", e),
            SegmentError::Serialization(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for SegmentError {
    fn description(&self) -> &str {
        match *self {
            SegmentError::ApiError(_, _) => "Response returned a non-200 status code.",
            SegmentError::ContentDecode(ref err) => err.description(),
            SegmentError::HttpClient(ref err) => err.description(),
            SegmentError::HttpClientParse(ref err) => err.description(),
            SegmentError::HttpResponse(_) => "Non-200 HTTP response.",
            SegmentError::IO(ref err) => err.description(),
            SegmentError::Serialization(ref err) => err.description(),
        }
    }
}
