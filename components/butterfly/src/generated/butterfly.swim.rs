#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct ZoneAddress {
    #[prost(string, optional, tag="1")]
    pub zone_id: ::std::option::Option<String>,
    /// really optional
    #[prost(string, optional, tag="2")]
    pub address: ::std::option::Option<String>,
    #[prost(int32, optional, tag="3")]
    pub swim_port: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="4")]
    pub gossip_port: ::std::option::Option<i32>,
    #[prost(string, optional, tag="5")]
    pub tag: ::std::option::Option<String>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Member {
    #[prost(string, optional, tag="1")]
    pub id: ::std::option::Option<String>,
    #[prost(uint64, optional, tag="2")]
    pub incarnation: ::std::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub address: ::std::option::Option<String>,
    #[prost(int32, optional, tag="4")]
    pub swim_port: ::std::option::Option<i32>,
    #[prost(int32, optional, tag="5")]
    pub gossip_port: ::std::option::Option<i32>,
    #[prost(bool, optional, tag="6", default="false")]
    pub persistent: ::std::option::Option<bool>,
    #[prost(bool, optional, tag="7", default="false")]
    pub departed: ::std::option::Option<bool>,
    #[prost(string, optional, tag="8")]
    pub zone_id: ::std::option::Option<String>,
    #[prost(message, repeated, tag="9")]
    pub additional_addresses: ::std::vec::Vec<ZoneAddress>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Zone {
    #[prost(string, optional, tag="1")]
    pub id: ::std::option::Option<String>,
    #[prost(uint64, optional, tag="2")]
    pub incarnation: ::std::option::Option<u64>,
    #[prost(string, optional, tag="3")]
    pub maintainer_id: ::std::option::Option<String>,
    /// really optional
    #[prost(string, optional, tag="4")]
    pub parent_zone_id: ::std::option::Option<String>,
    #[prost(string, repeated, tag="5")]
    pub child_zone_ids: ::std::vec::Vec<String>,
    /// really optional
    #[prost(string, optional, tag="6")]
    pub successor: ::std::option::Option<String>,
    #[prost(string, repeated, tag="7")]
    pub predecessors: ::std::vec::Vec<String>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Ping {
    #[prost(message, optional, tag="1")]
    pub from: ::std::option::Option<Member>,
    #[prost(message, optional, tag="2")]
    pub forward_to: ::std::option::Option<Member>,
    #[prost(message, optional, tag="3")]
    pub to: ::std::option::Option<Member>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Ack {
    #[prost(message, optional, tag="1")]
    pub from: ::std::option::Option<Member>,
    #[prost(message, optional, tag="2")]
    pub forward_to: ::std::option::Option<Member>,
    #[prost(message, optional, tag="3")]
    pub to: ::std::option::Option<Member>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct PingReq {
    #[prost(message, optional, tag="1")]
    pub from: ::std::option::Option<Member>,
    #[prost(message, optional, tag="2")]
    pub target: ::std::option::Option<Member>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct ZoneChange {
    #[prost(message, optional, tag="1")]
    pub from: ::std::option::Option<Member>,
    #[prost(string, optional, tag="2")]
    pub zone_id: ::std::option::Option<String>,
    #[prost(message, repeated, tag="3")]
    pub new_aliases: ::std::vec::Vec<Zone>,
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Membership {
    #[prost(message, optional, tag="1")]
    pub member: ::std::option::Option<Member>,
    #[prost(enumeration="membership::Health", optional, tag="2")]
    pub health: ::std::option::Option<i32>,
}
pub mod membership {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[derive(Serialize, Deserialize)]
    pub enum Health {
        Alive = 1,
        Suspect = 2,
        Confirmed = 3,
        Departed = 4,
    }
}
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Swim {
    /// Identifies which field is filled in.
    #[prost(enumeration="swim::Type", required, tag="1")]
    pub type_: i32,
    #[prost(message, repeated, tag="5")]
    pub membership: ::std::vec::Vec<Membership>,
    #[prost(message, repeated, tag="6")]
    pub zones: ::std::vec::Vec<Zone>,
    #[prost(oneof="swim::Payload", tags="2, 3, 4, 7")]
    pub payload: ::std::option::Option<swim::Payload>,
}
pub mod swim {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
    #[derive(Serialize, Deserialize)]
    pub enum Type {
        Ping = 1,
        Ack = 2,
        Pingreq = 3,
        ZoneChange = 4,
    }
    #[derive(Clone, Oneof, PartialEq)]
    #[derive(Serialize, Deserialize)]
    pub enum Payload {
        #[prost(message, tag="2")]
        Ping(super::Ping),
        #[prost(message, tag="3")]
        Ack(super::Ack),
        #[prost(message, tag="4")]
        Pingreq(super::PingReq),
        #[prost(message, tag="7")]
        ZoneChange(super::ZoneChange),
    }
}
