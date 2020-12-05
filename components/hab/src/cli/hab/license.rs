use configopt::ConfigOpt;
use structopt::StructOpt;

/// Commands relating to Habitat license agreements
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "license", no_version)]
pub enum License {
    /// Accept the Chef Binary Distribution Agreement without prompting
    Accept,
}
