mod bldr;
mod cli;
mod config;
mod file;
mod license;
mod origin;
mod pkg;
mod plan;
mod ring;
mod studio;
pub mod sup;
pub mod svc;
#[cfg(test)]
mod tests;
mod user;
pub mod util;

use self::{bldr::{Bldr,
                  ConfigOptBldr},
           cli::{Cli,
                 ConfigOptCli},
           config::{ConfigOptServiceConfig,
                    ServiceConfig},
           file::{ConfigOptFile,
                  File},
           license::{ConfigOptLicense,
                     License},
           origin::{ConfigOptOrigin,
                    Origin},
           pkg::{ConfigOptPkg,
                 Pkg},
           plan::{ConfigOptPlan,
                  Plan},
           ring::{ConfigOptRing,
                  Ring},
           studio::{ConfigOptStudio,
                    Studio},
           sup::{ConfigOptSup,
                 Sup},
           svc::{ConfigOptSvc,
                 Svc},
           user::{ConfigOptUser,
                  User}};
use crate::VERSION;
use configopt::ConfigOpt;
use structopt::{clap::AppSettings,
                StructOpt};

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "hab",
            version = VERSION,
            about = "\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            settings = &[AppSettings::GlobalVersion],
        )]
#[allow(clippy::large_enum_variant)]
pub enum Hab {
    #[structopt(no_version)]
    Bldr(Bldr),
    #[structopt(no_version)]
    Cli(Cli),
    #[structopt(no_version)]
    Config(ServiceConfig),
    #[structopt(no_version)]
    File(File),
    #[structopt(no_version)]
    License(License),
    #[structopt(no_version)]
    Origin(Origin),
    #[structopt(no_version)]
    Pkg(Pkg),
    #[structopt(no_version)]
    Plan(Plan),
    #[structopt(no_version)]
    Ring(Ring),
    #[structopt(no_version)]
    Studio(Studio),
    #[structopt(no_version)]
    Sup(Sup),
    /// Create a tarball of Habitat Supervisor data to send to support
    #[structopt(no_version)]
    Supportbundle,
    #[structopt(no_version)]
    Svc(Svc),
    #[structopt(no_version)]
    User(User),
}
