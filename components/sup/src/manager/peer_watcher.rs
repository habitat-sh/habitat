use crate::{error::{Error,
                    Result},
            manager::file_watcher::{Callbacks,
                                    FileWatcher}};

use habitat_butterfly::member::Member;
use habitat_common::{liveliness_checker,
                     outputln,
                     types::GossipListenAddr};
use std::{fs::File,
          io::{BufRead,
               BufReader},
          net::{SocketAddr,
                ToSocketAddrs},
          path::{Path,
                 PathBuf},
          sync::{atomic::{AtomicBool,
                          Ordering},
                 Arc},
          thread::Builder as ThreadBuilder};

static LOGKEY: &str = "PW";

pub struct PeerCallbacks {
    have_events: Arc<AtomicBool>,
}

impl Callbacks for PeerCallbacks {
    fn file_appeared(&mut self, _: &Path) { self.have_events.store(true, Ordering::Relaxed); }

    fn file_modified(&mut self, _: &Path) { self.have_events.store(true, Ordering::Relaxed) }

    fn file_disappeared(&mut self, _: &Path) { self.have_events.store(true, Ordering::Relaxed) }
}

pub struct PeerWatcher {
    path:        PathBuf,
    have_events: Arc<AtomicBool>,
}

impl PeerWatcher {
    pub fn run<P>(path: P) -> Result<Self>
        where P: Into<PathBuf>
    {
        let path = path.into();
        let have_events = Self::setup_watcher(path.clone())?;

        Ok(PeerWatcher { path, have_events })
    }

    fn setup_watcher(path: PathBuf) -> Result<Arc<AtomicBool>> {
        let have_events = Arc::new(AtomicBool::new(true));
        let have_events_for_thread = Arc::clone(&have_events);

        ThreadBuilder::new().name(format!("peer-watcher-[{}]", path.display()))
                            .spawn(move || -> liveliness_checker::ThreadUnregistered {
                                // debug!("PeerWatcher({}) thread starting", abs_path.display());
                                loop {
                                    let checked_thread = liveliness_checker::mark_thread_alive();
                                    let have_events_for_loop = Arc::clone(&have_events_for_thread);
                                    if Self::file_watcher_loop_body(&path, have_events_for_loop) {
                                        break checked_thread.unregister(Ok(()));
                                    }
                                }
                            })?;
        Ok(have_events)
    }

    fn file_watcher_loop_body(path: &Path, have_events: Arc<AtomicBool>) -> bool {
        let callbacks = PeerCallbacks { have_events };
        let mut file_watcher = match FileWatcher::<PeerCallbacks>::new(path, callbacks, true) {
            Ok(watcher) => watcher,
            Err(e) => {
                match e {
                    Error::NotifyError(err) => {
                        outputln!("PeerWatcher({}) failed to start watching the directories \
                                   ({}), {}",
                                  path.display(),
                                  err,
                                  "will try again");
                        return false;
                    }
                    _ => {
                        outputln!("PeerWatcher({}) could not create file watcher, ending thread \
                                   ({})",
                                  path.display(),
                                  e);
                        return true;
                    }
                }
            }
        };
        if let Err(err) = file_watcher.run() {
            outputln!("PeerWatcher({}) error during watching ({}), restarting",
                      path.display(),
                      err);
        }
        false
    }

    pub fn has_fs_events(&self) -> bool { self.have_events.load(Ordering::Relaxed) }

    pub fn get_members(&self) -> Result<Vec<Member>> {
        if !self.path.is_file() {
            self.have_events.store(false, Ordering::Relaxed);
            return Ok(Vec::new());
        }

        let file = File::open(&self.path).map_err(Error::Io)?;
        let reader = BufReader::new(file);
        let mut members: Vec<Member> = Vec::new();

        for line_result in reader.lines() {
            let line = line_result.map_err(Error::Io)?;
            let peer_addr = if line.find(':').is_some() {
                line
            } else {
                format!("{}:{}", line, GossipListenAddr::DEFAULT_PORT)
            };
            let addrs: Vec<SocketAddr> = match peer_addr.to_socket_addrs() {
                Ok(addrs) => addrs.collect(),
                Err(e) => {
                    outputln!("Failed to resolve peer: {}", peer_addr);
                    return Err(Error::NameLookup(e));
                }
            };
            let addr: SocketAddr = addrs[0];
            let member = Member { address: format!("{}", addr.ip()),
                                  swim_port: addr.port(),
                                  gossip_port: addr.port(),
                                  ..Default::default() };
            members.push(member);
        }

        self.have_events.store(false, Ordering::Relaxed);
        Ok(members)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use habitat_butterfly::member::Member;
    use habitat_core::locked_env_var;
    use std::{fs::{File,
                   OpenOptions},
              io::Write,
              path::{Path,
                     PathBuf}};
    use tempfile::TempDir;

    locked_env_var!(HAB_STUDIO_HOST_ARCH, lock_env_var);

    fn peer_watcher_member_load_test(watch_dir: &Path,
                                     peer_data: &[String])
                                     -> Result<Vec<Member>> {
        let path = PathBuf::from(watch_dir).join("some_file");
        let mut file = OpenOptions::new().append(true)
                                         .create_new(true)
                                         .open(path.clone())
                                         .unwrap();
        let watcher = PeerWatcher::run(path).unwrap();
        for line in peer_data {
            writeln!(file, "{}", line).unwrap();
        }
        watcher.get_members()
    }

    #[test]
    fn no_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("no_such_file");

        let lock = lock_env_var();
        lock.unset();
        let watcher = PeerWatcher::run(path).unwrap();

        // The watcher always has events initially
        assert!(watcher.has_fs_events());
        // We verify that the watcher finds no peers, since the file doesn't exist
        assert_eq!(watcher.get_members().unwrap(), vec![]);
        // The watcher now has no more events
        assert!(!watcher.has_fs_events());
    }

    #[test]
    fn empty_file() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("empty_file");
        File::create(&path).unwrap();

        let lock = lock_env_var();
        lock.unset();
        let watcher = PeerWatcher::run(path).unwrap();

        assert_eq!(watcher.get_members().unwrap(), vec![]);
    }

    #[test]
    fn empty_file_with_poll_watcher() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("empty_file");
        File::create(&path).unwrap();

        let lock = lock_env_var();
        lock.set("aarch64-darwin");
        let watcher = PeerWatcher::run(path).unwrap();
        lock.unset();

        assert_eq!(watcher.get_members().unwrap(), vec![]);
    }

    #[test]
    fn with_file() {
        let tmpdir = TempDir::new().unwrap();

        let peer_lines = vec!["1.2.3.4:5".to_string(), "4.3.2.1".to_string()];

        let lock = lock_env_var();
        lock.unset();
        let mut members = peer_watcher_member_load_test(tmpdir.path(), &peer_lines).unwrap();

        for member in &mut members {
            member.id = String::new();
        }
        let member1 = Member { id: String::new(),
                               address: String::from("1.2.3.4"),
                               swim_port: 5,
                               gossip_port: 5,
                               ..Default::default() };
        let member2 = Member { id: String::new(),
                               address: String::from("4.3.2.1"),
                               swim_port: GossipListenAddr::DEFAULT_PORT,
                               gossip_port: GossipListenAddr::DEFAULT_PORT,
                               ..Default::default() };
        let expected_members = vec![member1, member2];
        assert_eq!(expected_members, members);
    }

    #[test]
    fn with_file_using_poll_watcher() {
        let tmpdir = TempDir::new().unwrap();
        let peer_lines = vec!["1.2.3.5:5".to_string(), "5.4.3.2".to_string()];
        let lock = lock_env_var();
        lock.set("aarch64-darwin");
        let mut members = peer_watcher_member_load_test(tmpdir.path(), &peer_lines).unwrap();
        lock.unset();
        for member in &mut members {
            member.id = String::new();
        }
        let member1 = Member { id: String::new(),
                               address: String::from("1.2.3.5"),
                               swim_port: 5,
                               gossip_port: 5,
                               ..Default::default() };
        let member2 = Member { id: String::new(),
                               address: String::from("5.4.3.2"),
                               swim_port: GossipListenAddr::DEFAULT_PORT,
                               gossip_port: GossipListenAddr::DEFAULT_PORT,
                               ..Default::default() };
        let expected_members = vec![member1, member2];
        assert_eq!(expected_members, members);
    }
}
