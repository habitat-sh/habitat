//! Event subsystem-specific error handling

use nats::NatsError;
use std::{error,
          fmt,
          io,
          result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectEventServerError,
    NatsError(NatsError),
    SpawnEventThreadError(io::Error),
}

// TODO (CM): I would have like to have derived Fail on our Error
// type, thus getting rid of these Display and error::Error
// impls. However, until we can cleanly interoperate between Error and
// Fail's source/cause methods in the top-level Supervisor Error,
// we'll keep these for the time being.
//
// Perhaps if the Supervisor's Error became a Fail, we could do it?

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ConnectEventServerError => {
                "Could not establish streaming connection to NATS server".fmt(f)
            }
            Error::NatsError(e) => format!("NATS event stream error '{}'", e).fmt(f),
            Error::SpawnEventThreadError(_) => "Could not spawn eventing thread".fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConnectEventServerError => None,
            Error::NatsError(ref e) => Some(e),
            Error::SpawnEventThreadError(ref e) => Some(e),
        }
    }
}

impl From<NatsError> for Error {
    fn from(error: NatsError) -> Self { Error::NatsError(error) }
}
