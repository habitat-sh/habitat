//! Event subsystem-specific error handling

use habitat_http_client;
use nats::{native_tls,
           NatsError};
use std::{error,
          fmt,
          io,
          result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConnectEventServer,
    HabitatHttpClient(habitat_http_client::Error),
    Io(io::Error),
    NativeTls(native_tls::Error),
    Nats(NatsError),
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
            Error::ConnectEventServer => {
                "Could not establish streaming connection to NATS server".fmt(f)
            }
            Error::HabitatHttpClient(_) => "{}".fmt(f),
            Error::Io(_) => "{}".fmt(f),
            Error::NativeTls(e) => format!("TLS error '{}'", e).fmt(f),
            Error::Nats(e) => format!("NATS event stream error '{}'", e).fmt(f),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::ConnectEventServer => None,
            Error::HabitatHttpClient(ref e) => Some(e),
            Error::Io(ref e) => Some(e),
            Error::Nats(ref e) => Some(e),
            Error::NativeTls(ref e) => Some(e),
        }
    }
}

impl From<habitat_http_client::Error> for Error {
    fn from(error: habitat_http_client::Error) -> Self { Error::HabitatHttpClient(error) }
}

impl From<NatsError> for Error {
    fn from(error: NatsError) -> Self { Error::Nats(error) }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self { Error::Io(error) }
}

impl From<native_tls::Error> for Error {
    fn from(error: native_tls::Error) -> Self { Error::NativeTls(error) }
}
