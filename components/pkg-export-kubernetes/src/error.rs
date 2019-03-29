
use crate::hcore;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Invalid bind specification '{}'", _0)]
    InvalidBindSpec(String),
    #[fail(display = "Invalid topology '{}'. Possible values: standalone, leader",
           _0)]
    InvalidTopology(String),
    #[fail(display = "Invalid binding \"{}\", must be of the form <NAME>:<SERVICE_GROUP> where \
                      <NAME> is a service name and <SERVICE_GROUP> is a valid service group",
           _0)]
    InvalidBinding(String),
    #[fail(display = "Invalid environment variable \"{}\", must be in the form <NAME>=<VALUE>",
           _0)]
    InvalidEnvironmentVariable(String),
    #[fail(display = "Invalid persistent storage specification \"{}\", must in the form \
                      <SIZE>:<PATH>:<STORAGE_CLASS_NAME> where <SIZE> is a size in bytes (with \
                      E, P, T, G, M, K or Ei, Pi, Ti, Gi, Mi, Ki suffixes, or in 123e5 form, \
                      see K8s docs for more details), <PATH> is an absolute path",
           _0)]
    InvalidPersistentStorageSpec(String),
    #[fail(display = "{}", _0)]
    HabitatCore(hcore::Error),
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error { Error::HabitatCore(err) }
}
