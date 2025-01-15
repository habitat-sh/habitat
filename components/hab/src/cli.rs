pub mod gateway_util;
#[allow(legacy_derive_helpers)]
pub mod hab;

use crate::cli::hab::Hab;
use clap::{App,
           ArgMatches};
use habitat_common::{cli::{file_into_idents,
                           is_toml_file},
                     FeatureFlag};
use habitat_core::{origin::Origin as CoreOrigin,
                   package::{Identifiable,
                             PackageIdent}};
use std::{path::Path,
          result,
          str::FromStr};
use structopt::StructOpt;

/// Process exit code from Supervisor which indicates to Launcher that the Supervisor
/// ran to completion with a successful result. The Launcher should not attempt to restart
/// the Supervisor and should exit immediately with a successful exit code.
pub const OK_NO_RETRY_EXCODE: i32 = 84;

pub fn get(_feature_flags: FeatureFlag) -> App<'static, 'static> { Hab::clap() }

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
fn valid_origin(val: String) -> result::Result<(), String> { CoreOrigin::validate(val) }

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
                                                                   "--environment=env",]);
        assert!(r.is_ok());
        let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab",
                                                                   "svc",
                                                                   "load",
                                                                   "--application=app",
                                                                   "--environment",
                                                                   "pkg/ident",]);
        assert!(r.is_ok());
        let r = get(no_feature_flags()).get_matches_from_safe(vec!["hab",
                                                                   "svc",
                                                                   "load",
                                                                   "--application",
                                                                   "pkg/ident",]);
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
        use crate::cli::hab::sup::SupRun;

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
