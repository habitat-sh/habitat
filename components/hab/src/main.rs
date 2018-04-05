// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![recursion_limit="128"]
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

extern crate clap;
extern crate env_logger;
extern crate hab;
extern crate habitat_core as hcore;
extern crate habitat_common as common;
extern crate handlebars;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate base64;

use std::env;
use std::ffi::OsString;
use std::io::{self, Read};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::thread;

use clap::{ArgMatches, Shell};

use common::command::package::install::{InstallMode, InstallSource};
use common::ui::{UI, UIWriter};
use hcore::channel;
use hcore::crypto::{init, default_cache_key_path, SigKeyPair};
use hcore::crypto::keys::PairType;
use hcore::env as henv;
use hcore::fs::{cache_artifact_path, cache_analytics_path, cache_key_path};
use hcore::package::PackageIdent;
use hcore::service::ServiceGroup;
use hcore::url::default_bldr_url;
use hcore::binlink::default_binlink_dir;

use hab::{AUTH_TOKEN_ENVVAR, CTL_SECRET_ENVVAR, ORIGIN_ENVVAR, PRODUCT, VERSION};
use hab::analytics;
use hab::cli;
use hab::command;
use hab::config::{self, Config};
use hab::feat;
use hab::scaffolding;
use hab::error::{Error, Result};

/// Makes the --org CLI param optional when this env var is set
const HABITAT_ORG_ENVVAR: &'static str = "HAB_ORG";

lazy_static! {
    /// The default filesystem root path to base all commands from. This is lazily generated on
    /// first call and reflects on the presence and value of the environment variable keyed as
    /// `FS_ROOT_ENVVAR`.
    static ref FS_ROOT: PathBuf = {
        use hcore::fs::FS_ROOT_ENVVAR;
        if let Some(root) = henv::var(FS_ROOT_ENVVAR).ok() {
            PathBuf::from(root)
        } else {
            PathBuf::from("/")
        }
    };
}

fn main() {
    env_logger::init();
    let mut ui = UI::default_with_env();
    enable_features_from_env(&mut ui);
    thread::spawn(|| analytics::instrument_subcommand());
    if let Err(e) = start(&mut ui) {
        ui.fatal(e).unwrap();
        std::process::exit(1)
    }
}

fn start(ui: &mut UI) -> Result<()> {
    exec_subcommand_if_called(ui)?;

    let (args, remaining_args) = raw_parse_args();
    debug!("clap cli args: {:?}", &args);
    debug!("remaining cli args: {:?}", &remaining_args);

    // We build the command tree in a separate thread to eliminate
    // possible stack overflow crashes at runtime. OSX, for instance,
    // will crash with our large tree. This is a known issue:
    // https://github.com/kbknapp/clap-rs/issues/86
    let child = thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(move || {
            return cli::get()
                .get_matches_from_safe_borrow(&mut args.iter())
                .unwrap_or_else(|e| {
                    analytics::instrument_clap_error(&e);
                    e.exit();
                });
        })
        .unwrap();
    let app_matches = child.join().unwrap();

    match app_matches.subcommand() {
        ("cli", Some(matches)) => {
            match matches.subcommand() {
                ("setup", Some(_)) => sub_cli_setup(ui)?,
                ("completers", Some(m)) => sub_cli_completers(m)?,
                _ => unreachable!(),
            }
        }
        ("install", Some(m)) => sub_pkg_install(ui, m)?,
        ("origin", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("download", Some(sc)) => sub_origin_key_download(ui, sc)?,
                        ("export", Some(sc)) => sub_origin_key_export(sc)?,
                        ("generate", Some(sc)) => sub_origin_key_generate(ui, sc)?,
                        ("import", Some(_)) => sub_origin_key_import(ui)?,
                        ("upload", Some(sc)) => sub_origin_key_upload(ui, sc)?,
                        _ => unreachable!(),
                    }
                }
                ("secret", Some(m)) => {
                    match m.subcommand() {
                        ("upload", Some(sc)) => sub_origin_secret_upload(ui, sc)?,
                        ("delete", Some(sc)) => sub_origin_secret_delete(ui, sc)?,
                        ("list", Some(sc)) => sub_origin_secret_list(ui, sc)?,
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            }
        }
        ("bldr", Some(matches)) => {
            match matches.subcommand() {
                ("job", Some(m)) => {
                    match m.subcommand() {
                        ("start", Some(m)) => sub_bldr_job_start(ui, m)?,
                        ("cancel", Some(m)) => sub_bldr_job_cancel(ui, m)?,
                        ("promote", Some(m)) => sub_bldr_job_promote_or_demote(ui, m, true)?,
                        ("demote", Some(m)) => sub_bldr_job_promote_or_demote(ui, m, false)?,
                        ("status", Some(m)) => sub_bldr_job_status(ui, m)?,
                        _ => unreachable!(),
                    }
                }
                ("channel", Some(m)) => {
                    match m.subcommand() {
                        ("create", Some(m)) => sub_bldr_channel_create(ui, m)?,
                        ("destroy", Some(m)) => sub_bldr_channel_destroy(ui, m)?,
                        ("list", Some(m)) => sub_bldr_channel_list(ui, m)?,
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
                ("build", Some(m)) => sub_pkg_build(ui, m)?,
                ("channels", Some(m)) => sub_pkg_channels(ui, m)?,
                ("config", Some(m)) => sub_pkg_config(m)?,
                ("env", Some(m)) => sub_pkg_env(m)?,
                ("exec", Some(m)) => sub_pkg_exec(m, remaining_args)?,
                ("export", Some(m)) => sub_pkg_export(ui, m)?,
                ("hash", Some(m)) => sub_pkg_hash(m)?,
                ("install", Some(m)) => sub_pkg_install(ui, m)?,
                ("path", Some(m)) => sub_pkg_path(m)?,
                ("provides", Some(m)) => sub_pkg_provides(m)?,
                ("search", Some(m)) => sub_pkg_search(ui, m)?,
                ("sign", Some(m)) => sub_pkg_sign(ui, m)?,
                ("upload", Some(m)) => sub_pkg_upload(ui, m)?,
                ("verify", Some(m)) => sub_pkg_verify(ui, m)?,
                ("header", Some(m)) => sub_pkg_header(ui, m)?,
                ("info", Some(m)) => sub_pkg_info(ui, m)?,
                ("promote", Some(m)) => sub_pkg_promote(ui, m)?,
                ("demote", Some(m)) => sub_pkg_demote(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("plan", Some(matches)) => {
            match matches.subcommand() {
                ("init", Some(m)) => sub_plan_init(ui, m)?,
                _ => unreachable!(),
            }
        }
        ("ring", Some(matches)) => {
            match matches.subcommand() {
                ("key", Some(m)) => {
                    match m.subcommand() {
                        ("export", Some(sc)) => sub_ring_key_export(sc)?,
                        ("import", Some(_)) => sub_ring_key_import(ui)?,
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
                _ => unreachable!(),
            }
        }
        ("setup", Some(_)) => sub_cli_setup(ui)?,
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

fn sub_cli_setup(ui: &mut UI) -> Result<()> {
    init();

    command::cli::setup::start(
        ui,
        &default_cache_key_path(Some(&*FS_ROOT)),
        &cache_analytics_path(Some(&*FS_ROOT)),
    )
}

fn sub_cli_completers(m: &ArgMatches) -> Result<()> {
    let shell = m.value_of("SHELL").expect(
        "Missing Shell; A shell is required",
    );
    cli::get().gen_completions_to("hab", shell.parse::<Shell>().unwrap(), &mut io::stdout());
    Ok(())
}

fn sub_origin_key_download(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let origin = m.value_of("ORIGIN").unwrap(); // Required via clap
    let revision = m.value_of("REVISION");
    let with_secret = m.is_present("WITH_SECRET");
    let with_encryption = m.is_present("WITH_ENCRYPTION");
    let token = maybe_auth_token(ui, &m);
    let url = bldr_url_from_matches(m);

    command::origin::key::download::start(
        ui,
        &url,
        &origin,
        revision,
        with_secret,
        with_encryption,
        token.as_ref().map(String::as_str),
        &default_cache_key_path(Some(&*FS_ROOT)),
    )
}

fn sub_origin_key_export(m: &ArgMatches) -> Result<()> {
    let origin = m.value_of("ORIGIN").unwrap(); // Required via clap
    let pair_type = PairType::from_str(m.value_of("PAIR_TYPE").unwrap_or("public"))?;
    init();

    command::origin::key::export::start(origin, pair_type, &default_cache_key_path(Some(&*FS_ROOT)))
}

fn sub_origin_key_generate(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let origin = origin_param_or_env(&m)?;
    init();

    command::origin::key::generate::start(ui, &origin, &default_cache_key_path(Some(&*FS_ROOT)))
}

fn sub_origin_key_import(ui: &mut UI) -> Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    init();

    // Trim the content to lose line feeds added by Powershell pipeline
    command::origin::key::import::start(
        ui,
        content.trim(),
        &default_cache_key_path(Some(&*FS_ROOT)),
    )
}

fn sub_origin_key_upload(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let token = auth_token_param_or_env(ui, &m)?;

    init();

    if m.is_present("ORIGIN") {
        let origin = m.value_of("ORIGIN").unwrap(); // Required via clap
        // you can either specify files, or infer the latest key names
        let with_secret = m.is_present("WITH_SECRET");
        command::origin::key::upload_latest::start(
            ui,
            &url,
            &token,
            origin,
            with_secret,
            &default_cache_key_path(Some(&*FS_ROOT)),
        )
    } else {
        let keyfile = Path::new(m.value_of("PUBLIC_FILE").unwrap());
        let secret_keyfile = m.value_of("SECRET_FILE").map(|f| Path::new(f));
        command::origin::key::upload::start(ui, &url, &token, &keyfile, secret_keyfile)
    }
}

fn sub_origin_secret_upload(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let token = auth_token_param_or_env(ui, &m)?;
    let origin = origin_param_or_env(&m)?;
    let key = m.value_of("KEY_NAME").unwrap();
    let secret = m.value_of("SECRET").unwrap();
    command::origin::secret::upload::start(
        ui,
        &url,
        &token,
        &origin,
        &key,
        &secret,
        &default_cache_key_path(Some(&*FS_ROOT)),
    )
}

fn sub_origin_secret_delete(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let token = auth_token_param_or_env(ui, &m)?;
    let origin = origin_param_or_env(&m)?;
    let key = m.value_of("KEY_NAME").unwrap();
    command::origin::secret::delete::start(ui, &url, &token, &origin, &key)
}

fn sub_origin_secret_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let token = auth_token_param_or_env(ui, &m)?;
    let origin = origin_param_or_env(&m)?;
    command::origin::secret::list::start(ui, &url, &token, &origin)
}

fn sub_pkg_binlink(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let dest_dir = binlink_dest_dir_from_matches(m);
    let force = m.is_present("FORCE");
    match m.value_of("BINARY") {
        Some(binary) => {
            command::pkg::binlink::start(ui, &ident, &binary, &dest_dir, &*FS_ROOT, force)
        }
        None => command::pkg::binlink::binlink_all_in_pkg(ui, &ident, &dest_dir, &*FS_ROOT, force),
    }
}

fn sub_pkg_build(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let plan_context = m.value_of("PLAN_CONTEXT").unwrap(); // Required via clap
    let root = m.value_of("HAB_STUDIO_ROOT");
    let src = m.value_of("SRC_PATH");
    let keys_string = match m.values_of("HAB_ORIGIN_KEYS") {
        Some(keys) => {
            init();
            for key in keys.clone() {
                // Validate that all secret keys are present
                let pair = SigKeyPair::get_latest_pair_for(
                    key,
                    &default_cache_key_path(Some(&*FS_ROOT)),
                    None,
                )?;
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
    let windows = m.is_present("WINDOWS");

    command::pkg::build::start(ui, plan_context, root, src, keys, reuse, windows, docker)
}

fn sub_pkg_config(m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    common::command::package::config::start(&ident, &*FS_ROOT)?;
    Ok(())
}

fn sub_pkg_binds(m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    common::command::package::binds::start(&ident, &*FS_ROOT)?;
    Ok(())
}

fn sub_pkg_env(m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    command::pkg::env::start(&ident, &*FS_ROOT)
}

fn sub_pkg_exec(m: &ArgMatches, cmd_args: Vec<OsString>) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    let cmd = m.value_of("CMD").unwrap(); // Required via clap

    command::pkg::exec::start(&ident, cmd, cmd_args)
}

fn sub_pkg_export(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;
    let format = &m.value_of("FORMAT").unwrap();
    let url = bldr_url_from_matches(m);
    let channel = m.value_of("CHANNEL")
        .and_then(|c| Some(c.to_string()))
        .unwrap_or(channel::default());
    let export_fmt = command::pkg::export::format_for(ui, &format)?;
    command::pkg::export::start(ui, &url, &channel, &ident, &export_fmt)
}

fn sub_pkg_hash(m: &ArgMatches) -> Result<()> {
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
                command::pkg::hash::start(file.trim_right())?;
            }
            Ok(())
        }
    }
}

fn sub_bldr_channel_create(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let origin = origin_param_or_env(&m)?;
    let channel = m.value_of("CHANNEL").unwrap(); // Required via clap
    let token = auth_token_param_or_env(ui, &m)?;
    command::bldr::channel::create::start(ui, &url, &token, &origin, &channel)
}

fn sub_bldr_channel_destroy(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let origin = origin_param_or_env(&m)?;
    let channel = m.value_of("CHANNEL").unwrap(); // Required via clap
    let token = auth_token_param_or_env(ui, &m)?;
    command::bldr::channel::destroy::start(ui, &url, &token, &origin, &channel)
}

fn sub_bldr_channel_list(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let origin = origin_param_or_env(&m)?;
    command::bldr::channel::list::start(ui, &url, &origin)
}

fn sub_bldr_job_start(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    let url = bldr_url_from_matches(m);
    let group = m.is_present("GROUP");
    let token = auth_token_param_or_env(ui, &m)?;
    command::bldr::job::start::start(ui, &url, &ident, &token, group)
}

fn sub_bldr_job_cancel(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let group_id = m.value_of("GROUP_ID").unwrap(); // Required via clap
    let token = auth_token_param_or_env(ui, &m)?;
    command::bldr::job::cancel::start(ui, &url, &group_id, &token)
}

fn sub_bldr_job_promote_or_demote(ui: &mut UI, m: &ArgMatches, promote: bool) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let group_id = m.value_of("GROUP_ID").unwrap(); // Required via clap
    let channel = m.value_of("CHANNEL").unwrap(); // Required via clap
    let origin = m.value_of("ORIGIN");
    let interactive = m.is_present("INTERACTIVE");
    let verbose = m.is_present("VERBOSE");
    let token = auth_token_param_or_env(ui, &m)?;
    command::bldr::job::promote::start(
        ui,
        &url,
        &group_id,
        &channel,
        origin,
        interactive,
        verbose,
        &token,
        promote,
    )
}

fn sub_bldr_job_status(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let group_id = m.value_of("GROUP_ID");
    let origin = m.value_of("ORIGIN");
    let limit = m.value_of("LIMIT")
        .unwrap_or("10")
        .parse::<usize>()
        .unwrap();
    let show_jobs = m.is_present("SHOW_JOBS");

    command::bldr::job::status::start(ui, &url, group_id, origin, limit, show_jobs)
}

fn sub_plan_init(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let name = m.value_of("PKG_NAME").map(|v| v.into());
    let origin = origin_param_or_env(&m)?;
    let with_docs = m.is_present("WITH_DOCS");
    let with_callbacks = m.is_present("WITH_CALLBACKS");
    let with_all = m.is_present("WITH_ALL");
    let windows = m.is_present("WINDOWS");
    let scaffolding_ident = if windows {
        match m.value_of("SCAFFOLDING") {
            Some(scaffold) => Some(PackageIdent::from_str(scaffold)?),
            None => None,
        }
    } else {
        scaffolding::scaffold_check(ui, m.value_of("SCAFFOLDING"))?
    };

    command::plan::init::start(
        ui,
        origin,
        with_docs,
        with_callbacks,
        with_all,
        windows,
        scaffolding_ident,
        name,
    )
}

fn sub_pkg_install(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let channel = channel_from_matches(m);
    let install_sources = install_sources_from_matches(m)?;
    let token = maybe_auth_token(ui, &m);
    let install_mode = if feat::is_enabled(feat::OfflineInstall) && m.is_present("OFFLINE") {
        InstallMode::Offline
    } else {
        InstallMode::default()
    };

    init();

    for install_source in install_sources.iter() {
        let pkg_install = common::command::package::install::start(
            ui,
            &url,
            Some(&channel),
            install_source,
            PRODUCT,
            VERSION,
            &*FS_ROOT,
            &cache_artifact_path(Some(&*FS_ROOT)),
            token.as_ref().map(String::as_str),
            &install_mode,
        )?;

        if m.is_present("BINLINK") {
            let dest_dir = binlink_dest_dir_from_matches(m);
            let force = m.is_present("FORCE");
            command::pkg::binlink::binlink_all_in_pkg(
                ui,
                pkg_install.ident(),
                dest_dir,
                &*FS_ROOT,
                force,
            )?;
        }
    }
    Ok(())
}

fn sub_pkg_path(m: &ArgMatches) -> Result<()> {
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?;

    command::pkg::path::start(&ident, &*FS_ROOT)
}

fn sub_pkg_provides(m: &ArgMatches) -> Result<()> {
    let filename = m.value_of("FILE").unwrap(); // Required via clap

    let full_releases = m.is_present("FULL_RELEASES");
    let full_paths = m.is_present("FULL_PATHS");

    command::pkg::provides::start(&filename, &*FS_ROOT, full_releases, full_paths)
}

fn sub_pkg_search(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let search_term = m.value_of("SEARCH_TERM").unwrap(); // Required via clap
    let token = maybe_auth_token(ui, &m);
    command::pkg::search::start(&search_term, &url, token.as_ref().map(String::as_str))
}

fn sub_pkg_sign(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    let dst = Path::new(m.value_of("DEST").unwrap()); // Required via clap
    init();
    let pair = SigKeyPair::get_latest_pair_for(
        &origin_param_or_env(&m)?,
        &default_cache_key_path(Some(&*FS_ROOT)),
        Some(&PairType::Secret),
    )?;

    command::pkg::sign::start(ui, &pair, &src, &dst)
}

fn sub_pkg_upload(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let key_path = cache_key_path(Some(&*FS_ROOT));
    let url = bldr_url_from_matches(m);

    // When packages are uploaded, they *always* go to `unstable`;
    // they can optionally get added to another channel, too.
    let additional_release_channel: Option<&str> = m.value_of("CHANNEL");

    let token = auth_token_param_or_env(ui, &m)?;
    let artifact_paths = m.values_of("HART_FILE").unwrap(); // Required via clap
    for artifact_path in artifact_paths {
        command::pkg::upload::start(
            ui,
            &url,
            additional_release_channel,
            &token,
            &artifact_path,
            &key_path,
        )?;
    }
    Ok(())
}

fn sub_pkg_verify(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    init();

    command::pkg::verify::start(ui, &src, &default_cache_key_path(Some(&*FS_ROOT)))
}

fn sub_pkg_header(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    init();

    command::pkg::header::start(ui, &src)
}

fn sub_pkg_info(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let src = Path::new(m.value_of("SOURCE").unwrap()); // Required via clap
    let to_json = m.is_present("TO_JSON");
    init();

    command::pkg::info::start(ui, &src, to_json)
}

fn sub_pkg_promote(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let channel = m.value_of("CHANNEL").unwrap();
    let token = auth_token_param_or_env(ui, &m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    command::pkg::promote::start(ui, &url, &ident, &channel, &token)
}

fn sub_pkg_demote(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let channel = m.value_of("CHANNEL").unwrap();
    let token = auth_token_param_or_env(ui, &m)?;
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    command::pkg::demote::start(ui, &url, &ident, &channel, &token)
}

fn sub_pkg_channels(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let url = bldr_url_from_matches(m);
    let ident = PackageIdent::from_str(m.value_of("PKG_IDENT").unwrap())?; // Required via clap
    let token = maybe_auth_token(ui, &m);

    command::pkg::channels::start(ui, &url, &ident, token.as_ref().map(String::as_str))
}

fn sub_ring_key_export(m: &ArgMatches) -> Result<()> {
    let ring = m.value_of("RING").unwrap(); // Required via clap
    init();

    command::ring::key::export::start(ring, &default_cache_key_path(Some(&*FS_ROOT)))
}

fn sub_ring_key_generate(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let ring = m.value_of("RING").unwrap(); // Required via clap
    init();

    command::ring::key::generate::start(ui, ring, &default_cache_key_path(Some(&*FS_ROOT)))
}

fn sub_ring_key_import(ui: &mut UI) -> Result<()> {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;
    init();

    // Trim the content to lose line feeds added by Powershell pipeline
    command::ring::key::import::start(ui, content.trim(), &default_cache_key_path(Some(&*FS_ROOT)))
}

fn sub_service_key_generate(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let org = org_param_or_env(&m)?;
    let service_group = ServiceGroup::from_str(m.value_of("SERVICE_GROUP").unwrap())?;
    init();

    command::service::key::generate::start(
        ui,
        &org,
        &service_group,
        &default_cache_key_path(Some(&*FS_ROOT)),
    )
}

fn sub_user_key_generate(ui: &mut UI, m: &ArgMatches) -> Result<()> {
    let user = m.value_of("USER").unwrap(); // Required via clap
    init();

    command::user::key::generate::start(ui, user, &default_cache_key_path(Some(&*FS_ROOT)))
}

fn exec_subcommand_if_called(ui: &mut UI) -> Result<()> {
    let mut args = env::args();
    let config = config::load()?;
    // JW: Set the CtlGateway secret for the Supervisor process that we might start below. This
    // will be removed when we refactor the hab binary to contain the Supervisor's commands itself.
    set_ctl_secret_env_var(&config);
    match (
        args.nth(1).unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
    ) {
        ("butterfly", _, _) => command::butterfly::start(ui, env::args_os().skip(2).collect()),
        ("apply", _, _) => {
            let mut args: Vec<OsString> = env::args_os().skip(1).collect();
            args.insert(0, OsString::from("config"));
            command::butterfly::start(ui, args)
        }
        ("config", _, _) | ("file", _, _) => {
            command::butterfly::start(ui, env::args_os().skip(1).collect())
        }
        ("pkg", "export", "docker") => {
            command::pkg::export::docker::start(ui, env::args_os().skip(4).collect())
        }
        ("pkg", "export", "cf") => {
            command::pkg::export::cf::start(ui, env::args_os().skip(4).collect())
        }
        ("pkg", "export", "helm") => {
            command::pkg::export::helm::start(ui, env::args_os().skip(4).collect())
        }
        ("pkg", "export", "k8s") |
        ("pkg", "export", "kubernetes") => {
            command::pkg::export::kubernetes::start(ui, env::args_os().skip(4).collect())
        }
        ("pkg", "export", "tar") => {
            command::pkg::export::tar::start(ui, env::args_os().skip(4).collect())
        }
        ("run", _, _) => command::launcher::start(ui, env::args_os().skip(1).collect()),
        ("stu", _, _) | ("stud", _, _) | ("studi", _, _) | ("studio", _, _) => {
            command::studio::enter::start(ui, env::args_os().skip(2).collect())
        }
        ("sup", "run", _) => command::launcher::start(ui, env::args_os().skip(2).collect()),
        ("sup", "start", _) |
        ("sup", _, _) => command::sup::start(ui, env::args_os().skip(2).collect()),
        ("start", _, _) | ("stop", _, _) => {
            command::sup::start(ui, env::args_os().skip(1).collect())
        }
        ("svc", "start", _) |
        ("svc", "load", _) |
        ("svc", "unload", _) |
        ("svc", "status", _) |
        ("svc", "stop", _) => command::sup::start(ui, env::args_os().skip(2).collect()),
        ("term", _, _) => command::sup::start(ui, env::args_os().skip(1).collect()),
        _ => Ok(()),
    }
}

/// Parse the raw program arguments and split off any arguments that will skip clap's parsing.
///
/// **Note** with the current version of clap there is no clean way to ignore arguments after a
/// certain point, especially if those arguments look like further options and flags.
fn raw_parse_args() -> (Vec<OsString>, Vec<OsString>) {
    let mut args = env::args();
    match (
        args.nth(1).unwrap_or_default().as_str(),
        args.next().unwrap_or_default().as_str(),
    ) {
        ("pkg", "exec") => {
            if args.by_ref().count() > 2 {
                return (
                    env::args_os().take(5).collect(),
                    env::args_os().skip(5).collect(),
                );
            } else {
                (env::args_os().collect(), Vec::new())
            }
        }
        _ => (env::args_os().collect(), Vec::new()),
    }
}

// Temporary warning function
fn warn_accesss_token(token: &str, ui: &mut UI) {
    if !token.starts_with("_") {
        ui.warn("WARNING:").unwrap();
        ui.warn(
            "Github tokens are being deprecated, please migrate to a Habitat Personal \
                 Access Token instead.",
        ).unwrap();
        ui.warn(
            "To generate a Habitat Personal Access token, please visit the Builder Profile \
                page (https://bldr.habitat.sh/#/profile).",
        ).unwrap();
        ui.warn(
            "For more information, please read the documentation at \
                https://www.habitat.sh/docs/using-builder/",
        ).unwrap();
    }
}

/// Check to see if the user has passed in an AUTH_TOKEN param. If not, check the
/// HAB_AUTH_TOKEN env var. If not, check the CLI config to see if there is a default auth
/// token set. If that's empty too, then error.
fn auth_token_param_or_env(ui: &mut UI, m: &ArgMatches) -> Result<String> {
    match m.value_of("AUTH_TOKEN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(AUTH_TOKEN_ENVVAR) {
                Ok(v) => {
                    // Temporary warning until we deprecate Github tokens completely
                    warn_accesss_token(&v, ui);
                    Ok(v)
                }
                Err(_) => {
                    let config = config::load()?;
                    match config.auth_token {
                        Some(v) => Ok(v),
                        None => return Err(Error::ArgumentError("No auth token specified")),
                    }
                }
            }
        }
    }
}

/// Check if the HAB_CTL_SECRET env var. If not, check the CLI config to see if there is a ctl
/// secret set and set that value to HAB_CTL_SECRET.
fn set_ctl_secret_env_var(config: &Config) {
    match henv::var(CTL_SECRET_ENVVAR) {
        Ok(_) => (),
        Err(_) => {
            match config.ctl_secret {
                Some(ref v) => env::set_var(CTL_SECRET_ENVVAR, v),
                None => (),
            }
        }
    }
}

/// Check to see if an auth token exists and convert it to a string slice if it does. Unlike
/// auth_token_param_or_env, it's ok for no auth token to be present here. This is useful for
/// commands that can optionally take an auth token for operating on private packages.
fn maybe_auth_token(ui: &mut UI, m: &ArgMatches) -> Option<String> {
    match auth_token_param_or_env(ui, &m) {
        Ok(t) => {
            // Temporary warning until we deprecate Github tokens completely
            warn_accesss_token(&t, ui);
            Some(t)
        }
        Err(_) => None,
    }
}

/// Check to see if the user has passed in an ORIGIN param.  If not, check the HABITAT_ORIGIN env
/// var. If not, check the CLI config to see if there is a default origin set. If that's empty too,
/// then error.
fn origin_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORIGIN") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(ORIGIN_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => {
                    let config = config::load()?;
                    match config.origin {
                        Some(v) => Ok(v),
                        None => return Err(Error::CryptoCLI("No origin specified".to_string())),
                    }
                }
            }
        }
    }
}

/// Check to see if the user has passed in an ORG param.
/// If not, check the HABITAT_ORG env var. If that's
/// empty too, then error.
fn org_param_or_env(m: &ArgMatches) -> Result<String> {
    match m.value_of("ORG") {
        Some(o) => Ok(o.to_string()),
        None => {
            match henv::var(HABITAT_ORG_ENVVAR) {
                Ok(v) => Ok(v),
                Err(_) => return Err(Error::CryptoCLI("No organization specified".to_string())),
            }
        }
    }
}


/// Resolve a Builder URL. Taken from the environment or from CLI args,
/// if given.
fn bldr_url_from_matches(matches: &ArgMatches) -> String {
    match matches.value_of("BLDR_URL") {
        Some(url) => url.to_string(),
        None => default_bldr_url(),
    }
}

/// Resolve a channel. Taken from the environment or from CLI args, if
/// given.
fn channel_from_matches(matches: &ArgMatches) -> String {
    matches
        .value_of("CHANNEL")
        .and_then(|c| Some(c.to_string()))
        .unwrap_or(channel::default())
}

fn binlink_dest_dir_from_matches(matches: &ArgMatches) -> PathBuf {
    let env_or_default = default_binlink_dir();
    Path::new(matches.value_of("DEST_DIR").unwrap_or(&env_or_default)).to_path_buf()
}

fn install_sources_from_matches(matches: &ArgMatches) -> Result<Vec<InstallSource>> {
    matches.values_of("PKG_IDENT_OR_ARTIFACT")
        .unwrap() // Required via clap
        .map(|t| t.parse().map_err(Error::from))
        .collect()
}

fn enable_features_from_env(ui: &mut UI) {
    let features = vec![
        (feat::List, "LIST"),
        (feat::OfflineInstall, "OFFLINE_INSTALL"),
    ];

    for feature in &features {
        match henv::var(format!("HAB_FEAT_{}", feature.1)) {
            Ok(ref val) if ["true", "TRUE"].contains(&val.as_str()) => {
                feat::enable(feature.0);
                ui.warn(&format!("Enabling feature: {:?}", feature.0))
                    .unwrap();
            }
            _ => {}
        }
    }

    if feat::is_enabled(feat::List) {
        ui.warn("Listing feature flags environment variables:")
            .unwrap();
        for feature in &features {
            ui.warn(&format!("  * {:?}: HAB_FEAT_{}=true", feature.0, feature.1))
                .unwrap();
        }
    }
}
