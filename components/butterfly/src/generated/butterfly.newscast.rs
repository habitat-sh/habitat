#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Election {
    #[prost(string, optional, tag="1")]
    pub member_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub service_group: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub term: ::std::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub suitability: ::std::option::Option<u64>,
    #[prost(enumeration="election::Status", optional, tag="5")]
    pub status: ::std::option::Option<i32>,
    #[prost(string, repeated, tag="6")]
    pub votes: ::std::vec::Vec<std::string::String>,
}
pub mod election {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(Serialize, Deserialize)]
    pub enum Status {
        Running = 1,
        NoQuorum = 2,
        Finished = 3,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Service {
    #[prost(string, optional, tag="1")]
    pub member_id: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2")]
    pub service_group: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="3")]
    pub incarnation: ::std::option::Option<u64>,
    #[prost(bool, optional, tag="8")]
    pub initialized: ::std::option::Option<bool>,
    #[prost(string, optional, tag="9")]
    pub pkg: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="10")]
    pub cfg: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(message, optional, tag="12")]
    pub sys: ::std::option::Option<SysInfo>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct ServiceConfig {
    #[prost(string, optional, tag="1")]
    pub service_group: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub incarnation: ::std::option::Option<u64>,
    #[prost(bool, optional, tag="3")]
    pub encrypted: ::std::option::Option<bool>,
    #[prost(bytes, optional, tag="4")]
    pub config: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct ServiceFile {
    #[prost(string, optional, tag="1")]
    pub service_group: ::std::option::Option<std::string::String>,
    #[prost(uint64, optional, tag="2")]
    pub incarnation: ::std::option::Option<u64>,
    #[prost(bool, optional, tag="3")]
    pub encrypted: ::std::option::Option<bool>,
    #[prost(string, optional, tag="4")]
    pub filename: ::std::option::Option<std::string::String>,
    #[prost(bytes, optional, tag="5")]
    pub body: ::std::option::Option<std::vec::Vec<u8>>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct SysInfo {
    #[prost(string, optional, tag="1", default="127.0.0.1")]
    pub ip: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="2", default="localhost")]
    pub hostname: ::std::option::Option<std::string::String>,
    #[prost(string, optional, tag="3", default="127.0.0.1")]
    pub gossip_ip: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="4")]
    pub gossip_port: ::std::option::Option<u32>,
    #[prost(string, optional, tag="5", default="127.0.0.1")]
    pub http_gateway_ip: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="6")]
    pub http_gateway_port: ::std::option::Option<u32>,
    #[prost(string, optional, tag="7", default="127.0.0.1")]
    pub ctl_gateway_ip: ::std::option::Option<std::string::String>,
    #[prost(uint32, optional, tag="8", default="9632")]
    pub ctl_gateway_port: ::std::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Departure {
    #[prost(string, optional, tag="1")]
    pub member_id: ::std::option::Option<std::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Rumor {
    #[prost(enumeration="rumor::Type", required, tag="1")]
    pub r#type: i32,
    #[prost(string, repeated, tag="2")]
    pub tag: ::std::vec::Vec<std::string::String>,
    #[prost(string, optional, tag="3")]
    pub from_id: ::std::option::Option<std::string::String>,
    #[prost(oneof="rumor::Payload", tags="4, 5, 6, 7, 8, 9")]
    pub payload: ::std::option::Option<rumor::Payload>,
}
pub mod rumor {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(Serialize, Deserialize)]
    pub enum Type {
        Member = 1,
        Service = 2,
        Election = 3,
        ServiceConfig = 4,
        ServiceFile = 5,
        Fake = 6,
        Fake2 = 7,
        ElectionUpdate = 8,
        Departure = 9,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    #[allow(clippy::large_enum_variant)]
    #[derive(Serialize, Deserialize)]
    pub enum Payload {
        #[prost(message, tag="4")]
        Member(super::super::swim::Membership),
        #[prost(message, tag="5")]
        Service(super::Service),
        #[prost(message, tag="6")]
        ServiceConfig(super::ServiceConfig),
        #[prost(message, tag="7")]
        ServiceFile(super::ServiceFile),
        #[prost(message, tag="8")]
        Election(super::Election),
        #[prost(message, tag="9")]
        Departure(super::Departure),
    }
}
