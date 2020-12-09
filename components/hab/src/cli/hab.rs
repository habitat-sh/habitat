pub mod bldr;
pub mod cli;
pub mod config;
pub mod file;
pub mod license;
pub mod origin;
pub mod pkg;
pub mod plan;
pub mod ring;
pub mod studio;
pub mod sup;
pub mod svc;
#[cfg(test)]
mod tests;
pub mod user;
pub mod util;

use self::{bldr::{ConfigOptChannelCreate,
                  ConfigOptChannelDemote,
                  ConfigOptChannelDestroy,
                  ConfigOptChannelList,
                  ConfigOptChannelPromote,
                  ConfigOptJobCancel,
                  ConfigOptJobDemote,
                  ConfigOptJobPromote,
                  ConfigOptJobStart,
                  ConfigOptJobStatus,
                  ChannelCreate,
                  ChannelDemote,
                  ChannelDestroy,
                  ChannelList,
                  ChannelPromote,
                  JobCancel,
                  JobDemote,
                  JobPromote,
                  JobStart,
                  JobStatus},
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
           user::{ConfigOptUserKeyGenerate,
                  UserKeyGenerate},
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
    #[structopt(no_version, settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
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
    #[structopt(no_version, settings = &[AppSettings::Hidden], aliases = &["i", "in", "ins", "inst", "insta", "instal"])]
    Install(PkgInstall),
    #[cfg(not(target_os = "macos"))]
    /// Alias for 'sup run'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Run(SupRun),
    /// Alias for 'cli setup'
    #[structopt(no_version, settings = &[AppSettings::Hidden], aliases = &["set", "setu"])]
    Setup(CacheKeyPath),
    /// Alias for 'svc start'
    #[structopt(no_version, settings = &[AppSettings::Hidden], aliases = &["sta", "star"])]
    Start(SvcStart),
    /// Alias for 'svc stop'
    #[structopt(no_version, settings = &[AppSettings::Hidden], aliases = &["sto"])]
    Stop(SvcStop),
    #[cfg(not(target_os = "macos"))]
    /// Alias for 'sup term'
    #[structopt(no_version, settings = &[AppSettings::Hidden])]
    Term,
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, aliases = &["b", "bl", "bld"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder
pub enum Bldr {
    #[structopt(no_version)]
    Channel(Channel),
    #[structopt(no_version)]
    Job(Job),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, aliases = &["c", "ch", "cha", "chan", "chann", "channe"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder channels
pub enum Channel {
    #[structopt(no_version, aliases = &["c", "cr", "cre", "crea", "creat"])]
    Create(ChannelCreate),
    Demote(ChannelDemote),
    #[structopt(no_version, aliases = &["d", "de", "des", "dest", "destr", "destro"])]
    Destroy(ChannelDestroy),
    #[structopt(no_version, aliases = &["l", "li", "lis"])]
    List(ChannelList),
    Promote(ChannelPromote),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, aliases = &["j", "jo"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder jobs
pub enum Job {
    #[structopt(no_version, aliases = &["c", "ca", "can", "cance", "cancel"])]
    Cancel(JobCancel),
    #[structopt(no_version, aliases = &["d", "de", "dem", "demo", "demot"])]
    Demote(JobDemote),
    #[structopt(no_version, aliases = &["p", "pr", "pro", "prom", "promo", "promot"])]
    Promote(JobPromote),
    #[structopt(no_version, aliases = &["s", "st", "sta", "star"])]
    Start(JobStart),
    #[structopt(no_version, aliases = &["stat", "statu"])]
    Status(JobStatus),
}

/// Commands relating to Habitat users
#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "user", no_version, aliases = &["u", "us", "use"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
pub enum User {
    Key(UserKey),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "key", no_version, aliases = &["k", "ke"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat user keys
pub enum UserKey {
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(UserKeyGenerate),
}