// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::error;
use std::io;
use std::fmt;
use std::num;
use std::result;
use std::string;

use libarchive;
use regex;

use package;

pub type Result<T> = result::Result<T, Error>;

/// Core error types
#[derive(Debug)]
pub enum Error {
    /// Occurs when a `habitat_core::package::PackageArchive` is being read.
    ArchiveError(libarchive::error::ArchiveError),
    /// An invalid path to a keyfile was given.
    BadKeyPath(String),
    /// Crypto library error
    CryptoError(String),
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    /// Occurs when a package identifier string cannot be successfully parsed.
    InvalidPackageIdent(String),
    /// Occurs when a service group string cannot be successfully parsed.
    InvalidServiceGroup(String),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    /// Occurs when a package metadata file cannot be opened, read, or parsed.
    MetaFileMalformed(package::MetaFile),
    /// Occurs when a particular package metadata file is not found.
    MetaFileNotFound(package::MetaFile),
    /// When an IO error while accessing a MetaFile.
    MetaFileIO(io::Error),
    /// Occurs when a suitable installed pacakge cannot be found.
    PackageNotFound(package::PackageIdent),
    /// When an error occurs parsing an integer.
    ParseIntError(num::ParseIntError),
    /// Occurs when setting ownership or permissions on a file or directory fails.
    PermissionFailed,
    /// When an error occurs parsing or compiling a regular expression.
    RegexParse(regex::Error),
    /// When an error occurs converting a `String` from a UTF-8 byte vector.
    StringFromUtf8Error(string::FromUtf8Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ArchiveError(ref err) => format!("{}", err),
            Error::BadKeyPath(ref e) => {
                format!("Invalid keypath: {}. Specify an absolute path to a file on disk.",
                        e)
            }
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: acme/redis)",
                        e)
            }
            Error::InvalidServiceGroup(ref e) => {
                format!("Invalid service group: {:?}. A valid service group string is in the form \
                         service.group (example: redis.production)",
                        e)
            }
            Error::IO(ref err) => format!("{}", err),
            Error::MetaFileMalformed(ref e) => {
                format!("MetaFile: {:?}, didn't contain a valid UTF-8 string", e)
            }
            Error::MetaFileNotFound(ref e) => format!("Couldn't read MetaFile: {}, not found", e),
            Error::MetaFileIO(ref e) => format!("IO error while accessing MetaFile: {:?}", e),
            Error::PackageNotFound(ref pkg) => {
                if pkg.fully_qualified() {
                    format!("Cannot find package: {}", pkg)
                } else {
                    format!("Cannot find a release of package: {}", pkg)
                }
            }
            Error::ParseIntError(ref e) => format!("{}", e),
            Error::PermissionFailed => format!("Failed to set permissions"),
            Error::RegexParse(ref e) => format!("{}", e),
            Error::StringFromUtf8Error(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArchiveError(ref err) => err.description(),
            Error::BadKeyPath(_) => "An absolute path to a file on disk is required",
            Error::CryptoError(_) => "Crypto error",
            Error::FileNotFound(_) => "File not found",
            Error::InvalidPackageIdent(_) => "Package identifiers must be in origin/name format (example: acme/redis)",
            Error::InvalidServiceGroup(_) => "Service group strings must be in service.group format (example: redis.production)",
            Error::IO(ref err) => err.description(),
            Error::MetaFileMalformed(_) => "MetaFile didn't contain a valid UTF-8 string",
            Error::MetaFileNotFound(_) => "Failed to read an archive's metafile",
            Error::MetaFileIO(_) => "MetaFile could not be read or written to",
            Error::PackageNotFound(_) => "Cannot find a package",
            Error::ParseIntError(_) => "Failed to parse an integer from a string!",
            Error::PermissionFailed => "Failed to set permissions",
            Error::RegexParse(_) => "Failed to parse a regular expression",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::StringFromUtf8Error(err)
    }
}


impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl From<libarchive::error::ArchiveError> for Error {
    fn from(err: libarchive::error::ArchiveError) -> Error {
        Error::ArchiveError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::ParseIntError(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Error {
        Error::RegexParse(err)
    }
}
