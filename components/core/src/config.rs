// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std;
use std::fs::File;
use std::io::Read;
use std::net;
use std::path::Path;
use std::result;
use std::str::FromStr;

use toml;

use error::{Error, Result};

pub trait ConfigFile: Sized {
    type Error: std::error::Error + From<Error>;

    fn from_file<T: AsRef<Path>>(filepath: T) -> result::Result<Self, Self::Error> {
        let mut file = match File::open(filepath.as_ref()) {
            Ok(f) => f,
            Err(e) => return Err(Self::Error::from(Error::ConfigFileIO(e))),
        };
        let mut raw = String::new();
        match file.read_to_string(&mut raw) {
            Ok(_) => (),
            Err(e) => return Err(Self::Error::from(Error::ConfigFileIO(e))),
        }
        let mut parser = toml::Parser::new(&raw);
        match parser.parse() {
            Some(toml) => Self::from_toml(toml),
            None => {
                let msg = format_errors(&parser);
                Err(Self::Error::from(Error::ConfigFileSyntax(msg)))
            }
        }
    }

    fn from_toml(toml: toml::Table) -> result::Result<Self, Self::Error>;
}

pub trait ParseInto<T> {
    fn parse_into(&self, field: &'static str, out: &mut T) -> Result<bool>;
}

impl ParseInto<net::SocketAddrV4> for toml::Table {
    fn parse_into(&self, field: &'static str, out: &mut net::SocketAddrV4) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_str() {
                match net::SocketAddrV4::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::ConfigInvalidSocketAddrV4(field, val.clone())),
                }
            } else {
                Err(Error::ConfigInvalidSocketAddrV4(field, val.clone()))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<net::Ipv4Addr> for toml::Table {
    fn parse_into(&self, field: &'static str, out: &mut net::Ipv4Addr) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_str() {
                match net::Ipv4Addr::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::ConfigInvalidIpv4Addr(field, val.clone())),
                }
            } else {
                Err(Error::ConfigInvalidIpv4Addr(field, val.clone()))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<String> for toml::Table {
    fn parse_into(&self, field: &'static str, out: &mut String) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_str() {
                *out = v.to_string();
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field, val.clone()))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<usize> for toml::Table {
    fn parse_into(&self, field: &'static str, out: &mut usize) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_integer() {
                *out = v as usize;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field, val.clone()))
            }
        } else {
            Ok(false)
        }
    }
}

fn format_errors(parser: &toml::Parser) -> String {
    let mut msg = String::new();
    for err in &parser.errors {
        let (loline, locol) = parser.to_linecol(err.lo);
        let (hiline, hicol) = parser.to_linecol(err.hi);
        msg.push_str(&format!("\t{}:{}-{}:{} error: {}\n",
                              loline,
                              locol,
                              hiline,
                              hicol,
                              err.desc));
    }
    msg
}
