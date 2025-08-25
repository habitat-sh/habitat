use crate::error::Result;
use habitat_common::liveliness_checker;

use std::{fmt::Debug,
          path::{Path,
                 PathBuf},
          thread,
          time::{Duration,
                 SystemTime}};

#[allow(unused)]
pub const WATCHER_DELAY_MS: u64 = 2_000;

#[derive(Debug)]
/// WatchedFile bundles data together concerning the file that is to be watched.
struct WatchedFile {
    /// The file that is to be watched.
    path:   PathBuf,
    /// Did the file we are watching exist the last time we looked?
    exists: bool,
    /// The modification time captured the last time we needed to check.
    mtime:  Option<SystemTime>,
}

impl WatchedFile {
    fn new(path: &Path) -> Result<Self> {
        let exists = path.is_file();
        let mtime = if exists {
            Some(std::fs::metadata(path)?.modified()?)
        } else {
            None
        };
        let path = path.to_path_buf();
        Ok(Self { path,
                  exists,
                  mtime })
    }

    pub fn assess_existence(&self) -> bool { self.path.is_file() }

    pub fn read_mtime(&self) -> Option<SystemTime> {
        match std::fs::metadata(&self.path) {
            Ok(metadata) => metadata.modified().ok(),
            Err(_) => None,
        }
    }
}

/// A set of callbacks for the watched file events.
pub trait Callbacks {
    /// A function that gets called when a watched file appears on the file system.
    ///
    /// 'real_path' will contain a path to the watched file, which may be different from the one
    /// that was passed to 'FileWatcher', because of symlinks.
    fn file_appeared(&mut self, real_path: &Path);
    /// A function that gets called when the watched file changes.
    ///
    /// Note that this is called only when the real file is modified. In case when some symlink in
    /// the watched paths is atomically changed to point to something else, 'file_disappeared'
    /// followed by 'file_appeared' will be actually called.
    ///
    /// 'real_path' will contain a path to the
    /// watched file, which may be different from the one that was passed to 'FileWatcher', because
    /// of symlinks.
    fn file_modified(&mut self, real_path: &Path);
    /// A function that gets called when the watched file goes away.
    ///
    /// 'real_path' will contain a path to the watched file, which may be different from the one
    /// that was passed to 'FileWatcher', because of symlinks.
    fn file_disappeared(&mut self, real_path: &Path);
}

/// FileWatcher watches for a regular file at any path. The file does not have to exist when the
/// FileWatcher is created.
///
/// FileWatcher uses callbacks to notify about events occuring to the watched file. Note that it
/// issues the file_appeared callback during its first run if the file existed when the watcher was
/// created and ignore_initial is set false.
#[derive(Debug)]
pub struct FileWatcher<C: Callbacks> {
    /// The Callback implementation to be used by this FileWatcher instance.
    callbacks:          C,
    /// Data about the file path we are be watching
    watched_file:       WatchedFile,
    /// Used to signal if an initial Callbacks::file_appeared should be sent
    /// when creating the watched file.  The file must exist on the file system
    /// and this must be set true.  After the first execution this value will
    /// set back to false as the intial true value has fulfilled its purpose.
    send_initial_event: bool,
}

impl<C: Callbacks> FileWatcher<C> {
    /// Creates an instance of 'W' and creates a WatchedFile using the supplied path.  If
    /// ignore_initial is passed as false and the WatchedFile exists when this is called then the
    /// first loop will emit an initial "file_appeared" event.
    pub fn new(path: &Path, callbacks: C, ignore_initial: bool) -> Result<Self> {
        let watched_file = WatchedFile::new(path)?;
        Ok(Self { callbacks,
                  watched_file,
                  send_initial_event: !ignore_initial })
    }

    /// The run loop for FileWatcher instances.
    pub fn run(&mut self) -> Result<()> {
        let loop_value: liveliness_checker::ThreadUnregistered<_, _> = loop {
            let checked_thread = liveliness_checker::mark_thread_alive();
            if let result @ Err(_) = self.single_iteration() {
                break checked_thread.unregister(result);
            }
            thread::sleep(Duration::from_secs(1));
        };
        loop_value.into_result()
    }

    /// Called in our run loop, this is a single iteration wherein the FileWatcher captures its
    /// latest observation of the WatchedFile and issues callback based on previous vs. current
    /// state and/or notiications from the notify crate.
    pub fn single_iteration(&mut self) -> Result<()> {
        if self.send_initial_event {
            if self.watched_file.exists {
                self.callbacks.file_appeared(&self.watched_file.path);
            }
            self.send_initial_event = false;
            return Ok(());
        }

        let prev_existence = self.watched_file.exists;
        let next_existence = self.watched_file.assess_existence();

        match (prev_existence, next_existence) {
            (true, true) => {
                let mtime = self.watched_file.read_mtime();
                if mtime != self.watched_file.mtime {
                    self.callbacks.file_modified(&self.watched_file.path);
                    self.watched_file.mtime = mtime;
                };
                Ok(())
            }
            (true, false) => {
                self.callbacks.file_disappeared(&self.watched_file.path);
                self.watched_file.exists = false;
                self.watched_file.mtime = None;
                Ok(())
            }
            (false, true) => {
                self.callbacks.file_appeared(&self.watched_file.path);
                self.watched_file.exists = true;
                self.watched_file.mtime = self.watched_file.read_mtime();
                Ok(())
            }
            (false, false) => Ok(()),
        }
    }
}

// scenario, that involves symlinks.
#[cfg(test)]
mod tests {

    use super::*;

    use multimap::MultiMap;
    use tempfile::tempdir;

    mod file_watcher {
        use super::*;

        #[test]
        fn send_initial_event() -> Result<()> {
            _send_initial_event(true)?;
            _send_initial_event(false)
        }

        #[allow(clippy::cognitive_complexity)]
        fn _send_initial_event(with_relative_paths: bool) -> Result<()> {
            let tempdir = tempdir()?;
            let td_dir = if with_relative_paths {
                fs::make_relative_to_tempdir(&tempdir.keep())?.join("enclosing_dir")
            } else {
                tempdir.keep().join("enclosing_dir")
            };
            let td_file = td_dir.join("empty.txt");

            let mut fw = Tfw::new(&td_file, TestCallbacks::default(), false)?;
            fw.single_iteration()?;
            fw.single_iteration()?;
            assert_eq!(fw.callbacks.all_callbacks_count(), 0);

            fs::create_dir(&td_dir)?;
            fw = Tfw::new(&td_file, TestCallbacks::default(), false)?;
            fw.single_iteration()?;
            fw.single_iteration()?;
            assert_eq!(fw.callbacks.all_callbacks_count(), 0);

            fs::create_file(&td_file)?;
            fw = Tfw::new(&td_file, TestCallbacks::default(), false)?;
            assert_eq!(fw.callbacks.all_callbacks_count(), 0);
            fw.single_iteration()?;
            assert_eq!(fw.callbacks.callback_type_count(Appeared), 1);
            fw.single_iteration()?;
            assert_eq!(fw.callbacks.all_callbacks_count(), 1);

            Ok(())
        }

        mod handle_create_event {
            use super::*;

            #[test]
            fn create_watched_file_from_nonexistent() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::create_dir(&td_enclosing_dir)?;
                fs::create_file(&td_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn create_watched_file_from_existentdirectory() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::create_file(&td_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn create_watched_file_in_current_directory() -> Result<()> {
                let s = fs::make_uniq("unit-test-file");
                let rcd_watched_file = Path::new(&s);
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(rcd_watched_file, tc, true)?;

                fs::create_file(rcd_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(rcd_watched_file));

                fs::remove(rcd_watched_file)?;

                Ok(())
            }

            #[test]
            fn create_watched_file_in_ancestral_directory() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                let td_file = td_enclosing_dir.join("foo.txt");
                let rtd_watched_file = fs::make_relative_to_tempdir(&td_file)?;
                fs::create_dir(&td_enclosing_dir)?;
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&rtd_watched_file, tc, true)?;
                // watching the relative path but creating the file using the absolute path

                fs::create_file(&td_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&rtd_watched_file));

                Ok(())
            }

            #[test]
            fn create_watched_file_in_descendant_directory() -> Result<()> {
                let cur_dir = PathBuf::from(".");
                let descendent_dir = cur_dir.join(fs::make_uniq("dir"));
                let descendent_watched_file = descendent_dir.join(fs::make_uniq("file"));
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&descendent_watched_file, tc, true)?;

                // we're watching the relative path but creating the file with the absolute path
                fs::create_dir(&descendent_dir)?;
                fs::create_file(&descendent_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1,
                           fw.callbacks.path_instance_count(&descendent_watched_file));

                fs::remove(&descendent_watched_file)?;
                fs::remove(&descendent_dir)?;

                Ok(())
            }
        }

        mod handle_modify_event {
            use super::*;

            #[test]
            // This test was confirmed to hit the following for notify v5.1.0
            //  * Modify(Data(Any)) on Linux/inotify
            //  * Modify(Metadata(WriteTime)) on PollWatcher
            //  * hits Modify(Any) on Windows/ReadDirectoryChangesW
            fn watched_file_size_change() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");

                fs::create_dir(&td_enclosing_dir)?;
                fs::create_file(&td_watched_file)?;

                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                    fs::append_to_file(&td_watched_file)?;
                    // There seems to be threshold somewhere that necessitates appending until we
                    // go beyond it.  If we don't do this the PollWatcher version never alerts and
                    // this test
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Modified));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }
        }

        mod handle_remove_event {
            use super::*;

            #[test]
            fn delete_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_watched_file)?;
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::remove(&td_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn delete_directory_enclosing_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_watched_file)?;
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::remove(&td_enclosing_dir)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }
        }

        mod handle_rename_event {
            use super::*;

            #[test]
            // mv enclosing_dir/wathced_file.txt enclosing_dir/file.txt
            // the name of the watched file is changed such that it disappears
            fn rename_from_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_watched_file)?;
                let td_to_file = td_enclosing_dir.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::rename(&td_watched_file, &td_to_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            // mv enclosing_dir/file.txt enclosing_dir/watched_file.txt
            // the name of the watched file is changed such that it appears
            fn rename_to_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_from_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_from_file)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::rename(&td_from_file, &td_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            // mv enclosing_dir/file.txt dir/file.txt
            // file keeps its name but its location changes such that it disappears
            fn move_from_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_watched_file)?;
                let td_to_dir = td_enclosing_dir.join("dir");

                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_watched_file, &td_to_dir)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            // mv dir/watched_filefile.txt enclosing_dir/watched_file.txt
            // file keeps its name but its location is changes such that it appears
            fn move_to_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_from_dir = tempdir.path().join("dir");
                fs::create_dir(&td_from_dir)?;
                let td_from_file = td_from_dir.join("watched_file.txt");
                fs::create_file(&td_from_file)?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_from_file, &td_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn move_directory_enclosing_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_watched_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_watched_file)?;
                let td_to_dir = tempdir.path().join("dir");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_enclosing_dir, &td_to_dir)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }
                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn move_directory_that_would_enclose_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_dir_one = tempdir.path().join("dir_one");
                fs::create_dir(&td_dir_one)?;
                let td_dir_two = tempdir.path().join("dir_two");
                let td_watched_file = td_dir_two.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_dir_one, &td_dir_two)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if !fw.watched_file.exists {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(0, fw.callbacks.all_callbacks_count());
                fs::create_file(&td_watched_file)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }
        }

        mod symlinks {
            use super::*;

            #[test]
            fn create_watched_file_symlink() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_watched_symlink = td_enclosing_dir.join("symlink");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_symlink, tc, true)?;

                fs::symlink(&td_target_file, &td_watched_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_symlink));

                Ok(())
            }

            #[test]
            fn delete_watched_file_symlink() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_watched_symlink = td_enclosing_dir.join("symlink");
                fs::symlink(&td_target_file, &td_watched_symlink)?;
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_symlink, tc, true)?;

                fs::remove(&td_watched_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_symlink));

                Ok(())
            }

            #[test]
            fn create_enclosing_directory_symlink() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                let td_watched_file = td_dir_symlink.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::symlink(&td_enclosing_dir, &td_dir_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn delete_enclosing_directory_symlink() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                fs::symlink(&td_enclosing_dir, &td_dir_symlink)?;
                let td_watched_file = td_dir_symlink.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::remove(&td_dir_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn rename_symlink_from_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                fs::symlink(&td_enclosing_dir, &td_dir_symlink)?;
                let td_foo_symlink = tempdir.path().join("foo_symlink");
                let td_watched_file = td_dir_symlink.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::rename(&td_dir_symlink, &td_foo_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn rename_symlink_to_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                let td_foo_symlink = tempdir.path().join("foo_symlink");
                fs::symlink(&td_enclosing_dir, &td_foo_symlink)?;
                let td_watched_file = td_dir_symlink.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::rename(&td_foo_symlink, &td_dir_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn move_symlink_from_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                fs::symlink(&td_enclosing_dir, &td_dir_symlink)?;
                let td_foo_symlink = tempdir.path().join("foo_symlink");
                let td_watched_file = td_dir_symlink.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_dir_symlink, &td_foo_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn move_symlink_to_watched_file() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_target_file = td_enclosing_dir.join("file.txt");
                fs::create_file(&td_target_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                let td_foo_symlink = tempdir.path().join("foo_symlink");
                fs::symlink(&td_enclosing_dir, &td_foo_symlink)?;
                let td_watched_file = td_dir_symlink.join("file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_foo_symlink, &td_dir_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn rename_symlink_from_enclosing_directory() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_existent_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_existent_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                fs::symlink(&td_enclosing_dir, &td_dir_symlink)?;
                let td_watched_file = td_dir_symlink.join("watched_file.txt");
                let td_from_symlink = tempdir.path().join("from_symlink");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::rename(&td_dir_symlink, &td_from_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn rename_symlink_to_enclosing_directory() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_existent_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_existent_file)?;
                let td_from_symlink = tempdir.path().join("from_symlink");
                fs::symlink(&td_enclosing_dir, &td_from_symlink)?;
                let td_to_symlink = tempdir.path().join("dir_symlink");
                let td_watched_file = td_to_symlink.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::rename(&td_from_symlink, &td_to_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn move_symlink_from_enclosing_directory() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_existent_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_existent_file)?;
                let td_dir_symlink = tempdir.path().join("dir_symlink");
                fs::symlink(&td_enclosing_dir, &td_dir_symlink)?;
                let td_watched_file = td_dir_symlink.join("watched_file.txt");
                let td_from_symlink = tempdir.path().join("from_symlink");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_dir_symlink, &td_from_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }

            #[test]
            fn move_symlink_to_enclosing_directory() -> Result<()> {
                let tempdir = tempdir()?;
                let td_enclosing_dir = tempdir.path().join("enclosing_dir");
                fs::create_dir(&td_enclosing_dir)?;
                let td_existent_file = td_enclosing_dir.join("watched_file.txt");
                fs::create_file(&td_existent_file)?;
                let td_from_symlink = tempdir.path().join("from_symlink");
                fs::symlink(&td_enclosing_dir, &td_from_symlink)?;
                let td_to_symlink = tempdir.path().join("dir_symlink");
                let td_watched_file = td_to_symlink.join("watched_file.txt");
                let tc = TestCallbacks::default();
                let mut fw = Tfw::new(&td_watched_file, tc, true)?;

                fs::mv(&td_from_symlink, &td_to_symlink)?;

                for _ in 0..MAX_TEST_ATTEMPTS {
                    fw.single_iteration()?;
                    if fw.callbacks.all_callbacks_count() > 0 {
                        break;
                    }
                    sleep_well();
                }

                assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
                assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

                Ok(())
            }
        }
    }

    mod scenarios {
        use super::*;

        #[test]
        #[allow(clippy::cognitive_complexity)]
        // This unit test approximates an issue discovered via manual testing on Windows.
        // * deploy a service in habitat
        // * see that the user.toml is present and in effect
        // * rename the user.toml file to user.tuml
        // * see that Habitat begins to use the default configuration
        // * delete the config directory that user.toml (would) live in
        // * recreate the config directory
        // * recreate the user.toml file
        // The result was that the UserConfigWatcher was not seeing the user.toml when it returned
        fn first() -> Result<()> {
            let tempdir = tempdir()?;
            let td_enclosing_dir = tempdir.path().join("enclosing_dir");
            let td_watched_file = td_enclosing_dir.join("watched_file.txt");
            let tc = TestCallbacks::default();
            let mut fw = Tfw::new(&td_watched_file, tc, true)?;

            fs::create_dir(&td_enclosing_dir)?;
            fs::create_file(&td_watched_file)?;

            for _ in 0..MAX_TEST_ATTEMPTS {
                fw.single_iteration()?;
                if fw.callbacks.all_callbacks_count() > 0 {
                    break;
                }
                sleep_well();
            }

            assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
            assert_eq!(1, fw.callbacks.path_instance_count(&td_watched_file));

            // a --> o renaming
            let td_wotched_file = td_enclosing_dir.join("wotched_file.txt");
            fs::rename(&td_watched_file, &td_wotched_file)?;

            for _ in 0..MAX_TEST_ATTEMPTS {
                fw.single_iteration()?;
                if fw.callbacks.all_callbacks_count() > 1 {
                    break;
                }
                sleep_well();
            }

            assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
            assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
            assert_eq!(2, fw.callbacks.path_instance_count(&td_watched_file));

            fs::remove(&td_enclosing_dir)?;

            for _ in 0..MAX_TEST_ATTEMPTS {
                fw.single_iteration()?;
                if fw.callbacks.all_callbacks_count() > 1 {
                    break;
                }
                sleep_well();
            }

            assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
            assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
            assert_eq!(2, fw.callbacks.path_instance_count(&td_watched_file));

            fs::create_dir(&td_enclosing_dir)?;

            for _ in 0..MAX_TEST_ATTEMPTS {
                fw.single_iteration()?;
                if !fw.watched_file.exists {
                    break;
                }
                sleep_well();
            }

            assert_eq!(1, fw.callbacks.callback_type_count(Appeared));
            assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
            assert_eq!(2, fw.callbacks.path_instance_count(&td_watched_file));

            fs::create_file(&td_watched_file)?;

            for _ in 0..MAX_TEST_ATTEMPTS {
                fw.single_iteration()?;
                if fw.callbacks.all_callbacks_count() > 2 {
                    break;
                }
                sleep_well();
            }

            assert_eq!(2, fw.callbacks.callback_type_count(Appeared));
            assert_eq!(1, fw.callbacks.callback_type_count(Disappeared));
            assert_eq!(3, fw.callbacks.path_instance_count(&td_watched_file));

            Ok(())
        }
    }

    habitat_core::locked_env_var!(HAB_STUDIO_HOST_ARCH, lock_env_var);

    const TEST_SLEEP_MILLIS: u64 = 10;

    type Tfw = FileWatcher<TestCallbacks>;

    fn sleep_well() { thread::sleep(Duration::from_millis(TEST_SLEEP_MILLIS)); }

    const MAX_TEST_ATTEMPTS: u8 = 4;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
    enum CallbackType {
        Appeared,
        Modified,
        Disappeared,
    }

    use CallbackType::{Appeared,
                       Disappeared,
                       Modified};
    #[derive(Debug, Default)]
    struct TestCallbacks {
        type_occurrences: MultiMap<CallbackType, PathBuf>,
        path_occurrences: MultiMap<PathBuf, CallbackType>,
    }

    impl Callbacks for TestCallbacks {
        fn file_appeared(&mut self, real_path: &Path) { self.capture_callback(Appeared, real_path) }

        fn file_modified(&mut self, real_path: &Path) { self.capture_callback(Modified, real_path) }

        fn file_disappeared(&mut self, real_path: &Path) {
            self.capture_callback(Disappeared, real_path)
        }
    }

    impl TestCallbacks {
        fn capture_callback(&mut self, callback_type: CallbackType, path: &Path) {
            self.type_occurrences
                .insert(callback_type, path.to_path_buf());
            self.path_occurrences
                .insert(path.to_path_buf(), callback_type);
        }

        fn all_callbacks_count(&self) -> usize {
            self.type_occurrences.iter_all().map(|(_, v)| v.len()).sum()
        }

        fn callback_type_count(&self, t: CallbackType) -> usize {
            self.type_occurrences.get_vec(&t).map_or(0, Vec::len)
        }

        fn path_instance_count(&self, path: &Path) -> usize {
            self.path_occurrences
                .get_vec(&path.to_path_buf())
                .map_or(0, Vec::len)
        }
    }

    mod fs {
        use super::*;
        use std::{fs::File,
                  io,
                  io::Write,
                  path::{Component,
                         Path},
                  process::Command};

        /// Creates and syncs a file. The containing directory shuold exist before calling.
        pub fn create_file(path: &Path) -> io::Result<File> {
            let file = std::fs::File::create(path)?;
            file.sync_all()?;
            Ok(file)
        }

        /// Create a directory.  This is assumes "a complete path" from root to final directory.
        /// This function creates intermediate directories.
        pub fn create_dir(path: &Path) -> io::Result<()> { std::fs::create_dir_all(path) }

        /// if the path passed in
        pub fn remove(path: &Path) -> io::Result<()> {
            if path.is_dir() {
                std::fs::remove_dir_all(path)
                // Errors when path doesn’t exist, isn’t a directory,  we lack permissions,
                // or directory isn’t empty.
            } else {
                std::fs::remove_file(path)
                // Errors when file is a dir, doesn't exist, or we lack permissions to delete
                // handles symlinks on unix and windows
            }
        }

        pub fn append_to_file(path: &Path) -> io::Result<File> {
            let mut file = std::fs::OpenOptions::new().append(true).open(path)?;
            file.write_all(b"Lorem ipsum dolor sit amet,....")?;
            file.flush()?;
            file.sync_all()?;
            Ok(file)
        }

        pub fn make_uniq(s: &str) -> String { format!("{}-{}", s, uuid::Uuid::new_v4()) }

        pub fn make_relative_to_tempdir(path: &Path) -> Result<PathBuf> {
            let current_dir = std::env::current_dir()?;
            let mut relative_path = PathBuf::new();
            for component in current_dir.components() {
                if let Component::Normal(_) = component {
                    relative_path.push("..")
                }
            }
            for component in path.components() {
                if let Component::Normal(c) = component {
                    relative_path.push(c)
                }
            }
            Ok(relative_path)
        }

        pub fn rename(from: &Path, to: &Path) -> io::Result<()> {
            std::fs::rename(from, to)
            // https://doc.rust-lang.org/std/fs/fn.rename.html
        }

        pub fn mv(from: &Path, to: &Path) -> io::Result<()> {
            #[cfg(not(windows))]
            let status = Command::new("mv").arg(from).arg(to).status()?;
            #[cfg(windows)]
            let status = Command::new("powershell.exe").arg("Move-Item")
                                                       .arg(from)
                                                       .arg(to)
                                                       .status()?;
            if status.success() {
                Ok(())
            } else {
                let s = format!("mv {:?} {:?} failed", from, to);
                Err(io::Error::other(s))
            }
        }

        #[cfg(not(windows))]
        pub fn symlink(target: &Path, link: &Path) -> io::Result<()> {
            std::os::unix::fs::symlink(target, link)
        }

        #[cfg(windows)]
        pub fn symlink(target: &Path, link: &Path) -> io::Result<()> {
            if target.is_dir() {
                // https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
                std::os::windows::fs::symlink_dir(target, link)
            } else {
                // https://doc.rust-lang.org/std/os/windows/fs/fn.symlink_dir.html
                std::os::windows::fs::symlink_file(target, link)
            }
        }
    }
}
