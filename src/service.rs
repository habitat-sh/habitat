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

use std::fmt;
use std::result;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

use regex::Regex;

use error::{Error, Result};

lazy_static! {
    static ref FROM_STR_RE: Regex =
        Regex::new(r"\A(?P<service>[^.]+)\.(?P<group>[^.@]+)(@(?P<organization>.+))?\z").unwrap();
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct ServiceGroupIdent(String);

impl ServiceGroupIdent {
    pub fn new<S1, S2>(service: S1, group: S2, organization: Option<&str>) -> Result<Self>
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        let formatted = Self::format(service, group, organization);
        try!(Self::validate(&formatted));
        Ok(ServiceGroupIdent(formatted))
    }

    fn format<S1, S2>(service: S1, group: S2, organization: Option<&str>) -> String
        where S1: AsRef<str>,
              S2: AsRef<str>
    {
        if let Some(org) = organization {
            format!("{}.{}@{}", service.as_ref(), group.as_ref(), org)
        } else {
            format!("{}.{}", service.as_ref(), group.as_ref())
        }
    }

    pub fn validate(value: &str) -> Result<()> {
        let caps = FROM_STR_RE
            .captures(value)
            .ok_or(Error::InvalidServiceGroupIdent(value.to_string()))?;
        if caps.name("service").is_none() {
            return Err(Error::InvalidServiceGroupIdent(value.to_string()));
        }
        if caps.name("group").is_none() {
            return Err(Error::InvalidServiceGroupIdent(value.to_string()));
        }
        Ok(())
    }

    pub fn service(&self) -> &str {
        FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("service")
            .unwrap()
            .as_str()
    }

    pub fn group(&self) -> &str {
        FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("group")
            .unwrap()
            .as_str()
    }

    pub fn org(&self) -> Option<&str> {
        FROM_STR_RE
            .captures(&self.0)
            .unwrap()
            .name("organization")
            .and_then(|v| Some(v.as_str()))
    }
}

impl AsRef<str> for ServiceGroupIdent {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ServiceGroupIdent {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl DerefMut for ServiceGroupIdent {
    fn deref_mut(&mut self) -> &mut String {
        &mut self.0
    }
}

impl fmt::Display for ServiceGroupIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ServiceGroupIdent {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let caps = match FROM_STR_RE.captures(value) {
            Some(c) => c,
            None => return Err(Error::InvalidServiceGroupIdent(value.to_string())),
        };
        let service = match caps.name("service") {
            Some(s) => s.as_str(),
            None => return Err(Error::InvalidServiceGroupIdent(value.to_string())),
        };
        let group = match caps.name("group") {
            Some(g) => g.as_str(),
            None => return Err(Error::InvalidServiceGroupIdent(value.to_string())),
        };
        let org = match caps.name("organization") {
            Some(o) => Some(o.as_str()),
            None => None,
        };
        Ok(ServiceGroupIdent(ServiceGroupIdent::format(service, group, org)))
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::ServiceGroupIdent;

    #[test]
    fn service_groups_with_org() {
        let x = ServiceGroupIdent::from_str("foo.bar").unwrap();
        assert_eq!(x.service(), "foo");
        assert_eq!(x.group(), "bar");
        assert!(x.org().is_none());

        let y = ServiceGroupIdent::from_str("foo.bar@baz").unwrap();
        assert_eq!(y.service(), "foo");
        assert_eq!(y.group(), "bar");
        assert_eq!(y.org().unwrap(), "baz");

        assert!(ServiceGroupIdent::from_str("foo.bar@").is_err());
        assert!(ServiceGroupIdent::from_str("f.oo.bar@baz").is_err());
        assert!(ServiceGroupIdent::from_str("foo@baz").is_err());
    }

    #[test]
    #[should_panic(expected = "not.allowed@")]
    fn from_str_ending_with_at() {
        ServiceGroupIdent::from_str("not.allowed@").unwrap();
    }

    #[test]
    #[should_panic(expected = "only.one.period@allowed")]
    fn from_str_too_many_periods() {
        ServiceGroupIdent::from_str("only.one.period@allowed").unwrap();
    }

    #[test]
    #[should_panic(expected = "oh-noes")]
    fn from_str_not_enough_periods() {
        ServiceGroupIdent::from_str("oh-noes").unwrap();
    }
}
