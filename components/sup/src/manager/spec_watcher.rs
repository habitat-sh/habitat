// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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

use std::collections::{HashMap, HashSet};
use std::error::Error as StdErr;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use glob::glob;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use error::{Error, Result};
use manager::service::ServiceSpec;

static LOGKEY: &'static str = "SW";
const WATCHER_DELAY_MS: u64 = 2_000;
const SPEC_FILE_EXT: &'static str = "spec";
const SPEC_FILE_GLOB: &'static str = "*.spec";

#[derive(Debug, PartialEq)]
pub enum SpecWatcherEvent {
    AddService(ServiceSpec),
    RemoveService(ServiceSpec),
}

pub struct SpecWatcher {
    watch_path: PathBuf,
    have_events: Arc<AtomicBool>,
}

impl SpecWatcher {
    pub fn run<P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
    {
        Self::run_with::<RecommendedWatcher, _>(path)
    }

    pub fn spec_files<T>(watch_path: T) -> Result<Vec<PathBuf>>
    where
        T: AsRef<Path>,
    {
        Ok(glob(
            &watch_path
                .as_ref()
                .join(SPEC_FILE_GLOB)
                .display()
                .to_string(),
        )?.filter_map(|p| p.ok())
        .filter(|p| p.is_file())
        .collect())
    }

    pub fn initial_events(&mut self) -> Result<Vec<SpecWatcherEvent>> {
        self.generate_events(HashMap::new())
    }

    pub fn new_events(
        &mut self,
        active_specs: HashMap<String, ServiceSpec>,
    ) -> Result<Vec<SpecWatcherEvent>> {
        if self.have_fs_events() {
            self.generate_events(active_specs)
        } else {
            Ok(vec![])
        }
    }

    fn run_with<W, P>(path: P) -> Result<Self>
    where
        P: Into<PathBuf>,
        W: Watcher,
    {
        let path = path.into();
        if !path.is_dir() {
            return Err(sup_error!(Error::SpecWatcherDirNotFound(
                path.display().to_string()
            )));
        }
        let have_events = Arc::new(AtomicBool::new(false));
        Self::setup_watcher::<W>(path.clone(), have_events.clone())?;

        Ok(SpecWatcher {
            watch_path: path,
            have_events: have_events,
        })
    }

    fn setup_watcher<W>(watch_path: PathBuf, have_events: Arc<AtomicBool>) -> Result<()>
    where
        W: Watcher,
    {
        thread::Builder::new()
            .name(format!("spec-watcher-{}", watch_path.display()))
            .spawn(move || {
                debug!("SpecWatcher({}) thread starting", watch_path.display());
                let (tx, rx) = channel();
                let mut watcher = match W::new(tx, Duration::from_millis(WATCHER_DELAY_MS)) {
                    Ok(w) => w,
                    Err(err) => {
                        outputln!(
                            "SpecWatcher({}) could not start notifier, ending thread ({})",
                            watch_path.display(),
                            err
                        );
                        return;
                    }
                };
                if let Err(err) = watcher.watch(&watch_path, RecursiveMode::NonRecursive) {
                    outputln!(
                        "SpecWatcher({}) could not start fs watching, ending thread ({})",
                        watch_path.display(),
                        err
                    );
                    return;
                }
                while let Ok(event) = rx.recv() {
                    debug!(
                        "SpecWatcher({}) file system event: {:?}",
                        watch_path.display(),
                        event
                    );
                    have_events.store(true, Ordering::Relaxed);
                }
                outputln!(
                    "SpecWatcher({}) fs watching died, restarting thread",
                    watch_path.display()
                );
                drop(watcher);
                Self::setup_watcher::<W>(watch_path.clone(), have_events.clone()).unwrap();
            })?;
        Ok(())
    }

    fn have_fs_events(&mut self) -> bool {
        self.have_events.load(Ordering::Relaxed)
    }

    fn generate_events(
        &mut self,
        mut active_specs: HashMap<String, ServiceSpec>,
    ) -> Result<Vec<SpecWatcherEvent>> {
        let mut desired_specs = self.specs_from_watch_path()?;
        // Reset the "have events" flag to false, now that we've loaded specs off disk
        self.have_events.store(false, Ordering::Relaxed);
        let desired_names: HashSet<_> = desired_specs.keys().map(|n| n.clone()).collect();
        let active_names: HashSet<_> = active_specs.keys().map(|n| n.clone()).collect();

        let mut events = Vec::new();

        // Eneueue a `RemoveService` for all services that no longer have a spec on disk.
        for name in active_names.difference(&desired_names) {
            let remove_spec = active_specs
                .remove(name)
                .expect("value should exist for key");
            let event = SpecWatcherEvent::RemoveService(remove_spec);
            debug!(
                "Service spec for {} is gone, enqueuing {:?} event",
                &name, &event
            );
            events.push(event);
        }

        // Eneueue an `AddService` for all new specs on disk without a corresponding service.
        for name in desired_names.difference(&active_names) {
            let add_spec = desired_specs
                .remove(name)
                .expect("value should exist for key");
            let event = SpecWatcherEvent::AddService(add_spec);
            debug!(
                "Service spec for {} is new, enqueuing {:?} event",
                &name, &event
            );
            events.push(event);
        }

        // Ensure each running service doesn't have a different spec on disk. If a difference is
        // found we're going to do the simple thing and remove, then add the service. In the future
        // we should attempt to update a service in-place, if possible.
        for name in active_names.intersection(&desired_names) {
            let active_spec = active_specs
                .remove(name)
                .expect("value should exist for key");
            let desired_spec = desired_specs
                .remove(name)
                .expect("value should exist for key");
            if active_spec != desired_spec {
                let remove_event = SpecWatcherEvent::RemoveService(active_spec);
                let add_event = SpecWatcherEvent::AddService(desired_spec);
                debug!(
                    "Service spec for {} is different on disk than loaded state, \
                     enqueuing {:?} for existing and {:?} event for updated spec",
                    &name, &remove_event, &add_event
                );
                events.push(remove_event);
                events.push(add_event);
            }
        }

        // Both maps should be empty, meaning we've processed them all
        assert!(active_specs.is_empty());
        assert!(desired_specs.is_empty());

        Ok(events)
    }

    pub fn specs_from_watch_path<'a>(&self) -> Result<HashMap<String, ServiceSpec>> {
        let mut specs = HashMap::new();
        for spec_file in Self::spec_files(&self.watch_path)? {
            let spec = match ServiceSpec::from_file(&spec_file) {
                Ok(s) => s,
                Err(e) => {
                    match e.err {
                        // If the error is related to loading a `ServiceSpec`, emit a warning
                        // message and continue on to the next spec file. The best we can do to
                        // fail-safe is report and skip.
                        Error::ServiceSpecParse(_) | Error::MissingRequiredIdent => {
                            outputln!(
                                "Error when loading service spec file '{}' ({}). \
                                 This file will be skipped.",
                                spec_file.display(),
                                e.description()
                            );
                            continue;
                        }
                        // All other errors are unexpected and should be dealt with up the calling
                        // stack.
                        _ => return Err(e),
                    }
                }
            };
            let file_stem = match spec_file.file_stem().and_then(OsStr::to_str) {
                Some(s) => s,
                None => {
                    outputln!(
                        "Error when loading service spec file '{}' \
                         (File stem could not be determined). \
                         This file will be skipped.",
                        spec_file.display()
                    );
                    continue;
                }
            };
            if file_stem != &spec.ident.name {
                outputln!(
                    "Error when loading service spec file '{}' \
                     (File name does not match ident name '{}' from ident = \"{}\", \
                     it should be called '{}.{}'). \
                     This file will be skipped.",
                    spec_file.display(),
                    &spec.ident.name,
                    &spec.ident,
                    &spec.ident.name,
                    SPEC_FILE_EXT
                );
                continue;
            }
            specs.insert(spec.ident.name.clone(), spec);
        }
        Ok(specs)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use std::fs;
    use std::io::Write;
    use std::path::Path;
    use std::str::FromStr;
    use std::sync::mpsc::Sender;
    use std::thread;
    use std::time::{Duration, Instant};

    use hcore::package::PackageIdent;
    use notify;
    use tempfile::TempDir;

    use super::{SpecWatcher, SpecWatcherEvent};
    use error::Error::*;
    use manager::service::ServiceSpec;

    #[test]
    fn run_watch_dir_not_created() {
        let tmpdir = TempDir::new().unwrap();
        let not_a_dir = tmpdir.path().join("i-dont-exist");

        match SpecWatcher::run(&not_a_dir) {
            Err(e) => match e.err {
                SpecWatcherDirNotFound(dir) => assert_eq!(dir, not_a_dir.display().to_string()),
                wrong => panic!("Unexpected error returned: {:?}", wrong),
            },
            Ok(_) => panic!("Watcher should fail to run"),
        }
    }

    #[test]
    fn run_with_notify_error() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("throw_error");
        fs::create_dir(&path).unwrap();

        match SpecWatcher::run_with::<TestWatcher, _>(&path) {
            Ok(_) => assert!(true),
            Err(e) => panic!("This should not fail: {:?}", e.err),
        }
    }

    #[test]
    fn inital_events() {
        let tmpdir = TempDir::new().unwrap();
        let alpha = new_saved_spec(tmpdir.path(), "acme/alpha");
        let beta = new_saved_spec(tmpdir.path(), "acme/beta");
        let mut watcher = SpecWatcher::run(tmpdir.path()).unwrap();

        let events = watcher.initial_events().unwrap();

        assert_eq!(2, events.len());
        assert!(events.contains(&SpecWatcherEvent::AddService(alpha)));
        assert!(events.contains(&SpecWatcherEvent::AddService(beta)));
    }

    #[test]
    fn inital_events_no_specs() {
        let tmpdir = TempDir::new().unwrap();
        let mut watcher = SpecWatcher::run(tmpdir.path()).unwrap();

        let events = watcher.initial_events().unwrap();

        assert_eq!(events, vec![]);
    }

    #[test]
    fn new_events_no_change_with_no_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("no_events");
        fs::create_dir(&path).unwrap();

        let active_specs = map_for_specs(vec![]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = watcher.new_events(active_specs).unwrap();

        assert_eq!(events, vec![]);
    }

    #[test]
    fn new_events_no_change_with_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("no_events");
        fs::create_dir(&path).unwrap();
        new_saved_spec(&path, "acme/alpha");
        new_saved_spec(&path, "acme/beta");

        let active_specs = map_for_specs(vec!["acme/alpha", "acme/beta"]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = watcher.new_events(active_specs).unwrap();

        assert_eq!(events, vec![]);
    }

    #[test]
    fn new_events_new_spec_with_no_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("new_spec");
        fs::create_dir(&path).unwrap();
        let newbie = new_spec("acme/newbie");

        let active_specs = map_for_specs(vec![]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = waiting_for_new_events(&mut watcher, active_specs);

        assert_eq!(1, events.len());
        assert!(events.contains(&SpecWatcherEvent::AddService(newbie)));
    }

    #[test]
    fn new_events_new_spec_with_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("new_spec");
        fs::create_dir(&path).unwrap();
        new_saved_spec(&path, "acme/alpha");
        new_saved_spec(&path, "acme/beta");
        let newbie = new_spec("acme/newbie");

        let active_specs = map_for_specs(vec!["acme/alpha", "acme/beta"]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = waiting_for_new_events(&mut watcher, active_specs);

        assert_eq!(1, events.len());
        assert!(events.contains(&SpecWatcherEvent::AddService(newbie)));
    }

    #[test]
    fn new_events_removed_spec_with_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("removed_spec");
        fs::create_dir(&path).unwrap();
        new_saved_spec(&path, "acme/alpha");
        new_saved_spec(&path, "acme/beta");
        let oldie = new_saved_spec(&path, "acme/oldie");

        let active_specs = map_for_specs(vec!["acme/alpha", "acme/beta", "acme/oldie"]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = waiting_for_new_events(&mut watcher, active_specs);

        assert_eq!(1, events.len());
        assert!(events.contains(&SpecWatcherEvent::RemoveService(oldie)));
    }

    #[test]
    fn new_events_add_and_removed_spec_with_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("new_and_removed_spec");
        fs::create_dir(&path).unwrap();
        new_saved_spec(&path, "acme/alpha");
        new_saved_spec(&path, "acme/beta");
        let oldie = new_saved_spec(&path, "acme/oldie");
        let newbie = new_spec("acme/newbie");

        let active_specs = map_for_specs(vec!["acme/alpha", "acme/beta", "acme/oldie"]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = waiting_for_new_events(&mut watcher, active_specs);

        assert_eq!(2, events.len());
        assert!(events.contains(&SpecWatcherEvent::RemoveService(oldie)));
        assert!(events.contains(&SpecWatcherEvent::AddService(newbie)));
    }

    #[test]
    fn new_events_changed_spec_with_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("changed_spec");
        fs::create_dir(&path).unwrap();
        new_saved_spec(&path, "acme/alpha");
        new_saved_spec(&path, "acme/beta");
        let transformer_before = new_saved_spec(&path, "acme/transformer");
        let mut transformer_after = new_spec("acme/transformer");
        transformer_after.group = String::from("autobots");

        let active_specs = map_for_specs(vec!["acme/alpha", "acme/beta", "acme/transformer"]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = waiting_for_new_events(&mut watcher, active_specs);

        assert_eq!(2, events.len());
        assert_eq!(
            events[0],
            SpecWatcherEvent::RemoveService(transformer_before)
        );
        assert_eq!(events[1], SpecWatcherEvent::AddService(transformer_after));
    }

    #[test]
    fn new_events_crazytown_with_active_specs() {
        let tmpdir = TempDir::new().unwrap();
        let path = tmpdir.path().join("crazytown");
        fs::create_dir(&path).unwrap();
        new_saved_spec(&path, "acme/alpha");
        new_saved_spec(&path, "acme/beta");
        let oldie = new_saved_spec(&path, "acme/oldie");
        let newbie = new_spec("acme/newbie");
        let transformer_before = new_saved_spec(&path, "acme/transformer");
        let mut transformer_after = new_spec("acme/transformer");
        transformer_after.group = String::from("autobots");

        let active_specs = map_for_specs(vec![
            "acme/alpha",
            "acme/beta",
            "acme/oldie",
            "acme/transformer",
        ]);
        let mut watcher = SpecWatcher::run_with::<TestWatcher, _>(&path).unwrap();
        let events = waiting_for_new_events(&mut watcher, active_specs);

        assert_eq!(4, events.len());
        assert!(events.contains(&SpecWatcherEvent::RemoveService(oldie)));
        assert!(events.contains(&SpecWatcherEvent::AddService(newbie)));
        assert!(events.contains(&SpecWatcherEvent::RemoveService(transformer_before),));
        assert!(events.contains(&SpecWatcherEvent::AddService(transformer_after),));
    }

    #[test]
    fn loading_spec_missing_ident_doesnt_impact_others() {
        let tmpdir = TempDir::new().unwrap();
        let alpha = new_saved_spec(tmpdir.path(), "acme/alpha");
        fs::File::create(tmpdir.path().join(format!("beta.spec"))).expect("can't create file");

        let mut watcher = SpecWatcher::run(tmpdir.path()).unwrap();

        let events = watcher.initial_events().unwrap();

        assert_eq!(1, events.len());
        assert!(events.contains(&SpecWatcherEvent::AddService(alpha)));
    }

    #[test]
    fn loading_spec_bad_content_doesnt_impact_others() {
        let tmpdir = TempDir::new().unwrap();
        let alpha = new_saved_spec(tmpdir.path(), "acme/alpha");
        {
            let mut bad = fs::File::create(tmpdir.path().join(format!("beta.spec")))
                .expect("can't create file");
            bad.write_all(
                r#"ident = "acme/beta"
                          I am a bad bad file."#
                    .as_bytes(),
            ).expect("can't write file content");
        }

        let mut watcher = SpecWatcher::run(tmpdir.path()).unwrap();

        let events = watcher.initial_events().unwrap();

        assert_eq!(1, events.len());
        assert!(events.contains(&SpecWatcherEvent::AddService(alpha)));
    }

    #[test]
    fn loading_spec_ident_name_mismatch_doesnt_impact_others() {
        let tmpdir = TempDir::new().unwrap();
        let alpha = new_saved_spec(tmpdir.path(), "acme/alpha");
        {
            let mut bad = fs::File::create(tmpdir.path().join(format!("beta.spec")))
                .expect("can't create file");
            bad.write_all(r#"ident = "acme/NEAL_MORSE_BAND""#.as_bytes())
                .expect("can't write file content");
        }

        let mut watcher = SpecWatcher::run(tmpdir.path()).unwrap();

        let events = watcher.initial_events().unwrap();

        assert_eq!(1, events.len());
        assert!(events.contains(&SpecWatcherEvent::AddService(alpha)));
    }

    struct TestWatcher {
        tx: Sender<notify::DebouncedEvent>,
    }

    impl TestWatcher {
        fn behavior_new_spec<P: AsRef<Path>>(&mut self, path: P) {
            new_saved_spec(path.as_ref(), "acme/newbie");
            self.tx
                .send(notify::DebouncedEvent::Write(
                    path.as_ref().join("newbie.spec"),
                )).expect("couldn't send event");
        }

        fn behavior_removed_spec<P: AsRef<Path>>(&mut self, path: P) {
            let toml_path = path.as_ref().join("oldie.spec");
            fs::remove_file(&toml_path).expect("couldn't delete spec toml");
            self.tx
                .send(notify::DebouncedEvent::Remove(toml_path))
                .expect("couldn't send event");
        }

        fn behavior_changed_spec<P: AsRef<Path>>(&mut self, path: P) {
            let toml_path = path.as_ref().join("transformer.spec");
            let mut spec = ServiceSpec::from_file(&toml_path).expect("couldn't load spec file");
            spec.group = String::from("autobots");
            spec.to_file(&toml_path).expect("couldn't write spec file");
            self.tx
                .send(notify::DebouncedEvent::Write(toml_path))
                .expect("couldn't send event");
        }
    }

    impl notify::Watcher for TestWatcher {
        fn new(tx: Sender<notify::DebouncedEvent>, _delay: Duration) -> notify::Result<Self> {
            Ok(TestWatcher { tx: tx })
        }

        fn watch<P: AsRef<Path>>(
            &mut self,
            path: P,
            _recursive_mode: notify::RecursiveMode,
        ) -> notify::Result<()> {
            let behavior = path
                .as_ref()
                .file_name()
                .expect("file name is ..")
                .to_str()
                .expect("path isn't utf-8 valid");

            match behavior {
                "no_events" => {}
                "new_spec" => self.behavior_new_spec(path.as_ref()),
                "removed_spec" => self.behavior_removed_spec(path.as_ref()),
                "new_and_removed_spec" => {
                    self.behavior_new_spec(path.as_ref());
                    self.behavior_removed_spec(path.as_ref());
                }
                "changed_spec" => self.behavior_changed_spec(path.as_ref()),
                "crazytown" => {
                    self.behavior_changed_spec(path.as_ref());
                    self.behavior_new_spec(path.as_ref());
                    self.behavior_removed_spec(path.as_ref());
                }
                "throw_error" => {
                    return Err(notify::Error::Generic(String::from("we failed you, noes!")));
                }
                unknown => panic!("unknown fixture behavior: {}", unknown),
            }

            Ok(())
        }

        fn new_raw(_tx: Sender<notify::RawEvent>) -> notify::Result<Self> {
            unimplemented!()
        }

        fn unwatch<P: AsRef<Path>>(&mut self, _path: P) -> notify::Result<()> {
            unimplemented!()
        }
    }

    fn new_spec(ident: &str) -> ServiceSpec {
        ServiceSpec::default_for(PackageIdent::from_str(ident).expect("couldn't parse ident str"))
    }

    fn new_saved_spec(tmpdir: &Path, ident: &str) -> ServiceSpec {
        let spec = new_spec(ident);
        spec.to_file(tmpdir.join(format!("{}.spec", &spec.ident.name)))
            .expect("couldn't save spec to disk");
        spec
    }

    fn map_for_specs(idents: Vec<&str>) -> HashMap<String, ServiceSpec> {
        let mut map = HashMap::new();
        for ident in idents {
            let spec = ServiceSpec::default_for(
                PackageIdent::from_str(ident).expect("couldn't parse ident str"),
            );
            map.insert(spec.ident.name.clone(), spec);
        }
        map
    }

    fn waiting_for_new_events(
        watcher: &mut SpecWatcher,
        active_specs: HashMap<String, ServiceSpec>,
    ) -> Vec<SpecWatcherEvent> {
        let start = Instant::now();
        let timeout = Duration::from_millis(1000);
        while start.elapsed() < timeout {
            let events = watcher.new_events(active_specs.clone()).unwrap();
            if !events.is_empty() {
                return events;
            }
            thread::sleep(Duration::from_millis(1));
        }
        panic!("Waited for events but found none");
    }
}
