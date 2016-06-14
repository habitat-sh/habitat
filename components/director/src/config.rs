// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::net::SocketAddrV4;
use std::str::FromStr;

use hcore::config::ConfigFile;

use toml;
use error::{Error, Result};
use super::ServiceDef;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Service defs read from config.toml
    pub service_defs: Vec<ServiceDef>,
    /// We might not be running under a Habitat supervisor
    pub dir_sup_listen: Option<SocketAddrV4>,
}

impl ConfigFile for Config {
    type Error = Error;

    fn from_toml(toml: toml::Value) -> Result<Self> {
        let mut cfg = Config::default();

        cfg.dir_sup_listen = try!(Config::parse_gossip_ip_port(&toml));

        let c = match toml.lookup("cfg.services") {
            Some(c) => c,
            None => return Err(Error::NoServices),
        };

        let paths = Config::traverse(c);

        for p in &paths {
            let mut sd = ServiceDef::from_str(p).unwrap();
            println!("Loaded service def: {}", &sd.to_string());
            let sdname = sd.to_string();
            sd.cli_args = Self::lookup_service_param(&toml, &sdname, "start");
            sd.ident.release = Self::lookup_service_param(&toml, &sdname, "release");
            sd.ident.version = Self::lookup_service_param(&toml, &sdname, "version");
            cfg.service_defs.push(sd);
        }
        Ok(cfg)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            service_defs: Vec::new(),
            dir_sup_listen: None,
        }
    }
}

impl Config {
    /// try to parse sys.gossip and sys.gossip_port.
    /// If we're not running under a supervisor, these values will
    /// most likely be empty.
    fn parse_gossip_ip_port(toml: &toml::Value) -> Result<Option<SocketAddrV4>> {
        let ip = toml.lookup("sys.gossip_ip")
            .and_then(|s| s.as_str());
        let port = toml.lookup("sys.gossip_port")
            .and_then(|s| s.as_integer());
        match (ip, port) {
            (Some(ip), Some(port)) => {
                let ip_port = format!("{}:{}", ip, port);
                match SocketAddrV4::from_str(&ip_port) {
                    Ok(addr) => Ok(Some(addr)),
                    Err(e) => {
                        let msg = format!("Can't parse gossip ip:port ({}): {}", ip_port, e);
                        Err(Error::DirectorError(msg))
                    }
                }
            }
            _ => Ok(None),
        }
    }

    /// Perform a lookup on a dynamic toml path as part of a service.
    /// For example, for a value `valuename` in service name
    /// `origin.name.group.org`, we'll perform a lookup on
    /// `cfg.services.origin.name.group.org.valuename` and return
    /// a Some(string) if it's available.
    fn lookup_service_param(toml: &toml::Value,
                            service_name: &str,
                            param_name: &str)
                            -> Option<String> {
        let key = format!("cfg.services.{}.{}", service_name, param_name);
        if let Some(k) = toml.lookup(&key) {
            if let Some(k) = k.as_str() {
                return Some(k.to_string());
            }
        }
        None
    }

    /// traverse a toml tree of Tables, return a list of
    /// Strings for each unique path
    fn traverse(v: &toml::Value) -> Vec<String> {
        fn _traverse(v: &toml::Value, path: &mut Vec<String>, paths: &mut Vec<String>) {
            let current_path = path.join(".");
            if let Some(tbl) = v.as_table() {
                // return if this table doesn't have any child tables
                if tbl.values().all(|ref v| v.as_table().is_none()) {
                    paths.push(current_path);
                    return;
                }
                for (k, v) in tbl.iter() {
                    path.push(k.clone());
                    _traverse(v, path, paths);
                    let _ = path.pop();
                }
            }
        }

        let mut path: Vec<String> = Vec::new();
        let mut paths: Vec<String> = Vec::new();
        _traverse(v, &mut path, &mut paths);
        paths
    }
}

#[cfg(test)]
mod tests {
    use toml;

    use hcore::config::ConfigFile;
    use super::*;

    #[test]
    fn test_parse_traversal() {
        // NOTE: these toml tables DO NOT contain the cfg.services prefix,
        // that will be pulled out via toml.lookup()
        let service_toml = r#"
        [core.redis.somegroup.someorg]
        start = "foo"
        [core.rngd.foo.someorg]
        start = "bar"
        [myorigin.xyz.foo.otherorg]
        [foo]
        "#;

        let root: toml::Value = service_toml.parse().unwrap();
        let paths = Config::traverse(&root);
        assert!(paths.contains(&"core.redis.somegroup.someorg".to_string()));
        assert!(paths.contains(&"core.rngd.foo.someorg".to_string()));
        assert!(paths.contains(&"myorigin.xyz.foo.otherorg".to_string()));
        // segment length doesn't matter, we'll filter those out later
        assert!(paths.contains(&"foo".to_string()));
    }


    #[test]
    fn test_from_toml_with_gossip_ip_port() {
        let service_toml = r#"

        [sys]
        gossip_ip="192.168.1.5"
        gossip_port=1234
        [cfg.services.core.redis.somegroup.someorg]
        start = "foo"

        # Comment
        [cfg.services.core.rngd.foo.someorg]
        start = "bar"
        # release + version must be in double quotes!
        release="20160427205048"
        version="1.1"

        [cfg.services.myorigin.xyz.foo.otherorg]

        [cfg.foo]
        # foo should be skipped. check the count of services, we should have
        # 3, not 4.
        "#;

        let root: toml::Value = service_toml.parse().unwrap();
        let cfg = Config::from_toml(root).unwrap();

        assert_eq!("192.168.1.5:1234",
                   cfg.dir_sup_listen.as_ref().unwrap().to_string());
        assert_eq!(3, cfg.service_defs.len());

        // first service
        assert_eq!("core.redis.somegroup.someorg",
                   cfg.service_defs[0].to_string());
        assert_eq!(Some("foo".to_string()), cfg.service_defs[0].cli_args);
        assert_eq!(None, cfg.service_defs[0].ident.release);
        assert_eq!(None, cfg.service_defs[0].ident.version);

        // second service
        assert_eq!("core.rngd.foo.someorg", cfg.service_defs[1].to_string());
        assert_eq!(Some("bar".to_string()), cfg.service_defs[1].cli_args);
        assert_eq!(Some("20160427205048".to_string()),
                   cfg.service_defs[1].ident.release);
        assert_eq!(Some("1.1".to_string()), cfg.service_defs[1].ident.version);

        // third service
        assert_eq!("myorigin.xyz.foo.otherorg", cfg.service_defs[2].to_string());
        assert_eq!(None, cfg.service_defs[2].cli_args);
        assert_eq!(None, cfg.service_defs[2].ident.release);
        assert_eq!(None, cfg.service_defs[2].ident.version);

    }

    #[test]
    fn test_from_toml_without_gossip_listen() {
        let service_toml = r#"
     [cfg.services.core.redis.somegroup.someorg]
     "#;

        let root: toml::Value = service_toml.parse().unwrap();
        let cfg = Config::from_toml(root).unwrap();
        assert_eq!(None, cfg.dir_sup_listen);
        assert_eq!(1, cfg.service_defs.len());
    }

}
