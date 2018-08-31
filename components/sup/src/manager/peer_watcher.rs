// Copyright (c) 2017 Chef Software Inc. and/or applicable contributors
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

use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::PathBuf;

use butterfly::member::Member;
use config::GOSSIP_DEFAULT_PORT;
use error::{Error, Result};
use manager::simple_file_watcher::SimpleFileWatcher;

static LOGKEY: &'static str = "PW";

pub struct PeerWatcher(SimpleFileWatcher);

impl PeerWatcher {
    pub fn run<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        Ok(PeerWatcher(SimpleFileWatcher::run(
            "peer-watcher".to_string(),
            path,
        )?))
    }

    pub fn has_fs_events(&self) -> bool {
        self.0.has_fs_events()
    }

    pub fn get_members(&self) -> Result<Vec<Member>> {
        let file = match self.0.open_file()? {
            Some(file) => file,
            None => {
                self.0.clear_events();
                return Ok(Vec::new());
            }
        };
        let reader = BufReader::new(file);
        let mut members: Vec<Member> = Vec::new();
        for line in reader.lines() {
            if let Ok(peer) = line {
                let peer_addr = if peer.find(':').is_some() {
                    peer
                } else {
                    format!("{}:{}", peer, GOSSIP_DEFAULT_PORT)
                };
                let addrs: Vec<SocketAddr> = match peer_addr.to_socket_addrs() {
                    Ok(addrs) => addrs.collect(),
                    Err(e) => {
                        outputln!("Failed to resolve peer: {}", peer_addr);
                        return Err(sup_error!(Error::NameLookup(e)));
                    }
                };
                let addr: SocketAddr = addrs[0];
                let mut member = Member::default();
                member.address = format!("{}", addr.ip());
                member.swim_port = addr.port();
                member.gossip_port = addr.port();
                members.push(member);
            }
        }
        self.0.clear_events();
        Ok(members)
    }
}

#[cfg(test)]
mod tests {
    use super::PeerWatcher;
    use butterfly::member::Member;
    use config::GOSSIP_DEFAULT_PORT;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use tempdir::TempDir;

    #[test]
    fn no_file() {
        let tmpdir = TempDir::new("peerwatchertest").unwrap();
        let path = tmpdir.path().join("no_such_file");
        let watcher = PeerWatcher::run(path).unwrap();

        assert_eq!(watcher.get_members().unwrap(), vec![]);
    }

    #[test]
    fn empty_file() {
        let tmpdir = TempDir::new("peerwatchertest").unwrap();
        let path = tmpdir.path().join("empty_file");
        File::create(&path).unwrap();
        let watcher = PeerWatcher::run(path).unwrap();

        assert_eq!(watcher.get_members().unwrap(), vec![]);
    }

    #[test]
    fn with_file() {
        let tmpdir = TempDir::new("peerwatchertest").unwrap();
        let path = tmpdir.path().join("some_file");
        let mut file = OpenOptions::new()
            .append(true)
            .create_new(true)
            .open(path.clone())
            .unwrap();
        let watcher = PeerWatcher::run(path).unwrap();
        writeln!(file, "1.2.3.4:5").unwrap();
        writeln!(file, "4.3.2.1").unwrap();
        let mut member1 = Member::default();
        member1.id = String::new();
        member1.address = String::from("1.2.3.4");
        member1.swim_port = 5;
        member1.gossip_port = 5;
        let mut member2 = Member::default();
        member2.id = String::new();
        member2.address = String::from("4.3.2.1");
        member2.swim_port = GOSSIP_DEFAULT_PORT;
        member2.gossip_port = GOSSIP_DEFAULT_PORT;
        let expected_members = vec![member1, member2];
        let mut members = watcher.get_members().unwrap();
        for mut member in &mut members {
            member.id = String::new();
        }
        assert_eq!(expected_members, members);
    }
}
