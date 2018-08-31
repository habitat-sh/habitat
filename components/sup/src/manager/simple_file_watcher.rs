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
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::Builder as ThreadBuilder;

use error::{Error, Result};
use manager::file_watcher::{default_file_watcher, Callbacks};

static LOGKEY: &'static str = "SFW";

struct SimpleFileWatcherCallbacks {
    have_events: Arc<AtomicBool>,
}

impl Callbacks for SimpleFileWatcherCallbacks {
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

pub struct SimpleFileWatcher {
    path: PathBuf,
    have_events: Arc<AtomicBool>,
}

impl SimpleFileWatcher {
    pub fn run<P>(kind: String, path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        let have_events = Self::setup_watcher(kind, path.clone())?;

        Ok(Self {
            path: path,
            have_events: have_events,
        })
    }

    pub fn has_fs_events(&self) -> bool {
        self.have_events.load(Ordering::Relaxed)
    }

    pub fn clear_events(&self) {
        self.have_events.store(false, Ordering::Relaxed);
    }

    pub fn open_file(&self) -> Result<Option<File>> {
        match File::open(&self.path) {
            Ok(file) => Ok(Some(file)),
            Err(e) => if e.kind() == ErrorKind::NotFound {
                Ok(None)
            } else {
                Err(sup_error!(Error::Io(e)))
            },
        }
    }

    fn setup_watcher(kind: String, path: PathBuf) -> Result<Arc<AtomicBool>> {
        let have_events = Arc::new(AtomicBool::new(false));
        let have_events_for_thread = Arc::clone(&have_events);

        ThreadBuilder::new()
            .name(format!("simple-file-watcher-[{}:{}]", kind, path.display()))
            .spawn(move || {
                outputln!(
                    "SimpleFileWatcher({}:{}): started thread",
                    kind,
                    path.display(),
                );
                loop {
                    let have_events_for_loop = Arc::clone(&have_events_for_thread);
                    if Self::file_watcher_loop_body(&kind, &path, have_events_for_loop) {
                        break;
                    }
                }
            })?;

        Ok(have_events)
    }

    fn file_watcher_loop_body(kind: &str, path: &PathBuf, have_events: Arc<AtomicBool>) -> bool {
        let callbacks = SimpleFileWatcherCallbacks {
            have_events: have_events,
        };
        let mut file_watcher = match default_file_watcher(&path, callbacks) {
            Ok(w) => w,
            Err(sup_err) => match sup_err.err {
                Error::NotifyError(err) => {
                    outputln!(
                        "SimpleFileWatcher({}:{}) failed to start watching the directories ({}), {}",
                        kind,
                        path.display(),
                        err,
                        "will try again",
                    );
                    return false;
                }
                _ => {
                    outputln!(
                        "SimpleFileWatcher({}:{}) could not create file watcher, ending thread ({})",
                        kind,
                        path.display(),
                        sup_err
                    );
                    return true;
                }
            },
        };
        if let Err(err) = file_watcher.run() {
            outputln!(
                "SimpleFileWatcher({}:{}) error during watching ({}), restarting",
                kind,
                path.display(),
                err
            );
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::path::PathBuf;
    use std::thread;
    use std::time::{Duration, Instant};

    use super::SimpleFileWatcher;
    use tempdir::TempDir;

    fn run_watcher<P: Into<PathBuf>>(path: P) -> SimpleFileWatcher {
        SimpleFileWatcher::run("test".to_string(), path).unwrap()
    }

    #[test]
    fn no_file() {
        let tmpdir = TempDir::new("simplefilewatchertest").unwrap();
        let path = tmpdir.path().join("no_such_file");
        let watcher = run_watcher(path);

        assert_eq!(false, watcher.has_fs_events());
    }

    #[test]
    fn initial_file() {
        let tmpdir = TempDir::new("simplefilewatchertest").unwrap();
        let path = tmpdir.path().join("file");
        File::create(&path).unwrap();
        let watcher = run_watcher(path);
        let now = Instant::now();
        let check_duration = Duration::from_secs(10);
        let sleep_duration = Duration::from_millis(100);
        let mut ok = false;

        while now.elapsed() < check_duration {
            if watcher.has_fs_events() {
                ok = true;
                break;
            }
            thread::sleep(sleep_duration);
        }

        assert!(ok);
    }

    #[test]
    fn created_file() {
        let tmpdir = TempDir::new("simplefilewatchertest").unwrap();
        let path = tmpdir.path().join("file");
        let watcher = run_watcher(path.clone());

        let sleep_duration = Duration::from_millis(100);
        let now1 = Instant::now();
        let check1_duration = Duration::from_secs(2);

        while now1.elapsed() < check1_duration {
            assert_eq!(false, watcher.has_fs_events());
            thread::sleep(sleep_duration);
        }

        File::create(&path).unwrap();

        let now2 = Instant::now();
        let check2_duration = Duration::from_secs(10);
        let mut ok = false;

        while now2.elapsed() < check2_duration {
            if watcher.has_fs_events() {
                ok = true;
                break;
            }
            thread::sleep(sleep_duration);
        }

        assert!(ok);
    }
}
