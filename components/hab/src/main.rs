#![recursion_limit = "128"]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use clap::{ArgMatches,
           Shell};
use env_logger;
use futures::stream::StreamExt;
use hab::{cli::{self,
                parse_optional_arg},
          command::{self,
                    pkg::{download::{PackageSet,
                                     PackageSetFile},
                          list::ListingType}},
          config,
          error::{Error,
                  Result},
          license,
          scaffolding,
          AUTH_TOKEN_ENVVAR,
          BLDR_URL_ENVVAR,
          ORIGIN_ENVVAR,
          PRODUCT,
          VERSION};
use habitat_api_client::BuildOnUpload;
use habitat_common::{self as common,
                     cli::{cache_key_path_from_matches,
                           FS_ROOT},
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     output,
                     types::ListenCtlAddr,
                     ui::{Status,
                          UIWriter,
                          NONINTERACTIVE_ENVVAR,
                          UI},
                     FeatureFlag};
#[cfg(windows)]
use habitat_core::crypto::dpapi::encrypt;
use habitat_core::{crypto::{init,
                            keys::PairType,
                            BoxKeyPair,
                            SigKeyPair},
                   env::{self as henv,
                         Config as _},
                   fs::cache_artifact_path,
                   os::process::ShutdownTimeout,
                   package::{target,
                             PackageIdent,
                             PackageTarget},
                   service::{HealthCheckInterval,
                             ServiceGroup},
                   url::{bldr_url_from_env,
                         default_bldr_url},
                   ChannelIdent};
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol::{self as sup_proto,
                           codec::*,
                           ctl::ServiceBindList,
                           net::ErrCode,
                           types::*};
use pbr;
use std::{env,
          ffi::OsString,
          fs::File,
          io::{self,
               prelude::*,
               Read},
          net::ToSocketAddrs,
          path::{Path,
                 PathBuf},
          process,
          result,
          str::FromStr,
          string::ToString,
          thread};
use tabwriter::TabWriter;
use termcolor::{self,
                Color,
                ColorSpec};
use toml;

/// Makes the --org CLI param optional when this env var is set
const HABITAT_ORG_ENVVAR: &str = "HAB_ORG";
/// Makes the --user CLI param optional when this env var is set
const HABITAT_USER_ENVVAR: &str = "HAB_USER";

lazy_static! {
    static ref STATUS_HEADER: Vec<&'static str> = {
        vec!["package",
             "type",
             "desired",
             "state",
             "elapsed (s)",
             "pid",
             "group",]
    };
}

#[tokio::main]
async fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    let flags = FeatureFlag::from_env(&mut ui);
    if let Err(e) = start(&mut ui, flags).await {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

#[allow(clippy::cognitive_complexity)]
async fn start(ui: &mut UI, feature_flags: FeatureFlag) -> Result<()> {
    if std::env::args().skip(1).collect::<Vec<_>>() == vec!["license", "accept"] {
        license::accept_license(ui)?;
        return Ok(());
    } else {
        license::check_for_license_acceptance_and_prompt(ui)?;
    }

    // TODO JB: this feels like an anti-pattern to me. I get that in certain cases, we want to hand
    // off control from hab to a different binary to do the work, but this implementation feels
    // like it's duplicating a lot of what clap does for us. I think we should let clap do the work
    // it was designed to do, and hand off control a little bit later. Maybe there's a tiny
    // performance penalty, but the code would be much clearer.
    //
    // In addition, it creates a confusing UX because we advertise certain options via clap, e.g.
    // --url and --channel and since we're handing off control before clap has even had a chance to
    // parse the args, clap doesn't have a chance to do any validation that it needs to. We just
    // grab everything that was submitted and shove it all to the exporter or whatever other binary
    // is doing the job, and trust that it implements those flags. In some cases, e.g. the cf
    // exporter, it doesn't, so we're effectively lying to users.
    //
    // In my opinion, this function should go away and we should follow 1 standard flow for arg
    // parsing and delegation.
    exec_subcommand_if_called(ui).await?;

    let (args, remaining_args) = raw_parse_args();
    debug!("clap cli args: {:?}", &args);
    debug!("remaining cli args: {:?}", &remaining_args);

    // We build the command tree in a separate thread to eliminate
    // possible stack overflow crashes at runtime. OSX, for instance,
    // will crash with our large tree. This is a known issue:
    // https://github.com/kbknapp/clap-rs/issues/86
    let child = thread::Builder::new().stack_size(8 * 1024 * 1024)
                                      .spawn(move || {
                                          cli::get(feature_flags)
                .get_matches_from_safe_borrow(&mut args.iter())
                .unwrap_or_else(|e| {
                    e.exit();
                })
                                      })
                                      .unwrap();
    let app_matches = child.join().unwrap();

    match app_matches.subcommand() {
        ("apply", Some(m)) => sub_svc_set(m).await?,
        ("cli", Some(matches)) => {
            match matches.subcommand() {
                ("setup", Some(m)) => sub_cli_setup(ui, m)?,
                ("completers", Some(m)) => sub_cli_completers(m, feature_flags)?,
                _ => unreachable!(),
            }
        }
        ("config", Some(m)) => {
            match m.subcommand() {
                ("apply", Some(m)) => sub_svc_set(m).await?,
                ("show", Some(m)) => sub_svc_config(m).await?,
                _ => unreachable!(),
            }
        }
        ("file", Some(m)) => {
            match m.subcommand() {
                ("upload", Some(m)) => sub_file_put(m).await?,
                _ => unreachable!(),
            }
        }
        ("install", Some(m)) => sub_pkg_install(ui, m, feature_flags).await?,
        ("origin", Some(matches)) => {
            match matches.subcommand() {
                ("invitations", Some(m)) => {
                    match m.subcommand() {
                        ("accept", Some(sc)) => sub_accept_origin_invitation(ui, sc).await?,
                        ("ignore", Some(sc)) => sub_ignore_origin_invitation(ui, sc).await?,
                        ("list", Some(sc)) => sub_list_user_invitations(ui, sc).await?,
                        ("pending", Some(sc)) => sub_list_pending_origin_invitations(ui, sc).await?,
                        ("send", Some(sc)) => sub_send_origin_invitation(ui, sc).await?,
                        ("rescind", Some(sc)) => sub_rescind_origin_invitation(ui, sc).await?,
                        _ => unreachable!(),
                    }
                }
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("download", Some(sc)) => sub_origin_key_download(ui, sc).await?,
                        ("export", Some(sc)) => sub_origin_key_export(sc)?,
                        ("generate", Some(sc)) => sub_origin_key_generate(ui, sc)?,
                        ("import", Some(sc)) => sub_origin_key_import(ui, sc)?,
                        ("upload", Some(sc)) => sub_origin_key_upload(ui, sc).await?,
                        _ => unreachable!(),
                    }
                }
                ("secret", Some(m)) => {
                    match m.subcommand() {
                        ("upload", Some(sc)) => sub_origin_secret_upload(ui, sc).await?,
                        ("delete", Some(sc)) => sub_origin_secret_delete(ui, sc).await?,
                        ("list", Some(sc)) => sub_origin_secret_list(ui, sc).await?,
                        _ => unreachable!(),
                    }
                }
                ("create", Some(m)) => sub_origin_create(ui, m).await?,
                ("delete", Some(m)) => sub_origin_delete(ui, m).await?,
                ("transfer", Some(m)) => sub_origin_transfer_ownership(ui, m).await?,
                ("depart", Some(m)) => sub_origin_depart(ui, m).await?,
                ("info", Some(m)) => sub_origin_info(ui, m).await?,
                _ => unreachable!(),
            }
        }
        ("bldr", Some(matches)) => {
            match matches.subcommand() {
                ("job", Some(m)) => {
                    match m.subcommand() {
                        ("start", Some(m)) => sub_bldr_job_start(ui, m).await?,
                        ("cancel", Some(m)) => sub_bldr_job_cancel(ui, m).await?,
                        ("promote", Some(m)) => sub_bldr_job_promote_or_demote(ui, m, true).await?,
                        ("demote", Some(m)) => sub_bldr_job_promote_or_demote(ui, m, false).await?,
                        ("status", Some(m)) => sub_bldr_job_status(ui, m).await?,
                        _ => unreachable!(),
                    }
                }
                ("channel", Some(m)) => {
                    match m.subcommand() {
                        ("create", Some(m)) => sub_bldr_channel_create(ui, m).await?,
                        ("destroy", Some(m)) => sub_bldr_channel_destroy(ui, m).await?,
                        ("list", Some(m)) => sub_bldr_channel_list(ui, m).await?,
                        ("promote", Some(m)) => sub_bldr_channel_promote(ui, m).await?,
                        ("demote", Some(m)) => sub_bldr_channel_demote(ui, m).await?,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("pkg", Some(matches)) => {
            match matches.subcommand() {
                ("binds", Some(m)) => sub_pkg_binds(m)?,
                ("binlink", Some(m)) => sub_pkg_binlink(ui, m)?,
                ("build", Some(m)) => sub_pkg_build(ui, m).await?,
                ("channels", Some(m)) => sub_pkg_channels(ui, m).await?,
                ("config", Some(m)) => sub_pkg_config(m)?,
                ("dependencies", Some(m)) => sub_pkg_dependencies(m)?,
                ("download", Some(m)) => sub_pkg_download(ui, m, feature_flags).await?,
                ("env", Some(m)) => sub_pkg_env(m)?,
                ("exec", Some(m)) => sub_pkg_exec(m, &remaining_args)?,
                ("export", Some(m)) => sub_pkg_export(ui, m).await?,
                ("hash", Some(m)) => sub_pkg_hash(m)?,
                ("install", Some(m)) => sub_pkg_install(ui, m, feature_flags).await?,
                ("list", Some(m)) => sub_pkg_list(m)?,
                ("path", Some(m)) => sub_pkg_path(m)?,
                ("provides", Some(m)) => sub_pkg_provides(m)?,
                ("search", Some(m)) => sub_pkg_search(m).await?,
                ("sign", Some(m)) => sub_pkg_sign(ui, m)?,
                ("uninstall", Some(m)) => sub_pkg_uninstall(ui, m).await?,
                ("upload", Some(m)) => sub_pkg_upload(ui, m).await?,
                ("bulkupload", Some(m)) => sub_pkg_bulkupload(ui, m).await?,
                ("delete", Some(m)) => sub_pkg_delete(ui, m).await?,
                ("verify", Some(m)) => sub_pkg_verify(ui, m)?,
                ("header", Some(m)) => sub_pkg_header(ui, m)?,
                ("info", Some(m)) => sub_pkg_info(ui, m)?,
                ("promote", Some(m)) => sub_pkg_promote(ui, m).await?,
                ("demote", Some(m)) => sub_pkg_demote(ui, m).await?,
                _ => unreachable!(),
            }
        }
        ("plan", Some(matches)) => {
            match matches.subcommand() {
                ("init", Some(m)) => sub_plan_init(ui, m)?,
                ("render", Some(m)) => sub_plan_render(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("ring", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("export", Some(sc)) => sub_ring_key_export(sc)?,
                        ("import", Some(sc)) => sub_ring_key_import(ui, sc)?,
                        ("generate", Some(sc)) => sub_ring_key_generate(ui, sc)?,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("svc", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => sub_service_key_generate(ui, sc)?,
                        _ => unreachable!(),
                    }
                }
                ("load", Some(m)) => sub_svc_load(m).await?,
                ("unload", Some(m)) => sub_svc_unload(m).await?,
                ("start", Some(m)) => sub_svc_start(m).await?,
                ("stop", Some(m)) => sub_svc_stop(m).await?,
                ("status", Some(m)) => sub_svc_status(m).await?,
                _ => unreachable!(),
            }
        }
        ("sup", Some(m)) => {
            match m.subcommand() {
                ("depart", Some(m)) => sub_sup_depart(m).await?,
                ("secret", Some(m)) => {
                    match m.subcommand() {
                        ("generate", _) => sub_sup_secret_generate()?,
                        _ => unreachable!(),
                    }
                }
                // this is effectively an alias of `hab svc status`
                ("status", Some(m)) => sub_svc_status(m).await?,
                _ => unreachable!(),
            }
        }
        ("supportbundle", _) => sub_supportbundle(ui)?,
        ("setup", Some(m)) => sub_cli_setup(ui, m)?,
        ("start", Some(m)) => sub_svc_start(m).await?,
        ("stop", Some(m)) => sub_svc_stop(m).await?,
        ("user", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("generate", Some(sc)) => sub_user_key_generate(ui, sc)?,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

fn sub_cli_setup(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::cli::setup::start(ui, &cache_key_path)
}

fn sub_cli_completers(m: &ArgMatches<'_>, feature_flags: FeatureFlag) -> Result<()> {
    let shell = m.value_of("SHELL")
                 .expect("Missing Shell; A shell is required");

    // TODO (CM): Interesting... the completions generated can depend
    // on what feature flags happen to be enabled at the time you
    // generated the completions
    cli::get(feature_flags).gen_completions_to("hab",
                                               shell.parse::<Shell>().unwrap(),
                                               &mut io::stdout());
    Ok(())
}

async fn sub_origin_key_download(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").unwrap(); // Required via clap
    let revision = m.value_of("REVISION");
    let with_secret = m.is_present("WITH_SECRET");
    let with_encryption = m.is_present("WITH_ENCRYPTION");
    let token = maybe_auth_token(&m);
    let url = bldr_url_from_matches(&m)?;
    let cache_key_path = cache_key_path_from_matches(&m);

    command::origin::key::download::start(ui,
                                          &url,
                                          &origin,
                                          revision,
                                          with_secret,
                                          with_encryption,
                                          token.as_ref().map(String::as_str),
                                          &cache_key_path).await
}

fn sub_origin_key_export(m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").unwrap(); // Required via clap
    let pair_type = PairType::from_str(m.value_of("PAIR_TYPE").unwrap_or("public"))?;
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::origin::key::export::start(origin, pair_type, &cache_key_path)
}

fn sub_origin_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = origin_param_or_env(&m)?;
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::origin::key::generate::start(ui, &origin, &cache_key_path)
}

fn sub_origin_key_import(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let mut content = String::new();
    let cache_key_path = cache_key_path_from_matches(&m);
    init();
    io::stdin().read_to_string(&mut content)?;

    // Trim the content to lose line feeds added by Powershell pipeline
    command::origin::key::import::start(ui, content.trim(), &cache_key_path)
}

async fn sub_origin_key_upload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let cache_key_path = cache_key_path_from_matches(&m);

    init();

    if m.is_present("ORIGIN") {
        let origin = m.value_of("ORIGIN").unwrap();
        // you can either specify files, or infer the latest key names
        let with_secret = m.is_present("WITH_SECRET");
        command::origin::key::upload_latest::start(ui,
                                                   &url,
                                                   &token,
                                                   origin,
                                                   with_secret,
                                                   &cache_key_path).await
    } else {
        let keyfile = Path::new(m.value_of("PUBLIC_FILE").unwrap());
        let secret_keyfile = m.value_of("SECRET_FILE").map(|f| Path::new(f));
        command::origin::key::upload::start(ui, &url, &token, &keyfile, secret_keyfile).await
    }
}

async fn sub_origin_secret_upload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let origin = origin_param_or_env(&m)?;
    let key = m.value_of("KEY_NAME").unwrap();
    let secret = m.value_of("SECRET").unwrap();
    let cache_key_path = cache_key_path_from_matches(&m);
    command::origin::secret::upload::start(ui,
                                           &url,
                                           &token,
                                           &origin,
                                           &key,
                                           &secret,
                                           &cache_key_path).await
}

async fn sub_origin_secret_delete(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let origin = origin_param_or_env(&m)?;
    let key = m.value_of("KEY_NAME").unwrap();
    command::origin::secret::delete::start(ui, &url, &token, &origin, &key).await
}

async fn sub_origin_secret_list(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let origin = origin_param_or_env(&m)?;
    command::origin::secret::list::start(ui, &url, &token, &origin).await
}

async fn sub_origin_create(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::create::start(ui, &url, &token, &origin).await
}

async fn sub_origin_info(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let to_json = m.is_present("TO_JSON");
    command::origin::info::start(ui, &url, &token, &origin, to_json).await
}

async fn sub_origin_delete(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::delete::start(ui, &url, &token, &origin).await
}

async fn sub_origin_transfer_ownership(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let account = m.value_of("NEW_OWNER_ACCOUNT")
                   .expect("required NEW_OWNER_ACCOUNT");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::transfer::start(ui, &url, &token, &origin, &account).await
}

async fn sub_origin_depart(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::depart::start(ui, &url, &token, &origin).await
}

async fn sub_accept_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let invitation_id: u64 = m.value_of("INVITATION_ID")
                              .expect("required INVITATION_ID")
                              .parse()
                              .expect("INVITATION_ID should be valid at this point");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::invitations::accept::start(ui, &url, &origin, &token, invitation_id).await
}

async fn sub_ignore_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let invitation_id: u64 = m.value_of("INVITATION_ID")
                              .expect("required INVITATION_ID")
                              .parse()
                              .expect("INVITATION_ID should be valid at this point");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::invitations::ignore::start(ui, &url, &origin, &token, invitation_id).await
}

async fn sub_list_user_invitations(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::invitations::list_user::start(ui, &url, &token).await
}

async fn sub_list_pending_origin_invitations(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::invitations::list_pending_origin::start(ui, &url, &origin, &token).await
}

async fn sub_rescind_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let invitation_id: u64 = m.value_of("INVITATION_ID")
                              .expect("required INVITATION_ID")
                              .parse()
                              .expect("INVITATION_ID should be valid at this point");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::invitations::rescind::start(ui, &url, &origin, &token, invitation_id).await
}

async fn sub_send_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = m.value_of("ORIGIN").expect("required ORIGIN");
    let invitee_account = m.value_of("INVITEE_ACCOUNT")
                           .expect("required INVITEE_ACCOUNT");
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    command::origin::invitations::send::start(ui, &url, &origin, &token, &invitee_account).await
}

fn sub_pkg_binlink(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let dest_dir = Path::new(m.value_of("DEST_DIR").unwrap()); // required by clap
    let force = m.is_present("FORCE");
    match m.value_of("BINARY") {
        Some(binary) => {
            command::pkg::binlink::start(ui, &ident, &binary, dest_dir, &FS_ROOT, force)
        }
        None => command::pkg::binlink::binlink_all_in_pkg(ui, &ident, dest_dir, &FS_ROOT, force),
    }
}

async fn sub_pkg_build(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let plan_context = m.value_of("PLAN_CONTEXT").unwrap(); // Required via clap
    let root = m.value_of("HAB_STUDIO_ROOT");
    let src = m.value_of("SRC_PATH");
    let keys_string = match m.values_of("HAB_ORIGIN_KEYS") {
        Some(keys) => {
            init();
            let cache_key_path = cache_key_path_from_matches(&m);
            for key in keys.clone() {
                // Validate that all secret keys are present
                let pair = SigKeyPair::get_latest_pair_for(key, &cache_key_path, None)?;
                let _ = pair.secret();
            }
            Some(keys.collect::<Vec<_>>().join(","))
        }
        None => None,
    };
    let keys: Option<&str> = match keys_string.as_ref() {
        Some(s) => Some(s),
        None => None,
    };
    let docker = m.is_present("DOCKER");
    let reuse = m.is_present("REUSE");

    command::pkg::build::start(ui, plan_context, root, src, keys, reuse, docker).await
}

fn sub_pkg_config(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    common::command::package::config::start(&ident, &*FS_ROOT)?;
    Ok(())
}

fn sub_pkg_binds(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    common::command::package::binds::start(&ident, &*FS_ROOT)?;
    Ok(())
}

fn sub_pkg_dependencies(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let scope = if m.is_present("TRANSITIVE") {
        command::pkg::Scope::PackageAndDependencies
    } else {
        command::pkg::Scope::Package
    };

    let direction = if m.is_present("REVERSE") {
        command::pkg::DependencyRelation::Supports
    } else {
        command::pkg::DependencyRelation::Requires
    };
    command::pkg::dependencies::start(&ident, scope, direction, &*FS_ROOT)
}

async fn sub_pkg_download(ui: &mut UI,
                          m: &ArgMatches<'_>,
                          _feature_flags: FeatureFlag)
                          -> Result<()> {
    let token = maybe_auth_token(&m);
    let url = bldr_url_from_matches(&m)?;
    let download_dir = download_dir_from_matches(m);

    // Construct flat file based inputs
    let channel = channel_from_matches_or_default(m);
    let target = target_from_matches(m)?;

    let install_sources = idents_from_matches(m)?;

    let mut package_sets = vec![PackageSet { target,
                                             channel: channel.clone(),
                                             idents: install_sources }];

    let mut install_sources_from_file = idents_from_file_matches(ui, m, &channel, target)?;
    package_sets.append(&mut install_sources_from_file);
    package_sets.retain(|set| !set.idents.is_empty());

    let verify = verify_from_matches(m);
    let ignore_missing_seeds = ignore_missing_seeds_from_matches(m);

    init();

    command::pkg::download::start(ui,
                                  &url,
                                  PRODUCT,
                                  VERSION,
                                  &package_sets,
                                  download_dir.as_ref(),
                                  token.as_ref().map(String::as_str),
                                  verify,
                                  ignore_missing_seeds).await?;
    Ok(())
}

fn sub_pkg_env(m: &ArgMatches<'_>) -> Result<()> {
    let idents = idents_from_matches(m)?;
    let build = m.is_present("BUILD");
    let append_existing = m.is_present("APPEND_EXISTING");
    command::pkg::env::start(&idents, &*FS_ROOT, build, append_existing)
}

fn sub_pkg_exec(m: &ArgMatches<'_>, cmd_args: &[OsString]) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    let cmd = m.value_of("CMD").unwrap(); // Required via clap

    command::pkg::exec::start(&ident, cmd, cmd_args)
}

async fn sub_pkg_export(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let format = &m.value_of("FORMAT").unwrap();
    let url = bldr_url_from_matches(&m)?;
    let channel = channel_from_matches_or_default(&m);
    let export_fmt = command::pkg::export::format_for(ui, &format)?;
    command::pkg::export::start(ui, &url, &channel, &ident, &export_fmt).await
}

fn sub_pkg_hash(m: &ArgMatches<'_>) -> Result<()> {
    init();
    match m.value_of("SOURCE") {
        Some(source) => {
            // hash single file
            command::pkg::hash::start(&source)
        }
        None => {
            // read files from stdin
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                let file = line?;
                command::pkg::hash::start(file.trim_end())?;
            }
            Ok(())
        }
    }
}

async fn sub_pkg_uninstall(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let execute_strategy = if m.is_present("DRYRUN") {
        command::pkg::ExecutionStrategy::DryRun
    } else {
        command::pkg::ExecutionStrategy::Run
    };
    let mode = command::pkg::uninstall::UninstallMode::from(m);
    let scope = if m.is_present("NO_DEPS") {
        command::pkg::Scope::Package
    } else {
        command::pkg::Scope::PackageAndDependencies
    };
    let excludes = excludes_from_matches(&m);

    command::pkg::uninstall::start(ui,
                                   &ident,
                                   &*FS_ROOT,
                                   execute_strategy,
                                   mode,
                                   scope,
                                   &excludes).await
}

async fn sub_bldr_channel_create(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let origin = origin_param_or_env(&m)?;
    let channel = required_channel_from_matches(&m);
    let token = auth_token_param_or_env(&m)?;
    command::bldr::channel::create::start(ui, &url, &token, &origin, &channel).await
}

async fn sub_bldr_channel_destroy(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let origin = origin_param_or_env(&m)?;
    let channel = required_channel_from_matches(&m);
    let token = auth_token_param_or_env(&m)?;
    command::bldr::channel::destroy::start(ui, &url, &token, &origin, &channel).await
}

async fn sub_bldr_channel_list(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let origin = origin_param_or_env(&m)?;
    command::bldr::channel::list::start(ui, &url, &origin).await
}

async fn sub_bldr_channel_promote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let origin = origin_param_or_env(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let source_channel = required_source_channel_from_matches(&m);
    let target_channel = required_target_channel_from_matches(&m);
    command::bldr::channel::promote::start(ui,
                                           &url,
                                           &token,
                                           &origin,
                                           &source_channel,
                                           &target_channel).await
}

async fn sub_bldr_channel_demote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let origin = origin_param_or_env(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let source_channel = required_source_channel_from_matches(&m);
    let target_channel = required_target_channel_from_matches(&m);
    command::bldr::channel::demote::start(ui,
                                          &url,
                                          &token,
                                          &origin,
                                          &source_channel,
                                          &target_channel).await
}

async fn sub_bldr_job_start(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    let url = bldr_url_from_matches(&m)?;
    let target = target_from_matches(m)?;
    let group = m.is_present("GROUP");
    let token = auth_token_param_or_env(&m)?;
    command::bldr::job::start::start(ui, &url, (&ident, target), &token, group).await
}

async fn sub_bldr_job_cancel(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let group_id = m.value_of("GROUP_ID").unwrap(); // Required via clap
    let token = auth_token_param_or_env(&m)?;
    let force = m.is_present("FORCE");
    command::bldr::job::cancel::start(ui, &url, &group_id, &token, force).await
}

async fn sub_bldr_job_promote_or_demote(ui: &mut UI,
                                        m: &ArgMatches<'_>,
                                        promote: bool)
                                        -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let group_id = m.value_of("GROUP_ID").unwrap(); // Required via clap
    let channel = required_channel_from_matches(&m);
    let origin = m.value_of("ORIGIN");
    let interactive = m.is_present("INTERACTIVE");
    let verbose = m.is_present("VERBOSE");
    let token = auth_token_param_or_env(&m)?;
    command::bldr::job::promote::start(ui,
                                       &url,
                                       &group_id,
                                       &channel,
                                       origin,
                                       interactive,
                                       verbose,
                                       &token,
                                       promote).await
}

async fn sub_bldr_job_status(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let group_id = m.value_of("GROUP_ID");
    let origin = m.value_of("ORIGIN");
    let limit = m.value_of("LIMIT")
                 .unwrap_or("10")
                 .parse::<usize>()
                 .unwrap();
    let show_jobs = m.is_present("SHOW_JOBS");

    command::bldr::job::status::start(ui, &url, group_id, origin, limit, show_jobs).await
}

fn sub_plan_init(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let name = m.value_of("PKG_NAME").map(String::from);
    let origin = origin_param_or_env(&m)?;
    let minimal = m.is_present("MIN");
    let scaffolding_ident = if cfg!(windows) {
        match m.value_of("SCAFFOLDING") {
            Some(scaffold) => Some(PackageIdent::from_str(scaffold)?),
            None => None,
        }
    } else {
        scaffolding::scaffold_check(ui, m.value_of("SCAFFOLDING"))?
    };

    command::plan::init::start(ui, origin, minimal, scaffolding_ident, name)
}

fn sub_plan_render(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let template_path = Path::new(m.value_of("TEMPLATE_PATH").unwrap());

    let default_toml_path = Path::new(m.value_of("DEFAULT_TOML").unwrap());

    let user_toml_path = m.value_of("USER_TOML").map(Path::new);

    let mock_data_path = m.value_of("MOCK_DATA").map(Path::new);

    let print = m.is_present("PRINT");
    let render = !m.is_present("NO_RENDER");
    let quiet = m.is_present("QUIET");

    let render_dir = Path::new(m.value_of("RENDER_DIR").unwrap());

    command::plan::render::start(ui,
                                 template_path,
                                 default_toml_path,
                                 user_toml_path,
                                 mock_data_path,
                                 print,
                                 render,
                                 render_dir,
                                 quiet)
}

async fn sub_pkg_install(ui: &mut UI,
                         m: &ArgMatches<'_>,
                         feature_flags: FeatureFlag)
                         -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let channel = channel_from_matches_or_default(m);
    let install_sources = install_sources_from_matches(m)?;
    let token = maybe_auth_token(&m);
    let install_mode =
        if feature_flags.contains(FeatureFlag::OFFLINE_INSTALL) && m.is_present("OFFLINE") {
            InstallMode::Offline
        } else {
            InstallMode::default()
        };

    let local_package_usage =
        if feature_flags.contains(FeatureFlag::IGNORE_LOCAL) && m.is_present("IGNORE_LOCAL") {
            LocalPackageUsage::Ignore
        } else {
            LocalPackageUsage::default()
        };

    let install_hook_mode = if m.is_present("IGNORE_INSTALL_HOOK") {
        InstallHookMode::Ignore
    } else {
        InstallHookMode::default()
    };

    init();

    for install_source in install_sources.iter() {
        let pkg_install =
            common::command::package::install::start(ui,
                                                     &url,
                                                     &channel,
                                                     install_source,
                                                     PRODUCT,
                                                     VERSION,
                                                     &*FS_ROOT,
                                                     &cache_artifact_path(Some(&*FS_ROOT)),
                                                     token.as_ref().map(String::as_str),
                                                     &install_mode,
                                                     &local_package_usage,
                                                     install_hook_mode).await?;

        if let Some(dest_dir) = binlink_dest_dir_from_matches(m) {
            let force = m.is_present("FORCE");
            command::pkg::binlink::binlink_all_in_pkg(ui,
                                                      pkg_install.ident(),
                                                      &dest_dir,
                                                      &FS_ROOT,
                                                      force)?;
        }
    }
    Ok(())
}

fn sub_pkg_path(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    command::pkg::path::start(&ident, &*FS_ROOT)
}

fn sub_pkg_list(m: &ArgMatches<'_>) -> Result<()> {
    let listing_type = ListingType::from(m);

    command::pkg::list::start(&listing_type)
}

fn sub_pkg_provides(m: &ArgMatches<'_>) -> Result<()> {
    let filename = m.value_of("FILE").unwrap(); // Required via clap

    let full_releases = m.is_present("FULL_RELEASES");
    let full_paths = m.is_present("FULL_PATHS");

    command::pkg::provides::start(&filename, &*FS_ROOT, full_releases, full_paths)
}

async fn sub_pkg_search(m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let search_term = m.value_of("SEARCH_TERM").expect("required opt SEARCH_TERM");
    let limit = m.value_of("LIMIT")
                 .expect("required opt LIMIT")
                 .parse()
                 .expect("valid LIMIT");
    let token = maybe_auth_token(&m);
    command::pkg::search::start(&search_term,
                                &url,
                                limit,
                                token.as_ref().map(String::as_str)).await
}

fn sub_pkg_sign(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    let dst = Path::new(m.value_of("DEST").unwrap()); // Required via clap
    let cache_key_path = cache_key_path_from_matches(&m);
    init();
    let pair = SigKeyPair::get_latest_pair_for(&origin_param_or_env(&m)?,
                                               &cache_key_path,
                                               Some(&PairType::Secret))?;

    command::pkg::sign::start(ui, &pair, &src, &dst)
}

async fn sub_pkg_bulkupload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let upload_dir = bulkupload_dir_from_matches(m);
    let artifact_path = upload_dir.join("artifacts");
    let key_path = upload_dir.join("keys");
    let url = bldr_url_from_matches(m)?;
    let additional_release_channel = channel_from_matches(m);
    let force_upload = m.is_present("FORCE");
    let auto_build = if m.is_present("AUTO_BUILD") {
        BuildOnUpload::PackageDefault
    } else {
        BuildOnUpload::Disable
    };
    let auto_create_origins = m.is_present("AUTO_CREATE_ORIGINS");
    let token = auth_token_param_or_env(m)?;

    command::pkg::bulkupload::start(ui,
                                    &url,
                                    &additional_release_channel,
                                    &token,
                                    &artifact_path,
                                    force_upload,
                                    auto_build,
                                    auto_create_origins,
                                    &key_path).await
}

async fn sub_pkg_upload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let key_path = cache_key_path_from_matches(&m);
    let url = bldr_url_from_matches(&m)?;

    // When packages are uploaded, they *always* go to `unstable`;
    // they can optionally get added to another channel, too.
    let additional_release_channel = channel_from_matches(&m);

    // When packages are uploaded we check if they exist in the db
    // before allowing a write to the backend, this bypasses the check
    let force_upload = m.is_present("FORCE");

    let auto_build = if m.is_present("NO_BUILD") {
        BuildOnUpload::Disable
    } else {
        BuildOnUpload::PackageDefault
    };

    let token = auth_token_param_or_env(&m)?;
    let artifact_paths = m.values_of("HART_FILE").unwrap(); // Required via clap
    for artifact_path in artifact_paths.map(Path::new) {
        command::pkg::upload::start(ui,
                                    &url,
                                    &additional_release_channel,
                                    &token,
                                    artifact_path,
                                    force_upload,
                                    auto_build,
                                    &key_path).await?;
    }
    Ok(())
}

async fn sub_pkg_delete(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let token = auth_token_param_or_env(&m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let target = target_from_matches(m)?;

    command::pkg::delete::start(ui, &url, (&ident, target), &token).await?;

    Ok(())
}

fn sub_pkg_verify(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::pkg::verify::start(ui, &src, &cache_key_path)
}

fn sub_pkg_header(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    init();

    command::pkg::header::start(ui, &src)
}

fn sub_pkg_info(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    let to_json = m.is_present("TO_JSON");
    init();

    command::pkg::info::start(ui, &src, to_json)
}

async fn sub_pkg_promote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let channel = required_channel_from_matches(&m);
    let token = auth_token_param_or_env(&m)?;
    let target = target_from_matches(m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    command::pkg::promote::start(ui, &url, (&ident, target), &channel, &token).await
}

async fn sub_pkg_demote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let channel = required_channel_from_matches(&m);
    let token = auth_token_param_or_env(&m)?;
    let target = target_from_matches(m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    command::pkg::demote::start(ui, &url, (&ident, target), &channel, &token).await
}

async fn sub_pkg_channels(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(&m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    let token = maybe_auth_token(&m);
    let target = target_from_matches(m)?;

    command::pkg::channels::start(ui,
                                  &url,
                                  (&ident, target),
                                  token.as_ref().map(String::as_str)).await
}

async fn sub_svc_set(m: &ArgMatches<'_>) -> Result<()> {
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let service_group = ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap())?;
    let mut ui = ui();
    let mut validate = sup_proto::ctl::SvcValidateCfg::default();
    validate.service_group = Some(service_group.clone().into());
    let mut buf = Vec::with_capacity(sup_proto::butterfly::MAX_SVC_CFG_SIZE);
    let cfg_len = match m.value_of("FILE") {
        Some("-") | None => io::stdin().read_to_end(&mut buf)?,
        Some(f) => {
            let mut file = File::open(f)?;
            file.read_to_end(&mut buf)?
        }
    };
    if cfg_len > sup_proto::butterfly::MAX_SVC_CFG_SIZE {
        ui.fatal(format!("Configuration too large. Maximum size allowed is {} bytes.",
                         sup_proto::butterfly::MAX_SVC_CFG_SIZE))?;
        process::exit(1);
    }
    validate.cfg = Some(buf.clone());
    let cache = cache_key_path_from_matches(&m);
    let mut set = sup_proto::ctl::SvcSetCfg::default();
    match (service_group.org(), user_param_or_env(&m)) {
        (Some(_org), Some(username)) => {
            let user_pair = BoxKeyPair::get_latest_pair_for(username, &cache)?;
            let service_pair = BoxKeyPair::get_latest_pair_for(&service_group, &cache)?;
            ui.status(Status::Encrypting,
                      format!("TOML as {} for {}",
                              user_pair.name_with_rev(),
                              service_pair.name_with_rev()))?;
            set.cfg = Some(user_pair.encrypt(&buf, Some(&service_pair))?.into_bytes());
            set.is_encrypted = Some(true);
        }
        _ => set.cfg = Some(buf.to_vec()),
    }
    set.service_group = Some(service_group.into());
    set.version = Some(value_t!(m, "VERSION_NUMBER", u64).unwrap());
    ui.begin(format!("Setting new configuration version {} for {}",
                     set.version
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "UNKNOWN".to_string()),
                     set.service_group
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "UNKNOWN".to_string()),))?;
    ui.status(Status::Creating, "service configuration")?;
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, validate).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                match ErrCode::from_i32(m.code) {
                    Some(ErrCode::InvalidPayload) => {
                        ui.warn(m)?;
                    }
                    _ => return Err(SrvClientError::from(m).into()),
                }
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }
    ui.status(Status::Applying, format!("via peer {}", listen_ctl_addr))?;
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, set).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                return Err(SrvClientError::from(m).into());
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }
    ui.end("Applied configuration")?;
    Ok(())
}

async fn sub_svc_config(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let mut msg = sup_proto::ctl::SvcGetDefaultCfg::default();
    msg.ident = Some(ident.into());
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "ServiceCfg" => {
                reply.parse::<sup_proto::types::ServiceCfg>()
                     .map_err(SrvClientError::Decode)?;
            }
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                return Err(SrvClientError::from(m).into());
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }
    Ok(())
}

async fn sub_svc_load(m: &ArgMatches<'_>) -> Result<()> {
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let mut msg = svc_load_from_input(m)?;
    let ident: PackageIdent = m.value_of("PKG_IDENT").unwrap().parse()?;
    msg.ident = Some(ident.into());
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_ctl_reply(&reply)?;
    }
    Ok(())
}

async fn sub_svc_unload(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let timeout_in_seconds =
        parse_optional_arg::<ShutdownTimeout>("SHUTDOWN_TIMEOUT", m).map(u32::from);

    let msg = sup_proto::ctl::SvcUnload { ident: Some(ident.into()),
                                          timeout_in_seconds };
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_ctl_reply(&reply)?;
    }
    Ok(())
}

async fn sub_svc_start(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let mut msg = sup_proto::ctl::SvcStart::default();
    msg.ident = Some(ident.into());
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_ctl_reply(&reply)?;
    }
    Ok(())
}

async fn sub_svc_status(m: &ArgMatches<'_>) -> Result<()> {
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let mut msg = sup_proto::ctl::SvcStatus::default();
    if let Some(pkg) = m.value_of("PKG_IDENT") {
        msg.ident = Some(PackageIdent::from_str(pkg)?.into());
    }

    let mut out = TabWriter::new(io::stdout());
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    // Ensure there is at least one result from the server otherwise produce an error
    if let Some(message_result) = response.next().await {
        let reply = message_result?;
        print_svc_status(&mut out, &reply, true)?;
    } else {
        return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into());
    }
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        print_svc_status(&mut out, &reply, false)?;
    }
    out.flush()?;
    Ok(())
}

async fn sub_svc_stop(m: &ArgMatches<'_>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let timeout_in_seconds =
        parse_optional_arg::<ShutdownTimeout>("SHUTDOWN_TIMEOUT", m).map(u32::from);

    let msg = sup_proto::ctl::SvcStop { ident: Some(ident.into()),
                                        timeout_in_seconds };
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        handle_ctl_reply(&reply)?;
    }
    Ok(())
}

async fn sub_file_put(m: &ArgMatches<'_>) -> Result<()> {
    let service_group = ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap())?;
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let mut ui = ui();
    let mut msg = sup_proto::ctl::SvcFilePut::default();
    let file = Path::new(m.value_of("FILE").unwrap());
    if file.metadata()?.len() > sup_proto::butterfly::MAX_FILE_PUT_SIZE_BYTES as u64 {
        ui.fatal(format!("File too large. Maximum size allowed is {} bytes.",
                         sup_proto::butterfly::MAX_FILE_PUT_SIZE_BYTES))?;
        process::exit(1);
    };
    msg.service_group = Some(service_group.clone().into());
    msg.version = Some(value_t!(m, "VERSION_NUMBER", u64).unwrap());
    msg.filename = Some(file.file_name().unwrap().to_string_lossy().into_owned());
    let mut buf = Vec::with_capacity(sup_proto::butterfly::MAX_FILE_PUT_SIZE_BYTES);
    let cache = cache_key_path_from_matches(&m);
    ui.begin(format!("Uploading file {} to {} incarnation {}",
                     file.display(),
                     msg.version
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "UNKNOWN".to_string()),
                     msg.service_group
                        .as_ref()
                        .map(ToString::to_string)
                        .unwrap_or_else(|| "UKNOWN".to_string()),))?;
    ui.status(Status::Creating, "service file")?;
    File::open(&file)?.read_to_end(&mut buf)?;
    match (service_group.org(), user_param_or_env(&m)) {
        (Some(_org), Some(username)) => {
            let user_pair = BoxKeyPair::get_latest_pair_for(username, &cache)?;
            let service_pair = BoxKeyPair::get_latest_pair_for(&service_group, &cache)?;
            ui.status(Status::Encrypting,
                      format!("file as {} for {}",
                              user_pair.name_with_rev(),
                              service_pair.name_with_rev()))?;
            msg.content = Some(user_pair.encrypt(&buf, Some(&service_pair))?.into_bytes());
            msg.is_encrypted = Some(true);
        }
        _ => msg.content = Some(buf.to_vec()),
    }
    ui.status(Status::Applying, format!("via peer {}", listen_ctl_addr))
      .unwrap();
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                match ErrCode::from_i32(m.code) {
                    Some(ErrCode::InvalidPayload) => {
                        ui.warn(m)?;
                    }
                    _ => return Err(SrvClientError::from(m).into()),
                }
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }
    ui.end("Uploaded file")?;
    Ok(())
}

async fn sub_sup_depart(m: &ArgMatches<'_>) -> Result<()> {
    let cfg = config::load()?;
    let listen_ctl_addr = listen_ctl_addr_from_input(m)?;
    let secret_key = config::ctl_secret_key(&cfg)?;
    let mut ui = ui();
    let mut msg = sup_proto::ctl::SupDepart::default();
    msg.member_id = Some(m.value_of("MEMBER_ID").unwrap().to_string());

    ui.begin(format!("Permanently marking {} as departed",
                     msg.member_id
                        .as_ref()
                        .map(String::as_str)
                        .unwrap_or("UNKNOWN")))
      .unwrap();
    ui.status(Status::Applying, format!("via peer {}", listen_ctl_addr))
      .unwrap();
    let mut response = SrvClient::request(&listen_ctl_addr, &secret_key, msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                return Err(SrvClientError::from(m).into());
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }
    ui.end("Departure recorded.")?;
    Ok(())
}

fn sub_sup_secret_generate() -> Result<()> {
    let mut ui = ui();
    let mut buf = String::new();
    sup_proto::generate_secret_key(&mut buf);
    ui.info(buf)?;
    Ok(())
}

fn sub_supportbundle(ui: &mut UI) -> Result<()> {
    init();

    command::supportbundle::start(ui)
}

fn sub_ring_key_export(m: &ArgMatches<'_>) -> Result<()> {
    let ring = m.value_of("RING").unwrap(); // Required via clap
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::ring::key::export::start(ring, &cache_key_path)
}

fn sub_ring_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ring = m.value_of("RING").unwrap(); // Required via clap
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::ring::key::generate::start(ui, ring, &cache_key_path)
}

fn sub_ring_key_import(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let mut content = String::new();
    let cache_key_path = cache_key_path_from_matches(&m);
    init();
    io::stdin().read_to_string(&mut content)?;

    // Trim the content to lose line feeds added by Powershell pipeline
    command::ring::key::import::start(ui, content.trim(), &cache_key_path)
}

fn sub_service_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let org = org_param_or_env(&m)?;
    let service_group = ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap())?;
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::service::key::generate::start(ui, &org, &service_group, &cache_key_path)
}

fn sub_user_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let user = m.value_of("USER").unwrap(); // Required via clap
    let cache_key_path = cache_key_path_from_matches(&m);
    init();

    command::user::key::generate::start(ui, user, &cache_key_path)
}

fn args_after_first(args_to_skip: usize) -> Vec<OsString> {
    env::args_os().skip(args_to_skip).collect()
}

async fn exec_subcommand_if_called(ui: &mut UI) -> Result<()> {
    let mut args = env::args();
    let first = args.nth(1).unwrap_or_default();
    let second = args.next().unwrap_or_default();
    let third = args.next().unwrap_or_default();

    match (first.as_str(), second.as_str(), third.as_str()) {
        ("pkg", "export", "docker") => {
            command::pkg::export::docker::start(ui, &args_after_first(4)).await
        }
        ("pkg", "export", "cf") => command::pkg::export::cf::start(ui, &args_after_first(4)).await,
        ("pkg", "export", "tar") => {
            command::pkg::export::tar::start(ui, &args_after_first(4)).await
        }
        ("run", ..) => command::launcher::start(ui, &args_after_first(1)).await,
        ("stu", ..) | ("stud", ..) | ("studi", ..) | ("studio", ..) => {
            command::studio::enter::start(ui, &args_after_first(2)).await
        }
        // Skip invoking the `hab-sup` binary for sup cli help messages;
        // handle these from within `hab`
        ("help", "sup", _)
        | ("sup", _, "help")
        | ("sup", "help", _)
        | ("sup", _, "-h")
        | ("sup", "-h", _)
        | ("sup", _, "--help")
        | ("sup", "--help", _) => Ok(()),
        // Delegate remaining Supervisor subcommands to `hab-sup`
        ("sup", "", "")
        | ("sup", "term", _)
        | ("sup", "bash", _)
        | ("sup", "sh", _)
        | ("sup", "-V", _)
        | ("sup", "--version", _) => command::sup::start(ui, &args_after_first(2)).await,
        ("term", ..) => command::sup::start(ui, &args_after_first(1)).await,
        // Delegate `hab sup run *` to the Launcher
        ("sup", "run", _) => command::launcher::start(ui, &args_after_first(2)).await,
        _ => Ok(()),
    }
}

/// Parse the raw program arguments and split off any arguments that will skip clap's parsing.
///
/// **Note** with the current version of clap there is no clean way to ignore arguments after a
/// certain point, especially if those arguments look like further options and flags.
fn raw_parse_args() -> (Vec<OsString>, Vec<OsString>) {
    let mut args = env::args();
    match (args.nth(1).unwrap_or_default().as_str(), args.next().unwrap_or_default().as_str()) {
        ("pkg", "exec") => {
            if args.by_ref().count() > 2 {
                (env::args_os().take(5).collect(), env::args_os().skip(5).collect())
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        _ => (env::args_os().collect(), Vec::new()),
    }
}

/// Check to see if the user has passed in an AUTH_TOKEN param. If not, check the
/// HAB_AUTH_TOKEN env var. If not, check the CLI config to see if there is a default auth
/// token set. If that's empty too, then error.
fn auth_token_param_or_env(m: &ArgMatches<'_>) -> Result<String> {
    match m.value_of("AUTH_TOKEN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(AUTH_TOKEN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    config::load()?.auth_token.ok_or_else(|| {
                                                  Error::ArgumentError(
                    "No auth token \
                     specified"
                        .into(),
                )
                                              })
                }
            }
        }
    }
}

/// Check to see if an auth token exists and convert it to a string slice if it does. Unlike
/// auth_token_param_or_env, it's ok for no auth token to be present here. This is useful for
/// commands that can optionally take an auth token for operating on private packages.
fn maybe_auth_token(m: &ArgMatches<'_>) -> Option<String> {
    match auth_token_param_or_env(&m) {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

/// Check to see if the user has passed in an ORIGIN param.  If not, check the HABITAT_ORIGIN env
/// var. If not, check the CLI config to see if there is a default origin set. If that's empty too,
/// then error.
fn origin_param_or_env(m: &ArgMatches<'_>) -> Result<String> {
    match m.value_of("ORIGIN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(ORIGIN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    config::load()?.origin.ok_or_else(|| {
                                              Error::CryptoCLI("No origin specified".to_string())
                                          })
                }
            }
        }
    }
}

/// Check to see if the user has passed in an ORG param.
/// If not, check the HABITAT_ORG env var. If that's
/// empty too, then error.
fn org_param_or_env(m: &ArgMatches<'_>) -> Result<String> {
    match m.value_of("ORG") {
        Some(o) => Ok(o.to_string()),
        None => henv::var(HABITAT_ORG_ENVVAR)
            .map_err(|_| Error::CryptoCLI("No organization specified".to_string())),
    }
}

/// Check to see if the user has passed in a Builder URL param.  If not, check the HAB_BLDR_URL env
/// var. If not, check the CLI config to see if there is a default url set. If that's empty too,
/// then we'll use the default (https://bldr.habitat.sh).
fn bldr_url_from_matches(matches: &ArgMatches<'_>) -> Result<String> {
    match matches.value_of("BLDR_URL") {
        Some(url) => Ok(url.to_string()),
        None => {
            match henv::var(BLDR_URL_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = config::load()?;
                    match config.bldr_url {
                        Some(v) => Ok(v),
                        None => Ok(default_bldr_url()),
                    }
                }
            }
        }
    }
}

/// Resolve a channel. Taken from the environment or from CLI args, if
/// given.
fn channel_from_matches(matches: &ArgMatches<'_>) -> Option<ChannelIdent> {
    matches.value_of("CHANNEL").map(ChannelIdent::from)
}

/// Resolve a channel. Taken from the environment or from CLI args. This
/// should only be called when the argument is required by the CLAP config,
/// otherwise this would panic.
fn required_channel_from_matches(matches: &ArgMatches<'_>) -> ChannelIdent {
    channel_from_matches(matches).unwrap()
}

/// Resolve a target channel. Taken from the environment or from CLI args. This
/// should only be called when the argument is required by the CLAP config,
/// otherwise this would panic.
fn required_target_channel_from_matches(matches: &ArgMatches<'_>) -> ChannelIdent {
    matches.value_of("TARGET_CHANNEL")
           .map(ChannelIdent::from)
           .expect("TARGET_CHANNEL is a required argument!")
}

/// Resolve a source channel. Taken from the environment or from CLI args. This
/// should only be called when the argument is required by the CLAP config,
/// otherwise this would panic.
fn required_source_channel_from_matches(matches: &ArgMatches<'_>) -> ChannelIdent {
    matches.value_of("SOURCE_CHANNEL")
           .map(ChannelIdent::from)
           .expect("SOURCE_CHANNEl is a required argument!")
}
/// Resolve a channel. Taken from the environment or from CLI args, if
/// given or return the default channel value.
fn channel_from_matches_or_default(matches: &ArgMatches<'_>) -> ChannelIdent {
    channel_from_matches(matches).unwrap_or_else(ChannelIdent::configured_value)
}

/// Resolve a target. Default to x86_64-linux if none specified
fn target_from_matches(matches: &ArgMatches<'_>) -> Result<PackageTarget> {
    matches.value_of("PKG_TARGET")
           .map(PackageTarget::from_str)
           .unwrap_or_else(|| Ok(active_target()))
           .map_err(Error::HabitatCore)
}

/// Return the path to create our binlinks in, or None if no binlinking should occur
fn binlink_dest_dir_from_matches(matches: &ArgMatches<'_>) -> Option<PathBuf> {
    // is_present always returns true since BINLINK_DIR has a default value, so we need to use
    // occurrences_of to determine whether we actually want to do the binlinking
    if matches.is_present("BINLINK") || matches.occurrences_of("BINLINK_DIR") > 0 {
        matches.value_of("BINLINK_DIR").map(PathBuf::from)
    } else {
        None
    }
}

/// Helper function to determine active package target.
/// It overrides x86_64-darwin to be x86_64-linux in order
/// to provide a better user experience (ie, for the 99% case)
fn active_target() -> PackageTarget {
    match PackageTarget::active_target() {
        #[cfg(feature = "supported_targets")]
        target::X86_64_DARWIN => target::X86_64_LINUX,
        t => t,
    }
}

fn install_sources_from_matches(matches: &ArgMatches<'_>) -> Result<Vec<InstallSource>> {
    matches
        .values_of("PKG_IDENT_OR_ARTIFACT")
        .unwrap() // Required via clap
        .map(|t| t.parse().map_err(Error::from))
        .collect()
}

fn idents_from_matches(matches: &ArgMatches<'_>) -> Result<Vec<PackageIdent>> {
    match matches.values_of("PKG_IDENT") {
        Some(ident_strings) => {
            ident_strings.map(|t| PackageIdent::from_str(t).map_err(Error::from))
                         .collect()
        }
        _ => Ok(Vec::new()), // It's not an error to have no idents on command line
    }
}

fn idents_from_file_matches(ui: &mut UI,
                            matches: &ArgMatches<'_>,
                            cli_channel: &ChannelIdent,
                            cli_target: PackageTarget)
                            -> Result<Vec<PackageSet>> {
    let mut sources: Vec<PackageSet> = Vec::new();

    if let Some(files) = matches.values_of("PKG_IDENT_FILE") {
        for f in files {
            let filename = &f.to_string();
            if habitat_common::cli::is_toml_file(filename) {
                let mut package_sets = idents_from_toml_file(ui, filename)?;
                sources.append(&mut package_sets)
            } else {
                let idents_from_file = habitat_common::cli::file_into_idents(filename)?;
                let package_set = PackageSet { idents:  idents_from_file,
                                               channel: cli_channel.clone(),
                                               target:  cli_target, };
                sources.push(package_set)
            }
        }
    }
    Ok(sources)
}

fn idents_from_toml_file(ui: &mut UI, filename: &str) -> Result<Vec<PackageSet>> {
    let mut sources: Vec<PackageSet> = Vec::new();

    let file_data = std::fs::read_to_string(filename)?;
    let toml_data: PackageSetFile =
        toml::from_str(&file_data).map_err(habitat_common::Error::TomlParser)?;

    // We currently only accept version 1
    if toml_data.format_version.unwrap_or(1) != 1 {
        return Err(Error::PackageSetParseError(format!(
            "format_version invalid, only version 1 allowed ({} provided",
            toml_data.format_version.unwrap()
        )));
    }

    ui.status(Status::Using,
              format!("File {}, '{}'",
                      filename,
                      toml_data.file_descriptor.unwrap_or_else(|| "".to_string())))?;

    for (target, target_array) in toml_data.targets {
        for package_set_value in target_array {
            let channel = package_set_value.channel;
            let idents: Vec<PackageIdent> = strings_to_idents(&package_set_value.packages)?;
            let package_set = PackageSet { target,
                                           channel,
                                           idents };
            debug!("Package Set {:?}", package_set);
            sources.push(package_set)
        }
    }
    Ok(sources)
}

fn strings_to_idents(strings: &[String]) -> Result<Vec<PackageIdent>> {
    let ident_or_results: Result<Vec<PackageIdent>> =
        strings.iter()
               .map(|s| PackageIdent::from_str(&s).map_err(Error::from))
               .collect();
    ident_or_results
}

fn verify_from_matches(matches: &ArgMatches<'_>) -> bool { matches.is_present("VERIFY") }
fn ignore_missing_seeds_from_matches(matches: &ArgMatches<'_>) -> bool {
    matches.is_present("IGNORE_MISSING_SEEDS")
}

fn download_dir_from_matches(matches: &ArgMatches<'_>) -> Option<PathBuf> {
    matches.value_of("DOWNLOAD_DIRECTORY").map(PathBuf::from)
}

fn excludes_from_matches(matches: &ArgMatches<'_>) -> Vec<PackageIdent> {
    matches
        .values_of("EXCLUDE")
        .unwrap_or_default()
        .map(|i| PackageIdent::from_str(i).unwrap()) // unwrap safe as we've validated the input
        .collect()
}

fn handle_ctl_reply(reply: &SrvMessage) -> result::Result<(), SrvClientError> {
    let mut progress_bar = pbr::ProgressBar::<io::Stdout>::new(0);
    progress_bar.set_units(pbr::Units::Bytes);
    progress_bar.show_tick = true;
    progress_bar.message("    ");
    match reply.message_id() {
        "ConsoleLine" => {
            let m = reply.parse::<sup_proto::ctl::ConsoleLine>()
                         .map_err(SrvClientError::Decode)?;
            let mut new_spec = ColorSpec::new();
            let msg_spec = match m.color {
                Some(color) => {
                    new_spec.set_fg(Some(Color::from_str(&color)?))
                            .set_bold(m.bold)
                }
                None => new_spec.set_bold(m.bold),
            };
            common::ui::print(UI::default_with_env().out(), m.line.as_bytes(), msg_spec)?;
        }
        "NetProgress" => {
            let m = reply.parse::<sup_proto::ctl::NetProgress>()
                         .map_err(SrvClientError::Decode)?;
            progress_bar.total = m.total;
            if progress_bar.set(m.position) >= m.total {
                progress_bar.finish();
            }
        }
        "NetErr" => {
            let m = reply.parse::<sup_proto::net::NetErr>()
                         .map_err(SrvClientError::Decode)?;
            return Err(SrvClientError::from(m));
        }
        _ => (),
    }
    Ok(())
}

fn print_svc_status<T>(out: &mut T,
                       reply: &SrvMessage,
                       print_header: bool)
                       -> result::Result<(), SrvClientError>
    where T: io::Write
{
    let status = match reply.message_id() {
        "ServiceStatus" => {
            reply.parse::<sup_proto::types::ServiceStatus>()
                 .map_err(SrvClientError::Decode)?
        }
        "NetOk" => {
            println!("No services loaded.");
            return Ok(());
        }
        "NetErr" => {
            let err = reply.parse::<sup_proto::net::NetErr>()
                           .map_err(SrvClientError::Decode)?;
            return Err(SrvClientError::from(err));
        }
        _ => {
            warn!("Unexpected status message, {:?}", reply);
            return Ok(());
        }
    };
    let svc_desired_state = status.desired_state
                                  .map_or("<none>".to_string(), |s| s.to_string());
    let (svc_state, svc_pid, svc_elapsed) = {
        match status.process {
            Some(process) => {
                (process.state.to_string(),
                 process.pid
                        .map_or_else(|| "<none>".to_string(), |p| p.to_string()),
                 process.elapsed.unwrap_or_default().to_string())
            }
            None => {
                (ProcessState::default().to_string(), "<none>".to_string(), "<none>".to_string())
            }
        }
    };
    if print_header {
        writeln!(out, "{}", STATUS_HEADER.join("\t")).unwrap();
    }
    // Composites were removed in 0.75 but people could be
    // depending on the exact format of this output even if they
    // never used composites. We don't want to break their tooling
    // so we hardcode in 'standalone' as it's the only supported
    // package type
    //
    // TODO: Remove this when we have a stable machine-readable alternative
    // that scripts could depend on
    writeln!(out,
             "{}\tstandalone\t{}\t{}\t{}\t{}\t{}",
             status.ident,
             DesiredState::from_str(&svc_desired_state)?,
             ProcessState::from_str(&svc_state)?,
             svc_elapsed,
             svc_pid,
             status.service_group,)?;
    Ok(())
}

fn bulkupload_dir_from_matches(matches: &ArgMatches<'_>) -> PathBuf {
    matches.value_of("UPLOAD_DIRECTORY")
           .map(PathBuf::from)
           .expect("CLAP-validated upload dir")
}

/// A Builder URL, but *only* if the user specified it via CLI args or
/// the environment
fn bldr_url_from_input(m: &ArgMatches<'_>) -> Option<String> {
    m.value_of("BLDR_URL")
     .map(ToString::to_string)
     .or_else(bldr_url_from_env)
}

fn get_binds_from_input(m: &ArgMatches<'_>) -> Result<Option<ServiceBindList>> {
    match m.values_of("BIND") {
        Some(bind_strs) => {
            let mut list = ServiceBindList::default();
            for bind_str in bind_strs {
                list.binds.push(ServiceBind::from_str(bind_str)?);
            }
            Ok(Some(list))
        }
        None => Ok(None),
    }
}

fn get_binding_mode_from_input(m: &ArgMatches<'_>) -> Option<sup_proto::types::BindingMode> {
    // There won't be errors, because we validate with `valid_binding_mode`
    m.value_of("BINDING_MODE")
     .and_then(|b| BindingMode::from_str(b).ok())
}

fn get_group_from_input(m: &ArgMatches<'_>) -> Option<String> {
    m.value_of("GROUP").map(ToString::to_string)
}

fn get_health_check_interval_from_input(m: &ArgMatches<'_>)
                                        -> Option<sup_proto::types::HealthCheckInterval> {
    // Value will have already been validated by `cli::valid_health_check_interval`
    m.value_of("HEALTH_CHECK_INTERVAL")
     .and_then(|s| HealthCheckInterval::from_str(s).ok())
     .map(HealthCheckInterval::into)
}

#[cfg(target_os = "windows")]
fn get_password_from_input(m: &ArgMatches) -> Result<Option<String>> {
    if let Some(password) = m.value_of("PASSWORD") {
        Ok(Some(encrypt(password.to_string())?))
    } else {
        Ok(None)
    }
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn get_password_from_input(_m: &ArgMatches<'_>) -> Result<Option<String>> { Ok(None) }

fn get_topology_from_input(m: &ArgMatches<'_>) -> Option<Topology> {
    m.value_of("TOPOLOGY")
     .and_then(|f| Topology::from_str(f).ok())
}

fn get_strategy_from_input(m: &ArgMatches<'_>) -> Option<UpdateStrategy> {
    m.value_of("STRATEGY")
     .and_then(|f| UpdateStrategy::from_str(f).ok())
}

fn get_update_condition_from_input(m: &ArgMatches<'_>) -> Option<UpdateCondition> {
    m.value_of("UPDATE_CONDITION")
     .and_then(|f| UpdateCondition::from_str(f).ok())
}

fn listen_ctl_addr_from_input(m: &ArgMatches<'_>) -> Result<ListenCtlAddr> {
    m.value_of("REMOTE_SUP")
     .map_or(Ok(ListenCtlAddr::default()), resolve_listen_ctl_addr)
}

fn resolve_listen_ctl_addr(input: &str) -> Result<ListenCtlAddr> {
    let listen_ctl_addr = if input.find(':').is_some() {
        input.to_string()
    } else {
        format!("{}:{}", input, ListenCtlAddr::DEFAULT_PORT)
    };

    listen_ctl_addr.to_socket_addrs()
                   .and_then(|mut addrs| {
                       addrs.find(std::net::SocketAddr::is_ipv4).ok_or_else(|| {
                                                                    io::Error::new(
                    io::ErrorKind::AddrNotAvailable,
                    "Address could not be resolved.",
                )
                                                                })
                   })
                   .map(ListenCtlAddr::from)
                   .map_err(|e| Error::RemoteSupResolutionError(listen_ctl_addr, e))
}

/// Check to see if the user has passed in a USER param.
/// If not, check the HAB_USER env var. If that's
/// empty too, then return an error.
fn user_param_or_env(m: &ArgMatches<'_>) -> Option<String> {
    match m.value_of("USER") {
        Some(u) => Some(u.to_string()),
        None => {
            match env::var(HABITAT_USER_ENVVAR) {
                Ok(v) => Some(v),
                Err(_) => None,
            }
        }
    }
}

// Based on UI::default_with_env, but taking into account the setting
// of the global color variable.
//
// TODO: Ideally we'd have a unified way of setting color, so this
// function wouldn't be necessary. In the meantime, though, it'll keep
// the scope of change contained.
fn ui() -> UI {
    let isatty = if env::var(NONINTERACTIVE_ENVVAR).map(|val| val == "1" || val == "true")
                                                   .unwrap_or(false)
    {
        Some(false)
    } else {
        None
    };
    UI::default_with(output::get_format().color_choice(), isatty)
}

/// Set all fields for an `SvcLoad` message that we can from the given opts. This function
/// populates all *shared* options between `run` and `load`.
fn svc_load_from_input(m: &ArgMatches) -> Result<sup_proto::ctl::SvcLoad> {
    // TODO (DM): This check can eventually be removed.
    // See https://github.com/habitat-sh/habitat/issues/7339
    if m.is_present("APPLICATION") || m.is_present("ENVIRONMENT") {
        ui().warn("--application and --environment flags are deprecated and ignored.")?;
    }
    let mut msg = sup_proto::ctl::SvcLoad::default();
    msg.bldr_url = bldr_url_from_input(m);
    msg.bldr_channel = channel_from_matches(m).map(|c| c.to_string());
    msg.binds = get_binds_from_input(m)?;
    if m.is_present("FORCE") {
        msg.force = Some(true);
    }
    msg.group = get_group_from_input(m);
    msg.svc_encrypted_password = get_password_from_input(m)?;
    msg.health_check_interval = get_health_check_interval_from_input(m);
    msg.binding_mode = get_binding_mode_from_input(m).map(|v| v as i32);
    msg.topology = get_topology_from_input(m).map(|v| v as i32);
    msg.update_strategy = get_strategy_from_input(m).map(|v| v as i32);
    msg.update_condition = get_update_condition_from_input(m).map(|v| v as i32);
    msg.shutdown_timeout =
        parse_optional_arg::<ShutdownTimeout>("SHUTDOWN_TIMEOUT", m).map(u32::from);
    Ok(msg)
}

#[cfg(test)]
mod test {
    use super::*;

    mod binlink_dest_dir_from_matches {
        use super::*;

        habitat_core::locked_env_var!(HAB_BINLINK_DIR, lock_binlink_env_var);

        #[test]
        fn no_binlink_arg() {
            let env_var = lock_binlink_env_var();
            env_var.unset();

            assert!(dest_dir_from_pkg_install(&["origin/pkg"]).is_none(),
                    "without a --binlink arg, there should be no BINLINK matches");
        }

        #[test]
        fn env_var_but_no_binlink_arg() {
            let env_var = lock_binlink_env_var();
            env_var.set("/val/from/env/var");

            assert!(dest_dir_from_pkg_install(&["origin/pkg"]).is_none());
        }

        #[test]
        #[should_panic(expected = "Invalid value")]
        fn env_var_empty() {
            let env_var = lock_binlink_env_var();
            env_var.set("");

            dest_dir_from_pkg_install(&["origin/pkg"]);
        }

        #[test]
        fn env_var_overrides_binlink_default() {
            let env_var = lock_binlink_env_var();
            let env_var_val = "/val/from/env/var";
            env_var.set(env_var_val);

            assert_ne!(env_var_val, habitat_common::cli::DEFAULT_BINLINK_DIR);
            assert_eq!(dest_dir_from_pkg_install(&["origin/pkg", "--binlink"]),
                       Some(env_var_val.into()),
                       "with a no-value --binlink arg, the env var value should override the \
                        default");
        }

        #[test]
        fn binlink_dir_implies_binlink() {
            let env_var = lock_binlink_env_var();
            env_var.unset();

            let arg_val = "/val/from/args";
            assert_ne!(arg_val, habitat_common::cli::DEFAULT_BINLINK_DIR);
            assert_eq!(dest_dir_from_pkg_install(&["origin/pkg", "--binlink-dir", arg_val]),
                       Some(arg_val.into()));
        }

        #[test]
        fn arg_val_overrides_default() {
            let env_var = lock_binlink_env_var();
            env_var.unset();

            let arg_val = "/val/from/args";
            assert_ne!(arg_val, habitat_common::cli::DEFAULT_BINLINK_DIR);
            assert_eq!(dest_dir_from_pkg_install(&["origin/pkg",
                                                   "--binlink",
                                                   "--binlink-dir",
                                                   arg_val]),
                       Some(arg_val.into()),
                       "The --binlink value should override the default");
        }

        #[test]
        fn arg_val_overrides_env_var() {
            let env_var = lock_binlink_env_var();
            let env_var_val = "/val/from/env/var";
            env_var.set(env_var_val);
            assert_ne!(env_var_val, habitat_common::cli::DEFAULT_BINLINK_DIR);

            let arg_val = "/val/from/args";
            assert_ne!(arg_val, habitat_common::cli::DEFAULT_BINLINK_DIR);

            assert_eq!(dest_dir_from_pkg_install(&["origin/pkg",
                                                   "--binlink",
                                                   "--binlink-dir",
                                                   arg_val]),
                       Some(arg_val.into()),
                       "The --binlink value should override the env var value");
        }

        #[test]
        fn binlink_before_pkg_ident_ok() {
            let env_var = lock_binlink_env_var();
            env_var.unset();

            assert_eq!(dest_dir_from_pkg_install(&["--binlink", "origin/pkg"]),
                       Some(habitat_common::cli::DEFAULT_BINLINK_DIR.into()));
        }

        #[test]
        fn binlink_before_pkg_ident_with_env_var_ok() {
            let env_var = lock_binlink_env_var();
            let env_var_val = "/val/from/env/var";
            env_var.set(env_var_val);
            assert_ne!(env_var_val, habitat_common::cli::DEFAULT_BINLINK_DIR);

            assert_eq!(dest_dir_from_pkg_install(&["--binlink", "origin/pkg"]),
                       Some(env_var_val.into()));
        }

        fn matches_for_pkg_install<'a>(pkg_install_args: &'a [&'a str]) -> ArgMatches<'a> {
            let pre_pkg_install_args = &["hab", "pkg", "install"];
            let app_matches = cli::get(FeatureFlag::empty())
                .get_matches_from_safe(pre_pkg_install_args.iter().chain(pkg_install_args.iter()))
                .unwrap(); // Force panics on CLAP errors, so we can use #[should_panic]
            match app_matches.subcommand() {
                ("pkg", Some(matches)) => {
                    match matches.subcommand() {
                        ("install", Some(m)) => {
                            println!("{:#?}", m);
                            m.clone()
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }

        fn dest_dir_from_pkg_install(pkg_install_args: &[&str]) -> Option<PathBuf> {
            let pkg_install_matches = &matches_for_pkg_install(pkg_install_args);
            binlink_dest_dir_from_matches(pkg_install_matches)
        }
    }

    mod resolve_listen_ctl_addr {
        use super::*;

        #[test]
        fn ip_is_resolved() {
            let expected =
                ListenCtlAddr::from_str("127.0.0.1:8080").expect("Could not create ListenCtlAddr");
            let actual =
                resolve_listen_ctl_addr("127.0.0.1:8080").expect("Could not resolve string");

            assert_eq!(expected, actual);
        }

        #[test]
        fn localhost_is_resolved() {
            let expected =
                ListenCtlAddr::from_str("127.0.0.1:8080").expect("Could not create ListenCtlAddr");
            let actual =
                resolve_listen_ctl_addr("localhost:8080").expect("Could not resolve string");

            assert_eq!(expected, actual);
        }

        #[test]
        fn port_is_set_to_default_if_not_specified() {
            let expected =
                ListenCtlAddr::from_str(&format!("127.0.0.1:{}", ListenCtlAddr::DEFAULT_PORT))
                    .expect("Could not create ListenCtlAddr");
            let actual = resolve_listen_ctl_addr("localhost").expect("Could not resolve string");

            assert_eq!(expected, actual);
        }
    }
}
