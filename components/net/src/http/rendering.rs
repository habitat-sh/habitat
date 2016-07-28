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

use iron::headers::ContentType;
use iron::mime::{Mime, TopLevel, SubLevel};
use iron::modifiers::Header;
use iron::prelude::*;
use iron::status;
use protocol::net::{NetError, ErrCode};
use rustc_serialize::json::{self, ToJson};

pub fn render_json<T: ToJson>(status: status::Status, response: &T) -> Response {
    let encoded = json::encode(&response.to_json()).unwrap();
    let headers = Header(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
    Response::with((status, encoded, headers))
}

/// Return an IronResult containing the body of a NetError and the appropriate HTTP response status
/// for the corresponding NetError.
///
/// For example, a NetError::ENTITY_NOT_FOUND will result in an HTTP response containing the body
/// of the NetError with an HTTP status of 404.
///
/// # Panics
///
/// * The given encoded message was not a NetError
/// * The given messsage could not be decoded
/// * The NetError could not be encoded to JSON
pub fn render_net_error(err: &NetError) -> Response {
    let status = match err.get_code() {
        ErrCode::ENTITY_NOT_FOUND => status::NotFound,
        ErrCode::ENTITY_CONFLICT => status::Conflict,
        ErrCode::NO_SHARD => status::ServiceUnavailable,
        ErrCode::TIMEOUT => status::RequestTimeout,
        ErrCode::BAD_REMOTE_REPLY => status::BadGateway,
        ErrCode::SESSION_EXPIRED => status::Unauthorized,
        _ => status::InternalServerError,
    };
    render_json(status, err)
}
