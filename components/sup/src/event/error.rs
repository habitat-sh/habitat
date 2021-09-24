//! Event subsystem-specific error handling

use native_tls;
use std::{error,
          fmt,
          result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectNatsServer(std::io::Error),
    HabitatCore(habitat_core::Error),
    NativeTls(native_tls::Error),
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
            Error::ConnectNatsServer(_) => format!("Could not establish connection to NATS server").fmt(f),
            Error::HabitatCore(_) => "{}".fmt(f),
            Error::NativeTls(e) => format!("{}", e).fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConnectNatsServer(ref e) => Some(e),
            Error::HabitatCore(ref e) => Some(e),
            Error::NativeTls(ref e) => Some(e),
        }
    }
}

impl From<habitat_core::Error> for Error {
    fn from(error: habitat_core::Error) -> Self { Error::HabitatCore(error) }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self { Error::ConnectNatsServer(error) }
}

impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Self { Error::NativeTls(error) }
}
