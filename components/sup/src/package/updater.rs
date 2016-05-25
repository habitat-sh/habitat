// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::sync::{Arc, RwLock};
use std::path::Path;

use common::command::ProgressBar;
use depot_client;
use hcore::crypto::default_cache_key_path;
use hcore::fs::{CACHE_ARTIFACT_PATH, FS_ROOT_PATH};
use hcore::package::PackageIdent;
use wonder;
use wonder::actor::{GenServer, InitResult, HandleResult, ActorSender, ActorResult};

use error::SupError;
use package::Package;

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
    pub depot: String,
    pub package: Arc<RwLock<Package>>,
    pub status: UpdaterStatus,
}

impl UpdaterState {
    pub fn new(depot: String, package: Arc<RwLock<Package>>) -> Self {
        UpdaterState {
            depot: depot,
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
    type E = SupError;

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
        // JW TODO: Store and use the version if the package was started with a specific version.
        //          This will allow an operator to lock to a version and receive security updates
        //          in the form of release updates for a package.
        let ident = PackageIdent::new(package.origin.clone(), package.name.clone(), None, None);
        match depot_client::show_package(&state.depot, &ident) {
            Ok(remote) => {
                let latest_ident: &PackageIdent = remote.ident.as_ref();
                if latest_ident > &*package.ident() {
                    let mut progress = ProgressBar::default();
                    match depot_client::fetch_package(&state.depot,
                                                      latest_ident,
                                                      &Path::new(FS_ROOT_PATH)
                                                          .join(CACHE_ARTIFACT_PATH),
                                                      Some(&mut progress)) {
                        Ok(archive) => {
                            debug!("Updater downloaded new package to {:?}", archive);
                            // JW TODO: actually handle verify and unpack results
                            archive.verify(&default_cache_key_path(None)).unwrap();
                            archive.unpack(None).unwrap();
                            let latest_package = Package::load(latest_ident, None).unwrap();
                            state.status = UpdaterStatus::Stopped;
                            let msg =
                                wonder::actor::Message::Cast(UpdaterMessage::Update(latest_package));
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
