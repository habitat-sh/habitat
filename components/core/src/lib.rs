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

const STABLE_CHANNEL_IDENT: &str = "stable";
const UNSTABLE_CHANNEL_IDENT: &str = "unstable";

// A Builder channel
env_config_string!(#[derive(Deserialize, Serialize, Clone, Debug, Eq, Hash, PartialEq)],
                   pub ChannelIdent,
                   HAB_BLDR_CHANNEL,
                   STABLE_CHANNEL_IDENT.to_string());

impl ChannelIdent {
    pub fn as_str(&self) -> &str { self.0.as_str() }

    pub fn stable() -> Self { Self::from(STABLE_CHANNEL_IDENT) }

    pub fn unstable() -> Self { Self::from(UNSTABLE_CHANNEL_IDENT) }
}

impl fmt::Display for ChannelIdent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
}
