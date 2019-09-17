//! Messages for signaling or controlling networked applications. All types defined in this
//! module are building blocks for sockets speaking SrvProtocol.
//!
//! Note: See `protocols/net.proto` for type level documentation for generated types.

include!("generated/sup.net.rs");
include!("generated/sup.net.impl.rs");

use std::{error,
          fmt,
          io};

use crate::core;

pub type NetResult<T> = Result<T, NetErr>;

/// Helper function for quickly generating a `NetErr` from an `ErrCode` and message.
pub fn err<T>(code: ErrCode, msg: T) -> NetErr
    where T: fmt::Display
{
    NetErr { code: code as i32,
             msg:  msg.to_string(), }
}

/// Helper function for quickly generating a `NetOk` message.
pub fn ok() -> NetOk { NetOk::default() }

impl error::Error for NetErr {}

impl fmt::Display for NetErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Err: {}] {}", self.code, self.msg)
    }
}

impl From<io::Error> for NetErr {
    fn from(other: io::Error) -> Self { err(ErrCode::Io, other) }
}

impl From<core::Error> for NetErr {
    fn from(other: core::Error) -> Self { err(ErrCode::Internal, other) }
}
