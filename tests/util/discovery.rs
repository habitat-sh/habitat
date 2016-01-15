// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

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
