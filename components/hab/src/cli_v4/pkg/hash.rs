// Implementation of `hab pkg hash` command

use clap_v4 as clap;

use std::io::BufRead;

use clap::Parser;

use habitat_common::cli::clap_validators::FileExistsValueParser;

use crate::command::pkg::hash;

use crate::error::Result as HabResult;

#[derive(Debug, Clone, Parser)]
pub(crate) struct PkgHashOptions {
    /// Filepath to the Habitat Package file
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: Option<String>, /* TODO: Convert it to more semantic `PathBuf`, when we get rid of
                             * `clap-v2` functionality, revisit `command::pkg::hash` */
}

impl PkgHashOptions {
    pub(super) fn do_hash(&self) -> HabResult<()> {
        match &self.source {
            Some(source) => {
                // hash single file
                hash::start(source.as_str())
            }
            None => {
                // read files from stdin
                let stdin = std::io::stdin();
                for line in stdin.lock().lines() {
                    let file = line?;
                    hash::start(file.trim_end())?;
                }
                Ok(())
            }
        }
    }
}
