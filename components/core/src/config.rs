// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
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
use std::collections::BTreeMap;
use std::error::Error as StdError;
use std::fs::File;
use std::io::Read;
use std::net::{IpAddr, SocketAddr};
use std::path::Path;
use std::result;
use std::str::FromStr;

use toml;

use error::{Error, Result};
use package::{PackageIdent, PackageTarget};

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
                let msg = format!("\terror: {}\n", e.description());
                Err(Self::Error::from(Error::ConfigFileSyntax(msg)))
            }
        }
    }

    fn from_toml(toml: toml::Value) -> result::Result<Self, Self::Error>;
}

pub trait ParseInto<T> {
    fn parse_into(&self, field: &'static str, out: &mut T) -> Result<bool>;
}

impl ParseInto<bool> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut bool) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_bool() {
                *out = v as bool;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidBool(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<usize> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut usize) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_integer() {
                *out = v as usize;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidUsize(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u16> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u16) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u16;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidU16(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u16>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u16>) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_array() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u16);
                    } else {
                        return Err(Error::ConfigInvalidArrayU16(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArrayU16(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u32> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u32) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u32;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidU32(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u32>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u32>) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_array() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u32);
                    } else {
                        return Err(Error::ConfigInvalidArrayU32(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArrayU32(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<u64> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut u64) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_integer() {
                *out = v as u64;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidU64(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<u64>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<u64>) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_array() {
                let mut buf = vec![];
                for int in v.iter() {
                    if let Some(i) = int.as_integer() {
                        buf.push(i as u64);
                    } else {
                        return Err(Error::ConfigInvalidArrayU64(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArrayU64(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<String> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut String) -> Result<bool> {
        if let Some(val) = self.get(field) {
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
        if let Some(val) = self.get(field) {
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

impl ParseInto<BTreeMap<String, String>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut BTreeMap<String, String>) -> Result<bool> {
        match self.get(field) {
            Some(val) => {
                match val.as_table() {
                    Some(val_table) => {
                        let buf: Result<BTreeMap<String, String>> = val_table.iter()
                            .map(|(k, v)| match v.as_str() {
                                Some(val_str) => Ok((k.to_string(), val_str.to_string())),
                                None => Err(Error::ConfigInvalidTableString(field)),
                            })
                            .collect();
                        *out = match buf {
                            Ok(b) => b,
                            Err(e) => return Err(e),
                        };
                        Ok(true)
                    }
                    None => Err(Error::ConfigInvalidTableString(field)),
                }
            }
            None => Ok(false),
        }
    }
}

impl ParseInto<Vec<BTreeMap<String, String>>> for toml::Value {
    fn parse_into(&self,
                  field: &'static str,
                  out: &mut Vec<BTreeMap<String, String>>)
                  -> Result<bool> {
        match self.get(field) {
            Some(val) => {
                match val.as_array() {
                    Some(val_slice) => {
                        let mut buf = vec![];
                        for i in val_slice.iter() {
                            match i.as_table() {
                                Some(val_table) => {
                                    let map: Result<BTreeMap<String, String>> = val_table.iter()
                                        .map(|(k, v)| match v.as_str() {
                                            Some(val_str) => {
                                                Ok((k.to_string(), val_str.to_string()))
                                            }
                                            None => {
                                                Err(Error::ConfigInvalidArrayTableString(field))
                                            }
                                        })
                                        .collect();
                                    match map {
                                        Ok(m) => buf.push(m),
                                        Err(e) => return Err(e),
                                    }
                                }
                                None => return Err(Error::ConfigInvalidArrayTableString(field)),
                            }
                        }
                        *out = buf;
                        Ok(true)
                    }
                    None => Err(Error::ConfigInvalidArrayTableString(field)),
                }
            }
            None => Ok(false),
        }
    }
}

impl ParseInto<SocketAddr> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut SocketAddr) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_str() {
                match SocketAddr::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::ConfigInvalidSocketAddr(field)),
                }
            } else {
                Err(Error::ConfigInvalidSocketAddr(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<Vec<SocketAddr>> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut Vec<SocketAddr>) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(slice) = val.as_array() {
                let mut buf = vec![];
                for entry in slice.iter() {
                    if let Some(v) = entry.as_str() {
                        match SocketAddr::from_str(v) {
                            Ok(addr) => buf.push(addr),
                            Err(_) => return Err(Error::ConfigInvalidArraySocketAddr(field)),
                        }
                    } else {
                        return Err(Error::ConfigInvalidArraySocketAddr(field));
                    }
                }
                *out = buf;
                Ok(true)
            } else {
                Err(Error::ConfigInvalidArraySocketAddr(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<IpAddr> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut IpAddr) -> Result<bool> {
        if let Some(val) = self.get(field) {
            if let Some(v) = val.as_str() {
                match IpAddr::from_str(v) {
                    Ok(addr) => {
                        *out = addr;
                        Ok(true)
                    }
                    Err(_) => Err(Error::ConfigInvalidIpAddr(field)),
                }
            } else {
                Err(Error::ConfigInvalidIpAddr(field))
            }
        } else {
            Ok(false)
        }
    }
}

impl ParseInto<PackageIdent> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut PackageIdent) -> Result<bool> {
        match self.get(field) {
            Some(val) => {
                match val.as_str() {
                    Some(val_str) => {
                        match PackageIdent::from_str(val_str) {
                            Ok(ident) => {
                                *out = ident;
                                Ok(true)
                            }
                            Err(_) => Err(Error::ConfigInvalidIdent(field)),
                        }
                    }
                    None => Err(Error::ConfigInvalidIdent(field)),
                }
            }
            None => Ok(false),
        }
    }
}

impl ParseInto<PackageTarget> for toml::Value {
    fn parse_into(&self, field: &'static str, out: &mut PackageTarget) -> Result<bool> {
        match self.get(field) {
            Some(val) => {
                match val.as_str() {
                    Some(val_str) => {
                        match PackageTarget::from_str(val_str) {
                            Ok(target) => {
                                *out = target;
                                Ok(true)
                            }
                            Err(_) => Err(Error::ConfigInvalidTarget(field)),
                        }
                    }
                    None => Err(Error::ConfigInvalidTarget(field)),
                }
            }
            None => Ok(false),
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;
    use std::net::{IpAddr, SocketAddr};
    use std::str::FromStr;

    use toml;

    use super::ParseInto;
    use error::Error::*;
    use package::{PackageIdent, PackageTarget};

    fn toml_from_str(content: &str) -> toml::Value {
        toml::from_str(content).expect(&format!("Content should parse as TOML: {}", content))
    }

    #[test]
    fn parse_into_bool() {
        let mut actual = false;
        let toml = toml_from_str(r#"
            field = true
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(true, actual);
    }

    #[test]
    fn parse_into_bool_field_missing() {
        let mut actual = false;
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_bool_invalid() {
        let mut actual = false;
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidBool(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_usize() {
        let mut actual = 0 as usize;
        let toml = toml_from_str(r#"
            field = 12
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(12, actual);
    }

    #[test]
    fn parse_into_usize_field_missing() {
        let mut actual = 0 as usize;
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_usize_invalid() {
        let mut actual = 0 as usize;
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidUsize(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_u16() {
        let mut actual = 0 as u16;
        let toml = toml_from_str(r#"
            field = 12
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(12, actual);
    }

    #[test]
    fn parse_into_u16_field_missing() {
        let mut actual = 0 as u16;
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_u16_invalid() {
        let mut actual = 0 as u16;
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidU16(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_u16() {
        let mut actual: Vec<u16> = vec![];
        let toml = toml_from_str(r#"
            field = [12, 24]
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(vec![12, 24], actual);
    }

    #[test]
    fn parse_into_vec_u16_field_missing() {
        let mut actual: Vec<u16> = vec![];
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_vec_u16_invalid() {
        let mut actual: Vec<u16> = vec![];
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayU16(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_u16_invalid_entry() {
        let mut actual: Vec<u16> = vec![];
        let toml = toml_from_str(r#"
            field = ["nope"]
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayU16(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_u32() {
        let mut actual = 0 as u32;
        let toml = toml_from_str(r#"
            field = 12
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(12, actual);
    }

    #[test]
    fn parse_into_u32_field_missing() {
        let mut actual = 0 as u32;
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_u32_invalid() {
        let mut actual = 0 as u32;
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidU32(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_u32() {
        let mut actual: Vec<u32> = vec![];
        let toml = toml_from_str(r#"
            field = [12, 24]
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(vec![12, 24], actual);
    }

    #[test]
    fn parse_into_vec_u32_field_missing() {
        let mut actual: Vec<u32> = vec![];
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_vec_u32_invalid() {
        let mut actual: Vec<u32> = vec![];
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayU32(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_u32_invalid_entry() {
        let mut actual: Vec<u32> = vec![];
        let toml = toml_from_str(r#"
            field = ["nope"]
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayU32(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_u64() {
        let mut actual = 0 as u64;
        let toml = toml_from_str(r#"
            field = 12
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(12, actual);
    }

    #[test]
    fn parse_into_u64_field_missing() {
        let mut actual = 0 as u64;
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_u64_invalid() {
        let mut actual = 0 as u64;
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidU64(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_u64() {
        let mut actual: Vec<u64> = vec![];
        let toml = toml_from_str(r#"
            field = [12, 24]
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(vec![12, 24], actual);
    }

    #[test]
    fn parse_into_vec_u64_field_missing() {
        let mut actual: Vec<u64> = vec![];
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_vec_u64_invalid() {
        let mut actual: Vec<u64> = vec![];
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayU64(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_u64_invalid_entry() {
        let mut actual: Vec<u64> = vec![];
        let toml = toml_from_str(r#"
            field = ["nope"]
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayU64(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_string() {
        let mut actual = String::new();
        let toml = toml_from_str(r#"
            field = "val"
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(String::from("val"), actual);
    }

    #[test]
    fn parse_into_string_field_missing() {
        let mut actual = String::new();
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_string_invalid() {
        let mut actual = String::new();
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_option_string() {
        let mut actual = None;
        let toml = toml_from_str(r#"
            field = "val"
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(Some(String::from("val")), actual);
    }

    #[test]
    fn parse_into_option_string_field_missing() {
        let mut actual: Option<String> = None;
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(None, actual);
    }

    #[test]
    fn parse_into_option_string_invalid() {
        let mut actual: Option<String> = None;
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_btreemap_strings() {
        let mut actual: BTreeMap<String, String> = BTreeMap::new();
        let toml = toml_from_str(r#"
            [field]
            alpha = "beta"
            charlie = "delta"
            "#);
        let mut expected = BTreeMap::new();
        expected.insert(String::from("alpha"), String::from("beta"));
        expected.insert(String::from("charlie"), String::from("delta"));
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_into_btreemap_strings_field_missing() {
        let mut actual: BTreeMap<String, String> = BTreeMap::new();
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_btreemap_strings_invalid() {
        let mut actual: BTreeMap<String, String> = BTreeMap::new();
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidTableString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_btreemap_strings_invalid_entry() {
        let mut actual: BTreeMap<String, String> = BTreeMap::new();
        let toml = toml_from_str(r#"
            [field]
            alpha = false
            charlie = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidTableString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_btreemap_strings() {
        let mut actual: Vec<BTreeMap<String, String>> = vec![];
        let toml = toml_from_str(r#"
            [[field]]
            alpha = "beta"

            [[field]]
            charlie = "delta"
            "#);
        let expected = {
            let mut v = Vec::new();
            let mut entry = BTreeMap::new();
            entry.insert(String::from("alpha"), String::from("beta"));
            v.push(entry);
            let mut entry = BTreeMap::new();
            entry.insert(String::from("charlie"), String::from("delta"));
            v.push(entry);
            v
        };
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_into_vec_btreemap_strings_field_missing() {
        let mut actual: Vec<BTreeMap<String, String>> = vec![];
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_vec_btreemap_strings_invalid() {
        let mut actual: Vec<BTreeMap<String, String>> = vec![];
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayTableString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_btreemap_strings_invalid_entry() {
        let mut actual: Vec<BTreeMap<String, String>> = vec![];
        let toml = toml_from_str(r#"
            field = ["nope"]
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayTableString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_btreemap_strings_invalid_table_entry() {
        let mut actual: Vec<BTreeMap<String, String>> = vec![];
        let toml = toml_from_str(r#"
            [[field]]
            alpha = false

            [[field]]
            charlie = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArrayTableString(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_socket_addr() {
        let mut actual = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let toml = toml_from_str(r#"
            field = "192.168.0.100:80"
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(SocketAddr::from_str("192.168.0.100:80").unwrap(), actual);
    }

    #[test]
    fn parse_into_socket_addr_field_missing() {
        let mut actual = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_socket_addr_invalid_string() {
        let mut actual = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidSocketAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_socket_addr_invalid_socket() {
        let mut actual = SocketAddr::from_str("127.0.0.1:8080").unwrap();
        let toml = toml_from_str(r#"
            field = "127.1"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidSocketAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_socket_addr() {
        let mut actual: Vec<SocketAddr> = vec![];
        let toml = toml_from_str(r#"
            field = ["192.168.0.100:80", "10.0.0.4:22"]
            "#);
        let expected = vec![SocketAddr::from_str("192.168.0.100:80").unwrap(),
                            SocketAddr::from_str("10.0.0.4:22").unwrap()];
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_into_vec_socket_addr_field_missing() {
        let mut actual: Vec<SocketAddr> = vec![];
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_vec_socket_addr_invalid() {
        let mut actual: Vec<SocketAddr> = vec![];
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArraySocketAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_socket_addr_invalid_entry() {
        let mut actual: Vec<SocketAddr> = vec![];
        let toml = toml_from_str(r#"
            field = [false]
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArraySocketAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_vec_socket_addr_invalid_socket_addr() {
        let mut actual: Vec<SocketAddr> = vec![];
        let toml = toml_from_str(r#"
            field = ["127.1"]
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidArraySocketAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_ip_addr() {
        let mut actual = IpAddr::from_str("127.0.0.1").unwrap();
        let toml = toml_from_str(r#"
            field = "192.168.0.100"
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(IpAddr::from_str("192.168.0.100").unwrap(), actual);
    }

    #[test]
    fn parse_into_ip_addr_field_missing() {
        let mut actual = IpAddr::from_str("127.0.0.1").unwrap();
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_ip_addr_invalid_string() {
        let mut actual = IpAddr::from_str("127.0.0.1").unwrap();
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidIpAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_ip_addr_invalid_addr() {
        let mut actual = IpAddr::from_str("127.0.0.1").unwrap();
        let toml = toml_from_str(r#"
            field = "127.1"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidIpAddr(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_package_ident() {
        let mut actual = PackageIdent::from_str("just/nothing").unwrap();
        let toml = toml_from_str(r#"
            field = "origin/name/1.2.3"
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(PackageIdent::from_str("origin/name/1.2.3").unwrap(), actual);
    }

    #[test]
    fn parse_into_package_ident_field_missing() {
        let mut actual = PackageIdent::from_str("just/nothing").unwrap();
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_package_ident_invalid_string() {
        let mut actual = PackageIdent::from_str("just/nothing").unwrap();
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidIdent(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_package_ident_invalid_ident() {
        let mut actual = PackageIdent::from_str("just/nothing").unwrap();
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidIdent(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_package_target() {
        let mut actual = PackageTarget::default();
        let toml = toml_from_str(r#"
            field = "x86_64-linux"
            "#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(mutated);
        assert_eq!(PackageTarget::from_str("x86_64-linux").unwrap(), actual);
    }

    #[test]
    fn parse_into_package_target_field_missing() {
        let mut actual = PackageTarget::default();
        let toml = toml_from_str(r#""#);
        let mutated = toml.parse_into("field", &mut actual).unwrap();

        assert!(!mutated);
    }

    #[test]
    fn parse_into_package_target_invalid_string() {
        let mut actual = PackageTarget::default();
        let toml = toml_from_str(r#"
            field = false
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidTarget(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }

    #[test]
    fn parse_into_package_target_invalid_target() {
        let mut actual = PackageTarget::default();
        let toml = toml_from_str(r#"
            field = "nope"
            "#);

        match toml.parse_into("field", &mut actual) {
            Err(ConfigInvalidTarget(field)) => assert_eq!("field", field),
            Err(e) => panic!("Unexpected error returned: {}", e),
            Ok(_) => panic!("Value should fail to parse"),
        }
    }
}
