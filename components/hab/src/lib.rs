#![recursion_limit = "128"]

use habitat_api_client as api_client;
use habitat_common as common;
use habitat_core as hcore;
use habitat_sup_client as sup_client;
use habitat_sup_protocol as protocol;

mod cli_v4;

pub use cli_v4::cli_driver;

pub use cli_v4::{sup::sup_run::SupRunOptions,
                 utils::shared_load_cli_to_ctl};

// TODO : Make this a pub(crate) module once we move to Clap v4 completely.
pub mod gateway_util;

pub mod command;
pub mod error;
mod exec;
pub mod license;
pub mod scaffolding;

pub const PRODUCT: &str = "hab";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const ORIGIN_ENVVAR: &str = "HAB_ORIGIN";
pub const BLDR_URL_ENVVAR: &str = "HAB_BLDR_URL";
pub const REFRESH_CHANNEL_ENVVAR: &str = "HAB_REFRESH_CHANNEL";

pub use crate::hcore::AUTH_TOKEN_ENVVAR;

pub(crate) mod key_type;
