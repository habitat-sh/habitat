#![recursion_limit = "128"]

use habitat_api_client as api_client;
use habitat_common as common;
use habitat_core as hcore;
use habitat_sup_client as sup_client;
use habitat_sup_protocol as protocol;

#[cfg(feature = "v2")]
pub mod cli;

#[cfg(feature = "v4")]
mod cli_v4;

#[cfg(feature = "v4")]
pub use cli_v4::cli_driver;

pub mod command;
pub mod error;
mod exec;
pub mod license;
pub mod scaffolding;

pub const PRODUCT: &str = "hab";
pub const VERSION: &str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));
pub const ORIGIN_ENVVAR: &str = "HAB_ORIGIN";
pub const BLDR_URL_ENVVAR: &str = "HAB_BLDR_URL";

#[cfg(feature = "v4")]
pub const AFTER_HELP_V4: &str =
    "\x1B[1m\x1B[4mAliases:\x1B[0m\n  \x1B[1mapply\x1B[0m      Alias for: 'config apply'\n  \
     \x1B[1minstall\x1B[0m    Alias for: 'pkg install'\n  \x1B[1mrun\x1B[0m        Alias for: \
     'sup run'\n  \x1B[1msetup\x1B[0m      Alias for: 'cli setup'\n  \x1B[1mstart\x1B[0m      \
     Alias for: 'svc start'\n  \x1B[1mstop\x1b[0m       Alias for: 'svc stop'\n  \
     \x1B[1mterm\x1B[0m       Alias for: 'sup term'\n";

#[cfg(feature = "v2")]
pub const AFTER_HELP: &str = "ALIASES:\n    apply      Alias for: 'config apply'\n    install    \
                              Alias for: 'pkg install'\n    run        Alias for: 'sup run'\n    \
                              setup      Alias for: 'cli setup'\n    start      Alias for: 'svc \
                              start'\n    stop       Alias for: 'svc stop'\n    term       Alias \
                              for: 'sup term'\n";

pub use crate::hcore::AUTH_TOKEN_ENVVAR;

// TODO:agadgil: When Clap v2 support is gone, this should become `pub(crate)`
pub mod key_type;
