use super::package_update_worker::PackageUpdateWorker;
use crate::{census::{CensusGroup,
                     CensusRing},
            manager::service::{Service,
                               Topology}};
use habitat_butterfly;
use habitat_common::owning_refs::RwLockReadGuardRef;
use habitat_core::{package::PackageIdent,
                   service::ServiceGroup};
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

/// While the follower is waiting for its turn, it can either be promoted to leader (the leader
/// died) or it can be instructed to update to a specific ident.
enum FollowerWaitForTurn {
    PromotedToLeader,
    // TODO (DM): This should use FullyQualifiedPackageIdent.
    UpdateTo(PackageIdent),
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
               butterfly: habitat_butterfly::Server)
               -> Self {
        Self { service_group: service.service_group.clone(),
               topology: service.topology,
               package_update_worker: PackageUpdateWorker::from(service),
               census_ring,
               butterfly }
    }

    pub async fn run(self) -> PackageIdent {
        // Determine this services suitablity and start the update leader election
        let suitability = self.update_election_suitability(self.topology).await;
        self.butterfly
            .start_update_election_rsw_mlr_rhw(&self.service_group, suitability, 0);
        // Determine this services role in the rolling update
        match self.update_role().await {
            Role::Leader => self.leader_role().await,
            Role::Follower => {
                // Wait till it is our turn to update. It is possible that while we are waiting the
                // leader dies and we are promoted to update leader.
                match self.follower_wait_for_turn().await {
                    FollowerWaitForTurn::PromotedToLeader => self.leader_role().await,
                    FollowerWaitForTurn::UpdateTo(new_ident) => {
                        // Update to the package we were instructed to
                        self.package_update_worker.update_to(new_ident).await
                    }
                }
            }
        }
    }

    async fn leader_role(&self) -> PackageIdent {
        // Wait for followers to finish updating if they are have an older version of the
        // package
        self.leader_wait_for_followers().await;
        // Wait for an update
        self.package_update_worker.update().await
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
                                    trace!("This is the '{}' leader; using the minimum rolling \
                                            update election suitability",
                                           self.service_group);
                                    break u64::min_value();
                                } else {
                                    trace!("This is a '{}' follower; using the maximum rolling \
                                            update election suitability",
                                           self.service_group);
                                    break u64::max_value();
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
                    time::delay_for(DELAY).await;
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
            time::delay_for(DELAY).await;
        }
    }

    async fn leader_wait_for_followers(&self) {
        loop {
            {
                let census_group = self.census_group().await;
                if let Some(me) = census_group.me() {
                    if census_group.active_members()
                                   .all(|member| member.pkg >= me.pkg)
                    {
                        debug!("'{}' rolling update leader verified all follower package versions",
                               self.service_group);
                        break;
                    }
                } else {
                    error!("Supervisor does not know its own identity; rolling update of {} \
                            cannot proceed! Please notify the Habitat core team!",
                           self.service_group);
                    debug_assert!(false);
                }
            }
            debug!("'{}' rolling update leader waiting for followers to update",
                   self.service_group);
            time::delay_for(DELAY).await;
        }
    }

    async fn follower_wait_for_turn(&self) -> FollowerWaitForTurn {
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
                            debug!("'{}' rolling update follower is promoted to the leader",
                                   self.service_group);
                            break FollowerWaitForTurn::PromotedToLeader;
                        }
                        if leader.pkg < me.pkg {
                            debug!("'{}' rolling update leader has an outdated package and needs \
                                    to update",
                                   self.service_group);
                        } else if leader.pkg == me.pkg {
                            trace!("'{}' is not in a rolling update", self.service_group);
                        } else if leader.pkg != peer.pkg {
                            debug!("'{}' is in a rolling update but it is not this followers \
                                    turn to update",
                                   self.service_group);
                        } else {
                            let new_ident = leader.pkg.clone();
                            debug!("'{}' is in a rolling update and it is this followers turn to \
                                    update to '{}'",
                                   self.service_group, new_ident);
                            break FollowerWaitForTurn::UpdateTo(new_ident);
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
            time::delay_for(DELAY).await;
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
            time::delay_for(DELAY).await;
        }
    }
}
