#[derive(Clone, PartialEq, ::prost::Message)]
#[derive(Serialize, Deserialize)]
pub struct Wire {
    #[prost(bool, optional, tag="1", default="false")]
    pub encrypted: ::std::option::Option<bool>,
    #[prost(bytes, optional, tag="2")]
    pub nonce: ::std::option::Option<std::vec::Vec<u8>>,
    #[prost(bytes, optional, tag="3")]
    pub payload: ::std::option::Option<std::vec::Vec<u8>>,
}
