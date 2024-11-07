//! Implementation of a probe list - A list of Members to be 'probe'd when reported as `Confirmed`.

use std::collections::HashSet;

use habitat_common::sync::{Lock,
                           ReadGuard,
                           WriteGuard};

use crate::member::Member;

#[derive(Debug)]
pub struct ProbeList {
    members: Lock<HashSet<Member>>,
}

impl ProbeList {
    pub fn new() -> Self { Self { members: Lock::new(HashSet::new()), } }

    pub fn members_read(&self) -> ReadGuard<'_, HashSet<Member>> { self.members.read() }

    pub fn members_write(&self) -> WriteGuard<'_, HashSet<Member>> { self.members.write() }
}
