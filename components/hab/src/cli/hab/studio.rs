use super::util::ExternalCommandArgs;
use std::ffi::OsString;

use clap::Parser;

#[derive(Parser)]
/// Commands relating to Habitat Studios
pub struct Studio {
    #[command(flatten)]
    args: ExternalCommandArgs,
}

impl Studio {
    pub fn args(&self) -> &[OsString] { &self.args.args }
}
