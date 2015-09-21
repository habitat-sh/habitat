//! The CLI commands.
//!
//! Bldr's command line actions are defined here; one module per command. Their names map 1:1 to
//! the actual command line arguments, with one exception - `_` is translated to `-` on the CLI.
pub mod install;
pub mod start;
pub mod key;
pub mod key_upload;
pub mod upload;
pub mod repo;
pub mod configure;
