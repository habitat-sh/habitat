// Convenience importing of `debug!`/`info!` macros for entire crate.
#[cfg(test)]
extern crate lazy_static;
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
pub mod locked_env_var;
pub mod origin;
pub mod os;
pub mod package;
pub mod service;
pub mod url;
pub mod util;

use std::{fmt,
          io::Write};

pub use crate::os::{filesystem,
                    users};
use serde::Serialize as SerializeTrait;
use serde_derive::{Deserialize,
                   Serialize};
use serde_json::Value as Json;
use tabwriter::TabWriter;

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

// Returns a library object that implements elastic tabstops
pub fn tabw() -> TabWriter<Vec<u8>> { TabWriter::new(Vec::new()) }

// Format strings with elastic tab stops
pub fn tabify(mut tw: TabWriter<Vec<u8>>, s: &str) -> Result<String> {
    write!(&mut tw, "{}", s)?;
    tw.flush()?;
    String::from_utf8(tw.into_inner().expect("TABWRITER into_inner")).map_err(
        Error::StringFromUtf8Error)
}

pub trait TabularText {
    fn as_tabbed(&self) -> Result<String>;
}

pub trait PortableText: SerializeTrait {
    fn as_json(&self) -> Result<Json> {
        serde_json::to_value(self).map_err(Error::RenderContextSerialization)
    }
}
