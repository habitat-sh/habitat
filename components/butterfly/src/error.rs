

use std::{error,
          fmt,
          io,
          num,
          path::PathBuf,
          result,
          str};

use habitat_core;
use prost;
use toml;
use zmq;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BadDataPath(PathBuf, io::Error),
    BadDatFile(PathBuf, io::Error),
    CannotBind(io::Error),
    DatFileIO(PathBuf, io::Error),
    DecodeError(prost::DecodeError),
    EncodeError(prost::EncodeError),
    HabitatCore(habitat_core::error::Error),
    IncarnationIO(PathBuf, io::Error),
    IncarnationParse(PathBuf, num::ParseIntError),
    InvalidIncarnationSynchronization,
    InvalidRumorShareLimit,
    NonExistentRumor(String, String),
    ProtocolMismatch(&'static str),
    ServiceConfigDecode(String, toml::de::Error),
    ServiceConfigNotUtf8(String, str::Utf8Error),
    SocketCloneError,
    SocketSetReadTimeout(io::Error),
    SocketSetWriteTimeout(io::Error),
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
            Error::BadDatFile(ref path, ref err) => {
                format!("Unable to decode contents of DatFile, {}, {}",
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
            Error::InvalidIncarnationSynchronization => "Tried to synchronize own member \
                                                         incarnation from non-existent \
                                                         incarnation store"
                                                                           .to_string(),
            Error::InvalidRumorShareLimit => {
                "Rumor share limit should be a positive integer".to_string()
            }
            Error::NonExistentRumor(ref member_id, ref rumor_id) => {
                format!("Non existent rumor asked to be written to bytes: {} {}",
                        member_id, rumor_id)
            }
            Error::ProtocolMismatch(ref field) => {
                format!("Received an unsupported or bad protocol message. Missing field: {}",
                        field)
            }
            Error::ServiceConfigDecode(ref sg, ref err) => {
                format!("Cannot decode service config: group={}, {:?}", sg, err)
            }
            Error::ServiceConfigNotUtf8(ref sg, ref err) => {
                format!("Cannot read service configuration: group={}, {}", sg, err)
            }
            Error::SocketCloneError => "Cannot clone the underlying UDP socket".to_string(),
            Error::SocketSetReadTimeout(ref err) => {
                format!("Cannot set UDP socket read timeout: {}", err)
            }
            Error::SocketSetWriteTimeout(ref err) => {
                format!("Cannot set UDP socket write timeout: {}", err)
            }
            Error::ZmqConnectError(ref err) => format!("Cannot connect ZMQ socket: {}", err),
            Error::ZmqSendError(ref err) => {
                format!("Cannot send message through ZMQ socket: {}", err)
            }
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::BadDataPath(..) => "Unable to read or write to data directory",
            Error::BadDatFile(..) => "Unable to decode contents of DatFile",
            Error::CannotBind(_) => "Cannot bind to port",
            Error::DatFileIO(..) => "Error reading or writing to DatFile",
            Error::UnknownIOError(_) => "Unknown I/O error",
            Error::DecodeError(ref err) => err.description(),
            Error::EncodeError(ref err) => err.description(),
            Error::HabitatCore(_) => "Habitat core error",
            Error::IncarnationIO(..) => "Error reading or writing incarnation store file",
            Error::IncarnationParse(..) => "Error parsing value from incarnation store file",
            Error::InvalidIncarnationSynchronization => {
                "Tried to synchronize own member incarnation from non-existent incarnation store"
            }
            Error::InvalidRumorShareLimit => "Invalid rumor share limit",
            Error::NonExistentRumor(..) => "Cannot write rumor to bytes because it does not exist",
            Error::ProtocolMismatch(_) => {
                "Received an unprocessable wire message from another Supervisor"
            }
            Error::ServiceConfigDecode(..) => "Cannot decode service config into TOML",
            Error::ServiceConfigNotUtf8(..) => "Cannot read service config bytes to UTF-8",
            Error::SocketCloneError => "Cannot clone the underlying UDP socket",
            Error::SocketSetReadTimeout(_) => "Cannot set UDP socket read timeout",
            Error::SocketSetWriteTimeout(_) => "Cannot set UDP socket write timeout",
            Error::ZmqConnectError(_) => "Cannot connect ZMQ socket",
            Error::ZmqSendError(_) => "Cannot send message through ZMQ socket",
        }
    }
}

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
