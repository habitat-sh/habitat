mod package_update_worker;
mod rolling_update_worker;

use self::{package_update_worker::PackageUpdateWorker,
           rolling_update_worker::RollingUpdateWorker};
use crate::{census::CensusRing,
            manager::service::{Service,
                               UpdateStrategy}};
use futures::future::{self,
                      AbortHandle};
use habitat_common::outputln;
use habitat_core::{package::PackageIdent,
                   service::ServiceGroup};
use parking_lot::{Mutex,
                  RwLock};
use std::{self,
          collections::HashMap,
          future::Future,
          sync::Arc,
          time::Duration};

static LOGKEY: &str = "SU";

/// A handle to an update worker that automatically aborts the worker when dropped.
struct Worker(AbortHandle);

impl Drop for Worker {
    fn drop(&mut self) { self.0.abort(); }
}

/// The `ServiceUpdater` is in charge of updating a Service when a change in version of a package
/// has been published to a depot channel or installed to the local package cache. To use an update
/// strategy, the supervisor must be configured to watch a depot for new versions.
pub struct ServiceUpdater {
    butterfly:   habitat_butterfly::Server,
    census_ring: Arc<RwLock<CensusRing>>,
    updates:     Arc<Mutex<HashMap<ServiceGroup, PackageIdent>>>,
    workers:     HashMap<ServiceGroup, Worker>,
    period:      Duration,
}

impl ServiceUpdater {
    pub fn new(butterfly: habitat_butterfly::Server,
               census_ring: Arc<RwLock<CensusRing>>,
               period: Duration)
               -> Self {
        ServiceUpdater { butterfly,
                         census_ring,
                         updates: Arc::default(),
                         workers: HashMap::new(),
                         period }
    }

    /// Register a new service for updates.
    pub fn add(&mut self, service: &Service) {
        // Defensivly remove the service to prevent multiple update workers from running.
        self.remove(&service.service_group);
        // Determine what kind of worker we should use
        let service_group = service.service_group.clone();
        match service.update_strategy {
            UpdateStrategy::None => {}
            UpdateStrategy::AtOnce => {
                let worker = self.at_once_worker(service);
                self.spawn_worker(service_group, worker);
            }
            UpdateStrategy::Rolling => {
                let worker = self.rolling_worker(service, Arc::clone(&self.census_ring));
                self.spawn_worker(service_group, worker);
            }
        };
    }

    /// Unregister a service from updates (eg if the service was unloaded).
    pub fn remove(&mut self, service_group: &ServiceGroup) {
        self.workers.remove(service_group);
        self.updates.lock().remove(service_group);
    }

    /// Check if this service has an update. If it does return the package ident of the update.
    ///
    /// Once an update is detected, this function will always return an update until the service is
    /// removed from the `ServiceUpdater`. The expectation is that when an update is detected the
    /// service will be restarted inorder for the update to take effect. As part of this restart,
    /// the service should be removed from the `ServiceUpdater`.
    pub fn has_update(&self, service_group: &ServiceGroup) -> Option<PackageIdent> {
        self.updates.lock().get(service_group).cloned()
    }

    fn at_once_worker(&mut self, service: &Service) -> impl Future<Output = ()> + Send + 'static {
        debug!("'{}' service updater spawning at-once worker watching for changes to '{}' from \
                channel '{}'",
               service.service_group,
               service.spec_ident,
               service.channel());
        let service_group = service.service_group.clone();
        let full_ident = service.pkg.ident.clone();
        let updates = Arc::clone(&self.updates);
        let package_update_worker = PackageUpdateWorker::new(service, self.period);
        async move {
            let new_ident = package_update_worker.update().await;
            debug!("'{}' at-once updater found update from '{}' to '{}'",
                   service_group, full_ident, new_ident);
            Self::update_message(&new_ident, full_ident.as_ref());
            updates.lock().insert(service_group, new_ident);
        }
    }

    fn rolling_worker(&mut self,
                      service: &Service,
                      census_ring: Arc<RwLock<CensusRing>>)
                      -> impl Future<Output = ()> + Send + 'static {
        debug!("'{}' service updater spawning rolling worker watching for changes to '{}' from \
                channel '{}'",
               service.service_group,
               service.spec_ident,
               service.channel());
        let service_group = service.service_group.clone();
        let full_ident = service.pkg.ident.clone();
        let updates = Arc::clone(&self.updates);
        let worker =
            RollingUpdateWorker::new(service, census_ring, self.butterfly.clone(), self.period);
        async move {
            let new_ident = worker.run().await;
            debug!("'{}' rolling updater found update from '{}' to '{}'",
                   service_group, full_ident, new_ident);
            Self::update_message(&new_ident, full_ident.as_ref());
            updates.lock().insert(service_group, new_ident);
        }
    }

    fn update_message(new_ident: &PackageIdent, current_ident: &PackageIdent) {
        if new_ident > current_ident {
            outputln!("Updating from {} to {}", current_ident, new_ident);
        } else {
            outputln!("Rolling back from {} to {}", current_ident, new_ident);
        }
    }

    /// Make the worker abortable and spawn it
    fn spawn_worker(&mut self,
                    service_group: ServiceGroup,
                    worker: impl Future<Output = ()> + Send + 'static) {
        let (worker, abort_handle) = future::abortable(worker);
        self.workers.insert(service_group, Worker(abort_handle));
        tokio::spawn(worker);
    }
}
