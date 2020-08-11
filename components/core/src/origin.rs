use crate::{error::Error,
            package::ident::is_valid_origin_name};
use serde_derive::{Deserialize,
                   Serialize};
use std::{fmt,
          result,
          str::FromStr};

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
        if s.is_empty() {
            Err(Error::InvalidOrigin(s.to_string()))
        } else {
            Ok(Origin(s.to_string()))
        }
    }
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

impl std::convert::TryFrom<&str> for Origin {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> { Self::from_str(s) }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
            OriginMemberRole::ReadonlyMember => "readonly_member",
            OriginMemberRole::Member => "member",
            OriginMemberRole::Maintainer => "maintainer",
            OriginMemberRole::Administrator => "administrator",
            OriginMemberRole::Owner => "owner",
        };
        write!(f, "{}", value)
    }
}

impl FromStr for OriginMemberRole {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value.to_lowercase().as_ref() {
            "readonly_member" => Ok(OriginMemberRole::ReadonlyMember),
            "member" => Ok(OriginMemberRole::Member),
            "maintainer" => Ok(OriginMemberRole::Maintainer),
            "administrator" => Ok(OriginMemberRole::Administrator),
            "owner" => Ok(OriginMemberRole::Owner),
            _ => Err(Error::BadOriginMemberRole(value.to_string())),
        }
    }
}
