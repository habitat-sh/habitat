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

//! A module containing the HTTP server and handlers for servicing client requests

pub mod controller;
pub mod headers;
pub mod helpers;
pub mod middleware;
pub mod rendering;

use hab_net::ErrCode;
use iron::status::Status;

pub fn net_err_to_http(err: ErrCode) -> Status {
    match err {
        ErrCode::TIMEOUT => Status::GatewayTimeout,
        ErrCode::REMOTE_REJECTED => Status::NotAcceptable,
        ErrCode::ENTITY_NOT_FOUND => Status::NotFound,
        ErrCode::ENTITY_CONFLICT => Status::Conflict,

        ErrCode::ACCESS_DENIED |
        ErrCode::SESSION_EXPIRED => Status::Unauthorized,

        ErrCode::BAD_REMOTE_REPLY |
        ErrCode::SECRET_KEY_FETCH |
        ErrCode::VCS_CLONE => Status::BadGateway,

        ErrCode::NO_SHARD |
        ErrCode::SOCK |
        ErrCode::REMOTE_UNAVAILABLE => Status::ServiceUnavailable,

        ErrCode::AUTH_SCOPE |
        ErrCode::GROUP_NOT_COMPLETE |
        ErrCode::PARTIAL_JOB_GROUP_PROMOTE => Status::Forbidden,

        ErrCode::BUG |
        ErrCode::POST_PROCESSOR |
        ErrCode::BUILD |
        ErrCode::SYS |
        ErrCode::DATA_STORE |
        ErrCode::WORKSPACE_SETUP |
        ErrCode::SECRET_KEY_IMPORT |
        ErrCode::REG_CONFLICT |
        ErrCode::REG_NOT_FOUND => Status::InternalServerError,
    }
}
