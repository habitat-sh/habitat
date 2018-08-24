#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
pub struct Wire {
    #[prost(bool, optional, tag="1", default="false")]
    pub encrypted: ::std::option::Option<bool>,
    #[prost(bytes, optional, tag="2")]
    pub nonce: ::std::option::Option<Vec<u8>>,
    #[prost(bytes, optional, tag="3")]
    pub payload: ::std::option::Option<Vec<u8>>,
}
