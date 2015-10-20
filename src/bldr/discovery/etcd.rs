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

//! Etcd backend for service discovery.
//!
//! The functions in this module are typically used through the [discovery](../discovery) module's
//! [DiscoveryWatcher](struct.DiscoveryWatcher.html) and
//! [DiscoveryWriter](struct.DiscoveryWriter.html), which are in turn accessed through a particular
//! [topology](../topology).
//!
//! We do not implement the fullness of the [etcd api](http://...).

use hyper::header::ContentType;
use hyper::client::Client;
use hyper::status::StatusCode;
use url;
use rustc_serialize::json::Json;
use std::env;
use std::io::Read;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, TryRecvError};
use time;
use std::mem;

use error::BldrResult;
use util;

/// If the environment variable `$BLDR_CONFIG_ETCD` is set, returns the URL that it contains.
pub fn enabled() -> Option<String> {
    match env::var("BLDR_CONFIG_ETCD") {
        Ok(val) => Some(val),
        Err(_) => {
            debug!("No BLDR_CONFIG_ETCD, so not checking etcd for configuration values");
            return None;
        }
    }
}

/// Set a value in etcd.
///
/// Given a key and an array of option tuples (`[("recursive", "true)]`), sets the key to the given
/// value in etcd.
///
/// Valid option tuples [correspond directly to the etcd api](https://coreos.com/etcd/docs/latest/api.html).
///
/// # Examples
///
/// ```ignore
/// use discovery::etcd;
///
/// // Compare and set
/// etcd::set("/foo", &[("value", &status), ("prevValue", &status)]).unwrap();
/// // A directory with a ttl
/// etcd::set("/bar", &[("dir", "true"), ("ttl", "30")]).unwrap();
/// // Set a key unless it already exists
/// etcd::set(&key, &[("value", "[topology.leader]"), ("prevExist", "false")]).unwrap();
/// ```
///
/// # Failures
///
/// * If the request fails to send for any reason
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

/// The options for an EtcdWrite call.
///
/// Valid options [correspond directly to the etcd api](https://coreos.com/etcd/docs/latest/api.html).
#[allow(non_snake_case)]
pub struct EtcdWrite {
    /// The key to write to
    pub key: String,
    /// An optional value to write
    pub value: Option<String>,
    /// An optional ttl to set
    pub ttl: Option<u32>,
    /// Are we a directory?
    pub dir: Option<bool>,
    /// Check for previous existence of a key?
    pub prevExist: Option<bool>,
    /// Check for previous index number?
    pub prevIndex: Option<u64>,
    /// Check for previous value?
    pub prevValue: Option<String>
}

/// Write a value to etcd, in a new thread. Used by the
/// [DiscoveryWriter](../struct.DiscoveryWriter.html)
///
/// 1. Spawn a new thread named `etc-write:$options.key`
/// 1. Parse the options from the `EtcdWrite` options
/// 1. Loop forever
/// 1. Send an HTTP PUT request to etcd with the correct options
/// 1. Send the response status and body back to the `DiscoveryWriter`
/// 1. Calculate the time we should wait to write again based on the TTL in the `EtcdWrite`
/// 1. Loop
/// 1. Check for a stop signal from the `DiscoveryWriter`
/// 1. Check if the timer has elapsed
/// 1. Sleep or go back to the top of the outer loop.
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

            if let Err(_e) = watcher_tx.send((res.status, response_body)) {
                debug!("{}: Aborting watch on failed send - peer went away", preamble);
                return;
            }

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

/// Watch a value for changes in etcd, in a new thread. Used by the
/// [DiscoveryWatcher](../struct.DiscoveryWatcher.html).
///
/// 1. Spawns a new thread named `etcd:key`
/// 1. Loop forever
/// 1. If this is the first time we have run, make a direct call for the data rather than a watch.
///    Otherwise, watch the key for changes.
/// 1. If the watch returns data, decode the json
/// 1. Lookup the values returned in the etcd nodes (optionally recursively)
/// 1. Send the value back to the `DiscovertyWatcher`
/// 1. Calculate the time we will wait to reconnect to etcd
/// 1. Loop
/// 1. Watch for the stop signal
/// 1. Sleep or break to the outer loop when time has elapsed
///
pub fn watch(key: &str, reconnect_interval: u32, wait: bool, recursive: bool, watcher_tx: Sender<Option<String>>, watcher_rx: Receiver<bool>) {
    let key = String::from(key);
    let _newthread = thread::Builder::new().name(format!("etcd:{}", key)).spawn(move || {
        let mut first_run = true;
        let preamble = format!("etcd:{}", key);
        let base_url = match enabled() {
            Some(url) => url,
            None => panic!("How did you get to a watch without being enabled?")
        };
        let mut modified_index = 0u64;
        loop {
            let mut client = Client::new();
            // If it is the first time we've asked, just ask - we want to seed the right data
            // quickly
            let really_wait = if first_run { first_run = false; false } else { wait };
            let mut res = match client.get(&format!("{}/v2/keys/bldr/{}?wait={}&recursive={}&waitIndex={}&sorted=true", base_url, key, really_wait, recursive, modified_index)).send() {
                Ok(res) => res,
                Err(e) => {
                    debug!("   {}: Invalid request for config: {:?}", preamble, e);
                    if let Err(_e) = watcher_tx.send(None) {
                        debug!("{}: Aborting watch on failed send - peer went away", preamble);
                        return;
                    }
                    continue;
                }
            };
            modified_index = match res.headers.get_raw("x-etcd-index") {
                Some(x_etcd_index) => {
                 // The header is an array of Vec<u8>'s. We want to take the first one, if we have
                 // it, or '0' if we don't, and turn it into a string.
                 // If the response is not valid UTF-8, we want to just start from '0'.
                 // Then parse into a u64, and again, if its not valid, return 0.
                 // Then add 1.
                 // This means we should always get x-etcd-index, and if we can't, we get a
                 // reasonable number to start with.
                    String::from_utf8(x_etcd_index
                                      .iter()
                                      .nth(0)
                                      .map_or(vec![0 as u8], |v| v.to_owned()))
                     .unwrap_or(String::from("0"))
                     .parse::<u64>().unwrap_or(0u64) + 1
                },
                None => { debug!("No x-etcd-index received"); 0 },
            };

            debug!("Response: {:?}", res);
            let mut response_body = String::new();
            match res.read_to_string(&mut response_body) {
                Ok(_) => {},
                Err(e) => {
                    debug!("   {}: Failed to read request body: {:?}", preamble, e);
                    if let Err(_e) = watcher_tx.send(None) {
                        debug!("{}: aborting watch on failed send - peer went away", preamble);
                        return;
                    }

                    continue;
                }
            }
            debug!("JSON RESPONSE BODY: {:?}", response_body);
            let body_as_json = match Json::from_str(&response_body) {
                Ok(body) => body,
                Err(e) => {
                    debug!("   {}: Failed to parse request body as json: {:?}", preamble, e);
                    if let Err(_e) = watcher_tx.send(None) {
                        debug!("{}: Aborting watch on failed send - peer went away", preamble);
                        return;
                    }
                    continue;
                }
            };
            // If we are recursive, the events we get are garbage. We have to do a full GET if we
            // want all the data; otherwise, we get partial data based on the events we get back
            // from a watch. This is a chat-tastic kludge. We should remove it post demo.
            if (recursive == true) && (first_run == false) {
                match body_as_json.find("action") {
                    Some(action_value) => {
                        match action_value.as_string() {
                            Some("get") => {},
                            Some(_) => {
                                // So, yeah - sorry. Just go do the first get.
                                first_run = true;
                                modified_index = 0;
                                continue;
                            },
                            None => {
                                debug!("Received an etcd response an action that is not a string - shouldn't be possible");
                                continue;
                            }
                        }
                    },
                    None => {
                        debug!("Received an etcd response without an action - shouldn't be possible");
                        // This should also set the modified index back to zero - most of
                        // the time, we can just try again with the new value. This happens
                        // when the global etcd index gets beyond the last-modified number
                        // of the index of the root object in etcd. This fixes it.
                        first_run = true;
                        modified_index = 0;
                        continue;
                    }
                }
            }
            match body_as_json.find("node") {
                Some(json_value) => {
                    let mut results = String::new();
                    // let current_modified_index = json_value.find("modifiedIndex").unwrap().as_u64().unwrap();
                    // modified_index = current_modified_index + 1;

                    get_json_values_recursively(json_value, &mut results);
                    if results.is_empty() {
                        debug!("   {}: Invalid json value for node/values!", preamble);
                        if let Err(_e) = watcher_tx.send(None) {
                            debug!("{}: Aborting watch on failed send - peer went away", preamble);
                            return;
                        }
                    } else {
                        debug!("Sending back a value");
                        if let Err(_e) = watcher_tx.send(Some(String::from(results))) {
                            debug!("{}: Aborting watch on failed send - peer went away", preamble);
                            return;
                        }
                    }
                },
                None => {
                    debug!("   {}: No node/value present in response json", preamble);
                    if let Err(_e) = watcher_tx.send(None) {
                        debug!("{}: Aborting watch on failed send - peer went away", preamble);
                        return;
                    }
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
                            // Anything that starts with a '[' means it has a namespace
                            // in toml. Anything without a namespace (if its at the root)
                            // needs to be at the front of the toml string.
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
