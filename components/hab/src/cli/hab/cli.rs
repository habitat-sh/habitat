#![allow(dead_code)]

use super::util::CacheKeyPath;
use clap::arg_enum;
use serde::{Deserialize,
            Serialize};
use structopt::StructOpt;

arg_enum! {
    // Serialize required by the bound in toml::Value::try_from
    #[derive(Serialize, Deserialize)]
    pub enum Shell {
        Bash,
        Fish,
        Zsh,
        PowerShell,
    }
}

#[derive(StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat runtime config
pub enum Cli {
    Setup(CliSetup),
    Completers(CliCompleters),
}

/// Sets up the CLI with reasonable defaults
#[derive(StructOpt)]
#[structopt(name = "setup", no_version, rename_all = "screamingsnake")]
pub struct CliSetup {
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Creates command-line completers for your shell
#[derive(StructOpt)]
#[structopt(name = "completers", no_version, rename_all = "screamingsnake")]
pub struct CliCompleters {
    /// The name of the shell you want to generate the command-completion
    #[structopt(name = "SHELL",
                short = "s",
                long = "shell",
                possible_values = &Shell::variants(),
                case_insensitive = true)]
    shell: Shell,
}
