pub mod gateway_util;
pub mod hab;

use crate::{cli::hab::{bldr::{ChannelCreate,
                             ChannelDemote,
                             ChannelDestroy,
                             ChannelList,
                             ChannelPromote,
                             JobCancel,
                             JobDemote,
                             JobPromote,
                             JobStart,
                             JobStatus},
                      cli::{CliCompleters,
                            CliSetup},
                      config::{ServiceConfigApply,
                               ServiceConfigShow},
                      file::FileUpload,
                      license::License,
                      origin::{Accept,
                               Create as OriginCreate,
                               Delete as OriginDelete,
                               Depart as OriginDepart,
                               Download as KeyDownload,
                               Generate as OriginKeyGenerate,
                               Ignore,
                               Info as OriginInfo,
                               KeyExport as OriginKeyExport,
                               KeyImport as OriginKeyImport,
                               List,
                               Pending,
                               Rbac,
                               Rescind,
                               SecretDelete,
                               SecretList,
                               SecretUpload,
                               Send,
                               Transfer,
                               Upload as KeyUpload},
                      pkg::{ExportCommand,
                            List as PkgList,
                            PkgBinds,
                            PkgBinlink,
                            PkgBuild,
                            PkgBulkupload,
                            PkgChannels,
                            PkgConfig,
                            PkgDelete,
                            PkgDemote,
                            PkgDependencies,
                            PkgDownload,
                            PkgEnv,
                            PkgExec,
                            PkgHash,
                            PkgHeader,
                            PkgInfo,
                            PkgInstall,
                            PkgPath,
                            PkgPromote,
                            PkgProvides,
                            PkgSearch,
                            PkgSign,
                            PkgUninstall,
                            PkgUpload,
                            PkgVerify},
                      plan::{PlanInit,
                             PlanRender},
                      ring::{KeyExport,
                             KeyGenerate as RingKeyGenerate,
                             KeyImport},
                      studio::Studio,
                      sup::{HabSup,
                            SupRun,
                            SupTerm},
                      svc::{BulkLoad as SvcBulkLoad,
                            KeyGenerate,
                            Load as SvcLoad,
                            SvcStart,
                            SvcStatus,
                            SvcStop,
                            SvcUnload,
                            Update as SvcUpdate},
                      user::KeyGenerate as UserKeyGenerate,
                      Hab},
            VERSION};
use clap::{App,
           ArgMatches};
use habitat_common::{cli::{file_into_idents,
                           is_toml_file},
                     FeatureFlag};
use habitat_core::{origin::Origin,
                   package::{Identifiable,
                             PackageIdent}};
use std::{fmt,
          path::Path,
          result,
          str::FromStr};
use structopt::{clap::AppSettings,
                StructOpt};

/// Process exit code from Supervisor which indicates to Launcher that the Supervisor
/// ran to completion with a successful result. The Launcher should not attempt to restart
/// the Supervisor and should exit immediately with a successful exit code.
pub const OK_NO_RETRY_EXCODE: i32 = 84;
pub const AFTER_HELP: &str =
    "\nALIASES:\n    apply      Alias for: 'config apply'\n    install    Alias for: 'pkg \
     install'\n    run        Alias for: 'sup run'\n    setup      Alias for: 'cli setup'\n    \
     start      Alias for: 'svc start'\n    stop       Alias for: 'svc stop'\n    term       \
     Alias for: 'sup term'\n";

#[derive(StructOpt)]
#[structopt(name = "hab",
           version = VERSION,
           about = "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing",
           author = "\nThe Habitat Maintainers <humans@habitat.sh>\n",
           settings = &[AppSettings::GlobalVersion, AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp],
           after_help = AFTER_HELP
       )]
#[allow(clippy::large_enum_variant)]
pub enum Options {
    #[structopt(no_version, settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
    License(License),
    #[structopt(no_version)]
    Cli(Cli),
    #[structopt(no_version)]
    Config(ServiceConfig),
    #[structopt(no_version)]
    File(File),
    #[structopt(no_version)]
    Bldr(Bldr),
    #[structopt(no_version)]
    Origin(HabOrigin),
    #[structopt(no_version)]
    Pkg(Pkg),
    #[structopt(no_version)]
    Plan(Plan),
    #[structopt(no_version)]
    Ring(Ring),
    #[structopt(name = "sup", no_version)]
    HabSup(HabSup),
    #[structopt(no_version)]
    Svc(Svc),
    #[structopt(no_version, aliases = &["stu", "stud", "studi"])]
    Studio(Studio),
    #[structopt(name = "supportbundle", no_version, aliases = &["supp", "suppo", "suppor", "support-bundle"])]
    /// Create a tarball of Habitat Supervisor data to send to support
    SupportBundle,
    #[structopt(no_version)]
    User(User),
    #[structopt(no_version, aliases = &["u", "us", "use"], setting = AppSettings::Hidden)]
    /// Alias for 'config apply'
    Apply(ServiceConfigApply),
    #[structopt(no_version, aliases = &["i", "in", "ins", "inst", "insta", "instal"], setting = AppSettings::Hidden)]
    /// Alias for 'pkg install'
    Install(PkgInstall),
    #[structopt(no_version, setting = AppSettings::Hidden)]
    /// Alias for 'sup run'
    Run(SupRun),
    #[structopt(no_version, aliases =&["set", "setu"], setting = AppSettings::Hidden)]
    /// Alias for 'cli setup'
    Setup(CliSetup),
    #[structopt(no_version, aliases =&["sta", "star"], setting = AppSettings::Hidden)]
    /// Alias for 'svc start'
    Start(SvcStart),
    #[structopt(no_version, aliases = &["sto"], setting = AppSettings::Hidden)]
    /// Alias for 'svc stop'
    Stop(SvcStop),
    #[structopt(no_version, setting = AppSettings::Hidden)]
    /// Alias for 'sup term'
    Term(SupTerm),
}

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["f", "fi", "fil"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat files
pub enum File {
    #[structopt(no_version, aliases = &["u", "up", "upl", "uplo", "uploa"])]
    /// Uploads a file to be shared between members of a Service Group
    Upload(FileUpload),
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

/// Commands relating to Habitat users
#[derive(StructOpt)]
#[structopt(no_version, aliases = &["u", "us", "use"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
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

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["b", "bl", "bld"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder
pub enum Bldr {
    #[structopt(no_version)]
    Channel(Channel),
    #[structopt(no_version)]
    Job(Job),
}

#[derive(StructOpt)]
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

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["j", "jo"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder jobs
pub enum Job {
    #[structopt(no_version, aliases = &["s", "st", "sta", "star"])]
    Cancel(JobCancel),
    #[structopt(no_version, aliases = &["c", "ca", "can", "cance", "cancel"])]
    Demote(JobDemote),
    #[structopt(no_version, aliases = &["p", "pr", "pro", "prom", "promo", "promot"])]
    Promote(JobPromote),
    #[structopt(no_version, aliases = &["d", "de", "dem", "demo", "demot"])]
    Start(JobStart),
    #[structopt(no_version, aliases = &["stat", "statu"])]
    Status(JobStatus),
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
    Export(KeyExport),
    #[structopt(no_version, aliases = &["i", "im", "imp", "impo", "impor"])]
    Import(KeyImport),
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(RingKeyGenerate),
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
#[structopt(no_version, aliases = &["sv", "ser", "serv", "service"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat services
pub enum Svc {
    #[structopt(name = "bulkload")]
    BulkLoad(SvcBulkLoad),
    Key(SvcKey),
    #[structopt(no_version)]
    Load(Box<SvcLoad>), // Boxed due to clippy::large_enum_variant
    #[structopt(no_version)]
    Update(Box<SvcUpdate>),  // Boxed due to clippy::large_enum_variant
    #[structopt(aliases = &["star"])]
    Start(SvcStart),
    #[structopt(aliases = &["stat", "statu"])]
    Status(SvcStatus),
    #[structopt(aliases = &["sto"])]
    Stop(SvcStop),
    #[structopt(aliases = &["u", "un", "unl", "unlo", "unloa"])]
    Unload(SvcUnload),
}

#[derive(StructOpt)]
#[structopt(no_version, aliases = &["k", "ke"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat service keys
pub enum SvcKey {
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(KeyGenerate),
}

#[derive(StructOpt)]
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
    #[structopt(no_version, aliases = &["hea", "head", "heade", "header"])]
    Header(PkgHeader),
    #[structopt(no_version, aliases = &["inf", "info"])]
    Info(PkgInfo),
    #[structopt(no_version, aliases = &["dep", "deps"])]
    Dependencies(PkgDependencies),
}

#[derive(StructOpt)]
#[structopt(name = "origin", no_version, aliases = &["o", "or", "ori", "orig", "origi"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat Builder origins
pub enum HabOrigin {
    #[structopt(no_version, aliases = &["cre", "crea"])]
    Create(OriginCreate),
    #[structopt(no_version, aliases = &["del", "dele"])]
    Delete(OriginDelete),
    Depart(OriginDepart),
    Info(OriginInfo),
    Invitations(Invitations),
    Key(OriginKey),
    /// Role Based Access Control for origin members
    Rbac(Rbac),
    Secret(OriginSecret),
    Transfer(Transfer),
}

#[derive(StructOpt)]
#[structopt(no_version, settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Manage origin member invitations
pub enum Invitations {
    Accept(Accept),
    Ignore(Ignore),
    List(List),
    Pending(Pending),
    Rescind(Rescind),
    Send(Send),
}

#[derive(StructOpt)]
#[structopt(name = "key", no_version, aliases = &["k", "ke"], settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands relating to Habitat origin key maintenance
pub enum OriginKey {
    #[structopt(no_version, aliases = &["d", "do", "dow", "down", "downl", "downlo", "downloa"])]
    Download(KeyDownload),
    #[structopt(no_version, aliases = &["e", "ex", "exp", "expo", "expor"])]
    Export(OriginKeyExport),
    #[structopt(no_version, aliases = &["g", "ge", "gen", "gene", "gener", "genera", "generat"])]
    Generate(OriginKeyGenerate),
    #[structopt(no_version, aliases = &["i", "im", "imp", "impo", "impor"])]
    Import(OriginKeyImport),
    #[structopt(no_version, aliases = &["u", "up", "upl", "uplo", "uploa"])]
    Upload(KeyUpload),
}

#[derive(StructOpt)]
#[structopt(name = "secret", no_version, settings = &[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])]
/// Commands related to secret management
pub enum OriginSecret {
    Delete(SecretDelete),
    List(SecretList),
    Upload(SecretUpload),
}

pub fn get(feature_flags: FeatureFlag) -> App<'static, 'static> {
    if feature_flags.contains(FeatureFlag::STRUCTOPT_CLI) {
        return Hab::clap();
    }

    Options::clap()
}

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub enum KeyType {
    Public,
    Secret,
}

impl FromStr for KeyType {
    type Err = crate::error::Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value {
            "public" => Ok(Self::Public),
            "secret" => Ok(Self::Secret),
            _ => Err(Self::Err::KeyTypeParseError(value.to_string())),
        }
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyType::Public => write!(f, "Public"),
            KeyType::Secret => write!(f, "Secret"),
        }
    }
}

////////////////////////////////////////////////////////////////////////

pub fn parse_optional_arg<T: FromStr>(name: &str, m: &ArgMatches) -> Option<T>
    where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    m.value_of(name).map(|s| s.parse().expect("Valid argument"))
}

// CLAP Validation Functions
////////////////////////////////////////////////////////////////////////

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn dir_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_dir() {
        Ok(())
    } else {
        Err(format!("Directory: '{}' cannot be found", &val))
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("File: '{}' cannot be found", &val))
    }
}

fn file_exists_or_stdin(val: String) -> result::Result<(), String> {
    if val == "-" {
        Ok(())
    } else {
        file_exists(val)
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_ident_or_toml_file(val: String) -> result::Result<(), String> {
    if is_toml_file(&val) {
        // We could do some more validation (parse the whole toml file and check it) but that seems
        // excessive.
        Ok(())
    } else {
        valid_ident_file(val)
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_ident_file(val: String) -> result::Result<(), String> {
    file_into_idents(&val).map(|_| ())
                          .map_err(|e| e.to_string())
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_fully_qualified_ident(val: String) -> result::Result<(), String> {
    match PackageIdent::from_str(&val) {
        Ok(ref ident) if ident.fully_qualified() => Ok(()),
        _ => {
            Err(format!("'{}' is not valid. Fully qualified package \
                         identifiers have the form \
                         origin/name/version/release",
                        &val))
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_origin(val: String) -> result::Result<(), String> { Origin::validate(val) }

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

    use super::*;
    use habitat_common::types::{EventStreamMetadata,
                                EventStreamToken};

    #[test]
    fn legacy_appliction_and_environment_args() {
        let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab",
                                                                   "sup",
                                                                   "run",
                                                                   "--application",
                                                                   "--environment=env"]);
        assert!(r.is_ok());
        let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab",
                                                                   "svc",
                                                                   "load",
                                                                   "--application=app",
                                                                   "--environment",
                                                                   "pkg/ident"]);
        assert!(r.is_ok());
        let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab",
                                                                   "svc",
                                                                   "load",
                                                                   "--application",
                                                                   "pkg/ident"]);
        assert!(r.is_ok());
    }

    mod sup_commands {

        use super::*;
        use clap::ErrorKind;

        #[test]
        fn sup_subcommand_short_help() {
            let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab", "sup", "-h"]);
            assert!(r.is_err());
            // not `ErrorKind::InvalidSubcommand`
            assert_eq!(r.unwrap_err().kind, ErrorKind::HelpDisplayed);
        }

        #[test]
        fn sup_subcommand_run_with_peer() {
            let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab", "sup", "run",
                                                                       "--peer", "1.1.1.1"]);
            assert!(r.is_ok());
            let matches = r.expect("Error while getting matches");
            // validate `sup` subcommand
            assert_eq!(matches.subcommand_name(), Some("sup"));
            let (_, sup_matches) = matches.subcommand();
            let sup_matches = sup_matches.expect("Error while getting sup matches");
            assert_eq!(sup_matches.subcommand_name(), Some("run"));
            let (_, run_matches) = sup_matches.subcommand();
            let run_matches = run_matches.expect("Error while getting run matches");
            assert_eq!(run_matches.value_of("PEER"), Some("1.1.1.1"));
        }
    }

    mod event_stream_feature {
        use super::*;

        #[test]
        fn app_and_env_and_token_options_required_if_url_option() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::MissingRequiredArgument);
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::MissingRequiredArgument);
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::MissingRequiredArgument);
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_ok());
        }

        #[test]
        fn app_option_must_take_a_value() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
            assert_eq!(error.info,
                       Some(vec!["EVENT_STREAM_APPLICATION".to_string()]));
        }

        #[test]
        fn app_option_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
        }

        #[test]
        fn env_option_must_take_a_value() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
            assert_eq!(error.info,
                       Some(vec!["EVENT_STREAM_ENVIRONMENT".to_string()]));
        }

        #[test]
        fn env_option_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
        }

        #[test]
        fn event_meta_can_be_repeated() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-meta",
                                                                    "foo=bar",
                                                                    "--event-meta",
                                                                    "blah=boo",
                                                                    "--event-meta",
                                                                    "monkey=pants",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_ok());
            let matches = matches.unwrap();
            let meta = matches.values_of(EventStreamMetadata::ARG_NAME)
                              .expect("didn't have metadata")
                              .collect::<Vec<_>>();
            assert_eq!(meta, ["foo=bar", "blah=boo", "monkey=pants"]);
        }

        #[test]
        fn event_meta_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-meta",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            assert_eq!(matches.unwrap_err().kind, clap::ErrorKind::EmptyValue);
        }

        #[test]
        fn event_meta_must_have_an_equal_sign() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-meta",
                                                                    "foobar",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            assert_eq!(matches.unwrap_err().kind, clap::ErrorKind::ValueValidation);
        }

        #[test]
        fn event_meta_key_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-meta",
                                                                    "=bar",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            assert_eq!(matches.unwrap_err().kind, clap::ErrorKind::ValueValidation);
        }

        #[test]
        fn event_meta_value_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-meta",
                                                                    "foo=",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            assert_eq!(matches.unwrap_err().kind, clap::ErrorKind::ValueValidation);
        }

        #[test]
        fn token_option_must_take_a_value() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",
                                                                    "--event-stream-token",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
            assert_eq!(error.info,
                       Some(vec![EventStreamToken::ARG_NAME.to_string()]));
        }

        #[test]
        fn token_option_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::ValueValidation);
        }

        #[test]
        fn site_option_must_take_a_value() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",
                                                                    "--event-stream-site",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
            assert_eq!(error.info, Some(vec!["EVENT_STREAM_SITE".to_string()]));
        }

        #[test]
        fn site_option_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "127.0.0.1:4222",
                                                                    "--event-stream-site",
                                                                    "",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
        }

        #[test]
        fn url_option_must_take_a_value() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::EmptyValue);
            assert_eq!(error.info, Some(vec!["EVENT_STREAM_URL".to_string()]));
        }

        #[test]
        fn url_option_cannot_be_empty() {
            let matches = SupRun::clap().get_matches_from_safe(vec!["run",
                                                                    "--event-stream-application",
                                                                    "MY_APP",
                                                                    "--event-stream-environment",
                                                                    "MY_ENV",
                                                                    "--event-stream-token",
                                                                    "MY_TOKEN",
                                                                    "--event-stream-url",
                                                                    "",]);
            assert!(matches.is_err());
            let error = matches.unwrap_err();
            assert_eq!(error.kind, clap::ErrorKind::ValueValidation);
        }
    }
}
