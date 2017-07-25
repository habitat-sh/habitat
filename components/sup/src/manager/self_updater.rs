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

use std::path::Path;
use std::str::FromStr;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TryRecvError};
use std::thread;
use std::time::Duration;

use common::ui::ProgressBar;
use depot_client::Client as DepotClient;
use env;
use hcore::package::{PackageIdent, PackageInstall};
use hcore::crypto::default_cache_key_path;
use hcore::fs::{CACHE_ARTIFACT_PATH, FS_ROOT_PATH};
use time::{SteadyTime, Duration as TimeDuration};

use {PRODUCT, VERSION};
use error::Result;

pub const SUP_PKG_IDENT: &'static str = "core/hab-sup";
const DEFAULT_FREQUENCY: i64 = 60_000;
const FREQUENCY_ENVVAR: &'static str = "HAB_SUP_UPDATE_MS";

pub struct SelfUpdater {
    rx: Receiver<PackageInstall>,
    current: PackageIdent,
    update_url: String,
    update_channel: String,
}

impl SelfUpdater {
    pub fn new(current: PackageIdent, update_url: String, update_channel: String) -> Self {
        let rx = Self::init(current.clone(), &update_url, update_channel.clone());
        SelfUpdater {
            rx: rx,
            current: current,
            update_url: update_url,
            update_channel: update_channel,
        }
    }

    fn init(
        current: PackageIdent,
        update_url: &str,
        update_channel: String,
    ) -> Receiver<PackageInstall> {
        let (tx, rx) = sync_channel(0);
        let client = DepotClient::new(update_url, PRODUCT, VERSION, None).unwrap();
        thread::Builder::new()
            .name("self-updater".to_string())
            .spawn(move || Self::run(tx, current, client, update_channel))
            .expect("Unable to start self-updater thread");
        rx
    }

    fn run(
        sender: SyncSender<PackageInstall>,
        current: PackageIdent,
        depot: DepotClient,
        channel: String,
    ) {
        let spec_ident = PackageIdent::from_str(SUP_PKG_IDENT).unwrap();
        debug!("Self updater current package, {}", current);
        loop {
            let next_check = SteadyTime::now() + TimeDuration::milliseconds(update_frequency());
            match depot.show_package(&spec_ident, Some(&channel)) {
                Ok(mut remote) => {
                    debug!("Self updater found remote, {}", remote.get_ident());
                    let latest: PackageIdent = remote.take_ident().into();
                    if latest > current {
                        debug!("Self updater installing newer supervisor, {}", latest);
                        match install(&depot, &latest, true) {
                            Ok(package) => {
                                sender.send(package).unwrap();
                                break;
                            }
                            Err(err) => warn!("Self updater failed to install, {}", err),
                        }
                    } else {
                        debug!("Supervisor package found is not newer than ours");
                    }
                }
                Err(err) => warn!("Self updater failed to get latest, {}", err),
            }
            let time_to_wait = (next_check - SteadyTime::now()).num_milliseconds();
            if time_to_wait > 0 {
                thread::sleep(Duration::from_millis(time_to_wait as u64));
            }
        }
    }

    pub fn updated(&mut self) -> Option<PackageInstall> {
        match self.rx.try_recv() {
            Ok(package) => return Some(package),
            Err(TryRecvError::Empty) => return None,
            Err(TryRecvError::Disconnected) => (),
        }
        error!("Self updater crashed, restarting...");
        self.restart();
        None
    }

    fn restart(&mut self) {
        self.rx = Self::init(
            self.current.clone(),
            &self.update_url,
            self.update_channel.clone(),
        );
    }
}

fn download(depot: &DepotClient, package: &PackageIdent) -> Result<PackageInstall> {
    let mut archive = depot.fetch_package(
        package,
        &Path::new(&*FS_ROOT_PATH).join(CACHE_ARTIFACT_PATH),
        None::<ProgressBar>,
    )?;
    archive.verify(&default_cache_key_path(None))?;
    archive.unpack(None)?;
    let pkg = PackageInstall::load(archive.ident().as_ref().unwrap(), Some(&*FS_ROOT_PATH))?;
    Ok(pkg)
}

fn install(depot: &DepotClient, package: &PackageIdent, recurse: bool) -> Result<PackageInstall> {
    let package = match PackageInstall::load(package, Some(&*FS_ROOT_PATH)) {
        Ok(pkg) => pkg,
        Err(_) => download(depot, package)?,
    };
    if recurse {
        for ident in package.tdeps()?.iter() {
            install(depot, &ident, false)?;
        }
    }
    Ok(package)
}

fn update_frequency() -> i64 {
    match env::var(FREQUENCY_ENVVAR) {
        Ok(val) => val.parse::<i64>().unwrap_or(DEFAULT_FREQUENCY),
        Err(_) => DEFAULT_FREQUENCY,
    }
}
