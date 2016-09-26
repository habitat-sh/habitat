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
pub mod headers;
pub mod middleware;
pub mod rendering;

use hyper::status::StatusCode;
use protocol::net::ErrCode;

pub fn net_err_to_http(err: ErrCode) -> StatusCode {
    match err {
        ErrCode::BUG => StatusCode::InternalServerError,
        ErrCode::TIMEOUT => StatusCode::RequestTimeout,
        ErrCode::REMOTE_REJECTED => StatusCode::NotAcceptable,
        ErrCode::BAD_REMOTE_REPLY => StatusCode::BadGateway,
        ErrCode::ENTITY_NOT_FOUND => StatusCode::NotFound,
        ErrCode::NO_SHARD => StatusCode::ServiceUnavailable,
        ErrCode::ACCESS_DENIED => StatusCode::Unauthorized,
        ErrCode::SESSION_EXPIRED => StatusCode::Unauthorized,
        ErrCode::ENTITY_CONFLICT => StatusCode::Conflict,
        ErrCode::ZMQ => StatusCode::ServiceUnavailable,
        ErrCode::DATA_STORE => StatusCode::ServiceUnavailable,
        ErrCode::AUTH_SCOPE => StatusCode::Forbidden,
    }
}
