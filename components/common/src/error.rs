// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::error;
use std::io;
use std::fmt;
use std::result;

use depot_client;
use hcore;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    ConfigFileRelativePath(String),
    DepotClient(depot_client::Error),
    FileNameError,
    HabitatCore(hcore::Error),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ConfigFileRelativePath(ref s) => {
                format!("Path for configuration file cannot have relative components (eg: ..): {}",
                        s)
            }
            Error::DepotClient(ref err) => format!("{}", err),
            Error::FileNameError => format!("Failed to extract a filename"),
            Error::HabitatCore(ref e) => format!("{}", e),
            Error::IO(ref err) => format!("{}", err),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConfigFileRelativePath(_) => "Path for configuration file cannot have relative components (eg: ..)",
            Error::DepotClient(ref err) => err.description(),
            Error::FileNameError => "Failed to extract a filename from a path",
            Error::IO(ref err) => err.description(),
            Error::HabitatCore(ref err) => err.description(),
        }
    }
}

impl From<depot_client::Error> for Error {
    fn from(err: depot_client::Error) -> Error {
        Error::DepotClient(err)
    }
}

impl From<hcore::Error> for Error {
    fn from(err: hcore::Error) -> Error {
        Error::HabitatCore(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}
