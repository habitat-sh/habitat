use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat Studios
pub enum Studio {}
