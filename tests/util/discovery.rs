//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use hyper::header::ContentType;
use hyper::client::Client;
use hyper::status::StatusCode;
use url;
use std::thread;

pub fn clear(key: &str) {
    let url = format!("http://etcd:4001/v2/keys/bldr/simple_service/{}/{}?recursive=true",
                      thread::current().name().unwrap_or("main"),
                      key);
    let client = Client::new();
    let request = client.delete(&url);
    let res = request.send().unwrap_or_else(|x| panic!("Error: {:?}; Failed to delete {}", x, url));
    match res.status {
        StatusCode::Ok | StatusCode::Created | StatusCode::NotFound => {}
        e => panic!("Bad status code: {:?}", e),
    }
}

pub fn set(key: &str, body: &str) {
    let url = format!("http://etcd:4001/v2/keys/bldr/simple_service/{}/{}",
                      thread::current().name().unwrap_or("main"),
                      key);
    let client = Client::new();
    let req_options = vec![("value", body)];
    let req_body = url::form_urlencoded::serialize(&req_options);
    let request = client.put(&url)
                        .header(ContentType::form_url_encoded())
                        .body(&req_body);
    let res = request.send().unwrap_or_else(|x| {
        panic!("Error: {:?}; Failed to send {} with body: {}", x, url, body)
    });
    match res.status {
        StatusCode::Ok | StatusCode::Created => {}
        e => panic!("Bad status code: {:?}", e),
    }
}
