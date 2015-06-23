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
use toml;
use rustc_serialize::json::Json;
use std::env;
use std::collections::BTreeMap;
use std::io::Read;
use ansi_term::Colour::{White};

use error::BldrResult;

pub fn enabled() -> Option<String> {
    match env::var("BLDR_CONFIG_ETCD") {
        Ok(val) => Some(val),
        Err(_) => {
            debug!("No BLDR_CONFIG_ETCD, so not checking etcd for configuration values");
            return None;
        }
    }
}

// pub enum SetOption {
//     PrevValue(String),
//     PrevIndex(String),
//     PrevExist(bool),
//     Ttl(i64),
// }

// fn encode_options(options: &[SetOption]) -> String {
//     let mut encoded_opts = Vec::new();
//     for x in options {
//         match x {
//             SetOption::PrevValue(s) => encoded_opts.push(("prevValue", &s)),
//             SetOption::PrevIndex(s) => encoded_opts.push(("prevIndex", &s)),
//             SetOption::PrevExist(bool) => encoded_opts.push(("prevExist", if bool { "true" } else { "false" })),
//             SetOption::Ttl(time) => encoded_opts.push(("ttl", &format!("{}", time))),
//         }
//     }
//     url::form_urlencoded::serialize(encoded_opts)
// }

pub fn set(key: &str, options: &[(&str, &str)]) -> BldrResult<(StatusCode, String)> {
    let base_url = match enabled() {
        Some(url) => url,
        None => unreachable!()
    };
    let mut client = Client::new();
    let url = format!("{}/v2/keys/bldr/{}", base_url, key);
    debug!("Requesting {}", url);
    let req_body = url::form_urlencoded::serialize(options);
    debug!("Requesting body {}", req_body);
    let request = client.put(&url)
        .header(ContentType::form_url_encoded())
        .body(&req_body);
    let mut res = try!(request.send());
    debug!("Response: {:?}", res);
    let mut response_body = String::new();
    try!(res.read_to_string(&mut response_body));
    debug!("Response body: {:?}", response_body);
    Ok((res.status, response_body))
}

// pub fn get(key: &str, wait: bool) -> BldrResult<(StatusCode, BTreeMap<String, toml::Value>)> {
//     let base_url = match enabled() {
//         Some(url) => url,
//         None => unreachable!()
//     };
//     let mut client = Client::new();
//     let url = format!("{}/v2/keys/bldr/{}?wait={}", base_url, key, wait);
//     let mut res = try!(client.get(&url).send());
//     debug!("Response: {:?}", res);
//     let mut response_body = String::new();
//     try!(res.read_to_string(&mut response_body));
//     let body_as_json = try!(Json::from_str(&response_body));
//     let toml_config_value = match body_as_json.find_path(&["node", "value"]) {
//         Some(json_value) => {
//             match json_value.as_string() {
//                 Some(json_value_string) => json_value_string,
//                 None => {
//                     debug!("Invalid json value for etc node/value - not a string!");
//                     return None;
//                 }
//             }
//         },
//         None => {
//             debug!("No node/value present in etcd response json");
//             return None;
//         }
//     };
//     let mut toml_parser = toml::Parser::new(&toml_config_value);
//     let toml_value = try!(toml_parser.parse());
//     Ok((res.status, toml_value))
// }

pub fn get_config(pkg: &str, key: &str, wait: bool) -> Option<BTreeMap<String, toml::Value>> {
    let pkg_print = if wait {
        format!("{}({})", pkg, White.bold().paint("C"))
    } else {
        pkg.to_string()
    };
    let base_url = match enabled() {
        Some(url) => url,
        None => return None
    };
    let mut client = Client::new();
    let mut res = match client.get(&format!("{}/v2/keys/bldr/{}/{}?wait={}", base_url, pkg, key, wait)).send() {
        Ok(res) => res,
        Err(e) => {
            println!("   {}: Invalid request to etcd for config: {:?}", pkg_print, e);
            return None;
        }
    };
    debug!("Response: {:?}", res);
    let mut response_body = String::new();
    match res.read_to_string(&mut response_body) {
        Ok(_) => {},
        Err(e) => {
            println!("   {}: Failed to read request body from etcd request: {:?}", pkg_print, e);
            return None;
        }
    }
    let body_as_json = match Json::from_str(&response_body) {
        Ok(body) => body,
        Err(e) => {
            println!("   {}: Failed to parse request body as json: {:?}", pkg_print, e);
            return None;
        }
    };
    let toml_config_value = match body_as_json.find_path(&["node", "value"]) {
        Some(json_value) => {
            match json_value.as_string() {
                Some(json_value_string) => json_value_string,
                None => {
                    println!("   {}: Invalid json value for etc node/value - not a string!", pkg_print);
                    return None;
                }
            }
        },
        None => {
            println!("   {}: No node/value present in etcd response json", pkg_print);
            return None;
        }
    };
    let mut toml_parser = toml::Parser::new(&toml_config_value);
    match toml_parser.parse() {
        Some(toml_value) => return Some(toml_value),
        None => {
            println!("   {}: Invalid toml from etcd: {:?}", pkg_print, toml_parser.errors);
            return None
        }
    }
}

