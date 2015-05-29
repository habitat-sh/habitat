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

use hyper::client::Client;
use toml;
use rustc_serialize::json::Json;
use std::env;
use std::collections::BTreeMap;
use std::io::Read;

pub fn get_config(pkg: &str) -> Option<BTreeMap<String, toml::Value>> {
    let base_url = match env::var("BLDR_CONFIG_ETCD") {
        Ok(val) => val,
        Err(_) => {
            debug!("No BLDR_CONFIG_ETCD, so not checking etcd for configuration values");
            return None;
        }
    };
    println!("   {}: Overlaying etcd configuration", pkg);
    let mut client = Client::new();
    let mut res = match client.get(&format!("{}/v2/keys/bldr/{}/config", base_url, pkg)).send() {
        Ok(res) => res,
        Err(e) => {
            println!("   {}: Invalid request to etcd for config: {:?}", pkg, e);
            return None;
        }
    };
    debug!("Response: {:?}", res);
    let mut response_body = String::new();
    match res.read_to_string(&mut response_body) {
        Ok(_) => {},
        Err(e) => {
            println!("   {}: Failed to read request body from etcd request: {:?}", pkg, e);
            return None;
        }
    }
    let body_as_json = match Json::from_str(&response_body) {
        Ok(body) => body,
        Err(e) => {
            println!("   {}: Failed to parse request body as json: {:?}", pkg, e);
            return None;
        }
    };
    let toml_config_value = match body_as_json.find_path(&["node", "value"]) {
        Some(json_value) => {
            match json_value.as_string() {
                Some(json_value_string) => json_value_string,
                None => {
                    println!("   {}: Invalid json value for etc node/value - not a string!", pkg);
                    return None;
                }
            }
        },
        None => {
            println!("   {}: No node/value present in etcd response json", pkg);
            return None;
        }
    };
    let mut toml_parser = toml::Parser::new(&toml_config_value);
    match toml_parser.parse() {
        Some(toml_value) => return Some(toml_value),
        None => {
            println!("   {}: Invalid toml from etcd: {:?}", pkg, toml_parser.errors);
            return None
        }
    }
}
