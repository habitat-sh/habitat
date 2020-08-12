use crate::{error::Error,
            package::ident::is_valid_origin_name};
use serde_derive::{Deserialize,
                   Serialize};
use std::{fmt,
          result,
          str::FromStr};

const READONLY_MEMBER: &str = "readonly_member";
const MEMBER: &str = "member";
const MAINTAINER: &str = "maintainer";
const ADMINISTRATOR: &str = "administrator";
const OWNER: &str = "owner";

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Origin(String);

impl Origin {
    #[allow(clippy::needless_pass_by_value)]
    pub fn validate(value: String) -> result::Result<(), String> {
        if is_valid_origin_name(&value) {
            Ok(())
        } else {
            Err(format!("'{}' is not valid. A valid origin contains a-z, \
                         0-9, and _ or - after the first character",
                        &value))
        }
    }
}

impl FromStr for Origin {
    type Err = Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Origin::validate(s.to_string()).map_or_else(|e| Err(Error::InvalidOrigin(e)),
                                                    |_| Ok(Origin(s.to_string())))
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl std::convert::TryFrom<&str> for Origin {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> { Self::from_str(s) }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OriginMemberRole {
    ReadonlyMember,
    Member,
    Maintainer,
    Administrator,
    Owner,
}

impl Default for OriginMemberRole {
    fn default() -> OriginMemberRole { OriginMemberRole::ReadonlyMember }
}

impl fmt::Display for OriginMemberRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match *self {
            OriginMemberRole::ReadonlyMember => READONLY_MEMBER,
            OriginMemberRole::Member => MEMBER,
            OriginMemberRole::Maintainer => MAINTAINER,
            OriginMemberRole::Administrator => ADMINISTRATOR,
            OriginMemberRole::Owner => OWNER,
        };
        write!(f, "{}", value)
    }
}

impl FromStr for OriginMemberRole {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            READONLY_MEMBER => Ok(OriginMemberRole::ReadonlyMember),
            MEMBER => Ok(OriginMemberRole::Member),
            MAINTAINER => Ok(OriginMemberRole::Maintainer),
            ADMINISTRATOR => Ok(OriginMemberRole::Administrator),
            OWNER => Ok(OriginMemberRole::Owner),
            _ => Err(Error::BadOriginMemberRole(value.to_string())),
        }
    }
}
