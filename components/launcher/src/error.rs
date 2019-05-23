use crate::protocol;
use ipc_channel;
use std::{error,
          fmt,
          io,
          result};

use crate::{SUP_CMD,
            SUP_PACKAGE_IDENT};

#[derive(Debug)]
pub enum Error {
    AcceptConn,
    Connect(io::Error),
    ExecWait(io::Error),
    GroupNotFound(String),
    OpenPipe(io::Error),
    Protocol(protocol::Error),
    Send(ipc_channel::Error),
    Spawn(io::Error),
    SupBinaryVersion,
    SupBinaryNotFound,
    SupPackageNotFound,
    SupShutdown,
    SupSpawn(io::Error),
    UserNotFound(String),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::AcceptConn => "Unable to accept connection from Supervisor".to_string(),
            Error::Connect(ref e) => {
                format!("Unable to connect to Supervisor's comm channel, {}", e)
            }
            Error::ExecWait(ref e) => format!("Error waiting on PID, {}", e),
            Error::GroupNotFound(ref e) => format!("No GID for group '{}' could be found", e),
            Error::OpenPipe(ref e) => format!("Unable to open Launcher's comm channel, {}", e),
            Error::Protocol(ref e) => format!("{}", e),
            Error::Send(ref e) => format!("Unable to send to Launcher's comm channel, {}", e),
            Error::Spawn(ref e) => format!("Unable to spawn process, {}", e),
            Error::SupBinaryVersion => "Unsupported Supervisor binary version".to_string(),
            Error::SupBinaryNotFound => {
                format!("Supervisor package didn't contain '{}' binary", SUP_CMD)
            }
            Error::SupPackageNotFound => {
                format!("Unable to locate Supervisor package, {}", SUP_PACKAGE_IDENT)
            }
            Error::SupShutdown => "Error waiting for Supervisor to shutdown".to_string(),
            Error::SupSpawn(ref e) => format!("Unable to spawn Supervisor, {}", e),
            Error::UserNotFound(ref e) => format!("No UID for user '{}' could be found", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {}

impl From<Error> for protocol::ErrCode {
    fn from(err: Error) -> protocol::ErrCode {
        match err {
            Error::ExecWait(_) => protocol::ErrCode::ExecWait,
            Error::GroupNotFound(_) => protocol::ErrCode::GroupNotFound,
            Error::UserNotFound(_) => protocol::ErrCode::UserNotFound,
            _ => protocol::ErrCode::Unknown,
        }
    }
}

impl From<protocol::Error> for Error {
    fn from(err: protocol::Error) -> Error { Error::Protocol(err) }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error { Error::Spawn(err) }
}
