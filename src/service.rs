// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::fmt;
use std::result;
use std::str::FromStr;

use regex::Regex;

use error::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq, RustcDecodable, RustcEncodable)]
pub struct ServiceGroup {
    pub service: String,
    pub group: String,
    pub organization: Option<String>,
}

impl ServiceGroup {
    pub fn new<T: Into<String>>(service: T, group: T, organization: Option<String>) -> Self {
        ServiceGroup {
            service: service.into(),
            group: group.into(),
            organization: organization,
        }
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.organization.as_ref() {
            Some(org) => write!(f, "{}.{}@{}", self.service, self.group, org),
            None => write!(f, "{}.{}", self.service, self.group),
        }
    }
}

impl FromStr for ServiceGroup {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let regex = Regex::new(r"^([A-Za-z_0-9]+)\.([A-Za-z_0-9]+)(@([A-Za-z_0-9]+))?$").unwrap();
        let caps = match regex.captures(value) {
            Some(c) => c,
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };
        let name = match caps.at(1) {
            Some(n) => n,
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };

        let group = match caps.at(2) {
            Some(n) => n,
            None => return Err(Error::InvalidServiceGroup(value.to_string())),
        };

        // you can't specify a key with an "@", but without an org
        // ex: "foo.bar@"
        if value.ends_with('@') {
            return Err(Error::InvalidServiceGroup(value.to_string()));
        };

        Ok(ServiceGroup::new(name, group, caps.at(4).map(|s| s.to_string())))
    }
}

#[test]
fn service_groups_with_org() {
    let x = ServiceGroup::from_str("foo.bar").unwrap();
    assert!(x.service == "foo".to_string());
    assert!(x.group == "bar".to_string());
    assert!(x.organization.is_none());

    let y = ServiceGroup::from_str("foo.bar@baz").unwrap();
    assert!(y.service == "foo".to_string());
    assert!(y.group == "bar".to_string());
    assert!(y.organization.unwrap() == "baz");

    assert!(ServiceGroup::from_str("foo.bar@").is_err());
    assert!(ServiceGroup::from_str("f.oo.bar@baz").is_err());
    assert!(ServiceGroup::from_str("foo@baz").is_err());
}
