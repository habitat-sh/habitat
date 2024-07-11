#![allow(dead_code)]

use clap::Parser;
/// Commands relating to Habitat license agreements
#[derive(Parser)]
pub enum License {
    /// Accept the Chef Binary Distribution Agreement without prompting
    Accept,
}
