use habitat_core as core;
use habitat_launcher_protocol as protocol;

pub mod error;
pub mod server;
pub mod service;
mod sys;

pub const SUP_CMD: &str = "hab-sup";
pub const SUP_PACKAGE_IDENT: &str = "chef/hab-sup";
pub const VERSION: Option<&str> = option_env!("PLAN_VERSION");
