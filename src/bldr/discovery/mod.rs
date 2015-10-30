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

//! Service discovery support.
//!
//! This module defines the basics of our discovery support:
//!
//! * The [Discovery](struct.Discovery.html) Struct, which holds our writers, watchers, status, and
//! a reference to the backend
//! * The [DiscoveryWatcher](struct.DiscoveryWatcher.html) Struct, which sets up and manages a
//! particular watch
//! * The [DiscoveryWriter](struct.DiscoveryWriter.html) Struct, which sets up and manages writing
//! to a particular key
//!
//! The short version of what happens:
//!
//! 1. A [toplogy Worker](../topology/struct.Worker.html) creates a new `Discovery` with an
//!    appropriate backend.
//! 1. The selected topology adds `DiscoveryWriters` and `DiscoverWatchers` to the `Discovery`
//!    struct.
//! 1. Each pass of the topology Workers state machine, we check to see if we need to write or read
//!    from a watch.
//! 1. Responses are stored in the `DiscoveryWatcher`, and retrieved as-needed by the topology.
//!
pub mod etcd;

use toml;
use std::collections::{BTreeMap, HashMap};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::fmt::{self, Debug};
use ansi_term::Colour::{White};
use hyper::status::StatusCode;

use pkg::Package;
use error::{BldrResult, BldrError};
use config::Config;

/// The available discovery backends. Only etcd is supported right now.
#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Etcd
}

/// Holds the state for all discovery operations.
#[derive(Debug)]
pub struct Discovery {
    // A list of keys to watch
    watchers: Vec<DiscoveryWatcher>,
    // A list of keys to write
    writers: Vec<DiscoveryWriter>,
    // A map of responses; the key is the path we are watching
    status: HashMap<String, DiscoveryResponse>,
    // A map of write results; the key is the path we are writing
    write_status: HashMap<String, DiscoveryWriteResponse>,
    // The selected backend
    backend: Backend
}

impl Discovery {
    /// Given a backend, return an empty `Discovery` struct.
    pub fn new(backend: Backend) -> Discovery {
        Discovery{
            watchers: Vec::new(),
            writers: Vec::new(),
            status: HashMap::new(),
            write_status: HashMap::new(),
            backend: backend
        }
    }

    /// Add a watch.
    ///
    /// The backend of the `DiscoveryWatcher` is set to the `backend` of this `Discovery` instance.
    pub fn watch(&mut self, mut dw: DiscoveryWatcher) {
        dw.backend(self.backend);
        dw.start();
        self.watchers.push(dw);
    }

    /// Add a writer.
    ///
    /// The backend of the `DiscoveryWriter` is set to the `backend` of this `Discovery` instance.
    pub fn write(&mut self, mut dw: DiscoveryWriter) {
        dw.backend(self.backend);
        dw.start();
        self.writers.push(dw);
    }

    /// Clear the watchers and writers.
    pub fn clear(&mut self) {
        self.watchers.clear();
        self.writers.clear();
    }

    /// Fetch the last known status of a given watch.
    pub fn status(&self, key: &str) -> Option<&DiscoveryResponse> {
        self.status.get(&String::from(key))
    }

    /// Fetch the last known status of a given write.
    pub fn write_status(&self, key: &str) -> Option<&DiscoveryWriteResponse> {
        self.write_status.get(&String::from(key))
    }

    /// Process the next event from both writers and watchers.
    ///
    /// # Failures
    ///
    /// * If we cannot receive anything from a writer/watcher thread. Most likely this means the
    /// thread has straight up panic-ed, and gone away.
    pub fn next(&mut self) -> BldrResult<()> {
        // Writers should come first, then watchers - but baby steps
        for writer in self.writers.iter_mut() {
            let result = try!(writer.try_recv());
            if let Some(msg) = result {
                debug!("Write response {:?}: {:?}", writer, msg);
                self.write_status.insert(msg.key.clone(), msg);
            }
        }
        for watch in self.watchers.iter_mut() {
            let result = try!(watch.try_recv());
            if let Some(msg) = result {
                debug!("Watch response {:?}: {:?}", watch, msg);
                self.status.insert(msg.key.clone(), msg);
            }
        }
        Ok(())
    }

    /// Stop all watches and writes.
    pub fn stop(&mut self) {
        for writer in self.writers.iter_mut() {
            writer.stop();
        }

        for watch in self.watchers.iter_mut() {
            watch.stop();
        }
    }
}

/// A struct representing a particular watcher.
pub struct DiscoveryWatcher {
    package: Package,
    key: String,
    filename: String,
    wait: bool,
    recursive: bool,
    reconnect_interval: u32,
    backend: Option<Backend>,
    service: Option<String>,
    group: Option<String>,
    rx: Option<Receiver<Option<String>>>,
    tx: Option<Sender<bool>>,
}

impl Debug for DiscoveryWatcher {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "DiscoveryWatcher{{key: {}, filename: {}, reconnect_interval: {}, backend: {:?}, rx: {}}}",
               self.key, self.filename, self.reconnect_interval, self.backend, self.rx.is_some())
    }
}

impl DiscoveryWatcher {
    /// Creates a new struct.
    ///
    /// It will watch for changes on the key, and reconnect on failure.
    pub fn new(package: Package, key: String, filename: String, reconnect_interval: u32, wait: bool, recursive: bool) -> DiscoveryWatcher {
        DiscoveryWatcher{
            package: package,
            key: key,
            filename: filename,
            wait: wait,
            recursive: recursive,
            reconnect_interval: reconnect_interval,
            service: None,
            group: None,
            backend: None,
            tx: None,
            rx: None,
        }
    }

    /// Sets the backend; usually done by the `Discovery` struct.
    fn backend(&mut self, backend: Backend) {
        self.backend = Some(backend)
    }

    /// Set the service we are watching
    pub fn service(&mut self, service: String) {
        self.service = Some(service)
    }

    /// Set the group we are watching
    pub fn group(&mut self, group: String) {
        self.group = Some(group)
    }

    // Start the watch. This method will set up the correct communication channels, and then call
    // the appropriate `watch` method based on the `Backend`. The `watch` method then is run in a
    // separate thread, with communication flowing back through the channels we set up.
    fn start(&mut self) {
        let preamble = format!("{}({})", self.package.name, White.bold().paint("D"));
        println!("   {}: Watching {}", preamble, self.key);
        // Backend
        let (b_tx, b_rx) = channel();
        // Watch
        let (w_tx, w_rx) = channel();
        self.tx = Some(w_tx);
        self.rx = Some(b_rx);
        match self.backend {
            Some(Backend::Etcd) => etcd::watch(&self.key, self.reconnect_interval, self.wait, self.recursive, b_tx, w_rx),
            None => panic!("I don't have a discovery backend - so I can't start your watcher")
        }
    }

    // Stop the watch. Sends the signal to the backend thread to stop itself cleanly.
    fn stop(&mut self) {
        let tx = self.tx.as_ref().unwrap();
        let _result = tx.send(true);
    }

    // Check for a response from a watch.
    //
    // If we have a response, write the data out, making it available to the service. If this
    // watch has a `service` and `group` defined, we put the data returned into the `watch`
    // toml data. Otherwise, we accept the raw data. This is used to differentiate between data we
    // are watching from ourselves (and thus don't need to namespace) and data we are watching that
    // comes from someone else.
    //
    // # Failures
    //
    // * The discovery backend thread has gone away
    // * We cannot parse the toml in the reponse
    // * We cannot write the toml out to the filesystem
    fn try_recv(&mut self) -> BldrResult<Option<DiscoveryResponse>> {
        let rx = self.rx.as_ref().unwrap();
        let result = match rx.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return Ok(None),
            Err(e) => return Err(BldrError::from(e))
        };
        match result {
            Some(s) => {
                if self.service.is_some() && self.group.is_some() {
                    {
                        let mut toml_parser = toml::Parser::new(&s);
                        let discovery_toml = try!(toml_parser.parse().ok_or(BldrError::TomlParser(toml_parser.errors)));
                        let mut base_toml: toml::Table = BTreeMap::new();
                        let mut service_toml: toml::Table = BTreeMap::new();
                        let mut group_toml: toml::Table = BTreeMap::new();

                        let service = self.service.as_ref().unwrap().clone();
                        let group = self.group.as_ref().unwrap().clone();
                        group_toml.insert(String::from("config"), toml::Value::Table(discovery_toml));
                        group_toml.insert(String::from("group-name"), toml::Value::String(group));
                        let mut group_list: toml::Array = Vec::new();
                        group_list.push(toml::Value::Table(group_toml));
                        service_toml.insert(String::from("groups"), toml::Value::Array(group_list));
                        service_toml.insert(String::from("service-name"), toml::Value::String(service));
                        let mut service_list: toml::Array = Vec::new();
                        service_list.push(toml::Value::Table(service_toml));
                        base_toml.insert(String::from("watch"), toml::Value::Array(service_list));

                        try!(self.package.write_toml(&self.filename, base_toml));
                    }
                    Ok(Some(DiscoveryResponse{key: self.key.clone(), value: Some(String::from(s))}))
                } else {
                    try!(self.package.write_toml_string(&self.filename, &s));
                    Ok(Some(DiscoveryResponse{key: self.key.clone(), value: Some(String::from(s))}))
                }
            },
            None => Ok(Some(DiscoveryResponse{key: self.key.clone(), value: None}))
        }
    }
}

/// A struct representing a given Writer
pub struct DiscoveryWriter {
    package: Package,
    key: String,
    value: Option<String>,
    ttl: Option<u32>,
    backend: Option<Backend>,
    rx: Option<Receiver<(StatusCode, String)>>,
    tx: Option<Sender<bool>>,
}

impl Debug for DiscoveryWriter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "DiscoveryWriter{{package: {:?}, key: {}, value: {:?}, ttl: {:?}, backend: {:?}, rx: {}, tx: {}}}",
               self.package, self.key, self.value, self.ttl, self.backend, self.rx.is_some(), self.tx.is_some())
    }
}

impl DiscoveryWriter {
    /// Create a new `DiscoverWriter`.
    ///
    /// A writer will try to write `value` to `key` evey `ttl` time, in seconds.
    pub fn new(package: Package, key: String, value: Option<String>, ttl: Option<u32>) -> DiscoveryWriter {
        DiscoveryWriter{
            package: package,
            key: key,
            value: value,
            ttl: ttl,
            backend: None,
            tx: None,
            rx: None
        }
    }

    // Set the backend
    fn backend(&mut self, backend: Backend) {
        self.backend = Some(backend)
    }

    // Start the writer.
    //
    // We create a set of options for the backend's `write` function, then call it. This spawns a
    // new thread that handles the actual write.
    fn start(&mut self) {
        let preamble = format!("{}({})", self.package.name, White.bold().paint("D"));
        match self.ttl {
            Some(ttl) => println!("   {}: Writing {} every {}", preamble, self.key, ttl),
            None => println!("   {}: Writing {}", preamble, self.key)
        }
        // Backend channelds
        let (b_tx, b_rx) = channel();
        // Writer channels
        let (w_tx, w_rx) = channel();
        self.tx = Some(w_tx);
        self.rx = Some(b_rx);
        match self.backend {
            Some(Backend::Etcd) => {
                let options = etcd::EtcdWrite{
                    key: self.key.clone(),
                    value: self.value.clone(),
                    ttl: self.ttl,
                    dir: Some(true),
                    prevExist: Some(true),
                    prevIndex: None,
                    prevValue: None,
                };
                etcd::write(options, b_tx, w_rx);
            },
            None => panic!("I cannot start your writer without a backend")
        }
    }

    // Stops this writer.
    //
    // Sends the appropriate signal to the writer thread.
    fn stop(&mut self) {
        let tx = self.tx.as_ref().unwrap();
        let _result = tx.send(true);
    }

    // Check for a response to a write.
    //
    // # Failures
    //
    // * The thread has gone away
    fn try_recv(&mut self) -> BldrResult<Option<DiscoveryWriteResponse>> {
        let rx = self.rx.as_ref().unwrap();
        let (status_code, response_body) = match rx.try_recv() {
            Ok(result) => result,
            Err(TryRecvError::Empty) => return Ok(None),
            Err(e) => return Err(BldrError::from(e))
        };
        Ok(Some(DiscoveryWriteResponse{key: self.key.clone(), status: status_code, body: Some(String::from(response_body))}))
    }
}

/// A response to a watch.
#[derive(Debug, Clone)]
pub struct DiscoveryResponse {
    pub key: String,
    pub value: Option<String>,
}

/// A response to a write. Note the leaky abstraction!
#[derive(Debug, Clone)]
pub struct DiscoveryWriteResponse {
    pub key: String,
    pub body: Option<String>,
    pub status: StatusCode,
}


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

use wonder::actor::{self, GenServer, HandleResult, InitResult, StopReason};

const TIMEOUT_MS: u64 = 30;

#[derive(Debug)]
pub enum Message {
    Watch(DiscoveryWatcher),
    Write(DiscoveryWriter),
    Stop,
    Clear,
    Status(String),
    StatusReply(Option<DiscoveryResponse>),
    WriteStatus(String),
    WriteStatusReply(Option<DiscoveryWriteResponse>),
    Error(BldrError),
    Ok
}

#[derive(Debug)]
pub struct DiscoveryActor;

impl GenServer for DiscoveryActor {
    type T = Message;
    type S = Discovery;
    type E = BldrError;

    fn init(&self, _tx: &Sender<actor::Message<Self::T>>, _: &mut Self::S) -> InitResult<Self::E> {
        Ok(Some(TIMEOUT_MS))
    }

    fn handle_timeout(&self, _me: &Sender<actor::Message<Self::T>>, state: &mut Self::S) -> HandleResult<Self::T> {
        match state.next() {
            Ok(_) => HandleResult::NoReply(Some(TIMEOUT_MS)),
            Err(e) => return HandleResult::Stop(
                StopReason::Fatal(format!("Discovery actor caught unexpected error: {:?}", e)),
                None,
                ),
        }
    }

    fn handle_call(&self, message: Self::T, _caller: &Sender<actor::Message<Self::T>>, _me: &Sender<actor::Message<Self::T>>, state: &mut Self::S) -> HandleResult<Self::T> {
       match message {
           Message::Watch(dw) => {
               state.watch(dw);
               HandleResult::Reply(Message::Ok, Some(TIMEOUT_MS))
           },
           Message::Write(dw) => {
               state.write(dw);
               HandleResult::Reply(Message::Ok, Some(TIMEOUT_MS))
           },
           Message::Stop => {
               state.stop();
               HandleResult::Stop(StopReason::Normal, Some(Message::Ok))
           },
           Message::Clear => {
               state.clear();
               HandleResult::Reply(Message::Ok, Some(TIMEOUT_MS))
           },
           Message::Status(status) => {
               match state.status(&status) {
                   Some(dr) => HandleResult::Reply(Message::StatusReply(Some(dr.clone())), Some(TIMEOUT_MS)),
                   None => HandleResult::Reply(Message::StatusReply(None), Some(TIMEOUT_MS)),
               }
           },
           Message::WriteStatus(status) => {
               match state.write_status(&status) {
                   Some(dr) => HandleResult::Reply(Message::WriteStatusReply(Some(dr.clone())), Some(TIMEOUT_MS)),
                   None => HandleResult::Reply(Message::WriteStatusReply(None), Some(TIMEOUT_MS)),
               }
           },
           Message::WriteStatusReply(_) => HandleResult::Stop(StopReason::Fatal(format!("You don't send me a StatusReply! I send YOU a StatusReply")), Some(Message::Ok)),
           Message::StatusReply(_) => HandleResult::Stop(StopReason::Fatal(format!("You don't send me a StatusReply! I send YOU a StatusReply")), Some(Message::Ok)),
           Message::Error(_) => HandleResult::Stop(StopReason::Fatal(format!("You don't send me an Error! I send YOU an Error")), Some(Message::Ok)),
           Message::Ok => HandleResult::Stop(StopReason::Fatal(format!("You don't send me Ok! I send YOU Ok!")), Some(Message::Ok)),
       }
    }
}

// Client side
pub fn stop(actor: &actor::Actor<Message>) -> BldrResult<()> {
    match try!(actor.call(Message::Stop)) {
        Message::Ok => Ok(()),
        _ => unreachable!(),
    }
}

pub fn clear(actor: &actor::Actor<Message>) -> BldrResult<()> {
    match try!(actor.call(Message::Clear)) {
        Message::Ok => Ok(()),
        _ => unreachable!(),
    }
}

pub fn watch(actor: &actor::Actor<Message>, dw: DiscoveryWatcher) -> BldrResult<()> {
    match try!(actor.call(Message::Watch(dw))) {
        Message::Ok => Ok(()),
        _ => unreachable!(),
    }
}

pub fn status(actor: &actor::Actor<Message>, key: String) -> BldrResult<Option<DiscoveryResponse>> {
    match try!(actor.call(Message::Status(key))) {
        Message::StatusReply(status) => Ok(status),
        _ => unreachable!(),
    }
}

pub fn write_status(actor: &actor::Actor<Message>, key: String) -> BldrResult<Option<DiscoveryWriteResponse>> {
    match try!(actor.call(Message::WriteStatus(key))) {
        Message::WriteStatusReply(status) => Ok(status),
        _ => unreachable!(),
    }
}

pub fn write(actor: &actor::Actor<Message>, dw: DiscoveryWriter) -> BldrResult<()> {
    match try!(actor.call(Message::Write(dw))) {
        Message::Ok => Ok(()),
        _ => unreachable!(),
    }
}

pub fn setup_standard_watches(actor: &actor::Actor<Message>, package: Package, config: &Config) -> BldrResult<()> {
    let p_a = package.clone();
    let key = format!("{}/{}/config", p_a.name, config.group());
    let watcher = DiscoveryWatcher::new(p_a, key, String::from("100_discovery.toml"), 1, true, false);
    try!(watch(actor, watcher));

    // Configure watches!
    for watch_member in config.watch().iter() {
        let watch_parts: Vec<&str> = watch_member.split('.').collect();
        let (service, group) = match watch_parts.len() {
            1 => {
                (String::from(watch_parts[0]), String::from("default"))
            },
            2 => {
                (String::from(watch_parts[0]), String::from(watch_parts[1]))
            },
            _ => {
                return Err(BldrError::BadWatch(watch_member.clone()))
            }
        };
        let p_b = package.clone();
        let key = format!("{}/{}", service, group);
        let mut watcher = DiscoveryWatcher::new(p_b, key, format!("300_watch_{}_{}.toml", service, group), 1, true, true);
        watcher.service(service);
        watcher.group(group);
        try!(watch(actor, watcher));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use wonder::actor;
    use super::{stop, clear, watch, write, DiscoveryActor};
    use discovery::{Discovery, Backend, DiscoveryWatcher, DiscoveryWriter};
    use pkg;

    #[test]
    fn start_and_stop() {
        let actor = actor::Builder::new(DiscoveryActor).name("discovery".to_string()).start(Discovery::new(Backend::Etcd)).unwrap();
        match stop(&actor) {
            Ok(()) => assert!(true, "Stopped"),
            Err(e) => {
                assert!(false, format!("Failed to stop actor: {:?}", e))
            }
        }
    }

    #[test]
    fn clears_data() {
        let actor = actor::Builder::new(DiscoveryActor).name("discovery".to_string()).start(Discovery::new(Backend::Etcd)).unwrap();
        assert!(clear(&actor).is_ok(), "Failed to clear the discovery backend");
    }

    #[test]
    fn adds_watch() {
        let actor = actor::Builder::new(DiscoveryActor).name("discovery".to_string()).start(Discovery::new(Backend::Etcd)).unwrap();
        let pkg = pkg::new("bobo", "clown", "1.2.3", "20151029175941");
        let dw = DiscoveryWatcher::new(pkg, "snooples".to_string(), "42_chill_out.toml".to_string(), 1, true, false);
        assert!(watch(&actor, dw).is_ok(), "Failed to add a watch");
    }

    #[test]
    fn adds_writer() {
        let actor = actor::Builder::new(DiscoveryActor).name("discovery".to_string()).start(Discovery::new(Backend::Etcd)).unwrap();
        let pkg = pkg::new("bobo", "clown", "1.2.3", "20151029175941");
        let dw = DiscoveryWriter::new(pkg, "snooples".to_string(), None, Some(30));
        assert!(write(&actor, dw).is_ok(), "Failed to add a write");
    }

}

