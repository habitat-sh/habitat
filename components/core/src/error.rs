// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::error;
use std::io;
use std::fmt;
use std::num;
use std::result;
use std::string;

use gpgme;
use libarchive;
use regex;

use package;

pub type Result<T> = result::Result<T, Error>;

/// Core error types
#[derive(Debug)]
pub enum Error {
    /// Occurs when a `habitat_core::package::PackageArchive` is being read.
    ArchiveError(libarchive::error::ArchiveError),
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    /// When an error occurs in GpgME library calls.
    GPG(gpgme::Error),
    /// Occurs when a required GPG key is not found.
    InvalidKeyParameter(String),
    /// Occurs when a package identifier string cannot be successfully parsed.
    InvalidPackageIdent(String),
    /// Occurs when making lower level IO calls.
    IO(io::Error),
    /// Occurs when a package metadata file cannot be opened, read, or parsed.
    MetaFileMalformed(package::MetaFile),
    /// Occurs when a particular package metadata file is not found.
    MetaFileNotFound(package::MetaFile),
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
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::GPG(ref e) => format!("{}", e),
            Error::InvalidKeyParameter(ref e) => {
                format!("Invalid parameter for key generation: {:?}", e)
            }
            Error::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: chef/redis)",
                        e)
            }
            Error::IO(ref err) => format!("{}", err),
            Error::MetaFileNotFound(ref e) => format!("Couldn't read MetaFile: {}, not found", e),
            Error::MetaFileMalformed(ref e) => {
                format!("MetaFile: {:?}, didn't contain a valid UTF-8 string", e)
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
            Error::FileNotFound(_) => "File not found",
            Error::GPG(_) => "gpgme error",
            Error::InvalidKeyParameter(_) => "Key parameter error",
            Error::InvalidPackageIdent(_) => "Package identifiers must be in origin/name format (example: chef/redis)",
            Error::IO(ref err) => err.description(),
            Error::MetaFileNotFound(_) => "Failed to read an archive's metafile",
            Error::MetaFileMalformed(_) => "MetaFile didn't contain a valid UTF-8 string",
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

impl From<gpgme::Error> for Error {
    fn from(err: gpgme::Error) -> Error {
        Error::GPG(err)
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
