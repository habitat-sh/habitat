//! Event subsystem-specific error handling

use habitat_http_client;
use rants::{error::Error as RantsError,
            native_tls};
use std::{error,
          fmt,
          result};
use tokio::time::Elapsed;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    HabitatHttpClient(habitat_http_client::Error),
    NativeTls(native_tls::Error),
    Rants(RantsError),
    Timeout(Elapsed),
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
            Error::HabitatHttpClient(_) => "{}".fmt(f),
            Error::NativeTls(e) => format!("{}", e).fmt(f),
            Error::Rants(e) => format!("{}", e).fmt(f),
            Error::Timeout(e) => format!("{}", e).fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::HabitatHttpClient(ref e) => Some(e),
            Error::Rants(ref e) => Some(e),
            Error::NativeTls(ref e) => Some(e),
            Error::Timeout(e) => Some(e),
        }
    }
}

impl From<habitat_http_client::Error> for Error {
    fn from(error: habitat_http_client::Error) -> Self { Error::HabitatHttpClient(error) }
}

impl From<RantsError> for Error {
    fn from(error: RantsError) -> Self { Error::Rants(error) }
}

impl From<Elapsed> for Error {
    fn from(error: Elapsed) -> Self { Error::Timeout(error) }
}

impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Self { Error::NativeTls(error) }
}
