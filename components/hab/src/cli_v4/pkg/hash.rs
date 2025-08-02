// Implementation of `hab pkg hash` command

use clap_v4 as clap;

use std::{io::BufRead,
          path::PathBuf};

use clap::Parser;

use habitat_common::cli::clap_validators::FileExistsValueParser;

use crate::command::pkg::hash;

use crate::error::Result as HabResult;

#[derive(Debug, Clone, Parser)]
#[command(help_template = "{name} {version} {author-section} {about-section} \n{usage-heading} \
                           {usage}\n\n{all-args}\n")]
pub(crate) struct PkgHashOptions {
    /// Filepath to the Habitat Package file
    #[arg(name = "SOURCE", value_parser = FileExistsValueParser)]
    source: Option<PathBuf>,
}

impl PkgHashOptions {
    pub(super) fn do_hash(&self) -> HabResult<()> {
        match &self.source {
            Some(source) => {
                // hash single file
                hash::start(source.to_str().expect("Not a valud UTF-8 filename."))
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
