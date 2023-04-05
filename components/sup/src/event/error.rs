//! Event subsystem-specific error handling

use rants::{error::Error as RantsError,
            rustls};
use std::{error,
          fmt,
          result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectNatsServer,
    HabitatCore(habitat_core::Error),
    RustTls(rustls::Error),
    Rants(RantsError),
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
            Error::ConnectNatsServer => "Could not establish connection to NATS server".fmt(f),
            Error::HabitatCore(_) => "{}".fmt(f),
            Error::RustTls(e) => format!("{}", e).fmt(f),
            Error::Rants(e) => format!("{}", e).fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConnectNatsServer => None,
            Error::HabitatCore(ref e) => Some(e),
            Error::Rants(ref e) => Some(e),
            Error::RustTls(ref e) => Some(e),
        }
    }
}

impl From<habitat_core::Error> for Error {
    fn from(error: habitat_core::Error) -> Self { Error::HabitatCore(error) }
}

impl From<RantsError> for Error {
    fn from(error: RantsError) -> Self { Error::Rants(error) }
}

impl From<rustls::Error> for Error {
    fn from(error: rustls::Error) -> Self { Error::RustTls(error) }
}
