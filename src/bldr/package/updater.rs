//
// Copyright:: Copyright (c) 2015 Chef Software, Inc.
// License:: Apache License, Version 2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

use std::sync::{Arc, RwLock};

use wonder;
use wonder::actor::{GenServer, InitResult, HandleResult, ActorSender, ActorResult};

use error::BldrError;
use fs::PACKAGE_CACHE;
use package::Package;
use repo;

const TIMEOUT_MS: u64 = 60_000;

pub type PackageUpdaterActor = wonder::actor::Actor<UpdaterMessage>;

pub struct PackageUpdater;

impl PackageUpdater {
    pub fn start(url: &str, package: Arc<RwLock<Package>>) -> PackageUpdaterActor {
        let state = UpdaterState::new(url.to_string(), package);
        wonder::actor::Builder::new(PackageUpdater)
            .name("package-updater".to_string())
            .start(state)
            .unwrap()
    }

    /// Signal a package updater to transition it's status from `stopped` to `running`. An updater
    /// has the `stopped` status after it has found and notified the main thread of an updated
    /// package.
    pub fn run(actor: &PackageUpdaterActor) -> ActorResult<()> {
        actor.cast(UpdaterMessage::Run)
    }
}

pub struct UpdaterState {
    pub repo: String,
    pub package: Arc<RwLock<Package>>,
    pub status: UpdaterStatus,
}

impl UpdaterState {
    pub fn new(repo: String, package: Arc<RwLock<Package>>) -> Self {
        UpdaterState {
            repo: repo,
            package: package,
            status: UpdaterStatus::Stopped,
        }
    }
}

#[derive(Debug)]
pub enum UpdaterMessage {
    Ok,
    Run,
    Stop,
    Update(Package),
}

pub enum UpdaterStatus {
    Running,
    Stopped,
}

impl GenServer for PackageUpdater {
    type T = UpdaterMessage;
    type S = UpdaterState;
    type E = BldrError;

    fn init(&self, _tx: &ActorSender<Self::T>, state: &mut Self::S) -> InitResult<Self::E> {
        state.status = UpdaterStatus::Running;
        Ok(Some(TIMEOUT_MS))
    }

    fn handle_timeout(&self,
                      tx: &ActorSender<Self::T>,
                      _me: &ActorSender<Self::T>,
                      state: &mut Self::S)
                      -> HandleResult<Self::T> {
        let package = state.package.read().unwrap();
        match repo::client::show_package(&state.repo,
                                         &package.derivation,
                                         &package.name,
                                         None,
                                         None) {
            Ok(latest) => {
                if latest > *package {
                    match repo::client::fetch_package_exact(&state.repo, &latest, PACKAGE_CACHE) {
                        Ok(archive) => {
                            debug!("Updater downloaded new package to {:?}", archive);
                            // JW TODO: actually handle verify and unpack results
                            archive.verify().unwrap();
                            archive.unpack().unwrap();
                            state.status = UpdaterStatus::Stopped;
                            let msg = wonder::actor::Message::Cast(UpdaterMessage::Update(latest));
                            tx.send(msg).unwrap();
                            HandleResult::NoReply(None)
                        }
                        Err(e) => {
                            debug!("Failed to download package: {:?}", e);
                            HandleResult::NoReply(Some(TIMEOUT_MS))
                        }
                    }
                } else {
                    debug!("Package found is not newer than ours");
                    HandleResult::NoReply(Some(TIMEOUT_MS))
                }
            }
            Err(e) => {
                debug!("Updater failed to get latest package: {:?}", e);
                HandleResult::NoReply(Some(TIMEOUT_MS))
            }
        }
    }

    fn handle_cast(&self,
                   msg: Self::T,
                   _tx: &ActorSender<Self::T>,
                   _me: &ActorSender<Self::T>,
                   state: &mut Self::S)
                   -> HandleResult<Self::T> {
        match msg {
            UpdaterMessage::Run => HandleResult::NoReply(Some(TIMEOUT_MS)),
            _ => {
                match state.status {
                    UpdaterStatus::Running => HandleResult::NoReply(Some(TIMEOUT_MS)),
                    UpdaterStatus::Stopped => HandleResult::NoReply(None),
                }
            }
        }
    }
}
