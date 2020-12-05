pub mod gateway_util;
pub mod hab;

use crate::cli::hab::{bldr::{ChannelCreate,
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
                               Info as OriginInfo,
                               Ignore,
                               List,
                               Pending,
                               Rescind,
                               Rbac,
                               SecretList,
                               SecretUpload,
                               Send,
                               Transfer,
                               Upload as KeyUpload},
                      plan::{PlanInit,
                             PlanRender},
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
                            PkgInfo,
                            PkgInstall,
                            PkgHash,
                            PkgHeader,
                            PkgPath,
                            PkgPromote,
                            PkgProvides,
                            PkgSearch,
                            PkgSign,
                            PkgUninstall,
                            PkgUpload,
                            PkgVerify},
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
                      user::{Key as UserKey},
                      util::{self,
                             CACHE_KEY_PATH_DEFAULT},
                      Hab};
use clap::{App,
           AppSettings,
           Arg,
           ArgMatches};
use habitat_common::{cli::{file_into_idents,
                           is_toml_file},
                     FeatureFlag};
use habitat_core::{crypto::CACHE_KEY_PATH_ENV_VAR,
                   origin::Origin,
                   package::{Identifiable,
                             PackageIdent},
                   };
use std::{path::Path,
          result,
          str::FromStr};
use structopt::StructOpt;
use url::Url;

/// Process exit code from Supervisor which indicates to Launcher that the Supervisor
/// ran to completion with a successful result. The Launcher should not attempt to restart
/// the Supervisor and should exit immediately with a successful exit code.
pub const OK_NO_RETRY_EXCODE: i32 = 84;
pub const AFTER_HELP: &str =
    "\nALIASES:\n    apply      Alias for: 'config apply'\n    install    Alias for: 'pkg \
     install'\n    run        Alias for: 'sup run'\n    setup      Alias for: 'cli setup'\n    \
     start      Alias for: 'svc start'\n    stop       Alias for: 'svc stop'\n    term       \
     Alias for: 'sup term'\n";

pub fn get(feature_flags: FeatureFlag) -> App<'static, 'static> {
    if feature_flags.contains(FeatureFlag::STRUCTOPT_CLI) {
        return Hab::clap();
    }

    let alias_apply = ServiceConfigApply::clap().about("Alias for 'config apply'")
                                                .aliases(&["ap", "app", "appl"])
                                                .setting(AppSettings::Hidden);
    let alias_install = PkgInstall::clap().about("Alias for 'pkg install'")
                                          .aliases(&["i", "in", "ins", "inst", "insta", "instal"])
                                          .setting(AppSettings::Hidden);
    let alias_run = SupRun::clap().about("Alias for 'sup run'")
                                  .setting(AppSettings::Hidden);
    let alias_setup = CliSetup::clap().about("Alias for 'cli setup'")
                                      .aliases(&["set", "setu"])
                                      .setting(AppSettings::Hidden);
    let alias_start = SvcStart::clap().about("Alias for 'svc start'")
                                      .aliases(&["sta", "star"])
                                      .setting(AppSettings::Hidden);
    let alias_stop = SvcStop::clap().about("Alias for 'svc stop'")
                                    .aliases(&["sto"])
                                    .setting(AppSettings::Hidden);
    let alias_term = SupTerm::clap().about("Alias for 'sup term'")
                                    .setting(AppSettings::Hidden);

    clap_app!(hab =>
        (about: "Patents: https://chef.io/patents\n\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: super::VERSION)
        (author: "\nThe Habitat Maintainers <humans@habitat.sh>\n")
        (@setting GlobalVersion)
        (@setting ArgRequiredElseHelp)
        (@setting SubcommandRequiredElseHelp)
        (subcommand: License::clap().settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp]))
        (@subcommand cli =>
            (about: "Commands relating to Habitat runtime config")
            (aliases: &["cl"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: CliSetup::clap().aliases(&["s", "se", "set", "setu"]))
            (subcommand: CliCompleters::clap().aliases(&["c", "co", "com", "comp"]))
        )
        (@subcommand config =>
            (about: "Commands relating to a Service's runtime config")
            (aliases: &["co", "con", "conf", "confi"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: ServiceConfigApply::clap().aliases(&["ap", "app", "appl"]))
            (subcommand: ServiceConfigShow::clap().aliases(&["sh", "sho"]))
        )
        (@subcommand file =>
            (about: "Commands relating to Habitat files")
            (aliases: &["f", "fi", "fil"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: FileUpload::clap().aliases(&["u", "up", "upl", "uplo", "uploa"]))
        )
        (@subcommand bldr =>
            (about: "Commands relating to Habitat Builder")
            (aliases: &["b", "bl", "bld"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (@subcommand job =>
                (about: "Commands relating to Habitat Builder jobs")
                (aliases: &["j", "jo"])
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: JobStart::clap().aliases(&["s", "st", "sta", "star"]))
                (subcommand: JobCancel::clap().aliases(&["c", "ca", "can", "cance", "cancel"]))
                (subcommand: JobPromote::clap().aliases(&["p", "pr", "pro", "prom", "promo", "promot"]))
                (subcommand: JobDemote::clap().aliases(&["d", "de", "dem", "demo", "demot"])) 
                (subcommand: JobStatus::clap().aliases(&["stat", "statu"]))
            )
            (@subcommand channel =>
                (about: "Commands relating to Habitat Builder channels")
                (aliases: &["c", "ch", "cha", "chan", "chann", "channe"])
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: ChannelPromote::clap())
                (subcommand: ChannelDemote::clap())
                (subcommand: ChannelCreate::clap().aliases(&["c", "cr", "cre", "crea", "creat"]))
                (subcommand: ChannelDestroy::clap().aliases(&["d", "de", "des", "dest", "destr", "destro"]))
                (subcommand: ChannelList::clap().aliases(&["l", "li", "lis"]))
            )
        )
        (@subcommand origin =>
            (about: "Commands relating to Habitat Builder origins")
            (aliases: &["o", "or", "ori", "orig", "origi"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: OriginCreate::clap().aliases(&["cre", "crea"]))
            (subcommand: OriginDelete::clap().aliases(&["del", "dele"]))
            (subcommand: Transfer::clap() )
            (subcommand: OriginDepart::clap())
            (subcommand: OriginInfo::clap())
            (@subcommand invitations =>
                (about: "Manage origin member invitations")
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: Accept::clap())
                (subcommand: Ignore::clap())
                (subcommand: List::clap())
                (subcommand: Pending::clap())
                (subcommand: Rescind::clap())
                (subcommand: Send::clap())
            )
            (@subcommand key =>
                (about: "Commands relating to Habitat origin key maintenance")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: KeyDownload::clap().aliases(&["d", "do", "dow", "down", "downl", "downlo", "downloa"]))
                (@subcommand export =>
                    (about: "Outputs the latest origin key contents to stdout")
                    (aliases: &["e", "ex", "exp", "expo", "expor"])
                    (@arg ORIGIN: +required +takes_value {valid_origin} "The origin name")
                    (@arg KEY_TYPE: -t --type +takes_value {valid_key_type}
                        "Export either the 'public' or 'secret' key. The 'secret' key is the origin private key")
                    (arg: arg_cache_key_path())
                )
                (subcommand: OriginKeyGenerate::clap().aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"]))
                (@subcommand import =>
                    (about: "Reads a stdin stream containing a public or private origin key \
                        contents and writes the key to disk")
                    (aliases: &["i", "im", "imp", "impo", "impor"])
                    (arg: arg_cache_key_path())
                )
                (subcommand: KeyUpload::clap().aliases(&["u", "up", "upl", "uplo", "uploa"]))
            )
            (subcommand: Rbac::clap())
            (@subcommand secret =>
                (about: "Commands related to secret management")
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: SecretUpload::clap())
                (@subcommand delete =>
                    (about: "Delete a secret for your origin")
                    (@arg KEY_NAME: +required +takes_value
                        "The name of the variable key to be injected into the studio")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "The origin for which the secret will be deleted. Default is from \
                        'HAB_ORIGIN' or cli.toml")
                )
                (subcommand: SecretList::clap())
            )
        )
        (@subcommand pkg =>
            (about: "Commands relating to Habitat packages")
            (aliases: &["p", "pk", "package"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: PkgBinds::clap())
            (subcommand: PkgBinlink::clap().aliases(&["bi", "bin", "binl", "binli", "binlin"]))
            (subcommand: PkgBuild::clap())
            (subcommand: PkgConfig::clap().aliases(&["conf", "cfg"]))
            (subcommand: PkgDownload::clap())
            (subcommand: PkgEnv::clap())
            (subcommand: PkgExec::clap())
            (subcommand: ExportCommand::clap())
            (subcommand: PkgHash::clap().aliases(&["ha", "has"]))
            (subcommand: PkgInstall::clap().aliases(
                &["i", "in", "ins", "inst", "insta", "instal"]))
            (subcommand: PkgPath::clap().aliases(&["p", "pa", "pat"]))
            (subcommand: PkgList::clap().aliases(&["li"]))
            (subcommand: PkgProvides::clap())
            (subcommand: PkgSearch::clap())
            (subcommand: PkgSign::clap().aliases(&["s", "si", "sig"]))
            (subcommand: PkgUninstall::clap().aliases(&["un", "unin"]))
            // alas no hyphens in subcommand names..
            // https://github.com/clap-rs/clap/issues/1297
            (subcommand: PkgBulkupload::clap().aliases(&["bul", "bulk"]))
            (subcommand: PkgUpload::clap().aliases(&["u", "up", "upl", "uplo", "uploa"]))
            (subcommand: PkgDelete::clap().aliases(&["del", "dele"]))
            (subcommand: PkgPromote::clap().aliases(&["pr", "pro", "promo", "promot"]))
            (subcommand: PkgDemote::clap().aliases(&["de", "dem", "demo", "demot"]))
            (subcommand: PkgChannels::clap().aliases(&["ch", "cha", "chan", "chann", "channe", "channel"]))
            (subcommand: PkgVerify::clap().aliases(&["v", "ve", "ver", "veri", "verif"]))
            (subcommand: PkgHeader::clap().aliases(&["hea", "head", "heade", "header"]))
            (subcommand: PkgInfo::clap().aliases(&["inf", "info"]))
            (subcommand: PkgDependencies::clap().aliases(&["dep", "deps"]))
        )
        (@subcommand plan =>
            (about: "Commands relating to plans and other app-specific configuration")
            (aliases: &["pl", "pla"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: PlanInit::clap().aliases(&["i", "in", "ini"]))
            (subcommand: PlanRender::clap().aliases(&["r", "re", "ren", "rend", "rende"]))
        )
        (@subcommand ring =>
            (about: "Commands relating to Habitat rings")
            (aliases: &["r", "ri", "rin"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (@subcommand key =>
                (about: "Commands relating to Habitat ring keys")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: KeyExport::clap().aliases(&["e", "ex", "exp", "expo", "expor"]))
                (subcommand: KeyImport::clap().aliases(&["i", "im", "imp", "impo", "impor"]))
                (subcommand: RingKeyGenerate::clap().aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"]))
            )
        )
        (subcommand: HabSup::clap())
        (@subcommand svc =>
            (about: "Commands relating to Habitat services")
            (aliases: &["sv", "ser", "serv", "service"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: SvcBulkLoad::clap())
            (@subcommand key =>
                (about: "Commands relating to Habitat service keys")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@setting SubcommandRequiredElseHelp)
                (subcommand: KeyGenerate::clap().aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"]))
            )
            (subcommand: SvcLoad::clap())
            (subcommand: SvcUpdate::clap())
            (subcommand: SvcStart::clap().aliases(&["star"]))
            (subcommand: SvcStatus::clap().aliases(&["stat", "statu"]))
            (subcommand: SvcStop::clap().aliases(&["sto"]))
            (subcommand: SvcUnload::clap().aliases(&["u", "un", "unl", "unlo", "unloa"]))
        )
        (subcommand: Studio::clap().aliases(&["stu", "stud", "studi"]))
        (@subcommand supportbundle =>
            (about: "Create a tarball of Habitat Supervisor data to send to support")
            (aliases: &["supp", "suppo", "suppor", "support-bundle"])
        )
        (@subcommand user =>
            (about: "Commands relating to Habitat users")
            (aliases: &["u", "us", "use"])
            (@setting ArgRequiredElseHelp)
            (@setting SubcommandRequiredElseHelp)
            (subcommand: UserKey::clap().aliases(&["stu", "stud", "studi"])
                                        .settings(&[AppSettings::ArgRequiredElseHelp, AppSettings::SubcommandRequiredElseHelp])
            )
        )
        (subcommand: alias_apply)
        (subcommand: alias_install)
        (subcommand: alias_run)
        (subcommand: alias_setup)
        (subcommand: alias_start)
        (subcommand: alias_stop)
        (subcommand: alias_term)
        (after_help: AFTER_HELP)
    )
}

////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
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

////////////////////////////////////////////////////////////////////////

fn arg_cache_key_path() -> Arg<'static, 'static> {
    Arg::with_name("CACHE_KEY_PATH").long("cache-key-path")
                                    .validator(util::non_empty)
                                    .env(CACHE_KEY_PATH_ENV_VAR)
                                    .default_value(&*CACHE_KEY_PATH_DEFAULT)
                                    .help("Cache for creating and searching for encryption keys")
}

pub fn parse_optional_arg<T: FromStr>(name: &str, m: &ArgMatches) -> Option<T>
    where <T as std::str::FromStr>::Err: std::fmt::Debug
{
    m.value_of(name).map(|s| s.parse().expect("Valid argument"))
}

// CLAP Validation Functions
////////////////////////////////////////////////////////////////////////

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_key_type(val: String) -> result::Result<(), String> {
    KeyType::from_str(&val).map(|_| ()).map_err(|_| {
                                           format!("KEY_TYPE: {} is invalid, must be one of \
                                                    (public, secret)",
                                                   &val)
                                       })
}

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
fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
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
