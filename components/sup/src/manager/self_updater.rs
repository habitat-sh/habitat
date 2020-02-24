//! Encapsulates logic required for updating the Habitat Supervisor
//! itself.

use crate::util;
use habitat_common::command::package::install::InstallSource;
use habitat_core::{package::{PackageIdent,
                             PackageInstall},
                   ChannelIdent};
use std::time::Duration;
use tokio::{self,
            sync::oneshot::{self,
                            error::TryRecvError,
                            Receiver,
                            Sender},
            time as tokiotime};

pub const SUP_PKG_IDENT: &str = "core/hab-sup";
const DEFAULT_PERIOD: Duration = Duration::from_secs(60);

habitat_core::env_config_duration!(
    /// Represents how far apart checks for updates are, in milliseconds.
    SelfUpdatePeriod,
    HAB_SUP_UPDATE_MS => from_millis,
    DEFAULT_PERIOD);

pub struct SelfUpdater {
    rx:             Receiver<PackageInstall>,
    current:        PackageIdent,
    update_url:     String,
    update_channel: ChannelIdent,
}

// TODO (CM): Want to use the Periodic trait here, but can't due to
// how things are currently structured (The service updater had a worker)

impl SelfUpdater {
    pub fn new(current: PackageIdent, update_url: String, update_channel: ChannelIdent) -> Self {
        let rx = Self::init(current.clone(), update_url.clone(), update_channel.clone());
        SelfUpdater { rx,
                      current,
                      update_url,
                      update_channel }
    }

    /// Spawn a new Supervisor updater thread.
    fn init(current: PackageIdent,
            update_url: String,
            update_channel: ChannelIdent)
            -> Receiver<PackageInstall> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(Self::run(tx, current, update_url, update_channel));
        rx
    }

    async fn run(tx: Sender<PackageInstall>,
                 current: PackageIdent,
                 update_url: String,
                 update_channel: ChannelIdent) {
        debug!("Self updater current package, {}", current);
        // SUP_PKG_IDENT will always parse as a valid PackageIdent,
        // and thus a valid InstallSource
        let install_source: InstallSource = SUP_PKG_IDENT.parse().unwrap();
        let delay = SelfUpdatePeriod::configured_value().into();
        loop {
            match util::pkg::install_no_ui(&update_url, &install_source, &update_channel).await {
                Ok(package) => {
                    if &current < package.ident() {
                        debug!("Self updater installing newer Supervisor, {}",
                               package.ident());
                        tx.send(package).expect("Main thread has gone away!");
                        break;
                    } else {
                        debug!("Supervisor package found is not newer than ours");
                    }
                }
                Err(err) => {
                    warn!("Self updater failed to get latest, {}", err);
                }
            }
            tokiotime::delay_for(delay).await;
        }
    }

    pub async fn updated(&mut self) -> Option<PackageInstall> {
        match self.rx.try_recv() {
            Ok(package) => Some(package),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Closed) => {
                debug!("Self updater has died, restarting...");
                self.rx = Self::init(self.current.clone(),
                                     self.update_url.clone(),
                                     self.update_channel.clone());
                None
            }
        }
    }
}
