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

//! Handles 'watch' configuration events.
//!
//! Takes a list of watches provided on the command line with `-w service.group`, and sets up an
//! etcd watch for each one. Then provides them back within the 'watch' array inside configuration
//! templates.
//!
//! Has 3 components:
//!
//! * WatchEntry: represents a given watch
//! * WatchActor: a GenServer actor responsible for updating the aggregate 'watch' data
//! * WatchActorState: an array of WatchEntries, provided as the state to our WatchActor

use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::collections::BTreeMap;

use wonder::actor::{self, GenServer, HandleResult, InitResult, StopReason, ActorSender};
use toml;

use census;
use discovery::etcd;
use config::Config;
use error::{BldrError, BldrResult, ErrorKind};

static LOGKEY: &'static str = "WC";
const TIMEOUT_MS: u64 = 200;

/// Messages accepted by the WatchActor.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Message {
    /// The current watch data, if any, as a toml string
    ConfigToml(Option<String>),
    /// A request to fetch the current watch data
    Config,
    /// A simple 'ok' response
    Ok,
    /// Stop the actor
    Stop,
}

/// The GenServer Actor
#[derive(Debug)]
pub struct WatchActor;

impl WatchActor {
    /// Request the current watch data as a toml string.
    ///
    /// # Failures
    ///
    /// * Fails if the call to the actor fails (bad message, other side went away)
    /// * Fails if our response is anything other than a Message::ConfigToml<Option<String>>.
    ///   (Shouldn't be possible... ;)
    pub fn config_string(actor: &actor::Actor<Message>) -> BldrResult<Option<String>> {
        match try!(actor.call(Message::Config)) {
            Message::ConfigToml(config_string) => Ok(config_string),
            _ => unreachable!(),
        }
    }
}

/// A single watch endpoint.
#[allow(dead_code)]
pub struct WatchEntry {
    /// The sending channel to the etcd watch
    ctx: Sender<bool>,
    /// The receiving channel to the etcd watch
    crx: Receiver<Option<String>>,
    /// The latest string we recieved
    config_string: Option<String>,
    /// The key we are watching (redis/default)
    watch_key: String,
    /// The service name we are watching (redis)
    service: String,
    /// The group name we are watching (default)
    group: String,
}

impl WatchEntry {
    /// Create a new WatchEntry.
    pub fn new(service: String,
               group: String,
               watch_key: String,
               ctx: Sender<bool>,
               crx: Receiver<Option<String>>)
               -> WatchEntry {
        WatchEntry {
            ctx: ctx,
            crx: crx,
            config_string: None,
            service: service,
            group: group,
            watch_key: watch_key,
        }
    }
}

/// The state passed to the WatchActor's GenServer implementaiton.
///
/// Keeps track of all of our current watches.
pub struct WatchActorState {
    /// A WatchEntry per watch
    watches: Vec<WatchEntry>,
}

impl WatchActorState {
    /// Create a new WatchActorState with an empty watch list.
    pub fn new() -> WatchActorState {
        WatchActorState { watches: Vec::new() }
    }

    /// Set up all the watches requested on the command line.
    ///
    /// Walks the list of watches provided on the CLI, then creates a new WatchEntry and background
    /// etcd::watch.
    ///
    /// # Failures
    ///
    /// * Fails if a watch was specified incorrectly (must be `service.group`)
    pub fn set_watches(&mut self, config: &Config) -> BldrResult<()> {
        for watch_member in config.watch().iter() {
            let watch_parts: Vec<&str> = watch_member.split('.').collect();
            let (service, group) = match watch_parts.len() {
                1 => {
                    (String::from(watch_parts[0]), String::from("default"))
                }
                2 => {
                    (String::from(watch_parts[0]), String::from(watch_parts[1]))
                }
                _ => {
                    return Err(bldr_error!(ErrorKind::BadWatch(watch_member.clone())));
                }
            };

            let (ctx, wrx) = channel();
            let (wtx, crx) = channel();
            let watch_entry = WatchEntry::new(service.clone(),
                                              group.clone(),
                                              format!("{}/{}", service.clone(), group.clone()),
                                              ctx,
                                              crx);
            etcd::watch(&watch_entry.watch_key, 1, true, true, wtx, wrx);
            self.watches.push(watch_entry);
        }
        Ok(())
    }

    /// Gather the latest watch string for each watch.
    fn gather_watches(&mut self) {
        for watch in self.watches.iter_mut() {
            match watch.crx.try_recv() {
                Ok(Some(toml_string)) => {
                    watch.config_string = Some(toml_string);
                }
                Ok(None) => {
                    watch.config_string = None;
                }
                Err(TryRecvError::Empty) => {}
                Err(e) => outputln!("Watch actor caught unexpected error: {:?}", e),
            }
        }
    }

}

impl GenServer for WatchActor {
    type T = Message;
    type S = WatchActorState;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, _state: &mut Self::S) -> InitResult<Self::E> {
        Ok(Some(0))
    }

    /// Gathers watches, then waits to try again.
    fn handle_timeout(&self,
                      _tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        state.gather_watches();

        HandleResult::NoReply(Some(TIMEOUT_MS))
    }

    /// Gathers watches, then processess inboud requests.
    fn handle_call(&self,
                   message: Self::T,
                   _caller: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        {
            state.gather_watches();
        }

        match message {
            Message::Stop => {
                HandleResult::Stop(StopReason::Normal, Some(Message::Ok))
            }
            Message::Config => {
                let mut watch_toml: toml::Table = BTreeMap::new();
                let mut watch_list: toml::Array = Vec::new();

                // So, this code deserves an explanation.
                //
                // Basically, we are taking data out of the backend implementation, wrapping it up
                // so that it's in a new toml namespace, and then re-formatting the census so it
                // matches what you would expect as a regular user of bldr.
                //
                // It's gross, and it really shows some of the downsides of not managing all this
                // data ourselves.
                for watch in state.watches.iter_mut() {
                    match watch.config_string {
                        Some(ref toml_string) => {
                            let mut toml_parser = toml::Parser::new(toml_string);
                            let mut discovery_toml =
                                toml_parser.parse()
                                           .ok_or(bldr_error!(ErrorKind::TomlParser(toml_parser.errors)))
                                           .unwrap();
                            discovery_toml.insert(String::from("service-name"),
                                                  toml::Value::String(watch.service.clone()));
                            discovery_toml.insert(String::from("group-name"),
                                                  toml::Value::String(watch.group.clone()));
                            let census_toml = match discovery_toml.remove("census") {
                                Some(census_toml) => census_toml,
                                None => continue,
                            };
                            let mut census = census::Census::new(census::CensusEntry::new());
                            let mut census_map = BTreeMap::new();
                            census_map.insert(String::from("census"), census_toml);
                            match census.update(&toml::encode_str(&census_map)) {
                                Ok(_) => {}
                                Err(e) => {
                                    debug!("Error updating census for watch: {:?}", e);
                                    continue;
                                }
                            }
                            let census_string = census.to_toml().unwrap();
                            let mut census_parser = toml::Parser::new(&census_string);
                            let mut census_final_toml =
                                census_parser.parse()
                                             .ok_or(bldr_error!(ErrorKind::TomlParser(census_parser.errors)))
                                             .unwrap();
                            let final_census_obj = census_final_toml.remove("census").unwrap();
                            discovery_toml.insert(String::from("census"), final_census_obj);
                            watch_list.push(toml::Value::Table(discovery_toml));
                        }
                        None => {}
                    }
                }
                if watch_list.len() > 0 {
                    watch_toml.insert(String::from("watch"), toml::Value::Array(watch_list));
                    let watch_result = toml::encode_str(&watch_toml);
                    HandleResult::Reply(Message::ConfigToml(Some(watch_result)), Some(TIMEOUT_MS))
                } else {
                    HandleResult::Reply(Message::ConfigToml(None), Some(TIMEOUT_MS))
                }
            }
            Message::Ok => HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! \
                                                                         I send YOU Ok!")),
                                              Some(Message::Ok)),
            Message::ConfigToml(_) =>
                HandleResult::Stop(StopReason::Fatal(format!("You don't send me CensusToml(_)! \
                                                              I send YOU CensusToml(_)!")),
                                   Some(Message::Ok)),
        }
    }
}
