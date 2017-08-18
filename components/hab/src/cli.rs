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

use std::path::Path;
use std::result;
use std::str::FromStr;

use clap::{App, AppSettings, Arg};
use hcore::crypto::keys::PairType;
use regex::Regex;
use url::Url;

pub fn get() -> App<'static, 'static> {
    let alias_apply = sub_config_apply()
        .about("Alias for 'config apply'")
        .aliases(&["ap", "app", "appl"])
        .setting(AppSettings::Hidden);
    let alias_install = sub_pkg_install()
        .about("Alias for 'pkg install'")
        .aliases(&["i", "in", "ins", "inst", "insta", "instal"])
        .setting(AppSettings::Hidden);
    let alias_setup = sub_cli_setup()
        .about("Alias for 'cli setup'")
        .aliases(&["set", "setu"])
        .setting(AppSettings::Hidden);

    clap_app!(hab =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: super::VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand cli =>
            (about: "Commands relating to Habitat runtime config")
            (aliases: &["cl"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_cli_setup().aliases(&["s", "se", "set", "setu"]))
            (subcommand: sub_cli_completers().aliases(&["c", "co", "com", "comp"]))
        )
        (@subcommand config =>
            (about: "Commands relating to Habitat runtime config")
            (aliases: &["co", "con", "conf", "confi"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_config_apply().aliases(&["a", "ap", "app", "appl"]))
        )
        (@subcommand file =>
            (about: "Commands relating to Habitat files")
            (aliases: &["f", "fi", "fil"])
            (@setting ArgRequiredElseHelp)
            (@subcommand upload =>
                (about: "Upload a file to the supervisor ring.")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg SERVICE_GROUP: +required +takes_value {valid_service_group}
                    "Target service group (ex: redis.default)")
                (@arg FILE: +required {file_exists} "Path to local file on disk")
                (@arg VERSION_NUMBER: +required
                    "A version number (positive integer) for this configuration (ex: 42)")
                (@arg ORG: --org +takes_value "Name of service organization")
                (@arg USER: +takes_value "Name of the user key")
                (@arg PEER: -p --peer +takes_value
                    "A comma-delimited list of one or more Habitat Supervisor peers to infect \
                    (default: 127.0.0.1:9638)")
                (@arg RING: -r --ring +takes_value
                    "Ring key name, which will encrypt communication messages")
            )
        )
        (@subcommand job =>
            (about: "Commands relating to build job control")
            (aliases: &["j", "jo"])
            (@setting ArgRequiredElseHelp)
            (@subcommand start =>
                (about: "Schedule a job or group of jobs")
                (aliases: &["s", "st", "sta", "star"])
                (@arg PKG_IDENT: +required +takes_value
                    "The origin and name of the package to schedule a job for \
                    (ex: core/redis)")
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for the Depot")
                (@arg GROUP: -g --group "Schedule jobs for this package and all of its reverse dependencies")
            )
            (@subcommand promote =>
                (about: "Promote every package in a job group to a specified channel")
                (aliases: &["p", "pr", "pro", "prom", "promo", "promot"])
                (@arg GROUP_ID: +required +takes_value
                    "The job group ID that was returned from \"hab job start\" \
                    (ex: 771100000000000000)")
                (@arg CHANNEL: +takes_value +required "The channel name to promote the built packages into")
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for the Depot")
            )
        )
        (@subcommand origin =>
            (about: "Commands relating to Habitat origin keys")
            (aliases: &["o", "or", "ori", "orig", "origi"])
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                (about: "Commands relating to Habitat origin key maintenance")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@subcommand download =>
                    (about: "Download origin key(s) to HAB_CACHE_KEY_PATH")
                    (aliases: &["d", "do", "dow", "down", "downl", "downlo", "downloa"])
                    (@arg ORIGIN: +required "The origin name")
                    (@arg REVISION: "The key revision")
                    (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                        "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                )
                (@subcommand export =>
                    (about: "Outputs the latest origin key contents to stdout")
                    (aliases: &["e", "ex", "exp", "expo", "expor"])
                    (@arg ORIGIN: +required +takes_value)
                    (@arg PAIR_TYPE: -t --type +takes_value {valid_pair_type}
                    "Export either the `public' or `secret' key")
                )
                (@subcommand generate =>
                    (about: "Generates a Habitat origin key")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg ORIGIN: "The origin name")
                )
                (@subcommand import =>
                    (about: "Reads a stdin stream containing a public or secret origin key \
                        contents and writes the key to disk")
                    (aliases: &["i", "im", "imp", "impo", "impor"])
                )
                (@subcommand upload =>
                    (@group upload =>
                        (@attributes +required)
                        (@arg ORIGIN: +required "The origin name")
                        (@arg PUBLIC_FILE: --pubfile +takes_value {file_exists}
                            "Path to a local public origin key file on disk")
                    )
                    (about: "Upload origin keys to the depot")
                    (aliases: &["u", "up", "upl", "uplo", "uploa"])
                    (@arg WITH_SECRET: -s --secret conflicts_with[PUBLIC_FILE]
                        "Upload secret key in addition to the public key")
                    (@arg SECRET_FILE: --secfile +takes_value {file_exists} conflicts_with[ORIGIN]
                        "Path to a local secret origin key file on disk")
                    (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                        "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for the Depot")
                )
            )
        )
        (@subcommand pkg =>
            (about: "Commands relating to Habitat packages")
            (aliases: &["p", "pk", "package"])
            (@setting ArgRequiredElseHelp)
            (@subcommand binlink =>
                (about: "Creates a symlink for a package binary in a common 'PATH' location")
                (aliases: &["bi", "bin", "binl", "binli", "binlin"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg BINARY: +takes_value
                    "The command to symlink (ex: bash)")
                (@arg DEST_DIR: -d --dest +takes_value
                    "Sets the destination directory (default: /bin)")
            )
            (@subcommand config =>
                (about: "Displays the default configuration options for a service")
                (aliases: &["conf", "cfg"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (subcommand: sub_pkg_build())
            (@subcommand env =>
                (about: "Prints the runtime environment of a specific installed package")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (@subcommand exec =>
                (about: "Executes a command using the 'PATH' context of an installed package")
                (aliases: &["exe"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg CMD: +required +takes_value
                    "The command to execute (ex: ls)")
                (@arg ARGS: +takes_value +multiple
                    "Arguments to the command (ex: -l /tmp)")
            )
            (@subcommand export =>
                (about: "Exports the package to the specified format")
                (aliases: &["exp"])
                (@arg FORMAT: +required +takes_value
                    "The export format (ex: docker, aci, mesos, or tar)")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg DEPOT_URL: --url -u +takes_value {valid_url}
                    "Retrieve the container's package from the specified Depot \
                    (default: https://bldr.habitat.sh/v1/depot)")
                (@arg CHANNEL: --channel -c +takes_value
                    "Retrieve the container's package from the specified release channel \
                    (default: stable)")
                (@arg HAB_DEPOT_URL: --("hab-url") -U +takes_value {valid_url}
                    "Retrieve the Habitat toolchain for the container from the specified Depot \
                    (default: https://bldr.habitat.sh/v1/depot)")
                (@arg HAB_CHANNEL: --("hab-channel") -C +takes_value
                    "Retrieve the Habitat toolchain for the container from the specified release \
                    channel (default: stable)")
            )
            (@subcommand hash =>
                (about: "Generates a blake2b hashsum from a target at any given filepath")
                (aliases: &["ha", "has"])
                (@arg SOURCE: +takes_value {file_exists} "A filepath of the target")
            )
            (subcommand: sub_pkg_install().aliases(
                &["i", "in", "ins", "inst", "insta", "instal"]))
            (@subcommand path =>
                (about: "Prints the path to a specific installed release of a package")
                (aliases: &["p", "pa", "pat"])
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (@subcommand provides =>
                (about: "Search installed Habitat packages for a given file")
                (@arg FILE: +required +takes_value
                    "File name to find")
                (@arg FULL_RELEASES: -r
                    "Show fully qualified package names \
                    (ex: core/busybox-static/1.24.2/20160708162350)")
                (@arg FULL_PATHS: -p "Show full path to file")
            )
            (@subcommand search =>
                (about: "Search for a package on a Depot")
                (@arg SEARCH_TERM: +required +takes_value "Search term")
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
            )
            (@subcommand sign =>
                (about: "Signs an archive with an origin key, generating a Habitat Artifact")
                (aliases: &["s", "si", "sig"])
                (@arg ORIGIN: --origin +takes_value "Origin key used to create signature")
                (@arg SOURCE: +required {file_exists}
                    "A path to a source archive file \
                    (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)")
                (@arg DEST: +required
                    "The destination path to the signed Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand upload =>
                (about: "Uploads a local Habitat Artifact to a Depot")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for the Depot")
                (@arg CHANNEL: --channel -c +takes_value
                    "Additional release channel to upload package to. \
                     Packages are always uploaded to `unstable`, regardless \
                     of the value of this option. (default: none)")
                (@arg HART_FILE: +required +multiple {file_exists}
                    "One or more filepaths to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand promote =>
                (about: "Promote a package to a specified channel")
                (aliases: &["pr", "pro"])
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg CHANNEL: +required +takes_value
                    "Promote to the specified release channel")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for the Depot")
            )
            (@subcommand demote =>
                (about: "Demote a package from a specified channel")
                (aliases: &["de", "dem", "demo", "demot"])
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg CHANNEL: +required +takes_value
                    "Demote from the specified release channel")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for the Depot")
            )
            (@subcommand channels =>
                (about: "Find out what channels a package belongs to")
                (aliases: &["ch", "cha", "chan", "chann", "channe", "channel"])
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                    "Use a specific Depot URL (ex: http://depot.example.com/v1/depot)")
                (@arg PKG_IDENT: +required +takes_value
                    "A fully qualified package identifier (ex: core/redis/3.2.1/20160729052715)")
            )
            (@subcommand verify =>
                (about: "Verifies a Habitat Artifact with an origin key")
                (aliases: &["v", "ve", "ver", "veri", "verif"])
                (@arg SOURCE: +required {file_exists}
                    "A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand header =>
                (about: "Returns the Habitat Artifact header")
                (aliases: &["hea", "head", "heade", "header"])
                (@setting Hidden)
                (@arg SOURCE: +required {file_exists}
                    "A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
        )
        (@subcommand plan =>
            (about: "Commands relating to plans and other app-specific configuration.")
            (aliases: &["pl", "pla"])
            (@setting ArgRequiredElseHelp)
            (@subcommand init =>
                (about: "Generates common package specific configuration files. Executing without \
                    argument will create a `habitat` directory in your current folder for the \
                    plan. If `PKG_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'pkg_') that are set will be used \
                    in the generated plan")
                (aliases: &["i", "in", "ini"])
                (@arg PKG_NAME: +takes_value "Name for the new app")
                (@arg ORIGIN: --origin -o +takes_value "Origin for the new app")
                (@arg WITH_DOCS: --("with-docs") "Include plan options documentation")
                (@arg WITH_CALLBACKS: --("with-callbacks")
                    "Include callback functions in template")
                (@arg WITH_ALL: --("with-all")
                    "Generate omnibus plan with all available plan options")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit scaffolding type for your app")
            )
        )
        (@subcommand ring =>
            (about: "Commands relating to Habitat rings")
            (aliases: &["r", "ri", "rin"])
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                (about: "Commands relating to Habitat ring keys")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@subcommand export =>
                    (about: "Outputs the latest ring key contents to stdout")
                    (aliases: &["e", "ex", "exp", "expo", "expor"])
                    (@arg RING: +required +takes_value "Ring key name")
                )
                (@subcommand import =>
                    (about: "Reads a stdin stream containing ring key contents and writes \
                    the key to disk")
                    (aliases: &["i", "im", "imp", "impo", "impor"])
                )
                (@subcommand generate =>
                    (about: "Generates a Habitat ring key")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg RING: +required +takes_value "Ring key name")
                )
            )
        )
        (@subcommand svc =>
            (about: "Commands relating to Habitat services")
            (aliases: &["sv", "ser", "serv", "service"])
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                (about: "Commands relating to Habitat service keys")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@subcommand generate =>
                    (about: "Generates a Habitat service key")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg SERVICE_GROUP: +required +takes_value {valid_service_group}
                        "Target service group (ex: redis.default)")
                    (@arg ORG: "The service organization")
                )
            )
            (@subcommand load =>
                (about: "Load a service to be started and supervised by Habitat from a package or \
                    artifact. Services started in this manner will persist through Supervisor \
                    restarts.")
                (@setting Hidden)
            )
            (@subcommand unload =>
                (about: "Unload a persistent or transient service started by the Habitat \
                    supervisor. If the Supervisor is running when the service is unloaded the \
                    service will be stopped.")
                (@setting Hidden)
            )
            (@subcommand start =>
                (about: "Start a loaded, but stopped, Habitat service or a transient service from \
                    a package or artifact. If the Habitat Supervisor is not already running this \
                    will additionally start one for you.")
                (@setting Hidden)
            )
            (@subcommand stop =>
                (about: "Stop a running Habitat service.")
                (@setting Hidden)
            )
            (after_help: "\nALIASES:\
                \n    load       Alias for: 'sup load'\
                \n    unload     Alias for: 'sup unload'\
                \n    start      Alias for: 'sup start'\
                \n    stop       Alias for: 'sup stop'\
                \n    status     Alias for: 'sup status'\
                \n"
            )
        )
        (@subcommand studio =>
            (about: "Commands relating to Habitat Studios")
            (aliases: &["stu", "stud", "studi"])
        )
        (@subcommand sup =>
            (about: "Commands relating to the Habitat Supervisor")
            (aliases: &["su"])
        )
        (@subcommand user =>
            (about: "Commands relating to Habitat users")
            (aliases: &["u", "us", "use"])
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                (about: "Commands relating to Habitat user keys")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@subcommand generate =>
                    (about: "Generates a Habitat user key")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg USER: +required +takes_value "Name of the user key")
                )
            )
        )
        (subcommand: alias_apply)
        (subcommand: alias_install)
        (subcommand: alias_run())
        (subcommand: alias_setup)
        (subcommand: alias_start())
        (subcommand: alias_stop())
        (subcommand: alias_term())
        (after_help: "\nALIASES:\
            \n    apply      Alias for: 'config apply'\
            \n    install    Alias for: 'pkg install'\
            \n    run        Alias for: 'sup run'\
            \n    setup      Alias for: 'cli setup'\
            \n    start      Alias for: 'svc start'\
            \n    stop       Alias for: 'svc stop'\
            \n    term       Alias for: 'sup term'\
            \n"
        )
    )
}

fn alias_run() -> App<'static, 'static> {
    clap_app!(@subcommand run =>
        (about: "Run the Habitat Supervisor")
        (@setting Hidden)
    )
}

fn alias_start() -> App<'static, 'static> {
    clap_app!(@subcommand start =>
        (about: "Starts a Habitat-supervised service")
        (aliases: &["sta", "star"])
        (@setting Hidden)
    )
}

fn alias_stop() -> App<'static, 'static> {
    clap_app!(@subcommand stop =>
        (about: "Stop a running Habitat service.")
        (aliases: &["sto"])
        (@setting Hidden)
    )
}

fn alias_term() -> App<'static, 'static> {
    clap_app!(@subcommand term =>
        (about: "Gracefully terminate the Habitat Supervisor and all of it's running services")
        (@setting Hidden)
    )
}

fn sub_cli_setup() -> App<'static, 'static> {
    clap_app!(@subcommand setup =>
        (about: "Sets up the CLI with reasonable defaults.")
    )
}

fn sub_cli_completers() -> App<'static, 'static> {
    let sub = clap_app!(@subcommand completers =>
        (about: "Creates command-line completers for your shell."));

    let supported_shells = ["bash", "fish", "zsh", "powershell"];

    // The clap_app! macro above is great but does not support the ability to specify a range of
    // possible values. We wanted to fail here with an unsupported shell instead of pushing off a
    // bad value to clap.

    sub.arg(
        Arg::with_name("SHELL")
            .help(
                "The name of the shell you want to generate the command-completion. Supported \
               Shells: bash, fish, zsh, powershell",
            )
            .short("s")
            .long("shell")
            .required(true)
            .takes_value(true)
            .possible_values(&supported_shells),
    )
}

fn sub_config_apply() -> App<'static, 'static> {
    clap_app!(@subcommand apply =>
        (about: "Applies a configuration to a group of Habitat Supervisors")
        (@arg PEER: -p --peer +takes_value
            "A comma-delimited list of one or more Habitat Supervisor peers to infect \
            (default: 127.0.0.1:9638)")
        (@arg RING: -r --ring +takes_value
            "Ring key name, which will encrypt communication messages")
        (@arg SERVICE_GROUP: +required {valid_service_group}
            "Target service group (ex: redis.default)")
        (@arg VERSION_NUMBER: +required
            "A version number (positive integer) for this configuration (ex: 42)")
        (@arg FILE: {file_exists_or_stdin}
            "Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)")
        (@arg ORG: --org +takes_value "Name of service organization")
    )
}

fn sub_pkg_build() -> App<'static, 'static> {
    let sub = clap_app!(@subcommand build =>
        (about: "Builds a Plan using a Studio")
        (aliases: &["bu", "bui", "buil"])
        (@arg HAB_ORIGIN_KEYS: -k --keys +takes_value
            "Installs secret origin keys (ex: \"unicorn\", \"acme,other,acme-ops\")")
        (@arg HAB_STUDIO_ROOT: -r --root +takes_value
            "Sets the Studio root (default: /hab/studios/<DIR_NAME>)")
        (@arg SRC_PATH: -s --src +takes_value
            "Sets the source path (default: $PWD)")
        (@arg PLAN_CONTEXT: +required +takes_value
            "A directory containing a `plan.sh` file \
            or a `habitat/` directory which contains the `plan.sh` file")
    );
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    if cfg!(target_os = "linux") {
        sub.arg(
            Arg::with_name("REUSE")
                .help(
                    "Reuses a previous Studio for the build (default: clean up before building)",
                )
                .short("R")
                .long("reuse"),
        )
        .arg(
            Arg::with_name("DOCKER")
                .help(
                    "Uses a Dockerized Studio for the build (default: Studio uses a chroot on linux)"
                )
                .short("D")
                .long("docker"),
        )
    } else if cfg!(target_os = "windows") {
        sub.arg(
            Arg::with_name("WINDOWS")
                .help("Use a Windows studio instead of a docker studio")
                .short("w")
                .long("windows"),
        )
    } else {
        sub
    }
}

fn sub_pkg_install() -> App<'static, 'static> {
    let sub = clap_app!(@subcommand install =>
        (about: "Installs a Habitat package from a Depot or locally from a Habitat Artifact")
        (@arg DEPOT_URL: --url -u +takes_value {valid_url}
            "Use a specific Depot URL (default: https://bldr.habitat.sh/v1/depot)")
        (@arg CHANNEL: --channel -c +takes_value
            "Install from the specified release channel (default: stable)")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +multiple
            "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths \
            to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
        (@arg BINLINK: -b --binlink "Binlink all binaries from installed package(s)")
    );
    sub.arg(
        Arg::with_name("IGNORE_TARGET")
            .help("Skips target validation for package installation.")
            .short("i")
            .long("ignore-target")
            .hidden(true),
    )
}

fn file_exists(val: String) -> result::Result<(), String> {
    if Path::new(&val).is_file() {
        Ok(())
    } else {
        Err(format!("File: '{}' cannot be found", &val))
    }
}

fn file_exists_or_stdin(val: String) -> result::Result<(), String> {
    if val == "-" { Ok(()) } else { file_exists(val) }
}

fn valid_pair_type(val: String) -> result::Result<(), String> {
    match PairType::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(format!(
                "PAIR_TYPE: {} is invalid, must be one of (public, secret)",
                &val
            ))
        }
    }
}

fn valid_service_group(val: String) -> result::Result<(), String> {
    let regex = Regex::new(r"([A-Za-z_0-9]+)\.([A-Za-z_0-9]+)").unwrap();
    if regex.is_match(&val) {
        Ok(())
    } else {
        Err(format!("SERVICE_GROUP: '{}' is invalid", &val))
    }
}

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
