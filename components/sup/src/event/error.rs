//! Event subsystem-specific error handling

use std::{error,
          fmt,
          io,
          result,
          sync::mpsc};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectEventServerError(mpsc::RecvTimeoutError),
    SpawnEventThreadError(io::Error),
}

// TODO (CM): I would have like to have derived Fail on our Error
// type, thus getting rid of these Display and error::Error
// impls. However, until we can cleanly interoperate between Error and
// Fail's source/cause methods in the top-level SupError, we'll keep
// these for the time being.
//
// Perhaps if SupError became a Fail, we could do it?

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConnectEventServerError(_) => {
                "Could not establish streaming connection to NATS server".fmt(f)
            }
            Error::SpawnEventThreadError(_) => "Could not spawn eventing thread".fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConnectEventServerError(ref e) => Some(e),
            Error::SpawnEventThreadError(ref e) => Some(e),
        }
    }
}
