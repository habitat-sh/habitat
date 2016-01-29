// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

//! The membership system.
//!
//! This module tracks membership in the gossip ring. It consists of `Members` and a collection of
//! them in a `MemberList`.

use std::mem;
use std::collections::HashMap;
use rand::{thread_rng, Rng};
use uuid::Uuid;

use gossip::lamport_clock::LamportClock;

// How many members do we send a PingReq to for a failed node?
static PINGREQ_MEMBERS: usize = 5;

/// Every time we receive a Suspect or Confirmed message about our own entry in the MemberList, we
/// update our Incarnation.
pub type Incarnation = LamportClock;

/// The health of the Member.
#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub enum Health {
    Alive,
    Suspect,
    Confirmed,
}

/// Each member has a Uuid.
pub type MemberId = Uuid;

/// A member in the gossip ring. Members can be marked 'permanent', which means they will always be
/// ping-ed, even during failure.
#[derive(Clone, Debug, PartialEq, Eq, RustcDecodable, RustcEncodable)]
pub struct Member {
    pub id: MemberId,
    pub hostname: String,
    pub ip: String,
    pub gossip_listener: String,
    pub incarnation: Incarnation,
    pub health: Health,
    pub permanent: bool,
}

impl Member {
    /// Create a new member.
    pub fn new(id: Uuid,
               hostname: String,
               ip: String,
               gossip_listener: String,
               permanent: bool)
               -> Member {
        Member {
            id: id,
            hostname: hostname,
            ip: ip,
            gossip_listener: gossip_listener,
            incarnation: Incarnation::new(),
            health: Health::Alive,
            permanent: permanent,
        }
    }

    /// Updates this member based on another member; likely received via gossip.
    ///
    /// Returns true if we changed anything about our record; false if we didn't.
    ///
    /// * If the rhs has a higher incarnation, use the rhs
    /// * If the lhs has a higher incarnation, use the lhs
    /// * If the incarnation is equal, and health is equal, use the lhs
    /// * If the incarnation is equal, and the rhs health is confirmed, used rhs
    /// * If the incarnation is equal, and the lhs is alive, and the rhs is suspect, use rhs
    /// * If the incarnation is equal, and the lhs is confirmed, and the rhs is suspect or alive, use the lhs
    pub fn update_via(&mut self, my_id: &MemberId, rhs: Member) -> bool {
        if self.incarnation > rhs.incarnation {
            return false;
        } else if self.incarnation < rhs.incarnation {
            mem::replace(self, rhs);
            return true;
        } else {
            if self.health == rhs.health {
                return false;
            }
            if rhs.health == Health::Confirmed {
                if self.id == *my_id {
                    self.incarnation.increment();
                    self.health = Health::Alive;
                    return true;
                } else {
                    mem::replace(self, rhs);
                    return true;
                }
            }
            if self.health == Health::Alive && rhs.health == Health::Suspect {
                if self.id == *my_id {
                    self.incarnation.increment();
                    return true;
                } else {
                    mem::replace(self, rhs);
                    return true;
                }
            }
            if self.health == Health::Confirmed && rhs.health == Health::Alive {
                return false;
            }
            if self.health == Health::Confirmed && rhs.health == Health::Suspect {
                return false;
            }
        }
        return false;
    }
}

/// A list of members. Keeps track of both the members themselves, and provides an order to iterate
/// through them via the `next()` function.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct MemberList {
    members: HashMap<Uuid, Member>,
    position: usize,
    order: Vec<Uuid>,
}

impl MemberList {
    /// Create a new member list
    pub fn new() -> MemberList {
        MemberList {
            members: HashMap::new(),
            position: 0,
            order: Vec::new(),
        }
    }

    /// Insert a member into the list.
    pub fn insert(&mut self, member: Member) {
        let oid = member.id.clone();
        if let None = self.members.insert(member.id.clone(), member) {
            self.order.push(oid);
        }
    }

    /// Return a reference to the next member in the list. If the list is empty, we return `None`.
    /// Otherwise, we return `Some<&Member>`. When we have reached the end of the list, we
    /// randomize the order, and re-set our position to the top of the list.
    ///
    /// This ensures that every member gets touched in each gossip round.
    pub fn next(&mut self) -> Option<&Member> {
        if self.order.len() == 0 {
            return None;
        };
        if self.position < self.order.len() {
            let ref member = self.members.get(&self.order[self.position]).unwrap();
            self.position = self.position + 1;
            Some(member)
        } else {
            self.position = 0;
            let mut rng = thread_rng();
            rng.shuffle(&mut self.order);
            self.next()
        }
    }

    /// Given an incoming member entry, process it. If we have a record for the member already, we
    /// pass the member on to `update_via`. Otherwise, it's the first time we've seen this member,
    /// so we insert them.
    ///
    /// Either way, we return true if we added a new member or mutated an existing one; false if we
    /// did nothing.
    pub fn process(&mut self, my_id: &MemberId, remote_member: Member) -> bool {
        // This is strange - rust won't let this be an else.
        if let Some(mut current_member) = self.members.get_mut(&remote_member.id) {
            return current_member.update_via(my_id, remote_member);
        }
        match remote_member.health {
            Health::Alive => warn!("Member {} is alive", remote_member.id),
            Health::Suspect => warn!("Member {} is suspect", remote_member.id),
            Health::Confirmed => warn!("Member {} is confirmed dead", remote_member.id),
        }
        self.insert(remote_member);
        return true;
    }

    /// Set a members health to Alive
    pub fn alive(&mut self, member_id: &MemberId) {
        if let Some(mut member) = self.members.get_mut(member_id) {
            member.health = Health::Alive;
        }
    }

    /// Set a members health to Suspect.
    pub fn suspect(&mut self, member_id: &MemberId) {
        if let Some(mut member) = self.members.get_mut(member_id) {
            if member.health != Health::Confirmed {
                warn!("Member {} is suspect", member_id);
                member.health = Health::Suspect;
            }
        }
    }

    /// Set a members health to Confirmed.
    pub fn confirm(&mut self, member_id: &MemberId) {
        if let Some(mut member) = self.members.get_mut(member_id) {
            if member.health != Health::Confirmed {
                warn!("Member {} is confirmed dead", member_id);
                member.health = Health::Confirmed;
            }
        }
    }

    /// Selects PINGREQ_MEMBERS number of members to use as targets for a PingReq. The members are
    /// chosen completely randomly.
    pub fn pingreq_targets(&self, myself: &MemberId, target: &MemberId) -> Vec<Member> {
        let mut rng = thread_rng();
        let mut values: Vec<&Member> = self.members
                                           .values()
                                           .filter(|m| &m.id != myself && &m.id != target)
                                           .collect();
        rng.shuffle(&mut values);
        let mut results: Vec<Member> = Vec::new();
        for member in values.into_iter().take(PINGREQ_MEMBERS) {
            results.push(member.clone());
        }
        results
    }

    /// Return an reference to a given member, if it exists in the MemberList.
    pub fn get(&self, member_id: &MemberId) -> Option<&Member> {
        self.members.get(member_id)
    }

    /// Return all the members whose health is Suspect.
    pub fn suspect_members(&self) -> Vec<&MemberId> {
        let mut usual_suspects = Vec::new();
        for (id, member) in self.members.iter() {
            if member.health == Health::Suspect {
                usual_suspects.push(id);
            }
        }
        usual_suspects
    }

    /// Return true if all members other than the provided id are Confirmed.
    pub fn isolated(&self, myself: &MemberId) -> bool {
        self.members.iter().fold(true, |acc, (id, m)| {
            if id == myself {
                acc
            } else if m.health == Health::Confirmed && acc != false {
                true
            } else {
                false
            }
        })
    }
}

#[cfg(test)]
mod test {
    mod member {
        use gossip::member::{Member, Health, MemberId};

        #[test]
        fn update_via_rhs_higher_incarnation() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(MemberId::new_v4(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let mut other_bobo = bobo.clone();

            // If the RHS has a higher incarnation, use it
            other_bobo.incarnation.increment();
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(*bobo.incarnation.time(), 1);
            assert_eq!(r, true);
        }

        #[test]
        fn update_via_lhs_higher_incarnation() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(MemberId::new_v4(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let other_bobo = bobo.clone();

            // If the LHS has a higher incarnation, use it
            bobo.incarnation.increment();
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(*bobo.incarnation.time(), 1);
            assert_eq!(r, false);
        }

        #[test]
        fn update_via_equal_incarnation_and_health() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(MemberId::new_v4(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let other_bobo = bobo.clone();
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(*bobo.incarnation.time(), 0);
            assert_eq!(r, false);
        }

        #[test]
        fn update_via_equal_and_rhs_confirmed() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(MemberId::new_v4(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let mut other_bobo = bobo.clone();
            other_bobo.health = Health::Confirmed;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Confirmed);
            assert_eq!(r, true);
        }

        #[test]
        fn update_via_equal_while_lhs_alive_and_rhs_suspect() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(MemberId::new_v4(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let mut other_bobo = bobo.clone();
            other_bobo.health = Health::Suspect;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Suspect);
            assert_eq!(r, true);
        }

        #[test]
        fn update_via_equal_while_lhs_alive_and_rhs_suspect_and_lhs_is_me() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(my_id.clone(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let mut other_bobo = bobo.clone();
            other_bobo.health = Health::Suspect;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Alive);
            assert_eq!(*bobo.incarnation, 1);
            assert_eq!(r, true);
        }

        #[test]
        fn update_via_equal_while_lhs_alive_and_rhs_confirmed_and_lhs_is_me() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(my_id.clone(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let mut other_bobo = bobo.clone();
            other_bobo.health = Health::Confirmed;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Alive);
            assert_eq!(*bobo.incarnation, 1);
            assert_eq!(r, true);
        }


        #[test]
        fn update_via_equal_while_lhs_confirmed_and_rhs_suspect_or_alive() {
            let my_id = MemberId::new_v4();
            let mut bobo = Member::new(MemberId::new_v4(),
                                       String::from("bobo"),
                                       String::from("192.168.1.1"),
                                       String::from("192.168.1.2"),
                                       false);
            let mut other_bobo = bobo.clone();
            bobo.health = Health::Confirmed;
            other_bobo.health = Health::Suspect;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Confirmed);
            assert_eq!(r, false);

            let mut tclown = bobo.clone();
            tclown.health = Health::Alive;
            let r = bobo.update_via(&my_id, tclown);
            assert_eq!(bobo.health, Health::Confirmed);
            assert_eq!(r, false);
        }
    }

    mod member_list {
        use uuid::Uuid;
        use gossip::member::{Member, MemberList, Health};

        #[test]
        fn insert() {
            let mut ml = MemberList::new();
            let james = Member::new(Uuid::new_v4(),
                                    String::from("james"),
                                    String::from("192.168.1.1"),
                                    String::from("192.168.1.1:4312"),
                                    false);
            let jid = james.id.clone();
            ml.insert(james);
            assert!(ml.members.contains_key(&jid));
            assert_eq!(ml.order[0], jid);
        }

        #[test]
        fn next() {
            let member_names = [String::from("a.foo.com"),
                                String::from("b.foo.com"),
                                String::from("c.foo.com"),
                                String::from("d.foo.com")];
            let mut ml = MemberList::new();

            // Returns None when the list is empty
            assert!(ml.next().is_none());

            for name in member_names.iter() {
                let member = Member::new(Uuid::new_v4(),
                                         name.clone(),
                                         String::from("192.168.1.1"),
                                         String::from("192.168.1.1:4312"),
                                         false);
                ml.insert(member);
            }

            // The first pass through the list is in the order of initial insertion
            for x in 0..4 {
                match ml.next() {
                    Some(m) => assert_eq!(m.hostname, member_names[x]),
                    None => assert!(false, "Returned none rather than a member"),
                }
            }

            // The next pass is randomized, but should hit every member
            let mut hit_them_all = vec![];
            for _ in 0..4 {
                match ml.next() {
                    Some(m) => hit_them_all.push(m.hostname.clone()),
                    None => assert!(false, "Returned none rather than a member"),
                }
            }
            for name in member_names.iter() {
                assert!(hit_them_all.iter().any(|x| x == name))
            }
        }

        #[test]
        fn pingreq_members() {
            let member_names = [String::from("a.foo.com"),
                                String::from("b.foo.com"),
                                String::from("c.foo.com"),
                                String::from("d.foo.com")];
            let mut ml = MemberList::new();
            for name in member_names.iter() {
                let member = Member::new(Uuid::new_v4(),
                                         name.clone(),
                                         String::from("192.168.1.1"),
                                         String::from("192.168.1.1:4312"),
                                         false);
                ml.insert(member);
            }

            let my_id = Uuid::new_v4();
            let my_member = Member::new(my_id.clone(),
                                        String::from("myguy"),
                                        String::from("192.168.1.1"),
                                        String::from("192.168.1.1:4312"),
                                        false);
            ml.insert(my_member);

            let dead_id = Uuid::new_v4();
            let dead_member = Member::new(dead_id.clone(),
                                          String::from("deadguy"),
                                          String::from("192.168.1.1"),
                                          String::from("192.168.1.1:4312"),
                                          false);
            ml.insert(dead_member);

            // With fewer than 5 members (excluding the target), use them all
            assert_eq!(ml.pingreq_targets(&my_id, &dead_id).len(), 4);

            let newbie = Member::new(Uuid::new_v4(),
                                     String::from("newbie.foo.com"),
                                     String::from("192.168.1.1"),
                                     String::from("192.168.1.1:4312"),
                                     false);
            ml.insert(newbie);
            // With 5 members, use them all
            assert_eq!(ml.pingreq_targets(&my_id, &dead_id).len(), 5);

            let oldie = Member::new(Uuid::new_v4(),
                                    String::from("oldie.foo.com"),
                                    String::from("192.168.1.1"),
                                    String::from("192.168.1.1:4312"),
                                    false);
            ml.insert(oldie);
            // With more than 5 members, use them all
            assert_eq!(ml.pingreq_targets(&my_id, &dead_id).len(), 5);
        }

        #[test]
        fn isolated() {
            let member_names = [String::from("a.foo.com"),
                                String::from("b.foo.com"),
                                String::from("c.foo.com"),
                                String::from("d.foo.com")];
            let mut ml = MemberList::new();
            for name in member_names.iter() {
                let mut member = Member::new(Uuid::new_v4(),
                                             name.clone(),
                                             String::from("192.168.1.1"),
                                             String::from("192.168.1.1:4312"),
                                             false);
                member.health = Health::Confirmed;
                ml.insert(member);
            }
            let myself = Member::new(Uuid::new_v4(),
                                     String::from("me"),
                                     String::from("192.168.1.100"),
                                     String::from("192.168.1.100:4312"),
                                     false);
            let my_id = myself.id.clone();
            ml.insert(myself);

            // Everyone is confirmed, so we are isolated
            assert_eq!(ml.isolated(&my_id), true);

            let mut among_the_living = ml.next().unwrap().id.clone();
            if among_the_living == my_id {
                among_the_living = ml.next().unwrap().id.clone();
            }
            ml.alive(&among_the_living);

            // One member who is not us is alive, so we are not isolated
            assert_eq!(ml.isolated(&my_id), false);
        }
    }
}
