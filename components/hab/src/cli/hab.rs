mod bldr;
mod cli;
mod config;
mod file;
pub mod license;
pub mod origin;
pub mod pkg;
mod plan;
mod ring;
pub mod studio;
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
                    ConfigOptServiceConfigApply,
                    ServiceConfig,
                    ServiceConfigApply},
           file::{ConfigOptFile,
                  File},
           license::{ConfigOptLicense,
                     License},
           origin::{ConfigOptOrigin,
                    Origin},
           pkg::{ConfigOptPkg,
                 ConfigOptPkgInstall,
                 Pkg,
                 PkgInstall},
           plan::{ConfigOptPlan,
                  Plan},
           ring::{ConfigOptRing,
                  Ring},
           studio::{ConfigOptStudio,
                    Studio},
           sup::{ConfigOptHabSup,
                 ConfigOptSupRun,
                 HabSup,
                 SupRun},
           svc::{ConfigOptSvc,
                 ConfigOptSvcStart,
                 ConfigOptSvcStop,
                 Svc,
                 SvcStart,
                 SvcStop},
           user::{ConfigOptUser,
                  User},
           util::{CacheKeyPath,
                  ConfigOptCacheKeyPath}};
use crate::{cli::AFTER_HELP,
            VERSION};
use configopt::ConfigOpt;
use structopt::{clap::AppSettings,
                StructOpt};

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "hab",
            version = VERSION,
            about = "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing",
            author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
            settings = &[AppSettings::GlobalVersion],
            after_help = AFTER_HELP
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
    #[structopt(no_version, aliases = &["stu", "stud", "studi"])]
    Studio(Studio),
    #[structopt(no_version)]
    Sup(HabSup),
    /// Create a tarball of Habitat Supervisor data to send to support
    #[structopt(no_version)]
    Supportbundle,
    #[structopt(no_version)]
    Svc(Svc),
    #[structopt(no_version)]
    User(User),

    /// Alias for 'config apply'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Apply(ServiceConfigApply),
    /// Alias for 'pkg install'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Install(PkgInstall),
    #[cfg(not(target_os = "macos"))]
    /// Alias for 'sup run'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Run(SupRun),
    /// Alias for 'cli setup'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Setup(CacheKeyPath),
    /// Alias for 'svc start'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Start(SvcStart),
    /// Alias for 'svc stop'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Stop(SvcStop),
    #[cfg(not(target_os = "macos"))]
    /// Alias for 'sup term'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Term,
}
