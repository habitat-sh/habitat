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
#[cfg(any(target_os = "macos",
              any(all(target_os = "linux",
                      any(target_arch = "x86_64", target_arch = "aarch64")),
                  all(target_os = "windows", target_arch = "x86_64"))))]
use self::studio::Studio;
use self::{bldr::*,
           cli::{CliCompleters,
                 CliSetup},
           config::{ServiceConfigApply,
                    ServiceConfigShow},
           file::{ConfigOptFileUpload,
                  FileUpload},
           license::License,
           origin::*,
           pkg::*,
           plan::{PlanInit,
                  PlanRender},
           ring::{RingKeyExport,
                  RingKeyGenerate,
                  RingKeyImport},
           sup::HabSup,
           svc::{Svc,
                 SvcStart,
                 SvcStop},
           user::UserKeyGenerate,
           util::CacheKeyPath};
use crate::{cli::AFTER_HELP,
            VERSION};
use configopt::ConfigOpt;
use structopt::{clap::AppSettings,
                StructOpt};

#[cfg(not(target_os = "macos"))]
use crate::cli::hab::sup::{ConfigOptSupRun,
                           SupRun};

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
    #[cfg(any(target_os = "macos",
              any(all(target_os = "linux",
                      any(target_arch = "x86_64", target_arch = "aarch64")),
                  all(target_os = "windows", target_arch = "x86_64"))))]
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
    /// Alias for 'sup run'
    #[cfg(not(target_os = "macos"))]
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
    /// Alias for 'sup term'
    #[cfg(not(target_os = "macos"))]
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
    #[structopt(no_version, aliases = &["c", "ca", "can", "cance"])]
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

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["cl"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat runtime config
pub enum Cli {
    #[structopt(no_version, aliases = &["s", "se", "set", "setu"])]
    /// Sets up the CLI with reasonable defaults
    Setup(CliSetup),
    #[structopt(no_version, aliases = &["c", "co", "com", "comp"])]
    /// Creates command-line completers for your shell
    Completers(CliCompleters),
}

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["co", "con", "conf", "confi"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to a Service's runtime config
pub enum ServiceConfig {
    #[structopt(no_version, aliases = &["ap", "app", "appl"])]
    Apply(ServiceConfigApply),
    #[structopt(no_version, aliases = &["sh", "sho"])]
    Show(ServiceConfigShow),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, aliases = &["f", "fi", "fil"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat files
pub enum File {
    #[structopt(no_version, aliases = &["u", "up", "upl", "uplo", "uploa"])]
    /// Uploads a file to be shared between members of a Service Group
    Upload(FileUpload),
}

/// Commands relating to Habitat users
#[derive(StructOpt)]
#[structopt(name = "user", no_version, aliases = &["u", "us", "use"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
pub enum User {
    Key(UserKey),
}

#[derive(StructOpt)]
#[structopt(name = "key", no_version, aliases = &["k", "ke"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat user keys
pub enum UserKey {
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(UserKeyGenerate),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, aliases = &["o", "or", "ori", "orig", "origi"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder origins
pub enum Origin {
    #[structopt(no_version, aliases = &["cre", "crea"])]
    Create(OriginCreate),
    #[structopt(no_version, aliases = &["del", "dele"])]
    Delete(OriginDelete),
    Depart(OriginDepart),
    Info(OriginInfo),
    Invitations(OriginInvitations),
    Key(OriginKey),
    /// Role Based Access Control for origin members
    Rbac(Rbac),
    Secret(OriginSecret),
    Transfer(OriginTransfer),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Manage origin member invitations
pub enum OriginInvitations {
    Accept(InvitationsAccept),
    Ignore(InvitationsIgnore),
    List(InvitationsList),
    Pending(InvitationsPending),
    Rescind(InvitationsRescind),
    Send(InvitationsSend),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "key", no_version, aliases = &["k", "ke"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat origin key maintenance
pub enum OriginKey {
    #[structopt(no_version, aliases = &["d", "do", "dow", "down", "downl", "downlo", "downloa"])]
    Download(OriginKeyDownload),
    #[structopt(no_version, aliases = &["e", "ex", "exp", "expo", "expor"])]
    Export(OriginKeyExport),
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(OriginKeyGenerate),
    #[structopt(no_version, aliases = &["i", "im", "imp", "impo", "impor"])]
    Import(OriginKeyImport),
    #[structopt(no_version, aliases = &["u", "up", "upl", "uplo", "uploa"])]
    Upload(OriginKeyUpload),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(name = "secret", no_version, settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands related to secret management
pub enum OriginSecret {
    Delete(SecretDelete),
    List(SecretList),
    Upload(SecretUpload),
}

#[derive(ConfigOpt, StructOpt)]
#[structopt(no_version, aliases = &["p", "pk", "package"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat packages
pub enum Pkg {
    Binds(PkgBinds),
    #[structopt(no_version, aliases = &["bi", "bin", "binl", "binli", "binlin"])]
    Binlink(PkgBinlink),
    #[structopt(no_version)]
    Build(PkgBuild),
    #[structopt(no_version, aliases = &["conf", "cfg"])]
    Config(PkgConfig),
    Download(PkgDownload),
    Env(PkgEnv),
    Exec(PkgExec),
    #[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
    Export(ExportCommand),
    #[structopt(no_version, aliases = &["ha", "has"])]
    Hash(PkgHash),
    #[structopt(no_version, aliases = &["i", "in", "ins", "inst", "insta", "instal"])]
    Install(PkgInstall),
    #[structopt(no_version, aliases = &["p", "pa", "pat"])]
    Path(PkgPath),
    #[structopt(no_version, aliases = &["li"])]
    List(PkgList),
    Provides(PkgProvides),
    Search(PkgSearch),
    #[structopt(no_version, aliases = &["s", "si", "sig"])]
    Sign(PkgSign),
    #[structopt(no_version, aliases = &["un", "unin"])]
    Uninstall(PkgUninstall),
    #[structopt(no_version, aliases = &["bul", "bulk"])]
    Bulkupload(PkgBulkupload),
    #[structopt(no_version, aliases = &["u", "up", "upl", "uplo", "uploa"])]
    Upload(PkgUpload),
    #[structopt(no_version, aliases = &["del", "dele"])]
    Delete(PkgDelete),
    #[structopt(no_version, aliases = &["pr", "pro", "promo", "promot"])]
    Promote(PkgPromote),
    #[structopt(no_version, aliases = &["de", "dem", "demo", "demot"])]
    Demote(PkgDemote),
    #[structopt(no_version, aliases = &["ch", "cha", "chan", "chann", "channe", "channel"])]
    Channels(PkgChannels),
    #[structopt(no_version, aliases = &["v", "ve", "ver", "veri", "verif"])]
    Verify(PkgVerify),
    #[structopt(no_version, aliases = &["hea", "head", "heade"])]
    Header(PkgHeader),
    #[structopt(no_version, aliases = &["inf"])]
    Info(PkgInfo),
    #[structopt(no_version, aliases = &["dep", "deps"])]
    Dependencies(PkgDependencies),
}

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["pl", "pla"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to plans and other app-specific configuration
pub enum Plan {
    #[structopt(no_version, aliases = &["i", "in", "ini"])]
    Init(PlanInit),
    #[structopt(no_version, aliases = &["r", "re", "ren", "rend", "rende"])]
    Render(PlanRender),
}

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["r", "ri", "rin"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat rings
pub enum Ring {
    Key(RingKey),
}

#[derive(StructOpt)]
#[structopt(name = "key", no_version, aliases = &["k", "ke"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat ring keys
pub enum RingKey {
    #[structopt(no_version, aliases = &["e", "ex", "exp", "expo", "expor"])]
    Export(RingKeyExport),
    #[structopt(no_version, aliases = &["i", "im", "imp", "impo", "impor"])]
    Import(RingKeyImport),
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(RingKeyGenerate),
}
