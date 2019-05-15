extern crate habitat_core as core;
use habitat_launcher_protocol as protocol;
#[macro_use]
extern crate log;
#[cfg(windows)]
extern crate winapi;

pub mod error;
pub mod server;
pub mod service;
mod sys;

pub const SUP_CMD: &str = "hab-sup";
pub const SUP_PACKAGE_IDENT: &str = "core/hab-sup";
