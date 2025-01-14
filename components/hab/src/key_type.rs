use std::str::FromStr;

use serde::{Deserialize,
            Serialize};

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum KeyType {
    Public,
    Secret,
}

impl FromStr for KeyType {
    type Err = crate::error::Error;

    fn from_str(value: &str) -> std::result::Result<Self, Self::Err> {
        match value {
            "public" => Ok(Self::Public),
            "secret" => Ok(Self::Secret),
            _ => Err(Self::Err::KeyTypeParseError(value.to_string())),
        }
    }
}

impl std::fmt::Display for KeyType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            KeyType::Public => write!(f, "public"),
            KeyType::Secret => write!(f, "secret"),
        }
    }
}

////////////////////////////////////////////////////////////////////////
