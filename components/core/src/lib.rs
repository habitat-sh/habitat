// Convenience importing of `debug!`/`info!` macros for entire crate.

pub use self::error::{Error,
                      Result};

pub mod binlink;
pub mod crypto;
pub mod env;
pub mod error;
pub mod flowcontrol;
pub mod fs;
pub mod locked_env_var;
pub mod origin;
pub mod os;
pub mod package;
pub mod service;
pub mod tls;
pub mod url;
pub mod util;

use std::fmt;

pub use crate::os::{filesystem,
                    users};
use serde::{Deserialize,
            Serialize};

pub const AUTH_TOKEN_ENVVAR: &str = "HAB_AUTH_TOKEN";

// A Builder channel
env_config_string!(#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
                   pub ChannelIdent,
                   HAB_BLDR_CHANNEL,
                   ChannelIdent::LTS);

impl ChannelIdent {
    const LTS: &'static str = "LTS-2024";
    const STABLE: &'static str = "stable";
    const UNSTABLE: &'static str = "unstable";

    pub fn as_str(&self) -> &str { self.0.as_str() }

    pub fn stable() -> Self { Self::from(Self::STABLE) }

    pub fn unstable() -> Self { Self::from(Self::UNSTABLE) }

    pub fn lts() -> Self { Self::from(Self::LTS) }
}

impl fmt::Display for ChannelIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

pub mod base64 {
    use ::base64::{engine::{general_purpose::STANDARD,
                            Engine},
                   DecodeError};

    pub fn encode<T: AsRef<[u8]>>(input: T) -> String { STANDARD.encode(input) }

    pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(input)
    }
}
