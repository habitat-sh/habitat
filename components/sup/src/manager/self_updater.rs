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

//! Encapsulates logic required for updating the Habitat Supervisor
//! itself.

use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

use time::{Duration as TimeDuration, SteadyTime};

use common::command::package::install::InstallSource;
use common::ui::UI;
use env;
use hcore::package::{PackageIdent, PackageInstall};
use util;

pub const SUP_PKG_IDENT: &'static str = "core/hab-sup";
const DEFAULT_FREQUENCY: i64 = 60_000;
const FREQUENCY_ENVVAR: &'static str = "HAB_SUP_UPDATE_MS";

pub struct SelfUpdater {
    rx: Option<Receiver<PackageInstall>>,
    current: PackageIdent,
    update_url: String,
    update_channel: String,
}

// TODO (CM): Want to use the Periodic trait here, but can't due to
// how things are currently structured (The service updater had a worker)

impl SelfUpdater {
    pub fn new(
        current: PackageIdent,
        update_url: String,
        update_channel: String,
        auto_update: bool,
    ) -> Self {
        let rx = if auto_update {
            Some(Self::spawn_updater_thread(
                current.clone(),
                update_url.clone(),
                update_channel.clone(),
            ))
        } else {
            None
        };
        SelfUpdater {
            rx: rx,
            current: current,
            update_url: update_url,
            update_channel: update_channel,
        }
    }

    fn spawn_updater_thread(
        current: PackageIdent,
        update_url: String,
        update_channel: String,
    ) -> Receiver<PackageInstall> {
        let (tx, rx) = sync_channel(0);
        thread::Builder::new()
            .name("self-updater".to_string())
            .spawn(move || Self::run(tx, current, update_url, update_channel))
            .expect("Unable to start self-updater thread");
        rx
    }

    fn run(
        sender: SyncSender<PackageInstall>,
        current: PackageIdent,
        builder_url: String,
        channel: String,
    ) {
        debug!("Self updater current package, {}", current);
        // SUP_PKG_IDENT will always parse as a valid PackageIdent,
        // and thus a valid InstallSource
        loop {
            let next_check = SteadyTime::now() + TimeDuration::milliseconds(update_frequency());

            if let Some(package) = Self::try_update(&current, &builder_url, &channel) {
                sender.send(package).expect("Main thread has gone away!");
                break; // the supervisor will shut down now, so no need to keep checking
            }

            let time_to_wait = (next_check - SteadyTime::now()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            }
        }
    }

    fn try_update(
        current: &PackageIdent,
        builder_url: &String,
        channel: &String,
    ) -> Option<PackageInstall> {
        let install_source: InstallSource = SUP_PKG_IDENT.parse().unwrap();
        match util::pkg::install(
            // We don't want anything in here to print
            &mut UI::with_sinks(),
            &builder_url,
            &install_source,
            &channel,
        ) {
            Ok(package) => {
                if *current < *package.ident() {
                    debug!(
                        "Self updater installing newer Supervisor, {}",
                        package.ident()
                    );
                    return Some(package);
                } else {
                    debug!("Supervisor package found is not newer than ours");
                }
            }
            Err(err) => {
                warn!("Self updater failed to get latest, {}", err);
            }
        }
        None
    }

    pub fn updated(&mut self) -> Option<PackageInstall> {
        if let Some(ref mut rx) = self.rx {
            match rx.try_recv() {
                Ok(package) => Some(package),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => {
                    debug!("Self updater has died, restarting...");
                    *rx = Self::spawn_updater_thread(
                        self.current.clone(),
                        self.update_url.clone(),
                        self.update_channel.clone(),
                    );
                    None
                }
            }
        } else {
            Self::try_update(&self.current, &self.update_url, &self.update_channel)
        }
    }
}

fn update_frequency() -> i64 {
    match env::var(FREQUENCY_ENVVAR) {
        Ok(val) => val.parse::<i64>().unwrap_or(DEFAULT_FREQUENCY),
        Err(_) => DEFAULT_FREQUENCY,
    }
}
