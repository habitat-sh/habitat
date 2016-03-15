// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::sync::{Arc, RwLock};
use std::str::FromStr;

use wonder;
use wonder::actor::{GenServer, InitResult, HandleResult, ActorSender, ActorResult};

use error::BldrError;
use fs::PACKAGE_CACHE;
use package::{Package, PackageIdent};
use depot;

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
        // JW TODO: Store and use the version if the package was started with a specific version.
        //          This will allow an operator to lock to a version and receive security updates
        //          in the form of release updates for a package.
        let ident = PackageIdent::new(package.origin.clone(), package.name.clone(), None, None);
        match depot::client::show_package(&state.depot, &ident) {
            Ok(remote) => {
                let latest: Package = remote.into();
                if latest > *package {
                    match depot::client::fetch_package(&state.depot,
                                                       &PackageIdent::from_str(&latest.ident())
                                                            .unwrap(),
                                                       PACKAGE_CACHE) {
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
