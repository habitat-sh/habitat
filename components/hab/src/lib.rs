#![recursion_limit = "128"]

use habitat_api_client as api_client;
use habitat_common as common;
use habitat_core as hcore;
use habitat_http_client as http_client;
use habitat_sup_client as sup_client;
use habitat_sup_protocol as protocol;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate features;

#[macro_use]
extern crate log;

#[macro_use]
extern crate serde_derive;

extern crate serde_json;

#[cfg(windows)]
extern crate widestring;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate winreg;

pub mod analytics;
pub mod cli;
pub mod command;
pub mod config;
pub mod error;
mod exec;
pub mod scaffolding;

pub const PRODUCT: &str = "hab";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const CTL_SECRET_ENVVAR: &str = "HAB_CTL_SECRET";
pub const ORIGIN_ENVVAR: &str = "HAB_ORIGIN";
pub const BLDR_URL_ENVVAR: &str = "HAB_BLDR_URL";

pub use crate::hcore::AUTH_TOKEN_ENVVAR;

features! {
    pub mod feat {
        const List           = 0b0000_0001,
        const OfflineInstall = 0b0000_0010,
        const IgnoreLocal    = 0b0000_0100,
        const InstallHook    = 0b0000_1000
    }
}
