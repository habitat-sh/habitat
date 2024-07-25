use clap::{value_t,
           ArgMatches,
           ErrorKind as ClapErrorKind,
           Shell};
use configopt::{ConfigOpt,
                Error as ConfigOptError};
use futures::stream::StreamExt;
#[cfg(any(all(target_os = "linux",
                  any(target_arch = "x86_64", target_arch = "aarch64")),
              all(target_os = "windows", target_arch = "x86_64"),))]
use hab::cli::hab::pkg::ExportCommand as PkgExportCommand;
use hab::{cli::{self,
                gateway_util,
                hab::{license::License,
                      origin::{Rbac,
                               RbacSet,
                               RbacShow},
                      pkg::PkgExec,
                      svc::{self,
                            BulkLoad as SvcBulkLoad,
                            Load as SvcLoad,
                            Svc},
                      util::{bldr_auth_token_from_args_env_or_load,
                             bldr_url_from_args_env_load_or_default},
                      Hab,
                      Origin,
                      Pkg},
                parse_optional_arg},
          command::{self,
                    pkg::{download::{PackageSet,
                                     PackageSetFile},
                          list::ListingType,
                          uninstall::UninstallHookMode}},
          error::{Error,
                  Result},
          key_type::KeyType,
          license,
          scaffolding,
          AUTH_TOKEN_ENVVAR,
          BLDR_URL_ENVVAR,
          ORIGIN_ENVVAR,
          PRODUCT,
          VERSION};
use habitat_api_client::BuildOnUpload;
use habitat_common::{self as common,
                     cli::key_cache_from_matches,
                     cli_config::CliConfig,
                     command::package::install::{InstallHookMode,
                                                 InstallMode,
                                                 InstallSource,
                                                 LocalPackageUsage},
                     types::ResolvedListenCtlAddr,
                     ui::{self,
                          Status,
                          UIWriter,
                          UI},
                     FeatureFlag};
use habitat_core::{crypto::{init,
                            keys::{Key,
                                   KeyCache}},
                   env::{self as henv,
                         Config as _},
                   fs::{cache_artifact_path,
                        FS_ROOT_PATH},
                   os::process::ShutdownTimeout,
                   package::{target,
                             PackageIdent,
                             PackageTarget},
                   service::ServiceGroup,
                   url::default_bldr_url,
                   ChannelIdent};
use habitat_sup_client::{SrvClient,
                         SrvClientError};
use habitat_sup_protocol::{self as sup_proto,
                           codec::*,
                           net::ErrCode,
                           types::*};
use lazy_static::lazy_static;
use log::{debug,
          warn};
use std::{collections::HashMap,
          convert::TryFrom,
          env,
          ffi::OsString,
          fs::File,
          io::{self,
               prelude::*,
               Read},
          path::{Path,
                 PathBuf},
          process,
          result,
          str::FromStr,
          string::ToString,
          thread};
use tabwriter::TabWriter;

#[cfg(not(target_os = "macos"))]
use hab::cli::hab::sup::{HabSup,
                         Secret,
                         Sup};
#[cfg(not(target_os = "macos"))]
use habitat_core::tls::ctl_gateway as ctl_gateway_tls;
#[cfg(not(target_os = "macos"))]
use webpki::types::DnsName;

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

pub(crate) async fn main_v2() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    let flags = FeatureFlag::from_env(&mut ui);
    if let Err(e) = start(&mut ui, flags).await {
        let exit_code = e.exit_code();
        ui.fatal(e).unwrap();
        std::process::exit(exit_code)
    }
}

#[allow(clippy::cognitive_complexity)]
async fn start(ui: &mut UI, feature_flags: FeatureFlag) -> Result<()> {
    // We parse arguments with configopt in a separate thread to eliminate
    // possible stack overflow crashes at runtime. OSX or a debug Windows build,
    // for instance, will crash with our large tree. This is a known issue:
    // https://github.com/kbknapp/clap-rs/issues/86
    let child = thread::Builder::new().stack_size(8 * 1024 * 1024)
                                      .spawn(Hab::try_from_args_with_configopt)
                                      .unwrap();
    let hab = child.join().unwrap();

    if let Ok(Hab::License(License::Accept)) = hab {
        license::accept_license(ui)?;
        return Ok(());
    }

    // Allow checking version information and displaying command help without accepting the license.
    // TODO (DM): To prevent errors in discrepancy between the structopt and cli versions only do
    // this when the license has not yet been accepted. When we switch fully to structopt this can
    // be completely removed and we should just call `Hab::from_args_with_configopt` which will
    // automatically result in this functionality.
    if !license::check_for_license_acceptance().unwrap_or_default()
                                               .accepted()
    {
        if let Err(ConfigOptError::Clap(e)) = &hab {
            if e.kind == ClapErrorKind::VersionDisplayed || e.kind == ClapErrorKind::HelpDisplayed {
                e.exit()
            }
        }
    }

    // We must manually detect a supervisor version check and call the `hab-sup` binary to get the
    // true Supervisor version.
    // TODO (DM): This is an ugly consequence of having `hab sup` subcommands handled by both the
    // `hab` binary and the `hab-sup` binary. Potential fixes:
    // 1. Handle all `hab sup` subcommands with the `hab-sup` binary
    // 2. Have a dedicated subcommand for commands handled by the `hab-sup` binary
    let mut args = env::args();
    if matches!((args.next().unwrap_or_default().as_str(),
                 args.next().unwrap_or_default().as_str(),
                 args.next().unwrap_or_default().as_str()),
                (_, "sup", "--version") | (_, "sup", "-V"))
    {
        return command::sup::start(ui, &args_after_first(2)).await;
    }

    license::check_for_license_acceptance_and_prompt(ui)?;

    // Parse and handle commands which have been migrated to use `structopt` here. Once everything
    // is migrated to use `structopt` the parsing logic below this using clap directly will be gone.
    match hab {
        Ok(hab) => {
            match hab {
                Hab::Origin(Origin::Rbac(action)) => {
                    match action {
                        Rbac::Set(rbac_set) => {
                            return sub_origin_member_role_set(ui, rbac_set).await;
                        }
                        Rbac::Show(rbac_show) => {
                            return sub_origin_member_role_show(ui, rbac_show).await;
                        }
                    }
                }
                #[cfg(not(target_os = "macos"))]
                Hab::Run(sup_run) => {
                    ui.warn("'hab run' as an alias for 'hab sup run' is deprecated. Please \
                             update your automation and processes accordingly.")?;
                    return command::launcher::start(ui, sup_run, &args_after_first(1)).await;
                }
                #[cfg(any(target_os = "macos",
                          any(all(target_os = "linux",
                                  any(target_arch = "x86_64", target_arch = "aarch64")),
                              all(target_os = "windows", target_arch = "x86_64"),)))]
                Hab::Studio(studio) => {
                    return command::studio::enter::start(ui, studio.args()).await;
                }
                #[cfg(not(target_os = "macos"))]
                Hab::Sup(sup) => {
                    match sup {
                        HabSup::Sup(sup) => {
                            // These commands are handled by the `hab-sup` or `hab-launch` binaries.
                            // We need to pass the subcommand that was issued to the underlying
                            // binary. It is a bit hacky, but to do that we strip off the `hab sup`
                            // command prefix and pass the rest of the args to underlying binary.
                            let args = args_after_first(2);
                            match sup {
                                #[cfg(any(
                                    all(target_os = "linux", any(target_arch = "x86_64", target_arch = "aarch64")),
                                    all(target_os = "windows", target_arch = "x86_64"),
                                ))]
                                Sup::Bash | Sup::Sh => {
                                    return command::sup::start(ui, &args).await;
                                }
                                Sup::Term => {
                                    return command::sup::start(ui, &args).await;
                                }
                                Sup::Run(sup_run) => {
                                    return command::launcher::start(ui, sup_run, &args).await;
                                }
                            }
                        }
                        HabSup::Depart { member_id,
                                         remote_sup, } => {
                            return sub_sup_depart(member_id, remote_sup.inner()).await;
                        }
                        HabSup::Secret(secret) => {
                            match secret {
                                Secret::Generate => return sub_sup_secret_generate(),
                                Secret::GenerateTls { subject_alternative_name,
                                                      path, } => {
                                    return sub_sup_secret_generate_key(&subject_alternative_name.dns_name()?,
                                                                       path)
                                }
                            }
                        }
                        HabSup::Status { pkg_ident,
                                         remote_sup, } => {
                            ui.warn("'hab sup status' as an alias for 'hab svc status' is \
                                     deprecated. Please update your automation and processes \
                                     accordingly.")?;
                            return sub_svc_status(pkg_ident, remote_sup.inner()).await;
                        }
                        HabSup::Restart { remote_sup } => {
                            return sub_sup_restart(remote_sup.inner()).await;
                        }
                    }
                }
                Hab::Svc(svc) => {
                    match svc {
                        Svc::BulkLoad(svc_bulk_load) => {
                            if feature_flags.contains(FeatureFlag::SERVICE_CONFIG_FILES) {
                                return sub_svc_bulk_load(svc_bulk_load).await;
                            } else {
                                return Err(Error::ArgumentError(String::from("`hab svc bulkload` is only available when `HAB_FEAT_SERVICE_CONFIG_FILES` is set")));
                            }
                        }
                        Svc::Load(svc_load) => {
                            return sub_svc_load(svc_load).await;
                        }
                        Svc::Update(svc_update) => return sub_svc_update(svc_update).await,
                        Svc::Status(svc_status) => {
                            return sub_svc_status(svc_status.pkg_ident,
                                                  svc_status.remote_sup.inner()).await;
                        }
                        _ => {
                            // All other commands will be caught by the CLI parsing logic below.
                        }
                    }
                }
                #[cfg(not(target_os = "macos"))]
                Hab::Term => {
                    ui.warn("'hab term' as an alias for 'hab sup term' is deprecated. Please \
                             update your automation and processes accordingly.")?;
                    return command::sup::start(ui, &args_after_first(1)).await;
                }
                Hab::Pkg(pkg) => {
                    #[allow(clippy::collapsible_match)]
                    match pkg {
                        // package export is not available on platforms that have no package support
                        #[cfg(any(all(target_os = "linux",
                                      any(target_arch = "x86_64", target_arch = "aarch64")),
                                  all(target_os = "windows", target_arch = "x86_64"),))]
                        Pkg::Export(export) => {
                            match export {
                                #[cfg(target_os = "linux")]
                                PkgExportCommand::Cf(args) => {
                                    return command::pkg::export::cf::start(ui, &args.args).await;
                                }
                                #[cfg(any(target_os = "linux", target_os = "windows"))]
                                PkgExportCommand::Container(args) => {
                                    return command::pkg::export::container::start(ui, &args.args).await;
                                }
                                #[cfg(any(target_os = "linux", target_os = "windows"))]
                                PkgExportCommand::Docker(args) => {
                                    ui.warn("'hab pkg export docker' is now a deprecated alias \
                                             for 'hab pkg export container'. Please update your \
                                             automation and processes accordingly.")?;
                                    return command::pkg::export::container::start(ui, &args.args).await;
                                }
                                #[cfg(target_os = "linux")]
                                PkgExportCommand::Mesos(args) => {
                                    return command::pkg::export::mesos::start(ui, &args.args).await;
                                }
                                #[cfg(any(target_os = "linux", target_os = "windows"))]
                                PkgExportCommand::Tar(args) => {
                                    return command::pkg::export::tar::start(ui, &args.args).await;
                                }
                            }
                        }
                        Pkg::Exec(PkgExec { pkg_ident,
                                            cmd,
                                            args, }) => {
                            return command::pkg::exec::start(&pkg_ident.pkg_ident(),
                                                             cmd,
                                                             &args.args);
                        }
                        _ => {
                            // All other commands will be caught by the CLI parsing logic below.
                        }
                    }
                }
                _ => {
                    // All other commands will be caught by the CLI parsing logic below.
                }
            }
        }
        Err(e @ ConfigOptError::ConfigGenerated(_)
            | e @ ConfigOptError::ConfigFile(..)
            | e @ ConfigOptError::Toml(..)) => e.exit(),
        Err(_) => {
            // Completely ignore all other errors. They will be caught by the CLI parsing logic
            // below.
        }
    };

    // Similar to the configopt parsing above We build the command tree in a
    // separate thread to eliminate possible stack overflow crashes at runtime.
    // See known issue:https://github.com/kbknapp/clap-rs/issues/86
    let cli_child = thread::Builder::new().stack_size(8 * 1024 * 1024)
                                          .spawn(move || {
                                              cli::get(feature_flags).get_matches_safe()
                                                                     .unwrap_or_else(|e| {
                                                                         e.exit();
                                                                     })
                                          })
                                          .unwrap();
    let app_matches = cli_child.join().unwrap();

    match app_matches.subcommand() {
        ("apply", Some(m)) => {
            ui.warn("'hab apply' as an alias for 'hab config apply' is deprecated. Please \
                     update your automation and processes accordingly.")?;
            sub_svc_set(m).await?
        }
        ("cli", Some(matches)) => {
            match matches.subcommand() {
                ("setup", Some(m)) => sub_cli_setup(ui, m)?,
                ("completers", Some(m)) => sub_cli_completers(m, feature_flags),
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
        ("install", Some(m)) => {
            ui.warn("'hab install' as an alias for 'hab pkg install' is deprecated. Please \
                     update your automation and processes accordingly.")?;
            sub_pkg_install(ui, m, feature_flags).await?
        }
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
                ("build", Some(m)) => sub_pkg_build(ui, m, feature_flags).await?,
                ("channels", Some(m)) => sub_pkg_channels(ui, m).await?,
                ("config", Some(m)) => sub_pkg_config(m)?,
                ("dependencies", Some(m)) => sub_pkg_dependencies(m)?,
                ("download", Some(m)) => sub_pkg_download(ui, m, feature_flags).await?,
                ("env", Some(m)) => sub_pkg_env(m)?,
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
                ("unload", Some(m)) => sub_svc_unload(m).await?,
                ("start", Some(m)) => sub_svc_start(m).await?,
                ("stop", Some(m)) => sub_svc_stop(m).await?,
                _ => unreachable!(),
            }
        }
        ("supportbundle", _) => sub_supportbundle(ui)?,
        ("setup", Some(m)) => {
            ui.warn("'hab setup' as an alias for 'hab cli setup' is deprecated. Please update \
                     your automation and processes accordingly.")?;
            sub_cli_setup(ui, m)?
        }
        ("start", Some(m)) => {
            ui.warn("'hab start' as an alias for 'hab svc start' is deprecated. Please update \
                     your automation and processes accordingly.")?;
            sub_svc_start(m).await?
        }
        ("stop", Some(m)) => {
            ui.warn("'hab stop' as an alias for 'hab svc stop' is deprecated. Please update \
                     your automation and processes accordingly.")?;
            sub_svc_stop(m).await?
        }
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
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::cli::setup::start(ui, &key_cache)
}

fn sub_cli_completers(m: &ArgMatches<'_>, feature_flags: FeatureFlag) {
    let shell = m.value_of("SHELL")
                 .expect("Missing Shell; A shell is required");

    // TODO (CM): Interesting... the completions generated can depend
    // on what feature flags happen to be enabled at the time you
    // generated the completions
    cli::get(feature_flags).gen_completions_to("hab",
                                               shell.parse::<Shell>().unwrap(),
                                               &mut io::stdout());
}

async fn sub_origin_key_download(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN").parse()?;
    let revision = m.value_of("REVISION");
    let with_secret = m.is_present("WITH_SECRET");
    let with_encryption = m.is_present("WITH_ENCRYPTION");
    let token = maybe_auth_token(m);
    let url = bldr_url_from_matches(m)?;
    let key_cache = key_cache_from_matches(m)?;

    command::origin::key::download::start(ui,
                                          &url,
                                          &origin,
                                          revision,
                                          with_secret,
                                          with_encryption,
                                          token.as_deref(),
                                          &key_cache).await
}

fn sub_origin_key_export(m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN").parse()?;
    let key_type = KeyType::from_str(m.value_of("KEY_TYPE").unwrap_or("public"))?;
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::origin::key::export::start(&origin, key_type, &key_cache)
}

fn sub_origin_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = origin_param_or_env(m)?;
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::origin::key::generate::start(ui, &origin, &key_cache)
}

fn sub_origin_key_import(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let mut content = String::new();
    let key_cache = key_cache_from_matches(m)?;
    init()?;
    io::stdin().read_to_string(&mut content)?;

    // Trim the content to lose line feeds added by Powershell pipeline
    command::origin::key::import::start(ui, content.trim(), &key_cache)
}

async fn sub_origin_key_upload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    let key_cache = key_cache_from_matches(m)?;

    init()?;

    match m.value_of("ORIGIN") {
        Some(origin) => {
            let origin = origin.parse()?;
            // you can either specify files, or infer the latest key names
            let with_secret = m.is_present("WITH_SECRET");
            command::origin::key::upload_latest::start(ui,
                                                       &url,
                                                       &token,
                                                       &origin,
                                                       with_secret,
                                                       &key_cache).await
        }
        None => {
            let keyfile = Path::new(required_value_of(m, "PUBLIC_FILE"));
            let secret_keyfile = m.value_of("SECRET_FILE").map(Path::new);
            command::origin::key::upload::start(ui, &url, &token, keyfile, secret_keyfile).await
        }
    }
}

async fn sub_origin_secret_upload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    let origin = origin_param_or_env(m)?;
    let key = required_value_of(m, "KEY_NAME");
    let secret = required_value_of(m, "SECRET");
    let key_cache = key_cache_from_matches(m)?;
    command::origin::secret::upload::start(ui, &url, &token, &origin, key, secret, &key_cache).await
}

async fn sub_origin_secret_delete(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    let origin = origin_param_or_env(m)?;
    let key = required_value_of(m, "KEY_NAME");
    command::origin::secret::delete::start(ui, &url, &token, &origin, key).await
}

async fn sub_origin_secret_list(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    let origin = origin_param_or_env(m)?;
    command::origin::secret::list::start(ui, &url, &token, &origin).await
}

async fn sub_origin_create(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::create::start(ui, &url, &token, origin).await
}

async fn sub_origin_info(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    let to_json = m.is_present("TO_JSON");
    command::origin::info::start(ui, &url, &token, origin, to_json).await
}

async fn sub_origin_delete(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::delete::start(ui, &url, &token, origin).await
}

async fn sub_origin_transfer_ownership(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let account = required_value_of(m, "NEW_OWNER_ACCOUNT");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::transfer::start(ui, &url, &token, origin, account).await
}

async fn sub_origin_depart(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::depart::start(ui, &url, &token, origin).await
}

async fn sub_accept_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let invitation_id =
        required_value_of(m, "INVITATION_ID").parse()
                                             .expect("INVITATION_ID should be valid at this point");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::invitations::accept::start(ui, &url, origin, &token, invitation_id).await
}

async fn sub_ignore_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let invitation_id =
        required_value_of(m, "INVITATION_ID").parse()
                                             .expect("INVITATION_ID should be valid at this point");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::invitations::ignore::start(ui, &url, origin, &token, invitation_id).await
}

async fn sub_list_user_invitations(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::invitations::list_user::start(ui, &url, &token).await
}

async fn sub_list_pending_origin_invitations(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::invitations::list_pending_origin::start(ui, &url, origin, &token).await
}

async fn sub_rescind_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let invitation_id =
        required_value_of(m, "INVITATION_ID").parse()
                                             .expect("INVITATION_ID should be valid at this point");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::invitations::rescind::start(ui, &url, origin, &token, invitation_id).await
}

async fn sub_send_origin_invitation(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = required_value_of(m, "ORIGIN");
    let invitee_account = required_value_of(m, "INVITEE_ACCOUNT");
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    command::origin::invitations::send::start(ui, &url, origin, &token, invitee_account).await
}

async fn sub_origin_member_role_show(ui: &mut UI, r: RbacShow) -> Result<()> {
    let bldr_url = bldr_url_from_args_env_load_or_default(r.bldr_url.value)?;
    let auth_token = bldr_auth_token_from_args_env_or_load(r.auth_token.value)?;
    command::origin::rbac::show_role::start(ui,
                                            bldr_url,
                                            r.origin.inner,
                                            &auth_token,
                                            &r.member_account,
                                            r.to_json).await
}

async fn sub_origin_member_role_set(ui: &mut UI, r: RbacSet) -> Result<()> {
    let bldr_url = bldr_url_from_args_env_load_or_default(r.bldr_url.value)?;
    let auth_token = bldr_auth_token_from_args_env_or_load(r.auth_token.value)?;
    command::origin::rbac::set_role::start(ui,
                                           bldr_url,
                                           r.origin.inner,
                                           &auth_token,
                                           &r.member_account,
                                           r.role,
                                           r.no_prompt).await
}

fn sub_pkg_binlink(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    let dest_dir = Path::new(required_value_of(m, "DEST_DIR"));
    let force = m.is_present("FORCE");
    match m.value_of("BINARY") {
        Some(binary) => {
            command::pkg::binlink::start(ui, &ident, binary, dest_dir, &FS_ROOT_PATH, force)
        }
        None => {
            command::pkg::binlink::binlink_all_in_pkg(ui, &ident, dest_dir, &FS_ROOT_PATH, force)
        }
    }
}

/// Generate a (possibly empty) list of `Origin`s from the value of
/// the `HAB_ORIGIN_KEYS` environment variable / `--keys` argument.
fn hab_key_origins(m: &ArgMatches<'_>) -> Result<Vec<habitat_core::origin::Origin>> {
    m.values_of("HAB_ORIGIN_KEYS")
     .unwrap_or_default()
     .map(|n| n.parse().map_err(Into::into))
     .collect()
}

#[allow(unused_variables)]
async fn sub_pkg_build(ui: &mut UI, m: &ArgMatches<'_>, feature_flags: FeatureFlag) -> Result<()> {
    let plan_context = required_value_of(m, "PLAN_CONTEXT");
    let root = m.value_of("HAB_STUDIO_ROOT");
    let src = m.value_of("SRC_PATH");

    let origins = hab_key_origins(m)?;
    if !origins.is_empty() {
        init()?;
        let key_cache = key_cache_from_matches(m)?;
        for origin in origins.iter() {
            // Validate that a secret signing key is present on disk
            // for each origin.
            key_cache.latest_secret_origin_signing_key(origin)?;
        }
    }

    #[cfg(target_family = "unix")]
    let native_package = if m.is_present("NATIVE_PACKAGE") {
        if !feature_flags.contains(FeatureFlag::NATIVE_PACKAGE_SUPPORT) {
            return Err(Error::ArgumentError(String::from("`--native-package` is \
                                                          only available when \
                                                          `HAB_FEAT_NATIVE_PACKAGE_SUPPORT` \
                                                          is set")));
        }
        true
    } else {
        false
    };
    #[cfg(target_family = "windows")]
    let native_package = false;

    let docker = m.is_present("DOCKER");
    let reuse = m.is_present("REUSE");

    command::pkg::build::start(ui,
                               plan_context,
                               root,
                               src,
                               &origins,
                               native_package,
                               reuse,
                               docker).await
}

fn sub_pkg_config(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    common::command::package::config::start(&ident, &*FS_ROOT_PATH)?;
    Ok(())
}

fn sub_pkg_binds(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    common::command::package::binds::start(&ident, &*FS_ROOT_PATH)?;
    Ok(())
}

fn sub_pkg_dependencies(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
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
    command::pkg::dependencies::start(&ident, scope, direction, &FS_ROOT_PATH)
}

async fn sub_pkg_download(ui: &mut UI,
                          m: &ArgMatches<'_>,
                          _feature_flags: FeatureFlag)
                          -> Result<()> {
    let token = maybe_auth_token(m);
    let url = bldr_url_from_matches(m)?;
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

    init()?;

    command::pkg::download::start(ui,
                                  &url,
                                  PRODUCT,
                                  VERSION,
                                  &package_sets,
                                  download_dir.as_ref(),
                                  token.as_deref(),
                                  verify,
                                  ignore_missing_seeds).await?;
    Ok(())
}

fn sub_pkg_env(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    command::pkg::env::start(&ident, &FS_ROOT_PATH)
}

fn sub_pkg_hash(m: &ArgMatches<'_>) -> Result<()> {
    init()?;
    match m.value_of("SOURCE") {
        Some(source) => {
            // hash single file
            command::pkg::hash::start(source)
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
    let ident = required_pkg_ident_from_input(m)?;
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
    let excludes = excludes_from_matches(m);
    let uninstall_hook_mode = if m.is_present("IGNORE_UNINSTALL_HOOK") {
        UninstallHookMode::Ignore
    } else {
        UninstallHookMode::default()
    };

    command::pkg::uninstall::start(ui,
                                   &ident,
                                   &FS_ROOT_PATH,
                                   execute_strategy,
                                   mode,
                                   scope,
                                   &excludes,
                                   uninstall_hook_mode).await
}

async fn sub_bldr_channel_create(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let origin = origin_param_or_env(m)?;
    let channel = required_channel_from_matches(m);
    let token = auth_token_param_or_env(m)?;
    command::bldr::channel::create::start(ui, &url, &token, &origin, &channel).await
}

async fn sub_bldr_channel_destroy(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let origin = origin_param_or_env(m)?;
    let channel = required_channel_from_matches(m);
    let token = auth_token_param_or_env(m)?;
    command::bldr::channel::destroy::start(ui, &url, &token, &origin, &channel).await
}

async fn sub_bldr_channel_list(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let origin = origin_param_or_env(m)?;
    let include_sandbox_channels = m.is_present("SANDBOX");
    command::bldr::channel::list::start(ui, &url, &origin, include_sandbox_channels).await
}

async fn sub_bldr_channel_promote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let origin = origin_param_or_env(m)?;
    let token = auth_token_param_or_env(m)?;
    let source_channel = required_source_channel_from_matches(m);
    let target_channel = required_target_channel_from_matches(m);
    command::bldr::channel::promote::start(ui,
                                           &url,
                                           &token,
                                           &origin,
                                           &source_channel,
                                           &target_channel).await
}

async fn sub_bldr_channel_demote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let origin = origin_param_or_env(m)?;
    let token = auth_token_param_or_env(m)?;
    let source_channel = required_source_channel_from_matches(m);
    let target_channel = required_target_channel_from_matches(m);
    command::bldr::channel::demote::start(ui,
                                          &url,
                                          &token,
                                          &origin,
                                          &source_channel,
                                          &target_channel).await
}

async fn sub_bldr_job_start(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    let url = bldr_url_from_matches(m)?;
    let target = target_from_matches(m)?;
    let group = m.is_present("GROUP");
    let token = auth_token_param_or_env(m)?;
    command::bldr::job::start::start(ui, &url, (&ident, target), &token, group).await
}

async fn sub_bldr_job_cancel(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let group_id = required_value_of(m, "GROUP_ID");
    let token = auth_token_param_or_env(m)?;
    let force = m.is_present("FORCE");
    command::bldr::job::cancel::start(ui, &url, group_id, &token, force).await
}

async fn sub_bldr_job_promote_or_demote(ui: &mut UI,
                                        m: &ArgMatches<'_>,
                                        promote: bool)
                                        -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let group_id = required_value_of(m, "GROUP_ID");
    let channel = required_channel_from_matches(m);
    let origin = m.value_of("ORIGIN");
    let interactive = m.is_present("INTERACTIVE");
    let verbose = m.is_present("VERBOSE");
    let token = auth_token_param_or_env(m)?;
    command::bldr::job::promote::start(ui,
                                       &url,
                                       group_id,
                                       &channel,
                                       origin,
                                       interactive,
                                       verbose,
                                       &token,
                                       promote).await
}

async fn sub_bldr_job_status(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
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
    let origin = origin_param_or_env(m)?;
    let minimal = m.is_present("MIN");
    let scaffolding_ident = if cfg!(windows) {
        match m.value_of("SCAFFOLDING") {
            Some(scaffold) => Some(PackageIdent::from_str(scaffold)?),
            None => None,
        }
    } else {
        scaffolding::scaffold_check(ui, m.value_of("SCAFFOLDING"))?
    };

    command::plan::init::start(ui, &origin, minimal, scaffolding_ident, name)
}

fn sub_plan_render(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let template_path = required_value_of(m, "TEMPLATE_PATH");
    let template_path = Path::new(template_path);

    let default_toml_path = required_value_of(m, "DEFAULT_TOML");
    let default_toml_path = Path::new(default_toml_path);

    let user_toml_path = m.value_of("USER_TOML").map(Path::new);

    let mock_data_path = m.value_of("MOCK_DATA").map(Path::new);

    let print = m.is_present("PRINT");
    let render = !m.is_present("NO_RENDER");
    let quiet = m.is_present("QUIET");

    let render_dir = required_value_of(m, "RENDER_DIR");
    let render_dir = Path::new(render_dir);

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
    let url = bldr_url_from_matches(m)?;
    let channel = channel_from_matches_or_default(m);
    let install_sources = install_sources_from_matches(m)?;
    let token = maybe_auth_token(m);
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

    init()?;

    for install_source in install_sources.iter() {
        let pkg_install =
            common::command::package::install::start(ui,
                                                     &url,
                                                     &channel,
                                                     install_source,
                                                     PRODUCT,
                                                     VERSION,
                                                     &FS_ROOT_PATH,
                                                     &cache_artifact_path(Some(FS_ROOT_PATH.as_path())),
                                                     token.as_deref(),
                                                     &install_mode,
                                                     &local_package_usage,
                                                     install_hook_mode).await?;

        if let Some(dest_dir) = binlink_dest_dir_from_matches(m) {
            let force = m.is_present("FORCE");
            command::pkg::binlink::binlink_all_in_pkg(ui,
                                                      pkg_install.ident(),
                                                      &dest_dir,
                                                      &FS_ROOT_PATH,
                                                      force)?;
        }
    }
    Ok(())
}

fn sub_pkg_path(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    command::pkg::path::start(&ident, &FS_ROOT_PATH)
}

fn sub_pkg_list(m: &ArgMatches<'_>) -> Result<()> {
    let listing_type = ListingType::from(m);

    command::pkg::list::start(&listing_type)
}

fn sub_pkg_provides(m: &ArgMatches<'_>) -> Result<()> {
    let filename = required_value_of(m, "FILE");

    let full_releases = m.is_present("FULL_RELEASES");
    let full_paths = m.is_present("FULL_PATHS");

    command::pkg::provides::start(filename, &FS_ROOT_PATH, full_releases, full_paths)
}

async fn sub_pkg_search(m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let search_term = required_value_of(m, "SEARCH_TERM");
    let limit = required_value_of(m, "LIMIT").parse().expect("valid LIMIT");
    let token = maybe_auth_token(m);
    command::pkg::search::start(search_term, &url, limit, token.as_deref()).await
}

fn sub_pkg_sign(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let origin = origin_param_or_env(m)?;

    let src = Path::new(required_value_of(m, "SOURCE"));
    let dst = Path::new(required_value_of(m, "DEST"));

    let key_cache = key_cache_from_matches(m)?;

    init()?;

    let key = key_cache.latest_secret_origin_signing_key(&origin)?;
    command::pkg::sign::start(ui, &key, src, dst)
}

async fn sub_pkg_bulkupload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let upload_dir = bulkupload_dir_from_matches(m);
    let artifact_path = upload_dir.join("artifacts");
    let key_path = upload_dir.join("keys");
    let key_cache = KeyCache::new(key_path);
    key_cache.setup()?;

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
                                    &key_cache).await
}

async fn sub_pkg_upload(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let key_cache = key_cache_from_matches(m)?;
    let url = bldr_url_from_matches(m)?;

    // When packages are uploaded, they *always* go to `unstable`;
    // they can optionally get added to another channel, too.
    let additional_release_channel = channel_from_matches(m);

    // When packages are uploaded we check if they exist in the db
    // before allowing a write to the backend, this bypasses the check
    let force_upload = m.is_present("FORCE");

    let auto_build = if m.is_present("NO_BUILD") {
        BuildOnUpload::Disable
    } else {
        BuildOnUpload::PackageDefault
    };

    let token = auth_token_param_or_env(m)?;
    let artifact_paths = m.values_of("HART_FILE").unwrap(); // Required via clap
    for artifact_path in artifact_paths.map(Path::new) {
        command::pkg::upload::start(ui,
                                    &url,
                                    &additional_release_channel,
                                    &token,
                                    artifact_path,
                                    force_upload,
                                    auto_build,
                                    &key_cache).await?;
    }
    Ok(())
}

async fn sub_pkg_delete(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let token = auth_token_param_or_env(m)?;
    let ident = required_pkg_ident_from_input(m)?;
    let target = target_from_matches(m)?;

    command::pkg::delete::start(ui, &url, (&ident, target), &token).await?;

    Ok(())
}

fn sub_pkg_verify(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(required_value_of(m, "SOURCE"));
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::pkg::verify::start(ui, src, &key_cache)
}

fn sub_pkg_header(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(required_value_of(m, "SOURCE"));
    init()?;

    command::pkg::header::start(ui, src)
}

fn sub_pkg_info(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let src = Path::new(required_value_of(m, "SOURCE"));
    let to_json = m.is_present("TO_JSON");
    init()?;

    command::pkg::info::start(ui, src, to_json)
}

async fn sub_pkg_promote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let channel = required_channel_from_matches(m);
    let token = auth_token_param_or_env(m)?;
    let target = target_from_matches(m)?;
    let ident = required_pkg_ident_from_input(m)?;
    command::pkg::promote::start(ui, &url, (&ident, target), &channel, &token).await
}

async fn sub_pkg_demote(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let channel = required_channel_from_matches(m);
    let token = auth_token_param_or_env(m)?;
    let target = target_from_matches(m)?;
    let ident = required_pkg_ident_from_input(m)?;
    command::pkg::demote::start(ui, &url, (&ident, target), &channel, &token).await
}

async fn sub_pkg_channels(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let url = bldr_url_from_matches(m)?;
    let ident = required_pkg_ident_from_input(m)?;
    let token = maybe_auth_token(m);
    let target = target_from_matches(m)?;

    command::pkg::channels::start(ui, &url, (&ident, target), token.as_deref()).await
}

async fn sub_svc_set(m: &ArgMatches<'_>) -> Result<()> {
    let remote_sup_addr = remote_sup_from_input(m)?;
    let remote_sup_addr = SrvClient::ctl_addr(remote_sup_addr.as_ref())?;
    let service_group = required_value_of(m, "SERVICE_GROUP").parse::<ServiceGroup>()?;
    let mut ui = ui::ui();
    let mut validate = sup_proto::ctl::SvcValidateCfg { service_group:
                                                            Some(service_group.clone().into()),
                                                        ..Default::default() };
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
    let key_cache = key_cache_from_matches(m)?;

    let mut set = sup_proto::ctl::SvcSetCfg::default();
    match (service_group.org(), user_param_or_env(m)) {
        (Some(_org), Some(username)) => {
            let user_key = key_cache.latest_user_secret_key(&username)?;
            let service_key = key_cache.latest_service_public_key(&service_group)?;
            ui.status(Status::Encrypting,
                      format!("TOML as {} for {}",
                              user_key.named_revision(),
                              service_key.named_revision()))?;
            set.cfg = Some(user_key.encrypt_for_service(&buf, &service_key)
                                   .to_string()
                                   .into_bytes());
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
    let mut response = SrvClient::request(Some(&remote_sup_addr), validate).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                match ErrCode::try_from(m.code) {
                    Ok(ErrCode::InvalidPayload) => {
                        ui.warn(m)?;
                    }
                    _ => return Err(SrvClientError::from(m).into()),
                }
            }
            _ => return Err(SrvClientError::from(io::Error::from(io::ErrorKind::UnexpectedEof)).into()),
        }
    }
    ui.status(Status::Applying, format!("via peer {}", remote_sup_addr))?;
    let mut response = SrvClient::request(Some(&remote_sup_addr), set).await?;
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
    let ident = required_pkg_ident_from_input(m)?;
    let remote_sup_addr = remote_sup_from_input(m)?;
    let msg = sup_proto::ctl::SvcGetDefaultCfg { ident: Some(ident.into()), };
    let mut response = SrvClient::request(remote_sup_addr.as_ref(), msg).await?;
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

async fn sub_svc_load(svc_load: SvcLoad) -> Result<()> {
    let remote_sup_addr = svc_load.remote_sup.clone();
    let msg = habitat_sup_protocol::ctl::SvcLoad::try_from(svc_load)?;
    gateway_util::send(remote_sup_addr.inner(), msg).await
}

async fn sub_svc_bulk_load(svc_bulk_load: SvcBulkLoad) -> Result<()> {
    let mut errors = HashMap::new();
    for svc_load in svc::svc_loads_from_paths(&svc_bulk_load.svc_config_paths)? {
        let ident = svc_load.pkg_ident.clone().pkg_ident();
        if let Err(e) = sub_svc_load(svc_load).await {
            errors.insert(ident, e);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.into())
    }
}

async fn sub_svc_unload(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    let timeout_in_seconds =
        parse_optional_arg::<ShutdownTimeout>("SHUTDOWN_TIMEOUT", m).map(u32::from);
    let msg = sup_proto::ctl::SvcUnload { ident: Some(ident.into()),
                                          timeout_in_seconds };
    let remote_sup_addr = remote_sup_from_input(m)?;
    gateway_util::send(remote_sup_addr.as_ref(), msg).await
}

async fn sub_svc_update(u: hab::cli::hab::svc::Update) -> Result<()> {
    let ctl_addr = u.remote_sup.clone();
    let msg: sup_proto::ctl::SvcUpdate = TryFrom::try_from(u)?;
    gateway_util::send(ctl_addr.inner(), msg).await
}

async fn sub_svc_start(m: &ArgMatches<'_>) -> Result<()> {
    let ident = required_pkg_ident_from_input(m)?;
    let msg = sup_proto::ctl::SvcStart { ident: Some(ident.into()), };
    let remote_sup_addr = remote_sup_from_input(m)?;
    gateway_util::send(remote_sup_addr.as_ref(), msg).await
}

async fn sub_svc_status(pkg_ident: Option<PackageIdent>,
                        remote_sup: Option<&ResolvedListenCtlAddr>)
                        -> Result<()> {
    let msg = sup_proto::ctl::SvcStatus { ident: pkg_ident.map(Into::into), };

    let mut out = TabWriter::new(io::stdout());
    let mut response = SrvClient::request(remote_sup, msg).await?;
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
    let ident = required_pkg_ident_from_input(m)?;
    let timeout_in_seconds =
        parse_optional_arg::<ShutdownTimeout>("SHUTDOWN_TIMEOUT", m).map(u32::from);
    let msg = sup_proto::ctl::SvcStop { ident: Some(ident.into()),
                                        timeout_in_seconds };
    let remote_sup_addr = remote_sup_from_input(m)?;
    gateway_util::send(remote_sup_addr.as_ref(), msg).await
}

async fn sub_file_put(m: &ArgMatches<'_>) -> Result<()> {
    let service_group = required_value_of(m, "SERVICE_GROUP").parse::<ServiceGroup>()?;
    let remote_sup_addr = remote_sup_from_input(m)?;
    let remote_sup_addr = SrvClient::ctl_addr(remote_sup_addr.as_ref())?;
    let mut ui = ui::ui();
    let mut msg = sup_proto::ctl::SvcFilePut::default();
    let file = Path::new(required_value_of(m, "FILE"));
    if file.metadata()?.len() > sup_proto::butterfly::MAX_FILE_PUT_SIZE_BYTES as u64 {
        ui.fatal(format!("File too large. Maximum size allowed is {} bytes.",
                         sup_proto::butterfly::MAX_FILE_PUT_SIZE_BYTES))?;
        process::exit(1);
    };
    msg.service_group = Some(service_group.clone().into());
    msg.version = Some(value_t!(m, "VERSION_NUMBER", u64).unwrap());
    msg.filename = Some(file.file_name().unwrap().to_string_lossy().into_owned());
    let mut buf = Vec::with_capacity(sup_proto::butterfly::MAX_FILE_PUT_SIZE_BYTES);
    let key_cache = key_cache_from_matches(m)?;

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
    File::open(file)?.read_to_end(&mut buf)?;
    match (service_group.org(), user_param_or_env(m)) {
        (Some(_org), Some(username)) => {
            // That Some(_org) bit is really "was an org specified for
            // this service group?"
            let user_key = key_cache.latest_user_secret_key(&username)?;
            let service_key = key_cache.latest_service_public_key(&service_group)?;
            ui.status(Status::Encrypting,
                      format!("file as {} for {}",
                              user_key.named_revision(),
                              service_key.named_revision()))?;
            msg.content = Some(user_key.encrypt_for_service(&buf, &service_key)
                                       .to_string()
                                       .into_bytes());
            msg.is_encrypted = Some(true);
        }
        _ => msg.content = Some(buf.to_vec()),
    }
    ui.status(Status::Applying, format!("via peer {}", remote_sup_addr))
      .unwrap();
    let mut response = SrvClient::request(Some(&remote_sup_addr), msg).await?;
    while let Some(message_result) = response.next().await {
        let reply = message_result?;
        match reply.message_id() {
            "NetOk" => (),
            "NetErr" => {
                let m = reply.parse::<sup_proto::net::NetErr>()
                             .map_err(SrvClientError::Decode)?;
                match ErrCode::try_from(m.code) {
                    Ok(ErrCode::InvalidPayload) => {
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

#[cfg(not(target_os = "macos"))]
async fn sub_sup_depart(member_id: String,
                        remote_sup: Option<&ResolvedListenCtlAddr>)
                        -> Result<()> {
    let remote_sup = SrvClient::ctl_addr(remote_sup)?;
    let mut ui = ui::ui();
    let msg = sup_proto::ctl::SupDepart { member_id: Some(member_id), };

    ui.begin(format!("Permanently marking {} as departed",
                     msg.member_id.as_deref().unwrap_or("UNKNOWN")))
      .unwrap();
    ui.status(Status::Applying, format!("via peer {}", remote_sup))
      .unwrap();
    let mut response = SrvClient::request(Some(&remote_sup), msg).await?;
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

#[cfg(not(target_os = "macos"))]
async fn sub_sup_restart(remote_sup: Option<&ResolvedListenCtlAddr>) -> Result<()> {
    let remote_sup = SrvClient::ctl_addr(remote_sup)?;
    let mut ui = ui::ui();
    let msg = sup_proto::ctl::SupRestart::default();

    ui.begin(format!("Restarting supervisor {}", remote_sup))?;
    let mut response = SrvClient::request(Some(&remote_sup), msg).await?;
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
    ui.end("Restart recorded.")?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn sub_sup_secret_generate() -> Result<()> {
    let mut ui = ui::ui();
    let mut buf = String::new();
    sup_proto::generate_secret_key(&mut buf);
    ui.info(buf)?;
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn sub_sup_secret_generate_key(subject_alternative_name: &DnsName, path: PathBuf) -> Result<()> {
    Ok(ctl_gateway_tls::generate_self_signed_certificate_and_key(subject_alternative_name, path)
        .map_err(habitat_core::Error::from)?)
}

fn sub_supportbundle(ui: &mut UI) -> Result<()> {
    init()?;

    command::supportbundle::start(ui)
}

fn sub_ring_key_export(m: &ArgMatches<'_>) -> Result<()> {
    let ring = required_value_of(m, "RING");
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::ring::key::export::start(ring, &key_cache)
}

fn sub_ring_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let ring = required_value_of(m, "RING");
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::ring::key::generate::start(ui, ring, &key_cache)
}

fn sub_ring_key_import(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let mut content = String::new();
    let key_cache = key_cache_from_matches(m)?;
    init()?;
    io::stdin().read_to_string(&mut content)?;

    // Trim the content to lose line feeds added by Powershell pipeline
    command::ring::key::import::start(ui, content.trim(), &key_cache)
}

fn sub_service_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let org = org_param_or_env(m)?;
    let service_group = required_value_of(m, "SERVICE_GROUP").parse()?;
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::service::key::generate::start(ui, &org, &service_group, &key_cache)
}

fn sub_user_key_generate(ui: &mut UI, m: &ArgMatches<'_>) -> Result<()> {
    let user = required_value_of(m, "USER");
    let key_cache = key_cache_from_matches(m)?;
    init()?;

    command::user::key::generate::start(ui, user, &key_cache)
}

fn args_after_first(args_to_skip: usize) -> Vec<OsString> {
    env::args_os().skip(args_to_skip).collect()
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
                    CliConfig::load()?.auth_token.ok_or_else(|| {
                                                     Error::ArgumentError("No auth token \
                                                                                specified"
                                                                                          .into())
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
    match auth_token_param_or_env(m) {
        Ok(t) => Some(t),
        Err(_) => None,
    }
}

/// Check to see if the user has passed in an ORIGIN param.  If not, check the HABITAT_ORIGIN env
/// var. If not, check the CLI config to see if there is a default origin set. If that's empty too,
/// then error.
// TODO (CM): sort out types better... there's a conflict with the CLI
// Origin in this module
fn origin_param_or_env(m: &ArgMatches<'_>) -> Result<habitat_core::origin::Origin> {
    match m.value_of("ORIGIN") {
        Some(o) => Ok(o.parse()?),
        None => {
            match henv::var(ORIGIN_ENVVAR) {
                Ok(v) => Ok(v.parse()?),
                Err(_) => {
                    CliConfig::load()?.origin.ok_or_else(|| {
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
                    let config = CliConfig::load()?;
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
                      toml_data.file_descriptor.unwrap_or_default()))?;

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
               .map(|s| PackageIdent::from_str(s).map_err(Error::from))
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

fn remote_sup_from_input(m: &ArgMatches<'_>) -> Result<Option<ResolvedListenCtlAddr>> {
    Ok(m.value_of("REMOTE_SUP")
        .map(ResolvedListenCtlAddr::from_str)
        .transpose()?)
}

fn required_pkg_ident_from_input(m: &ArgMatches<'_>) -> Result<PackageIdent> {
    Ok(m.value_of("PKG_IDENT")
        .expect("PKG_IDENT is a required argument")
        .parse()?)
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

/// Helper function to get information about the argument given its name
fn required_value_of<'a>(matches: &'a ArgMatches<'a>, name: &str) -> &'a str {
    matches.value_of(name)
           .unwrap_or_else(|| panic!("{} CLAP required arg missing", name))
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
}
