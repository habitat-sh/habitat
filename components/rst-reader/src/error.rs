use std::{error,
          fmt,
          result};

#[derive(Debug)]
pub enum Error {
    Butterfly(habitat_butterfly::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match *self {
            Error::Butterfly(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Butterfly(_) => "Error reading RST file",
        }
    }
}

impl From<habitat_butterfly::error::Error> for Error {
    fn from(err: habitat_butterfly::error::Error) -> Error { Error::Butterfly(err) }
}
