use std::{error,
          fmt,
          io,
          num,
          path::PathBuf,
          result,
          str};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadDataPath(PathBuf, io::Error),
    CannotBind(io::Error),
    DatFileIO(PathBuf, io::Error),
    DecodeError(prost::DecodeError),
    EncodeError(prost::EncodeError),
    HabitatCore(habitat_core::error::Error),
    IncarnationIO(PathBuf, io::Error),
    IncarnationParse(PathBuf, num::ParseIntError),
    NonExistentRumor(String, String),
    OsError(io::Error),
    ProtocolMismatch(&'static str),
    ServiceConfigDecode(String, String),
    ServiceConfigNotUtf8(String, str::Utf8Error),
    SocketSetReadTimeout(io::Error),
    Timeout(String),
    UnknownMember(String),
    ZmqConnectError(zmq::Error),
    ZmqSendError(zmq::Error),
    UnknownIOError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::BadDataPath(ref path, ref err) => {
                format!("Unable to read or write to data directory, {}, {}",
                        path.display(),
                        err)
            }
            Error::CannotBind(ref err) => format!("Cannot bind to port: {:?}", err),
            Error::DatFileIO(ref path, ref err) => {
                format!("Error reading or writing to DatFile, {}, {}",
                        path.display(),
                        err)
            }
            Error::UnknownIOError(ref err) => format!("Error reading or writing: {}", err),
            Error::DecodeError(ref err) => format!("Failed to decode protocol message: {}", err),
            Error::EncodeError(ref err) => format!("Failed to encode protocol message: {}", err),
            Error::HabitatCore(ref err) => format!("{}", err),
            Error::IncarnationIO(ref path, ref err) => {
                format!("Error reading or writing incarnation store file {}: {}",
                        path.display(),
                        err)
            }
            Error::IncarnationParse(ref path, ref err) => {
                format!("Error parsing value from incarnation store file {}: {}",
                        path.display(),
                        err)
            }
            Error::NonExistentRumor(ref member_id, ref rumor_id) => {
                format!("Non existent rumor asked to be written to bytes: {} {}",
                        member_id, rumor_id)
            }
            Error::OsError(ref err) => format!("OS error: {}", err),
            Error::ProtocolMismatch(ref field) => {
                format!("Received an unsupported or bad protocol message. Missing field: {}",
                        field)
            }
            Error::ServiceConfigDecode(ref sg, ref err) => {
                format!("Cannot decode service config: group={}, {}", sg, err)
            }
            Error::ServiceConfigNotUtf8(ref sg, ref err) => {
                format!("Cannot read service configuration: group={}, {}", sg, err)
            }
            Error::SocketSetReadTimeout(ref err) => {
                format!("Cannot set UDP socket read timeout: {}", err)
            }
            Error::Timeout(ref msg) => format!("Timed out {}", msg),
            Error::UnknownMember(ref member_id) => format!("Unknown member ID: {}", member_id),
            Error::ZmqConnectError(ref err) => format!("Cannot connect ZMQ socket: {}", err),
            Error::ZmqSendError(ref err) => {
                format!("Cannot send message through ZMQ socket: {}", err)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<prost::DecodeError> for Error {
    fn from(err: prost::DecodeError) -> Error { Error::DecodeError(err) }
}

impl From<prost::EncodeError> for Error {
    fn from(err: prost::EncodeError) -> Error { Error::EncodeError(err) }
}
impl From<habitat_core::error::Error> for Error {
    fn from(err: habitat_core::error::Error) -> Error { Error::HabitatCore(err) }
}
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::UnknownIOError(err) }
}
