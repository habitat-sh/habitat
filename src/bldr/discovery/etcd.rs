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
use std::thread;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use time;
use std::mem;

use error::BldrResult;
use util;

pub fn enabled() -> Option<String> {
    match env::var("BLDR_CONFIG_ETCD") {
        Ok(val) => Some(val),
        Err(_) => {
            debug!("No BLDR_CONFIG_ETCD, so not checking etcd for configuration values");
            return None;
        }
    }
}

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

#[allow(non_snake_case)]
pub struct EtcdWrite {
    pub key: String,
    pub value: Option<String>,
    pub ttl: Option<u32>,
    pub dir: Option<bool>,
    pub prevExist: Option<bool>,
    pub prevIndex: Option<u64>,
    pub prevValue: Option<String>
}

pub fn write(options: EtcdWrite, watcher_tx: Sender<(StatusCode, String)>, watcher_rx: Receiver<bool>) {
    let _join = thread::Builder::new().name(format!("etcd-write:{}", options.key)).spawn(move || {
        let mut client = Client::new();

        let ttl_string = match options.ttl {
            Some(v) => format!("{}", v),
            None => String::new(),
        };
        let dir_string = match options.dir {
            Some(v) => format!("{}", v),
            None => String::new()
        };
        let pe_string = match options.prevExist {
            Some(v) => format!("{}", v),
            None => String::new()
        };
        let pi_string = match options.prevIndex {
            Some(v) => format!("{}", v),
            None => String::new()
        };

        let mut req_options = Vec::new();

        if let Some(ref value) = options.value {
            req_options.push(("value", value));
        }
        if let Some(ref value) = options.prevValue {
            req_options.push(("prevValue", value));
        }
        if ! ttl_string.is_empty() {
            req_options.push(("ttl", &ttl_string))
        }
        if ! dir_string.is_empty() {
            req_options.push(("dir", &dir_string))
        }
        if ! pe_string.is_empty() {
            req_options.push(("prevExist", &pe_string))
        }
        if ! pi_string.is_empty() {
            req_options.push(("prevIndex", &pi_string))
        }

        let base_url = match enabled() {
            Some(url) => url,
            None => unreachable!()
        };
        let preamble = format!("etcd-write:{}", options.key);
        let url = format!("{}/v2/keys/bldr/{}", base_url, options.key);
        let req_body = url::form_urlencoded::serialize(&req_options);

        loop {
            debug!("{}: Requesting {}", preamble, url);
            debug!("{}: Requesting body {}", preamble, req_body);
            let request = client.put(&url)
                .header(ContentType::form_url_encoded())
                .body(&req_body);
            let mut res = match request.send() {
                Ok(res) => res,
                Err(e) => {
                    debug!("{}: Cannot send request: {:?}", preamble, e);
                    continue;
                }
            };
            debug!("{}: Response: {:?}", preamble, res);
            let mut response_body = String::new();
            match res.read_to_string(&mut response_body) {
                Ok(_) => {},
                Err(e) => {
                    debug!("{}: Cannot read response body: {:?}", preamble, e);
                    continue;
                }
            }
            debug!("{}: Response body: {:?}", preamble, response_body);
            watcher_tx.send((res.status, response_body)).unwrap();

            let sleepy_time = options.ttl.unwrap() as i64;
            // We get the jump on the TTL by 5 seconds. Lets hope
            // that's enough.
            let stop_time = util::stop_time(sleepy_time - 5);

            loop {
                match watcher_rx.try_recv() {
                    Ok(_stop) => {
                        debug!("   {}: Watch exiting", preamble);
                        return;
                    },
                    Err(TryRecvError::Empty) => {},
                    Err(e) => {
                        debug!("   {}: Watch exiting - watcher disappeared: {:?}", preamble, e);
                        return;
                    }
                }
                let time = time::now_utc().to_timespec();
                if time > stop_time {
                    break;
                } else {
                    thread::sleep_ms(1000);
                }
            }
        }
    });
}
//

pub fn watch(key: &str, reconnect_interval: u32, wait: bool, recursive: bool, watcher_tx: Sender<Option<String>>, watcher_rx: Receiver<bool>) {
    let key = String::from(key);
    let _newthread = thread::Builder::new().name(format!("etcd:{}", key)).spawn(move || {
        let mut first_run = true;
        let preamble = format!("etcd:{}", key);
        let base_url = match enabled() {
            Some(url) => url,
            None => panic!("How did you get to a watch without being enabled?")
        };
        loop {
            let mut client = Client::new();
            // If it is the first time we've asked, just ask - we want to seed the right data
            // quickly
            let really_wait = if first_run { first_run = false; false } else { wait };
            let mut res = match client.get(&format!("{}/v2/keys/bldr/{}?wait={}&recursive={}", base_url, key, really_wait, recursive)).send() {
                Ok(res) => res,
                Err(e) => {
                    debug!("   {}: Invalid request for config: {:?}", preamble, e);
                    watcher_tx.send(None).unwrap();
                    continue;
                }
            };
            debug!("Response: {:?}", res);
            let mut response_body = String::new();
            match res.read_to_string(&mut response_body) {
                Ok(_) => {},
                Err(e) => {
                    debug!("   {}: Failed to read request body: {:?}", preamble, e);
                    watcher_tx.send(None).unwrap();
                    continue;
                }
            }
            let body_as_json = match Json::from_str(&response_body) {
                Ok(body) => body,
                Err(e) => {
                    debug!("   {}: Failed to parse request body as json: {:?}", preamble, e);
                    watcher_tx.send(None).unwrap();
                    continue;
                }
            };
            match body_as_json.find("node") {
                Some(json_value) => {
                    let mut results = String::new();
                    get_json_values_recursively(json_value, &mut results);
                    if results.is_empty() {
                        debug!("   {}: Invalid json value for node/values!", preamble);
                        watcher_tx.send(None).unwrap();
                    } else {
                        debug!("Sending back a value");
                        watcher_tx.send(Some(String::from(results))).unwrap()
                    }
                },
                None => {
                    debug!("   {}: No node/value present in response json", preamble);
                    watcher_tx.send(None).unwrap();
                }
            }

            let stop_time = util::stop_time(reconnect_interval as i64);

            loop {
                match watcher_rx.try_recv() {
                    Ok(_stop) => {
                        debug!("   {}: Watch exiting", preamble);
                        return;
                    },
                    Err(TryRecvError::Empty) => {},
                    Err(e) => {
                        debug!("   {}: Watch exiting - watcher disappeared - {:?}", preamble, e);
                        return;
                    }
                }
                let time = time::now_utc().to_timespec();
                if time > stop_time {
                    break;
                } else {
                    thread::sleep_ms(100);
                }
            }
        }
    });
}

// Given an etcd 'node', it will recursively accumulate all the values
// underneath it into a single string. Since we simply write TOML strings
// out, this should make it easy to grab a bunch of things in one go.
fn get_json_values_recursively(json: &Json, result_acc: &mut String) {
    match json.find("nodes") {
        Some(nodes_list) => {
            for node in nodes_list.as_array().unwrap().iter() {
                get_json_values_recursively(node, result_acc);
            }
        },
        None => {
            match json.find("value") {
                Some(json_value) => {
                    match json_value.as_string() {
                        Some(value) => {
                            if value.starts_with("[") {
                                result_acc.push_str(&format!("{}\n", value))
                            } else {
                                let new_string = if result_acc.is_empty() {
                                    String::from(value)
                                } else {
                                    format!("{}\n{}", value, result_acc)
                                };
                                mem::replace(result_acc, new_string);
                            }
                        },
                        None => debug!("node.value should be a string - I have no idea whats up")
                    }
                },
                None => {}
            }
        }
    }
}

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
    if wait {
        println!("   {}: Waiting to overlay etcd configuration", pkg_print);
    } else {
        println!("   {}: Overlaying etcd configuration", pkg_print);
    }
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

#[cfg(test)]
mod tests {
    use super::get_json_values_recursively as get_json_values_recursively_etcd;
    use rustc_serialize::json::Json;

    #[test]
    fn get_json_values_recursively() {
        let json_string = r#"
{
  "action": "get",
  "node": {
    "key": "/bldr/redis/default",
    "dir": true,
    "nodes": [
      {
        "key": "/bldr/redis/default/topology",
        "dir": true,
        "nodes": [
          {
            "key": "/bldr/redis/default/topology/leader",
            "dir": true,
            "nodes": [
              {
                "key": "/bldr/redis/default/topology/leader/init",
                "value": "[topology.init]\nip = '172.17.0.1'\nport = '6379'",
                "modifiedIndex": 186683,
                "createdIndex": 186476
              },
              {
                "key": "/bldr/redis/default/topology/leader/government",
                "dir": true,
                "expiration": "2015-07-06T23:14:31.187124534Z",
                "ttl": 10,
                "nodes": [
                  {
                    "key": "/bldr/redis/default/topology/leader/government/leader",
                    "value": "[topology.leader]\nhostname = 'd48611215f82'\nip = '172.17.0.1'\nport = '6379'",
                    "modifiedIndex": 186685,
                    "createdIndex": 186685
                  }
                ],
                "modifiedIndex": 186684,
                "createdIndex": 186684
              },
              {
                "key": "/bldr/redis/default/topology/leader/census",
                "dir": true,
                "nodes": [
                  {
                    "key": "/bldr/redis/default/topology/leader/census/d48611215f82",
                    "dir": true,
                    "expiration": "2015-07-06T23:14:31.181055623Z",
                    "ttl": 10,
                    "nodes": [
                      {
                        "key": "/bldr/redis/default/topology/leader/census/d48611215f82/data",
                        "value": "[[topology.follower]]\nhostname = 'd48611215f82'\nip = '172.17.0.1'\nport = '6379'",
                        "modifiedIndex": 186687,
                        "createdIndex": 186687
                      }
                    ],
                    "modifiedIndex": 186686,
                    "createdIndex": 186686
                  },
                  {
                    "key": "/bldr/redis/default/topology/leader/census/437b1d502710",
                    "dir": true,
                    "expiration": "2015-07-06T23:14:39.690122089Z",
                    "ttl": 19,
                    "nodes": [
                      {
                        "key": "/bldr/redis/default/topology/leader/census/437b1d502710/data",
                        "value": "[[topology.follower]]\nhostname = '437b1d502710'\nip = '172.17.0.4'\nport = '6379'",
                        "modifiedIndex": 186691,
                        "createdIndex": 186691
                      }
                    ],
                    "modifiedIndex": 186690,
                    "createdIndex": 186690
                  }
                ],
                "modifiedIndex": 186479,
                "createdIndex": 186479
              }
            ],
            "modifiedIndex": 186476,
            "createdIndex": 186476
          }
        ],
        "modifiedIndex": 186476,
        "createdIndex": 186476
      },
      {
        "key": "/bldr/redis/default/config",
        "value": "loglevel = 'debug'\ntcp-backlog = 128",
        "modifiedIndex": 186699,
        "createdIndex": 186699
      }
    ],
    "modifiedIndex": 186476,
    "createdIndex": 186476
  }
}"#;
        let json_top = match Json::from_str(json_string) {
            Ok(json_top) => json_top,
            Err(e) => panic!("{}", e)
        };
        let nodes_list = match json_top.find_path(&["node"]) {
            Some(nl) => nl,
            None => panic!("No node/nodes path found")
        };
        let mut results = String::new();
        get_json_values_recursively_etcd(nodes_list, &mut results);
        let match_string = r#"loglevel = 'debug'
tcp-backlog = 128
[topology.init]
ip = '172.17.0.1'
port = '6379'
[topology.leader]
hostname = 'd48611215f82'
ip = '172.17.0.1'
port = '6379'
[[topology.follower]]
hostname = 'd48611215f82'
ip = '172.17.0.1'
port = '6379'
[[topology.follower]]
hostname = '437b1d502710'
ip = '172.17.0.4'
port = '6379'
"#;
        assert_eq!(match_string, &results);
    }
}
