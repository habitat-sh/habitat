pub mod etcd;

use toml;
use std::collections::{BTreeMap, HashMap};
use std::sync::mpsc::{channel, Sender, Receiver, TryRecvError};
use std::fmt::{self, Debug};
use ansi_term::Colour::{White};
use hyper::status::StatusCode;

use pkg::Package;
use error::{BldrResult, BldrError};

// 1. Watch a key for changes with a reconnect timer
// 2. Write values to a key
// 3. Write values to a key with a TTL, and keep it alive
// 4. Check for the absence of a key
//
// worker.discovery.watch("/foo/bar/baz", "101_leader", reconnect);
// worker.discovery.write("/foo/bar/baz", "101_leader", toml_string, Some(ttl));
// # Returns the last value we got for the watch location - none indicates the key is absent
// worker.discovery.status("/foo/bar/baz") -> Some(data)

#[derive(Debug)]
pub struct Discovery {
    watchers: Vec<DiscoveryWatcher>,
    writers: Vec<DiscoveryWriter>,
    status: HashMap<String, DiscoveryResponse>,
    write_status: HashMap<String, DiscoveryWriteResponse>,
    backend: Backend
}

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Etcd
}

impl Discovery {
    pub fn new(backend: Backend) -> Discovery {
        Discovery{
            watchers: Vec::new(),
            writers: Vec::new(),
            status: HashMap::new(),
            write_status: HashMap::new(),
            backend: backend
        }
    }

    pub fn watch(&mut self, mut dw: DiscoveryWatcher) {
        dw.backend(self.backend);
        dw.start();
        self.watchers.push(dw);
    }

    pub fn write(&mut self, mut dw: DiscoveryWriter) {
        dw.backend(self.backend);
        dw.start();
        self.writers.push(dw);
    }

    pub fn clear(&mut self) {
        self.watchers.clear();
        self.writers.clear();
    }

    pub fn status(&self, key: &str) -> Option<&DiscoveryResponse> {
        self.status.get(&String::from(key))
    }

    pub fn write_status(&self, key: &str) -> Option<&DiscoveryWriteResponse> {
        self.write_status.get(&String::from(key))
    }

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

    pub fn stop(&mut self) {
        for writer in self.writers.iter_mut() {
            writer.stop();
        }

        for watch in self.watchers.iter_mut() {
            watch.stop();
        }
    }
}

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

    fn backend(&mut self, backend: Backend) {
        self.backend = Some(backend)
    }

    pub fn service(&mut self, service: String) {
        self.service = Some(service)
    }

    pub fn group(&mut self, group: String) {
        self.group = Some(group)
    }

    fn start(&mut self) {
        let preamble = format!("{}({})", self.package.name, White.bold().paint("D"));
        println!("   {}: Watching {}", preamble, self.key);
        let (b_tx, b_rx) = channel();
        let (w_tx, w_rx) = channel();
        self.tx = Some(w_tx);
        self.rx = Some(b_rx);
        match self.backend {
            Some(Backend::Etcd) => etcd::watch(&self.key, self.reconnect_interval, self.wait, self.recursive, b_tx, w_rx),
            None => panic!("I don't have a discovery backend - so I can't start your watcher")
        }
    }

    fn stop(&mut self) {
        let tx = self.tx.as_ref().unwrap();
        tx.send(true).unwrap();
    }

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

    fn backend(&mut self, backend: Backend) {
        self.backend = Some(backend)
    }

    fn start(&mut self) {
        let preamble = format!("{}({})", self.package.name, White.bold().paint("D"));
        match self.ttl {
            Some(ttl) => println!("   {}: Writing {} every {}", preamble, self.key, ttl),
            None => println!("   {}: Writing {}", preamble, self.key)
        }
        let (b_tx, b_rx) = channel();
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

    fn stop(&mut self) {
        let tx = self.tx.as_ref().unwrap();
        tx.send(true).unwrap();
    }

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

#[derive(Debug)]
pub struct DiscoveryResponse {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub struct DiscoveryWriteResponse {
    pub key: String,
    pub body: Option<String>,
    pub status: StatusCode,
}

