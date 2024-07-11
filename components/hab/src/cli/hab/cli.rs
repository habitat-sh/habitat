#![allow(dead_code)]

use super::util::CacheKeyPath;
use clap::Parser;
use serde::{Deserialize,
            Serialize};

// Serialize required by the bound in toml::Value::try_from
#[derive(Parser)]
#[derive(Serialize, Deserialize)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
    PowerShell,
}

#[derive(Parser)]
/// Commands relating to Habitat runtime config
pub enum Cli {
    Setup(CliSetup),
    Completers(CliCompleters),
}

/// Sets up the CLI with reasonable defaults
#[derive(Parser)]
pub struct CliSetup {
    #[structopt(flatten)]
    cache_key_path: CacheKeyPath,
}

/// Creates command-line completers for your shell
#[derive(Parser)]
pub struct CliCompleters {
    /// The name of the shell you want to generate the command-completion
    #[clap(name = "SHELL",
                short = "s",
                long = "shell",
                possible_values = &Shell::variants(),
                case_insensitive = true)]
    shell: Shell,
}
