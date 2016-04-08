// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::fmt;
use std::result;
use std::str::FromStr;

use error::Error;

#[derive(Clone, Debug, Eq, Hash, PartialEq, RustcDecodable, RustcEncodable)]
pub struct ServiceGroup {
    pub service: String,
    pub group: String,
}

impl ServiceGroup {
    pub fn new<T: Into<String>>(service: T, group: T) -> Self {
        ServiceGroup {
            service: service.into(),
            group: group.into(),
        }
    }
}

impl fmt::Display for ServiceGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.service, self.group)
    }
}

impl FromStr for ServiceGroup {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        let parts: Vec<&str> = value.split(".").collect();
        let (service, group) = match parts.len() {
            2 => (parts[0], parts[1]),
            _ => return Err(Error::InvalidServiceGroup(value.to_string())),
        };

        Ok(ServiceGroup::new(service, group))
    }
}
