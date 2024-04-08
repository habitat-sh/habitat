//! The Departure rumor.
//!
//! Deaprture rumors declare that a given member has "departed" the gossip ring manually. When this
//! happens, we ensure that the member can no longer come back into the fold, unless an
//! administrator reverses the decision.

use serde::Serialize;

use crate::{error::{Error,
                    Result},
            protocol::{self,
                       newscast::{self,
                                  Rumor as ProtoRumor},
                       FromProto},
            rumor::{ConstKeyRumor,
                    Rumor,
                    RumorPayload,
                    RumorType}};
use std::{cmp::Ordering,
          fmt};

#[derive(Debug, Clone, Serialize)]
pub struct Departure {
    pub member_id: String,
}

impl fmt::Display for Departure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Departure m/{}", self.member_id)
    }
}

impl Departure {
    pub fn new(member_id: &str) -> Self { Departure { member_id: member_id.to_string(), } }
}

impl protocol::Message<ProtoRumor> for Departure {
    const MESSAGE_ID: &'static str = "Departure";
}

impl FromProto<ProtoRumor> for Departure {
    fn from_proto(rumor: ProtoRumor) -> Result<Self> {
        let payload = match rumor.payload.ok_or(Error::ProtocolMismatch("payload"))? {
            RumorPayload::Departure(payload) => payload,
            _ => panic!("from-bytes departure"),
        };
        Ok(Departure { member_id: payload.member_id
                                         .ok_or(Error::ProtocolMismatch("member-id"))?, })
    }
}

impl From<Departure> for newscast::Departure {
    fn from(value: Departure) -> Self { newscast::Departure { member_id: Some(value.member_id), } }
}

impl Rumor for Departure {
    fn merge(&mut self, other: Departure) -> bool { *self < other }

    fn kind(&self) -> RumorType { RumorType::Departure }

    fn key(&self) -> &str { Self::const_key() }

    fn id(&self) -> &str { &self.member_id }
}

impl ConstKeyRumor for Departure {
    fn const_key() -> &'static str { "departure" }
}

impl PartialOrd for Departure {
    fn partial_cmp(&self, other: &Departure) -> Option<Ordering> {
        if self.member_id != other.member_id {
            None
        } else {
            Some(self.member_id.cmp(&other.member_id))
        }
    }
}

impl PartialEq for Departure {
    fn eq(&self, other: &Departure) -> bool { self.member_id == other.member_id }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::Departure;
    use crate::rumor::{ConstKeyRumor as _,
                       Rumor,
                       RumorStore};

    fn create_departure(member_id: &str) -> Departure { Departure::new(member_id) }

    fn create_rumor_store() -> RumorStore<Departure> { RumorStore::default() }

    #[test]
    fn multiple_departures_are_all_under_the_same_key() {
        let rs = create_rumor_store();
        let d1 = Departure::new("member_1");
        let d2 = Departure::new("member_2");
        rs.insert_rsw(d1);
        rs.insert_rsw(d2);

        let list = rs.lock_rsr();
        assert_eq!(list.len(), 1); // for the "departure" key

        let sub_list = list.get(Departure::const_key()).unwrap();
        assert_eq!(sub_list.len(), 2); // for each of the members we inserted
    }

    #[test]
    fn identical_departures_are_equal() {
        let s1 = create_departure("mastodon");
        let s2 = create_departure("mastodon");
        assert_eq!(s1, s2);
    }

    #[test]
    fn departures_with_different_member_ids_are_not_equal() {
        let s1 = create_departure("mastodon");
        let s2 = create_departure("limpbizkit");
        assert_ne!(s1, s2);
    }

    // Order
    #[test]
    fn departures_that_are_identical_are_equal_via_cmp() {
        let s1 = create_departure("adam");
        let s2 = create_departure("adam");
        assert_eq!(s1.partial_cmp(&s2), Some(Ordering::Equal));
    }

    #[test]
    fn merge_returns_false_if_nothing_changed() {
        let mut s1 = create_departure("mastodon");
        let s1_check = s1.clone();
        let s2 = create_departure("mastodon");
        assert!(!s1.merge(s2));
        assert_eq!(s1, s1_check);
    }
}
