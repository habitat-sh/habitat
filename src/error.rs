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
use std::str;
use std::string;

use extern_url;
use libarchive;
use regex;

use package::{self, Identifiable};

pub type Result<T> = result::Result<T, Error>;

/// Core error types
#[derive(Debug)]
pub enum Error {
    /// Occurs when a `habitat_core::package::PackageArchive` is being read.
    ArchiveError(libarchive::error::ArchiveError),
    /// An invalid path to a keyfile was given.
    BadKeyPath(String),
    /// Error reading raw contents of configuration file.
    ConfigFileIO(io::Error),
    /// Parsing error while reading a configuratino file.
    ConfigFileSyntax(String),
    /// Expected a valid array of values for configuration field value.
    ConfigInvalidArray(&'static str),
    /// Expected a valid Ipv4 network address for configuration field value.
    ConfigInvalidIpv4Addr(&'static str),
    /// Expected a valid SocketAddrV4 address pair for configuration field value.
    ConfigInvalidSocketAddrV4(&'static str),
    /// Expected a string for configuration field value.
    ConfigInvalidString(&'static str),
    /// Crypto library error
    CryptoError(String),
    /// Occurs when a file that should exist does not or could not be read.
    FileNotFound(String),
    /// Occurs when a package identifier string cannot be successfully parsed.
    InvalidPackageIdent(String),
    /// Occurs when an improper http or https proxy value is given.
    InvalidProxyValue(String),
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
    /// Occurs when we can't find an outbound IP address
    NoOutboundAddr,
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
    /// Occurs when a `uname` libc call returns an error.
    UnameFailed(String),
    /// When an error occurs attempting to interpret a sequence of u8 as a string.
    Utf8Error(str::Utf8Error),
    /// When an error occurs attempting to parse a string into a URL.
    UrlParseError(extern_url::ParseError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::ArchiveError(ref err) => format!("{}", err),
            Error::BadKeyPath(ref e) => {
                format!("Invalid keypath: {}. Specify an absolute path to a file on disk.",
                        e)
            }
            Error::ConfigFileIO(ref e) => format!("Error reading configuration file: {}", e),
            Error::ConfigFileSyntax(ref e) => {
                format!("Syntax errors while parsing TOML configuration file:\n\n{}",
                        e)
            }
            Error::ConfigInvalidArray(ref f) => {
                format!("Invalid array of values in config, field={}", f)
            }
            Error::ConfigInvalidIpv4Addr(ref f) => {
                format!("Invalid Ipv4 address in config, field={}. (example: \"127.0.0.0\")",
                        f)
            }
            Error::ConfigInvalidSocketAddrV4(ref f) => {
                format!("Invalid Ipv4 network address pair in config, field={}. (example: \
                         \"127.0.0.0:8080\")",
                        f)
            }
            Error::ConfigInvalidString(ref f) => {
                format!("Invalid string value in config, field={}.", f)
            }
            Error::CryptoError(ref e) => format!("Crypto error: {}", e),
            Error::FileNotFound(ref e) => format!("File not found at: {}", e),
            Error::InvalidPackageIdent(ref e) => {
                format!("Invalid package identifier: {:?}. A valid identifier is in the form \
                         origin/name (example: acme/redis)",
                        e)
            }
            Error::InvalidProxyValue(ref e) => format!("Invalid proxy value: {:?}", e),
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
            Error::NoOutboundAddr => format!("Failed to discover this hosts outbound IP address"),
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
            Error::UnameFailed(ref e) => format!("{}", e),
            Error::Utf8Error(ref e) => format!("{}", e),
            Error::UrlParseError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ArchiveError(ref err) => err.description(),
            Error::BadKeyPath(_) => "An absolute path to a file on disk is required",
            Error::ConfigFileIO(_) => "Unable to read the raw contents of a configuration file",
            Error::ConfigFileSyntax(_) => "Error parsing contents of configuration file",
            Error::ConfigInvalidArray(_) => {
                "Invalid array of values encountered while parsing a configuration file"
            }
            Error::ConfigInvalidIpv4Addr(_) => {
                "Invalid Ipv4 network address encountered while parsing a configuration file"
            }
            Error::ConfigInvalidSocketAddrV4(_) => {
                "Invalid Ipv4 network address pair encountered while parsing a configuration file"
            }
            Error::ConfigInvalidString(_) => {
                "Invalid string value encountered while parsing a configuration file"
            }
            Error::CryptoError(_) => "Crypto error",
            Error::FileNotFound(_) => "File not found",
            Error::InvalidPackageIdent(_) => {
                "Package identifiers must be in origin/name format (example: acme/redis)"
            }
            Error::InvalidProxyValue(_) => "Invalid proxy value",
            Error::InvalidServiceGroup(_) => {
                "Service group strings must be in service.group format (example: redis.production)"
            }
            Error::IO(ref err) => err.description(),
            Error::MetaFileMalformed(_) => "MetaFile didn't contain a valid UTF-8 string",
            Error::MetaFileNotFound(_) => "Failed to read an archive's metafile",
            Error::MetaFileIO(_) => "MetaFile could not be read or written to",
            Error::NoOutboundAddr => "Failed to discover the outbound IP address",
            Error::PackageNotFound(_) => "Cannot find a package",
            Error::ParseIntError(_) => "Failed to parse an integer from a string!",
            Error::PermissionFailed => "Failed to set permissions",
            Error::RegexParse(_) => "Failed to parse a regular expression",
            Error::StringFromUtf8Error(_) => "Failed to convert a string from a Vec<u8> as UTF-8",
            Error::UnameFailed(_) => "uname failed",
            Error::Utf8Error(_) => "Failed to interpret a sequence of bytes as a string",
            Error::UrlParseError(ref err) => err.description(),
        }
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Self {
        Error::StringFromUtf8Error(err)
    }
}

impl From<str::Utf8Error> for Error {
    fn from(err: str::Utf8Error) -> Self {
        Error::Utf8Error(err)
    }
}

impl From<extern_url::ParseError> for Error {
    fn from(err: extern_url::ParseError) -> Self {
        Error::UrlParseError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<libarchive::error::ArchiveError> for Error {
    fn from(err: libarchive::error::ArchiveError) -> Self {
        Error::ArchiveError(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::RegexParse(err)
    }
}
