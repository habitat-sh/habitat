// Copyright (c) 2018 Chef Software Inc. and/or applicable contributors
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

use std::collections::HashMap;
//use std::fs::File;
use std::io::Read;
use std::net::IpAddr;
use std::path::PathBuf;
//use std::result::Result as StdResult;

use error::{Error, Result};
use manager::simple_file_watcher::SimpleFileWatcher;

use butterfly::rumor::service::{Tag, TaggedPorts};
use butterfly::zone::AdditionalAddress;

use toml;

static LOGKEY: &'static str = "AAW";

pub type SupAdditionalAddress = AdditionalAddress<IpAddr>;
pub type SvcName = String;
pub type SupAdditionalAddresses = HashMap<Tag, SupAdditionalAddress>;
pub type SvcPorts = HashMap<SvcName, TaggedPorts>;

#[derive(Debug, PartialEq)]
pub struct AdditionalAddresses {
    pub sup: SupAdditionalAddresses,
    pub svc: SvcPorts,
}

#[derive(Deserialize)]
struct AdditionalAddressesToParse {
    sup: Option<SupAdditionalAddresses>,
    svc: Option<SvcPorts>,
}

impl From<AdditionalAddressesToParse> for AdditionalAddresses {
    fn from(aa: AdditionalAddressesToParse) -> Self {
        Self {
            sup: aa.sup.unwrap_or_else(|| HashMap::new()),
            svc: aa.svc.unwrap_or_else(|| HashMap::new()),
        }
    }
}

impl AdditionalAddresses {
    pub fn new() -> Self {
        Self {
            sup: HashMap::new(),
            svc: HashMap::new(),
        }
    }

    pub fn create_from_toml_string(s: &str) -> Result<Self> {
        let aa_to_parse: AdditionalAddressesToParse =
            toml::from_str(s).map_err(|e| sup_error!(Error::TomlParser(e)))?;

        Ok(aa_to_parse.into())
    }
}

pub struct AdditionalAddressesWatcher(SimpleFileWatcher);

impl AdditionalAddressesWatcher {
    pub fn run<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        Ok(AdditionalAddressesWatcher(SimpleFileWatcher::run(
            "additional-addresses-watcher".to_string(),
            path,
        )?))
    }

    pub fn has_fs_events(&self) -> bool {
        self.0.has_fs_events()
    }

    pub fn get_additional_addresses(&self) -> Result<AdditionalAddresses> {
        let mut file = match self.0.open_file()? {
            Some(file) => file,
            None => {
                self.0.clear_events();
                return Ok(AdditionalAddresses::new());
            }
        };
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .map_err(|e| sup_error!(Error::Io(e)))?;

        let additional_addresses = AdditionalAddresses::create_from_toml_string(&contents)?;

        self.0.clear_events();

        Ok(additional_addresses)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use std::net::{IpAddr, Ipv4Addr};

    use super::{AdditionalAddresses, AdditionalAddressesWatcher, SupAdditionalAddress};

    use tempdir::TempDir;

    #[test]
    fn no_file() {
        let tmpdir = TempDir::new("aawatchertest").unwrap();
        let path = tmpdir.path().join("no_such_file");
        let watcher = AdditionalAddressesWatcher::run(path).unwrap();

        assert_eq!(
            watcher.get_additional_addresses().unwrap(),
            AdditionalAddresses::new(),
        );
    }

    #[test]
    fn empty_file() {
        let tmpdir = TempDir::new("aawatchertest").unwrap();
        let path = tmpdir.path().join("empty_file");
        File::create(&path).unwrap();
        let watcher = AdditionalAddressesWatcher::run(path).unwrap();

        assert_eq!(
            watcher.get_additional_addresses().unwrap(),
            AdditionalAddresses::new(),
        );
    }

    fn ipaddr(a: u8, b: u8, c: u8, d: u8) -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(a, b, c, d))
    }

    #[test]
    fn with_file() {
        let tmpdir = TempDir::new("aawatchertest").unwrap();
        let path = tmpdir.path().join("some_file");
        let mut file = OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(path.clone())
            .unwrap();
        let watcher = AdditionalAddressesWatcher::run(path).unwrap();

        writeln!(
            file,
            r##"[sup.tag1]
                address = "1.2.3.4"
                swim_port = 1234
                gossip_port = 4321

                [sup.tag2]
                address = "2.3.4.5"
                swim_port = 2345
                gossip_port = 5432

                [svc.nginx.tag1]
                port = 3456
                ssl-port = 3457

                [svc.redis.tag1]
                port = 4567

                [svc.redis.tag2]
                port = 7654

            "##
        ).unwrap();
        let suptag1 = SupAdditionalAddress::new(ipaddr(1, 2, 3, 4), 1234, 4321);
        let suptag2 = SupAdditionalAddress::new(ipaddr(2, 3, 4, 5), 2345, 5432);
        let mut sup = HashMap::new();

        sup.insert("tag1".to_string(), suptag1);
        sup.insert("tag2".to_string(), suptag2);

        let mut svcnginxtag1 = HashMap::new();
        let mut svcnginx = HashMap::new();

        svcnginxtag1.insert("port".to_string(), 3456);
        svcnginxtag1.insert("ssl-port".to_string(), 3457);
        svcnginx.insert("tag1".to_string(), svcnginxtag1);

        let mut svcredistag1 = HashMap::new();
        let mut svcredistag2 = HashMap::new();
        let mut svcredis = HashMap::new();

        svcredistag1.insert("port".to_string(), 4567);
        svcredistag2.insert("port".to_string(), 7654);
        svcredis.insert("tag1".to_string(), svcredistag1);
        svcredis.insert("tag2".to_string(), svcredistag2);

        let mut svc = HashMap::new();

        svc.insert("nginx".to_string(), svcnginx);
        svc.insert("redis".to_string(), svcredis);

        let mut expected_aa = AdditionalAddresses::new();

        expected_aa.sup = sup;
        expected_aa.svc = svc;

        let gotten_aa = watcher.get_additional_addresses().unwrap();

        assert_eq!(expected_aa, gotten_aa);
    }
}
