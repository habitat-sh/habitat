// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::net;
use std::str::FromStr;

use hcore::config::ConfigFile;
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
    pub fn depot_addr(&self) -> net::SocketAddrV4 {
        net::SocketAddrV4::new(self.listen_addr.0.clone(), self.port.0.clone())
    }
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Table) -> Result<Self> {
        let mut cfg = Config::default();
        if let Some(value) = toml.get("path") {
            match value {
                &toml::Value::String(ref path) => cfg.path = path.clone(),
                _ => panic!("JW TODO: handle this error"),
            }
        }
        if let Some(value) = toml.get("bind_addr") {
            match value {
                &toml::Value::String(ref addr_str) => {
                    // JW TODO: handle this
                    let bind_addr = net::Ipv4Addr::from_str(addr_str).unwrap();
                    cfg.listen_addr = ListenAddr(bind_addr)
                }
                _ => panic!("JW TODO: handle this error"),
            }
        }
        if let Some(value) = toml.get("port") {
            match value {
                &toml::Value::Integer(port) => cfg.port = ListenPort(port as u16),
                _ => panic!("JW TODO: handle this error"),
            }
        }
        if toml.contains_key("datastore_addr") || toml.contains_key("datastore_port") {
            let ip = match toml.get("datastore_addr") {
                Some(&toml::Value::String(ref addr_str)) => {
                    // JW TODO: handle this error
                    net::Ipv4Addr::from_str(addr_str).unwrap()
                }
                Some(_) => panic!("JW TODO: handle this error"),
                None => net::Ipv4Addr::new(127, 0, 0, 1),
            };
            let port = match toml.get("datastore_port") {
                Some(&toml::Value::Integer(port)) => port as u16,
                Some(_) => panic!("handle"),
                None => 6379,
            };
            cfg.datastore_addr = net::SocketAddrV4::new(ip, port);
        }
        Ok(cfg)
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
