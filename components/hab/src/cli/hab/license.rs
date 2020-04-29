use configopt::ConfigOpt;
use structopt::StructOpt;

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version)]
/// Commands relating to Habitat license agreements
pub enum License {
    /// Accept the Chef Binary Distribution Agreement without prompting
    Accept,
}
