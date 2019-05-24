// Convenience importing of `debug!`/`info!` macros for entire crate.
#[macro_use]
extern crate log;

pub use self::error::{Error,
                      Result};

pub mod binlink;
pub mod config;
pub mod crypto;
pub mod env;
pub mod error;
pub mod fs;
pub mod os;
pub mod package;
pub mod service;
pub mod url;
pub mod util;

use std::fmt;

use serde_derive::{Deserialize,
                   Serialize};

pub use crate::os::{filesystem,
                    users};

/// A type which can't be instantiated
/// Use this when a generic requires a type which will never be used.
/// For example, FromStr::Err on an infallible conversion
pub enum Impossible {}

pub const AUTH_TOKEN_ENVVAR: &str = "HAB_AUTH_TOKEN";

// A Builder channel
env_config_string!(#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)]
                   pub ChannelIdent,
                   HAB_BLDR_CHANNEL,
                   ChannelIdent::STABLE);

impl ChannelIdent {
    const STABLE: &'static str = "stable";
    const UNSTABLE: &'static str = "unstable";

    pub fn as_str(&self) -> &str { self.0.as_str() }

    pub fn stable() -> Self { Self::from(Self::STABLE) }

    pub fn unstable() -> Self { Self::from(Self::UNSTABLE) }
}

impl fmt::Display for ChannelIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}
