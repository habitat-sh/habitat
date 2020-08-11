use super::util::{ConfigOptExternalCommandArgs,
                  ExternalCommandArgs};
use configopt::ConfigOpt;
use std::ffi::OsString;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "studio", no_version)]
/// Commands relating to Habitat Studios
pub struct Studio {
    #[structopt(flatten)]
    args: ExternalCommandArgs,
}

impl Studio {
    pub fn args(&self) -> &[OsString] { &self.args.args }
}
