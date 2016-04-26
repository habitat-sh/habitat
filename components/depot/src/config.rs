// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::fs::File;
use std::io::Read;
use std::net;
use std::path::Path;
use std::str::FromStr;

use redis;
use toml;

use super::{ListenAddr, ListenPort};
use error::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct Config {
    pub path: String,
    pub listen_addr: ListenAddr,
    pub port: ListenPort,
    pub datastore_addr: net::SocketAddrV4,
}

impl Config {
    pub fn from_file<T: AsRef<Path>>(filepath: T) -> Result<Self> {
        // don't try, use our own error about config not found
        let mut file = try!(File::open(filepath.as_ref()));
        let mut raw = String::new();
        // don't try, use our own error about invalid config syntax
        try!(file.read_to_string(&mut raw));
        let mut parser = toml::Parser::new(&raw);
        match parser.parse() {
            Some(toml) => Ok(Config::from(toml)),
            None => {
                let mut msg = String::new();
                for err in &parser.errors {
                    let (loline, locol) = parser.to_linecol(err.lo);
                    let (hiline, hicol) = parser.to_linecol(err.hi);
                    msg.push_str(&format!("\t{}:{}-{}:{} error: {}\n",
                                          loline,
                                          locol,
                                          hiline,
                                          hicol,
                                          err.desc));
                }
                Err(Error::ConfigFileSyntax(msg))
            }
        }
    }

    pub fn depot_addr(&self) -> net::SocketAddrV4 {
        net::SocketAddrV4::new(self.listen_addr.0.clone(), self.port.0.clone())
    }
}

impl From<toml::Table> for Config {
    fn from(table: toml::Table) -> Self {
        let mut cfg = Config::default();
        if let Some(value) = table.get("path") {
            match value {
                &toml::Value::String(ref path) => cfg.path = path.clone(),
                _ => panic!("JW TODO: handle this error"),
            }
        }
        if let Some(value) = table.get("bind_addr") {
            match value {
                &toml::Value::String(ref addr_str) => {
                    // JW TODO: handle this
                    let bind_addr = net::Ipv4Addr::from_str(addr_str).unwrap();
                    cfg.listen_addr = ListenAddr(bind_addr)
                }
                _ => panic!("JW TODO: handle this error"),
            }
        }
        if let Some(value) = table.get("port") {
            match value {
                &toml::Value::Integer(port) => cfg.port = ListenPort(port as u16),
                _ => panic!("JW TODO: handle this error"),
            }
        }
        if table.contains_key("datastore_addr") || table.contains_key("datastore_port") {
            let ip = match table.get("datastore_addr") {
                Some(&toml::Value::String(ref addr_str)) => {
                    // JW TODO: handle this error
                    net::Ipv4Addr::from_str(addr_str).unwrap()
                }
                Some(_) => panic!("JW TODO: handle this error"),
                None => net::Ipv4Addr::new(127, 0, 0, 1),
            };
            let port = match table.get("datastore_port") {
                Some(&toml::Value::Integer(port)) => port as u16,
                Some(_) => panic!("handle"),
                None => 6379,
            };
            cfg.datastore_addr = net::SocketAddrV4::new(ip, port);
        }
        cfg
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: "/hab/svc/hab-depot/data".to_string(),
            port: super::ListenPort::default(),
            listen_addr: super::ListenAddr::default(),
            datastore_addr: net::SocketAddrV4::new(net::Ipv4Addr::new(127, 0, 0, 1), 6379),
        }
    }
}

impl<'a> redis::IntoConnectionInfo for &'a Config {
    fn into_connection_info(self) -> redis::RedisResult<redis::ConnectionInfo> {
        format!("redis://{}:{}",
                self.datastore_addr.ip(),
                self.datastore_addr.port())
            .into_connection_info()
    }
}
