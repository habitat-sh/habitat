use super::util::{CacheKeyPath,
                  ConfigOptCacheKeyPath};
use configopt::ConfigOpt;
use structopt::StructOpt;

arg_enum! {
    #[derive(Deserialize)]
    pub enum Shell {
        Bash,
        Fish,
        Zsh,
        PowerShell,
    }
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat runtime config
pub enum Cli {
    /// Sets up the CLI with reasonable defaults
    #[structopt(no_version)]
    Setup(CacheKeyPath),
    /// Creates command-line completers for your shell
    #[structopt(no_version)]
    Completers {
        /// The name of the shell you want to generate the command-completion
        #[structopt(name = "SHELL",
                    short = "s",
                    long = "shell",
                    possible_values = &Shell::variants(),
                    case_insensitive = true)]
        shell: Shell,
    },
}
