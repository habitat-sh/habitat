/// Returned when a transactional request is successful but no entities are returned. Useful
/// when making a request which requires no response but the caller wants to block for completion.
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetOk {
}
/// Returned when a transactional request is a failure. Contains an `ErrCode` indicating a failure
/// domain or reason and a string message containing a user friendly failure reason.
///
/// Failure reasons are ideally unique and should be user readable. Localization doesn't matter at
/// this time.
#[derive(Clone, PartialEq, Message)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NetErr {
    #[prost(enumeration="ErrCode", required, tag="1")]
    pub code: i32,
    #[prost(string, required, tag="2")]
    pub msg: String,
}
/// Error codes mapping to a high level failure reason for a `NetErr`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Enumeration)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrCode {
    Internal = 0,
    Io = 1,
    NotFound = 2,
    Conflict = 3,
    Unauthorized = 4,
    /// Requestor supplied a valid payload but a detail of the request was not supported by the
    /// remote. The requestor should not make the same request again.
    NotSupported = 5,
    /// Requestor supplied a bad or unreadable value for one or more fields of one or more messages
    /// for a request. The requestor should not make the same request again.
    BadPayload = 6,
    /// Requestor supplied a well-formed payload but it was rejected as invalid by the remote. The
    /// requestor should not make the same request again.
    InvalidPayload = 7,
    /// Requestor sent a well-formed payload but it exceeded an allowed limit.
    EntityTooLarge = 8,
    /// Requestor sent a message which the server cannot process. The requestor should update their
    /// client before making the same request again.
    UpdateClient = 9,
}
