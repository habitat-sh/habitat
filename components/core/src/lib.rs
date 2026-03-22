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
                   ChannelIdent::STABLE);

/// Origins owned by Chef that should default to the `base` channel instead of `stable`.
/// To add a new Chef-owned origin in the future, simply append its name to this list.
pub const CHEF_OWNED_ORIGINS: &[&str] = &["core", "chef","chef-platform"];

impl ChannelIdent {
    const BASE: &'static str = "base";
    const STABLE: &'static str = "stable";
    const UNSTABLE: &'static str = "unstable";

    pub fn as_str(&self) -> &str { self.0.as_str() }

    pub fn base() -> Self { Self::from(Self::BASE) }

    pub fn stable() -> Self { Self::from(Self::STABLE) }

    pub fn unstable() -> Self { Self::from(Self::UNSTABLE) }

    /// Returns the default channel for a given origin.
    ///
    /// Chef-owned origins (see [`CHEF_OWNED_ORIGINS`]) default to the `base` channel to align
    /// with the Habitat 2.0 LTS approach. All other origins default to `stable`.
    pub fn default_for_origin(origin: &str) -> Self {
        if CHEF_OWNED_ORIGINS.contains(&origin) {
            Self::base()
        } else {
            Self::stable()
        }
    }
}

impl fmt::Display for ChannelIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}

pub mod base64 {
    use ::base64::{DecodeError,
                   engine::{Engine,
                            general_purpose::STANDARD}};

    pub fn encode<T: AsRef<[u8]>>(input: T) -> String { STANDARD.encode(input) }

    pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>, DecodeError> {
        STANDARD.decode(input)
    }
}
