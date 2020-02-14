use crate::{census::CensusRing,
            manager::{periodic::Periodic,
                      service::{Service,
                                Topology,
                                UpdateStrategy}},
            util};
use futures::future::{self,
                      AbortHandle};
use habitat_butterfly;
use habitat_common::outputln;
use habitat_core::{env as henv,
                   package::{PackageIdent,
                             PackageInstall,
                             PackageTarget},
                   service::ServiceGroup,
                   ChannelIdent};
use parking_lot::Mutex;
use std::{self,
          collections::HashMap,
          future::Future,
          sync::Arc,
          time::{Duration,
                 Instant}};
use tokio::{self,
            sync::watch::{self,
                          Receiver as WatchReceiver,
                          Sender as WatchSender},
            time};

static LOGKEY: &str = "SU";
// TODO (CM): Yes, the variable value should be "period" and not
// "frequency"... we need to fix that.
const PERIOD_BYPASS_CHECK_ENVVAR: &str = "HAB_UPDATE_STRATEGY_FREQUENCY_BYPASS_CHECK";

habitat_core::env_config_duration!(
    /// Represents how far apart checks for updates to individual services
    /// are, in milliseconds.
    ServiceUpdatePeriod,
    // TODO (CM): Yes, the variable value should be "period" and not
    // "frequency"... we need to fix that.
    HAB_UPDATE_STRATEGY_FREQUENCY_MS => from_millis,
    ServiceUpdatePeriod::MIN_ALLOWED);

impl ServiceUpdatePeriod {
    const MIN_ALLOWED: Duration = Duration::from_secs(60);

    fn get() -> Duration {
        let val = ServiceUpdatePeriod::configured_value().into();
        if val >= ServiceUpdatePeriod::MIN_ALLOWED || henv::var(PERIOD_BYPASS_CHECK_ENVVAR).is_ok()
        {
            val
        } else {
            ServiceUpdatePeriod::MIN_ALLOWED
        }
    }
}

struct Worker {
    abort_handle:         AbortHandle,
    watcher_abort_handle: AbortHandle,
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.abort_handle.abort();
        self.watcher_abort_handle.abort();
    }
}

/// The `ServiceUpdater` is in charge of updating a Service when a more recent version of a package
/// has been published to a depot or installed to the local package cache.
/// To use an update strategy, the supervisor must be configured to watch a depot for new versions.
pub struct ServiceUpdater {
    butterfly: habitat_butterfly::Server,
    updates:   Arc<Mutex<HashMap<ServiceGroup, PackageIdent>>>,
    workers:   HashMap<ServiceGroup, Worker>,
}

impl ServiceUpdater {
    pub fn new(butterfly: habitat_butterfly::Server) -> Self {
        ServiceUpdater { butterfly,
                         updates: Arc::default(),
                         workers: HashMap::new() }
    }

    /// Register a new service for updates.
    pub fn add(&mut self, service: &Service) {
        // Defensivly remove the service to prevent multiple update workers from running.
        self.remove(&service.service_group);
        match service.update_strategy {
            UpdateStrategy::None => {}
            UpdateStrategy::AtOnce => self.spawn_at_once_worker(service),
            UpdateStrategy::Rolling => todo!(),
        }
    }

    /// Unregister a service from updates (eg if the service was unloaded).
    pub fn remove(&mut self, service: &ServiceGroup) {
        self.workers.remove(service);
        self.updates.lock().remove(service);
    }

    // Check if this service has an update. If it does return the package ident of the update.
    pub fn has_update(&self, service: &ServiceGroup) -> Option<PackageIdent> {
        self.updates.lock().get(service).cloned()
    }

    fn spawn_at_once_worker(&mut self, service: &Service) {
        debug!("Service updater spawning at-once worker for service group '{}' watching for \
                changes to '{}' from channel '{}'",
               service.service_group, service.spec_ident, service.channel);
        let service_group = service.service_group.clone();
        let full_ident = service.pkg.ident.clone();
        let updates = Arc::clone(&self.updates);
        let (mut watcher, watcher_abort_handle) = Self::package_ident_change_watcher(service);
        let worker = async move {
            // Ignore the first value from the watcher. This is just the initial value and resolves
            // immediately.
            watcher.recv().await;
            loop {
                if let Some(new_ident) = watcher.recv().await {
                    // Only allow updates never rollbacks
                    if new_ident > full_ident {
                        debug!("Service updater found update to '{}' from '{}' for service group \
                                '{}'.",
                               new_ident, full_ident, service_group);
                        updates.lock().insert(service_group.clone(), new_ident);
                    } else {
                        debug!("Service updater found rollback to '{}' from '{}' for service \
                                group '{}', but rollbacks are not prohibited.",
                               new_ident, full_ident, service_group);
                    }
                } else {
                    error!("The service updater package ident watcher for '{}' was unexpectantly \
                            dropped. This service will no longer be able to update!",
                           service_group);
                    debug_assert!(false);
                    break;
                }
            }
        };
        let (worker, abort_handle) = future::abortable(worker);
        self.workers.insert(service.service_group.clone(),
                            Worker { abort_handle,
                                     watcher_abort_handle });
        tokio::spawn(worker);
    }

    fn spawn_rollback_worker(&mut self, service: &Service) { todo!() }

    fn spawn_rollback_leader_worker() { todo!() }

    fn spawn_rollback_follower_worker() { todo!() }

    /// Spawns a worker continuously checking for a change in version of the package being run by a
    /// service. Returns a watch channel that If a change is detected the package is installed and
    /// its identifier returned.
    // TODO (DM): The returned package ident should always be fully qualified. We need a type to
    // encapsulate that.
    fn package_ident_change_watcher(service: &Service)
                                    -> (WatchReceiver<PackageIdent>, AbortHandle) {
        let service_group = service.service_group.clone();
        let ident = service.spec_ident.clone();
        let full_ident = service.pkg.ident.clone();
        println!("!!! IDENT {}", ident);
        println!("!!! FULL IDENT {}", full_ident);
        let channel = service.channel.clone();
        let builder_url = service.bldr_url.clone();
        let install_source = ident.clone().into();
        let (tx, rx) = watch::channel(full_ident.clone());
        let watcher = rx.clone();
        let worker = async move {
            loop {
                match util::pkg::install_no_ui(&builder_url, &install_source, &channel).await {
                    Ok(package) => {
                        if *rx.borrow() != package.ident {
                            debug!("Service updater found change to '{}' from channel '{}' for \
                                    service group '{}' replacing '{}'",
                                   ident, channel, service_group, full_ident);
                            if let Err(e) = tx.broadcast(package.ident) {
                                error!("Failed to broadcast change to '{}' from channel '{}' for \
                                        service group '{}' replacing '{}'",
                                       ident, channel, service_group, full_ident);
                            }
                        } else {
                            trace!("Service updater did not find changes to '{}' from channel \
                                    '{}' for service group '{}'",
                                   ident,
                                   channel,
                                   service_group)
                        }
                    }
                    Err(err) => {
                        warn!("Service updater failed to install '{}' from channel '{}' for \
                               service group '{}', err: {}",
                              ident, channel, service_group, err)
                    }
                }
                // TODO (DM): correct delay
                time::delay_for(Duration::from_secs(5)).await;
            }
        };
        let (worker, abort_handle) = future::abortable(worker);
        tokio::spawn(worker);
        (watcher, abort_handle)
    }
}
