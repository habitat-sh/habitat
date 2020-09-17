use crate::{error::Error,
            package::ident::is_valid_origin_name};
use serde_derive::{Deserialize,
                   Serialize};
use std::{fmt,
          result,
          str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
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

impl AsRef<str> for Origin {
    fn as_ref(&self) -> &str { &self.0 }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum OriginMemberRole {
    ReadonlyMember,
    Member,
    Maintainer,
    Administrator,
    Owner,
}

impl OriginMemberRole {
    pub const ADMINISTRATOR: &'static str = "administrator";
    pub const MAINTAINER: &'static str = "maintainer";
    pub const MEMBER: &'static str = "member";
    pub const OWNER: &'static str = "owner";
    pub const READONLY_MEMBER: &'static str = "readonly_member";
}

impl Default for OriginMemberRole {
    fn default() -> OriginMemberRole { OriginMemberRole::ReadonlyMember }
}

impl fmt::Display for OriginMemberRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match *self {
            OriginMemberRole::ReadonlyMember => OriginMemberRole::READONLY_MEMBER,
            OriginMemberRole::Member => OriginMemberRole::MEMBER,
            OriginMemberRole::Maintainer => OriginMemberRole::MAINTAINER,
            OriginMemberRole::Administrator => OriginMemberRole::ADMINISTRATOR,
            OriginMemberRole::Owner => OriginMemberRole::OWNER,
        };
        write!(f, "{}", value)
    }
}

impl FromStr for OriginMemberRole {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            OriginMemberRole::READONLY_MEMBER => Ok(OriginMemberRole::ReadonlyMember),
            OriginMemberRole::MEMBER => Ok(OriginMemberRole::Member),
            OriginMemberRole::MAINTAINER => Ok(OriginMemberRole::Maintainer),
            OriginMemberRole::ADMINISTRATOR => Ok(OriginMemberRole::Administrator),
            OriginMemberRole::OWNER => Ok(OriginMemberRole::Owner),
            _ => Err(Error::BadOriginMemberRole(value.to_string())),
        }
    }
}
