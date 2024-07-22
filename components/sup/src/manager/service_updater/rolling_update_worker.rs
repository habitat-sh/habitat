use super::{package_update_worker::PackageUpdateWorker,
            IncarnatedPackageIdent};
use crate::{census::{CensusGroup,
                     CensusRing},
            manager::service::{Service,
                               Topology}};
use habitat_common::owning_refs::RwLockReadGuardRef;
use habitat_core::service::ServiceGroup;
use log::{debug,
          error,
          trace,
          warn};
use parking_lot::RwLock;
use std::{self,
          sync::Arc,
          time::Duration};
use tokio::{self,
            time};

// The census ring does not have an async API. We make it look async by making API calls in a
// loop with this delay after each call.
const DELAY: Duration = Duration::from_secs(1);

/// The role of the supervisor in the rolling update.
enum Role {
    Leader,
    Follower,
}

/// Possible events when a follower is waiting for a rolling update to start.
enum FollowerUpdateStartEvent {
    /// The leader died and this follower was chosen as the leader.
    PromotedToLeader,
    /// An update started and we have a specific package to update to.
    // TODO (DM): This should use FullyQualifiedPackageIdent.
    UpdateTo(IncarnatedPackageIdent),
}

/// Possible events when a follower is waiting for its turn to update.
enum FollowerUpdateTurnEvent {
    /// The leader died and this follower was chosen as the leader.
    PromotedToLeader,
    /// The leader died in the middle of a rolling update and this follower was chosen as the
    /// leader. When this happens the new leader needs to update to the exact version the
    /// rolling update was started with.
    // TODO (DM): This should use FullyQualifiedPackageIdent.
    PromotedToLeaderMidUpdate(IncarnatedPackageIdent),
    /// An update started and we have a specific package to update to.
    // TODO (DM): This should use FullyQualifiedPackageIdent.
    UpdateTo(IncarnatedPackageIdent),
}

/// The worker for handling rolling updates.
///
/// The basic behavior of the update is to elect an update leader. The leader waits for an update.
/// When an update is detected, the leader is updated and each follower takes a turn to update.
pub struct RollingUpdateWorker {
    service_group:         ServiceGroup,
    topology:              Topology,
    package_update_worker: PackageUpdateWorker,
    census_ring:           Arc<RwLock<CensusRing>>,
    butterfly:             habitat_butterfly::Server,
}

impl RollingUpdateWorker {
    pub fn new(service: &Service,
               census_ring: Arc<RwLock<CensusRing>>,
               butterfly: habitat_butterfly::Server,
               period: Duration)
               -> Self {
        Self { service_group: service.service_group.clone(),
               topology: service.topology(),
               package_update_worker: PackageUpdateWorker::new(service, period),
               census_ring,
               butterfly }
    }

    pub async fn run(self) -> IncarnatedPackageIdent {
        // Determine this services suitablity and start the update leader election
        let suitability = self.update_election_suitability(self.topology).await;
        self.butterfly
            .start_update_election_rsw_mlr_rhw(&self.service_group, suitability, 0);
        // Determine this services role in the rolling update
        match self.update_role().await {
            Role::Leader => {
                // Wait for an update which will trigger follower updates through the census
                // protocol
                self.update_and_bump_incarnation().await
            }
            Role::Follower => {
                // Wait till it is our turn to update. It is possible that while we are waiting
                // the leader dies and we are promoted to update leader.
                match self.follower_wait_for_update_turn().await {
                    FollowerUpdateTurnEvent::PromotedToLeader => {
                        // Wait for an update which will trigger follower updates through the
                        // census protocol
                        self.update_and_bump_incarnation().await
                    }
                    FollowerUpdateTurnEvent::PromotedToLeaderMidUpdate(new_ident) => {
                        // Update to the same package as the old leader allowing all followers
                        // to finish updating
                        self.package_update_worker.update_to(new_ident).await
                    }
                    FollowerUpdateTurnEvent::UpdateTo(new_ident) => {
                        // Update to the package we were instructed to
                        self.package_update_worker.update_to(new_ident).await
                    }
                }
            }
        }
    }

    async fn update_and_bump_incarnation(&self) -> IncarnatedPackageIdent {
        let mut pkg = self.package_update_worker.update().await;
        // bump the incarnation of the update that the leader performed
        // this will eventually get gossiped after the service restarts
        pkg.incarnation = Some(self.census_group().await.pkg_incarnation + 1);
        pkg
    }

    async fn update_election_suitability(&self, topology: Topology) -> u64 {
        match topology {
            Topology::Standalone => {
                debug!("'{}' rolling update detected standalone topology; using default \
                        suitability",
                       self.service_group);
                0
            }
            Topology::Leader => {
                debug!("'{}' rolling update determining proper suitability for leader topology",
                       self.service_group);
                loop {
                    {
                        let census_group = self.census_group().await;
                        match (census_group.me(), census_group.leader()) {
                            (Some(me), Some(leader)) => {
                                if me.member_id == leader.member_id {
                                    debug!("This is the '{}' leader; using the minimum rolling \
                                            update election suitability",
                                           self.service_group);
                                    break u64::MIN;
                                } else {
                                    debug!("This is a '{}' follower; using the maximum rolling \
                                            update election suitability",
                                           self.service_group);
                                    break u64::MAX;
                                };
                            }
                            (Some(_), None) => {
                                debug!("No group leader; the rolling update cannot proceed until \
                                        the '{}' group election finishes",
                                       self.service_group);
                            }
                            (None, _) => {
                                // It looks like a Supervisor finds out "who it is" by being told by
                                // the rest of the network. While this does have the advantage of
                                // unifying code paths, it could result in some counter-intuitive
                                // situations (like census_group.me() returning None!)
                                error!("Supervisor does not know its own identity; rolling \
                                        update of {} cannot proceed! Please notify the Habitat \
                                        core team!",
                                       self.service_group);
                                debug_assert!(false);
                            }
                        }
                    }
                    time::sleep(DELAY).await;
                }
            }
        }
    }

    async fn update_role(&self) -> Role {
        loop {
            {
                let census_group = self.census_group().await;
                match (census_group.me(), census_group.update_leader()) {
                    (Some(me), Some(leader)) => {
                        if me.member_id == leader.member_id {
                            debug!("This is the '{}' rolling update leader", self.service_group);
                            break Role::Leader;
                        } else {
                            debug!("This is a '{}' rolling update follower", self.service_group);
                            break Role::Follower;
                        }
                    }
                    (Some(_), None) => {
                        debug!("Rolling update leader election for '{}' is not yet finished",
                               self.service_group);
                    }
                    (None, _) => {
                        error!("Supervisor does not know its own identity; rolling update of {} \
                                cannot proceed! Please notify the Habitat core team!",
                               self.service_group);
                        debug_assert!(false);
                    }
                }
            }
            time::sleep(DELAY).await;
        }
    }

    /// Detect when the rolling update leader has a new package which starts a rolling update. The
    /// rolling update leaders new package is the package all followers need to update to.
    async fn follower_wait_for_update_start(&self) -> FollowerUpdateStartEvent {
        loop {
            {
                let census_group = self.census_group().await;
                match (census_group.update_leader(), census_group.me()) {
                    (Some(leader), Some(me)) => {
                        // If the current leader is no longer alive, it is possible that this
                        // follower is now a leader.
                        if leader.member_id == me.member_id {
                            debug!("'{}' rolling update follower was promoted to the leader",
                                   self.service_group);
                            break FollowerUpdateStartEvent::PromotedToLeader;
                        }

                        if leader.pkg_incarnation != census_group.pkg_incarnation {
                            debug!("leader with member id {} has incarnation {} that is not \
                                    caught up with the census group incarnation {}, waiting for \
                                    update to start",
                                   leader.member_id,
                                   leader.pkg_incarnation,
                                   census_group.pkg_incarnation);
                        }

                        if leader.pkg_incarnation > me.pkg_incarnation {
                            // The leader has a new package starting a rolling update
                            debug!("'{}' started a rolling update: leader='{}/{}/{}' \
                                    follower='{}/{}/{}'",
                                   self.service_group,
                                   leader.member_id,
                                   leader.pkg_incarnation,
                                   leader.pkg,
                                   me.member_id,
                                   me.pkg_incarnation,
                                   me.pkg);
                            break FollowerUpdateStartEvent::UpdateTo(IncarnatedPackageIdent::new(leader.pkg.clone(),
                                                                     Some(leader.pkg_incarnation)));
                        } else {
                            // The leader still has the same package as this follower so an update
                            // has not started
                            trace!("'{}' is not in a rolling update: leader='{}/{}/{}' \
                                    follower='{}/{}/{}'",
                                   self.service_group,
                                   leader.member_id,
                                   leader.pkg_incarnation,
                                   leader.pkg,
                                   me.member_id,
                                   me.pkg_incarnation,
                                   me.pkg);
                        }
                    }
                    _ => {
                        error!("The census group for '{}' is in a bad state. It could not \
                                determine the update leader, previous peer, or its own identity.",
                               self.service_group);
                        debug_assert!(false);
                    }
                }
            }
            time::sleep(DELAY).await;
        }
    }

    async fn follower_wait_for_update_turn(&self) -> FollowerUpdateTurnEvent {
        let update_to = match self.follower_wait_for_update_start().await {
            FollowerUpdateStartEvent::PromotedToLeader => {
                return FollowerUpdateTurnEvent::PromotedToLeader
            }
            FollowerUpdateStartEvent::UpdateTo(ident) => ident,
        };
        loop {
            {
                let census_group = self.census_group().await;
                match (census_group.update_leader(),
                       census_group.previous_peer(),
                       census_group.me())
                {
                    (Some(leader), Some(peer), Some(me)) => {
                        // If the current leader is no longer alive, it is possible that this
                        // follower is now a leader.
                        if leader.member_id == me.member_id {
                            debug!("'{}' rolling update follower was promoted to the leader mid \
                                    update. Immediately updating to '{}'.",
                                   self.service_group, update_to.ident);
                            break FollowerUpdateTurnEvent::PromotedToLeaderMidUpdate(update_to);
                        }

                        if leader.pkg_incarnation != census_group.pkg_incarnation {
                            debug!("leader with member id {} has incarnation {} that is not \
                                    caught up with the census group incarnation {}",
                                   leader.member_id,
                                   leader.pkg_incarnation,
                                   census_group.pkg_incarnation);
                        } else if peer.pkg_incarnation == leader.pkg_incarnation {
                            // It is now this followers turn. The previous peer is done updating.
                            // The first time this condition is true the previous peer is the
                            // rolling update leader making this condition trivially true. This
                            // will trigger all the followers to start their updates one after
                            // another.
                            debug!("'{}' is in a rolling update and it is this followers turn to \
                                    update: leader='{}/{}/{}' peer='{}/{}/{}' follower='{}/{}/{}'",
                                   self.service_group,
                                   leader.member_id,
                                   leader.pkg_incarnation,
                                   leader.pkg,
                                   peer.member_id,
                                   peer.pkg_incarnation,
                                   peer.pkg,
                                   me.member_id,
                                   me.pkg_incarnation,
                                   me.pkg);
                            break FollowerUpdateTurnEvent::UpdateTo(IncarnatedPackageIdent::new(leader.pkg.clone(),
                            Some(leader.pkg_incarnation)));
                        } else {
                            // It is not this followers turn to update. The previous peer has not
                            // updated yet.
                            debug!("'{}' is in a rolling update but it is not this followers \
                                    turn to update: leader='{}/{}/{}' peer='{}/{}/{}' \
                                    follower='{}/{}/{}'",
                                   self.service_group,
                                   leader.member_id,
                                   leader.pkg_incarnation,
                                   leader.pkg,
                                   peer.member_id,
                                   peer.pkg_incarnation,
                                   peer.pkg,
                                   me.member_id,
                                   me.pkg_incarnation,
                                   me.pkg);
                        }
                    }
                    _ => {
                        error!("The census group for '{}' is in a bad state. It could not \
                                determine the update leader, previous peer, or its own identity.",
                               self.service_group);
                        debug_assert!(false);
                    }
                }
            }
            time::sleep(DELAY).await;
        }
    }

    /// Returns a reference to the services census group. The reference is behind a read write lock
    /// so the lifetime of the reference should be minimized to avoid uneccesarily holding the lock.
    async fn census_group(&self) -> RwLockReadGuardRef<'_, CensusRing, CensusGroup> {
        loop {
            {
                let census_ring = RwLockReadGuardRef::new(self.census_ring.read().into());
                let maybe_census_group = census_ring.try_map(|census_ring| {
                                             census_ring.census_group_for(&self.service_group)
                                                        .ok_or(())
                                         });
                if let Ok(census_group) = maybe_census_group {
                    break census_group;
                } else {
                    warn!("'{}' rolling update could not find census group",
                          self.service_group);
                }
            }
            time::sleep(DELAY).await;
        }
    }
}
