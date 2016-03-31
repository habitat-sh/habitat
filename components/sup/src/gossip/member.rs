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
use std::fmt;
use std::ops::Deref;

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
    pub fn new(hostname: String, ip: String, gossip_listener: String, permanent: bool) -> Member {
        Member {
            id: MemberId::new_v4(),
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

impl fmt::Display for Member {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.ip, self.id)
    }
}

/// A list of members. Keeps track of both the members themselves, and provides an order to iterate
/// through them via the `next()` function.
#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct MemberList {
    members: HashMap<MemberId, Member>,
    position: usize,
    order: Vec<Uuid>,
    pub my_id: MemberId,
}

impl MemberList {
    /// Create a new member list
    pub fn new(my_member: Member) -> MemberList {
        let mut ml = MemberList {
            members: HashMap::new(),
            position: 0,
            order: Vec::new(),
            my_id: my_member.id.clone(),
        };
        ml.insert(my_member);
        ml
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
    pub fn process(&mut self, remote_member: Member) -> bool {
        // This is strange - rust won't let this be an else.
        if let Some(mut current_member) = self.members.get_mut(&remote_member.id) {
            return current_member.update_via(&self.my_id, remote_member);
        }
        match remote_member.health {
            Health::Alive => warn!("Member {} is alive", remote_member.id),
            Health::Suspect => warn!("Member {} is suspect", remote_member.id),
            Health::Confirmed => warn!("Member {} is confirmed dead", remote_member.id),
        }
        self.insert(remote_member);
        return true;
    }

    /// Return true if this member is alive
    pub fn is_alive(&self, member_id: &MemberId) -> bool {
        if let Some(member) = self.members.get(member_id) {
            if member.health == Health::Alive {
                return true;
            }
        }
        false
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

impl Deref for MemberList {
    type Target = HashMap<MemberId, Member>;

    fn deref(&self) -> &HashMap<MemberId, Member> {
        &self.members
    }
}

#[cfg(test)]
mod test {
    mod member {
        use gossip::member::{Member, Health, MemberId};

        fn bobo() -> Member {
            Member::new(String::from("bobo"),
                        String::from("192.168.1.1"),
                        String::from("192.168.1.2"),
                        false)
        }

        #[test]
        fn update_via_rhs_higher_incarnation() {
            let my_id = MemberId::new_v4();
            let mut bobo = bobo();
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
            let mut bobo = bobo();
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
            let mut bobo = bobo();
            let other_bobo = bobo.clone();
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(*bobo.incarnation.time(), 0);
            assert_eq!(r, false);
        }

        #[test]
        fn update_via_equal_and_rhs_confirmed() {
            let my_id = MemberId::new_v4();
            let mut bobo = bobo();
            let mut other_bobo = bobo.clone();
            other_bobo.health = Health::Confirmed;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Confirmed);
            assert_eq!(r, true);
        }

        #[test]
        fn update_via_equal_while_lhs_alive_and_rhs_suspect() {
            let my_id = MemberId::new_v4();
            let mut bobo = bobo();
            let mut other_bobo = bobo.clone();
            other_bobo.health = Health::Suspect;
            let r = bobo.update_via(&my_id, other_bobo);
            assert_eq!(bobo.health, Health::Suspect);
            assert_eq!(r, true);
        }

        #[test]
        fn update_via_equal_while_lhs_alive_and_rhs_suspect_and_lhs_is_me() {
            let my_id = MemberId::new_v4();
            let mut bobo = bobo();
            bobo.id = my_id.clone();
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
            let mut bobo = bobo();
            bobo.id = my_id.clone();
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
            let mut bobo = bobo();
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

        fn new_member_list() -> MemberList {
            let james = Member::new(String::from("james"),
                                    String::from("192.168.1.1"),
                                    String::from("192.168.1.1:4312"),
                                    false);
            MemberList::new(james)
        }

        fn confirm_member_list(member_list: &mut MemberList) {
            for (_id, mut member) in member_list.members.iter_mut() {
                member.health = Health::Confirmed;
            }
        }

        #[test]
        fn next() {
            let mut ml = new_member_list();
            let mut member_names = vec![String::from("a.foo.com"),
                                        String::from("b.foo.com"),
                                        String::from("c.foo.com"),
                                        String::from("d.foo.com")];

            for name in member_names.iter() {
                let member = Member::new(name.clone(),
                                         String::from("192.168.1.1"),
                                         String::from("192.168.1.1:4312"),
                                         false);
                ml.insert(member);
            }

            member_names.insert(0, String::from("james"));

            // The first pass through the list is in the order of initial insertion
            for x in 0..5 {
                match ml.next() {
                    Some(m) => assert_eq!(m.hostname, member_names[x]),
                    None => assert!(false, "Returned none rather than a member"),
                }
            }

            // The next pass is randomized, but should hit every member
            let mut hit_them_all = vec![];
            for _ in 0..5 {
                match ml.next() {
                    Some(m) => hit_them_all.push(m.hostname.clone()),
                    None => assert!(false, "Returned none rather than a member"),
                }
            }
            for name in member_names.iter() {
                assert!(hit_them_all.iter().any(|x| x == name))
            }
            assert!(hit_them_all.iter().any(|x| x == "james"))
        }

        #[test]
        fn pingreq_members() {
            let mut ml = new_member_list();
            let member_names = [String::from("a.foo.com"),
                                String::from("b.foo.com"),
                                String::from("c.foo.com"),
                                String::from("d.foo.com")];
            for name in member_names.iter() {
                let member = Member::new(name.clone(),
                                         String::from("192.168.1.1"),
                                         String::from("192.168.1.1:4312"),
                                         false);
                ml.insert(member);
            }

            let my_id = Uuid::new_v4();
            let mut my_member = Member::new(String::from("myguy"),
                                            String::from("192.168.1.1"),
                                            String::from("192.168.1.1:4312"),
                                            false);
            my_member.id = my_id.clone();
            ml.insert(my_member);

            let dead_id = Uuid::new_v4();
            let mut dead_member = Member::new(String::from("deadguy"),
                                              String::from("192.168.1.1"),
                                              String::from("192.168.1.1:4312"),
                                              false);
            dead_member.id = dead_id.clone();
            ml.insert(dead_member);

            // With fewer than 6 members (excluding the target), use them all
            assert_eq!(ml.pingreq_targets(&my_id, &dead_id).len(), 5);

            let newbie = Member::new(String::from("newbie.foo.com"),
                                     String::from("192.168.1.1"),
                                     String::from("192.168.1.1:4312"),
                                     false);
            ml.insert(newbie);
            // With 5 members, use them all
            assert_eq!(ml.pingreq_targets(&my_id, &dead_id).len(), 5);

            let oldie = Member::new(String::from("oldie.foo.com"),
                                    String::from("192.168.1.1"),
                                    String::from("192.168.1.1:4312"),
                                    false);
            ml.insert(oldie);
            // With more than 5 members, use them all
            assert_eq!(ml.pingreq_targets(&my_id, &dead_id).len(), 5);
        }

        #[test]
        fn isolated() {
            let mut ml = new_member_list();
            let member_names = [String::from("a.foo.com"),
                                String::from("b.foo.com"),
                                String::from("c.foo.com"),
                                String::from("d.foo.com")];
            for name in member_names.iter() {
                let member = Member::new(name.clone(),
                                         String::from("192.168.1.1"),
                                         String::from("192.168.1.1:4312"),
                                         false);
                ml.insert(member);
            }

            confirm_member_list(&mut ml);

            let myself = Member::new(String::from("me"),
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
