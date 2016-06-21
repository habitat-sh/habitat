// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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
        match raw.parse() {
            Ok(toml) => Self::from_toml(toml),
            Err(e) => {
                let msg = format_errors(&e);
                Err(Self::Error::from(Error::ConfigFileSyntax(msg)))
            }
        }
    }

    fn from_toml(toml: toml::Value) -> result::Result<Self, Self::Error>;
}

pub trait ParseInto<T> {
    fn parse_into(&self, field: &'static str, out: &mut T) -> Result<bool>;
}

impl ParseInto<Vec<net::SocketAddrV4>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<net::SocketAddrV4>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(slice) = val.as_slice() {
                let mut buf = vec![];
                for entry in slice.iter() {
                    if let Some(v) = entry.as_str() {
                        match net::SocketAddrV4::from_str(v) {
                            Ok(addr) => buf.push(addr),
                            Err(_) => return Err(Error::ConfigInvalidSocketAddrV4(field)),
                        }
                    } else {
                        return Err(Error::ConfigInvalidSocketAddrV4(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                // error, expected array
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<net::SocketAddrV4> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut net::SocketAddrV4) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                match net::SocketAddrV4::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::ConfigInvalidSocketAddrV4(field)),
                }
            } else {
                Err(Error::ConfigInvalidSocketAddrV4(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<net::Ipv4Addr> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut net::Ipv4Addr) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                match net::Ipv4Addr::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::ConfigInvalidIpv4Addr(field)),
                }
            } else {
                Err(Error::ConfigInvalidIpv4Addr(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<String> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut String) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                *out = v.to_string();
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Option<String>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Option<String>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_str() {
                *out = Some(v.to_string());
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field))
            }
        } else {
            *out = None;
            Ok(true)
        }
    }
}

impl ParseInto<usize> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut usize) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as usize;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u16> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u16) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u16;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u32> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u32) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u32;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u64> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u64) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u64;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidString(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u16>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u16>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u16);
                    } else {
                        return Err(Error::ConfigInvalidArray(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u32>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u32>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u32);
                    } else {
                        return Err(Error::ConfigInvalidArray(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u64>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u64>) -> Result<bool> {
        if let Some(val) = self.lookup(field) {
            if let Some(v) = val.as_slice() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u64);
                    } else {
                        return Err(Error::ConfigInvalidArray(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArray(field))
            }
        } else {
            Ok(false)
        }
    }
}

fn format_errors(errors: &Vec<toml::ParserError>) -> String {
    let mut msg = String::new();
    for err in errors {
        msg.push_str(&format!("\terror: {}\n", err.desc));
    }
    msg
}
