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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{SocketAddr, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::Builder as ThreadBuilder;

use butterfly::member::Member;
use common::cli_defaults::GOSSIP_DEFAULT_PORT;
use error::{Error, Result};
use manager::file_watcher::{default_file_watcher, Callbacks};

static LOGKEY: &'static str = "PW";

pub struct PeerCallbacks {
    have_events: Arc<AtomicBool>,
}

impl Callbacks for PeerCallbacks {
    fn file_appeared(&mut self, _: &Path) {
        self.have_events.store(true, Ordering::Relaxed);
    }

    fn file_modified(&mut self, _: &Path) {
        self.have_events.store(true, Ordering::Relaxed)
    }

    fn file_disappeared(&mut self, _: &Path) {
        self.have_events.store(true, Ordering::Relaxed)
    }
}

pub struct PeerWatcher {
    path: PathBuf,
    have_events: Arc<AtomicBool>,
}

impl PeerWatcher {
    pub fn run<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let have_events = Self::setup_watcher(path.clone())?;

        Ok(PeerWatcher {
            path: path,
            have_events: have_events,
        })
    }

    fn setup_watcher(path: PathBuf) -> Result<Arc<AtomicBool>> {
        let have_events = Arc::new(AtomicBool::new(false));
        let have_events_for_thread = Arc::clone(&have_events);

        ThreadBuilder::new()
            .name(format!("peer-watcher-[{}]", path.display()))
            .spawn(move || {
                //debug!("PeerWatcher({}) thread starting", abs_path.display());
                loop {
                    let have_events_for_loop = Arc::clone(&have_events_for_thread);
                    if Self::file_watcher_loop_body(&path, have_events_for_loop) {
                        break;
                    }
                }
            })?;
        Ok(have_events)
    }

    fn file_watcher_loop_body(path: &PathBuf, have_events: Arc<AtomicBool>) -> bool {
        let callbacks = PeerCallbacks {
            have_events: have_events,
        };
        let mut file_watcher = match default_file_watcher(&path, callbacks) {
            Ok(w) => w,
            Err(sup_err) => match sup_err.err {
                Error::NotifyError(err) => {
                    outputln!(
                        "PeerWatcher({}) failed to start watching the directories ({}), {}",
                        path.display(),
                        err,
                        "will try again",
                    );
                    return false;
                }
                _ => {
                    outputln!(
                        "PeerWatcher({}) could not create file watcher, ending thread ({})",
                        path.display(),
                        sup_err
                    );
                    return true;
                }
            },
        };
        if let Err(err) = file_watcher.run() {
            outputln!(
                "PeerWatcher({}) error during watching ({}), restarting",
                path.display(),
                err
            );
        }
        false
    }

    pub fn has_fs_events(&self) -> bool {
        self.have_events.load(Ordering::Relaxed)
    }

    pub fn get_members(&self) -> Result<Vec<Member>> {
        if !self.path.is_file() {
            self.have_events.store(false, Ordering::Relaxed);
            return Ok(Vec::new());
        }
        let file = File::open(&self.path).map_err(|err| {
            return sup_error!(Error::Io(err));
        })?;
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
        self.have_events.store(false, Ordering::Relaxed);
        Ok(members)
    }
}

#[cfg(test)]
mod tests {
    use super::PeerWatcher;
    use butterfly::member::Member;
    use common::cli_defaults::GOSSIP_DEFAULT_PORT;
    use std::fs::{File, OpenOptions};
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn no_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("no_such_file");
        let watcher = PeerWatcher::run(path).unwrap();

        assert_eq!(false, watcher.has_fs_events());
        assert_eq!(watcher.get_members().unwrap(), vec![]);
    }

    #[test]
    fn empty_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("empty_file");
        File::create(&path).unwrap();
        let watcher = PeerWatcher::run(path).unwrap();

        assert_eq!(watcher.get_members().unwrap(), vec![]);
    }

    #[test]
    fn with_file() {
        let tmpdir = TempDir::new().unwrap();
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
