// Copyright:: Copyright (c) 2015-2016 The Habitat Maintainers
//
// The terms of the Evaluation Agreement (Habitat) between Chef Software Inc.
// and the party accessing this file ("Licensee") apply to Licensee's use of
// the Software until such time that the Software is made available under an
// open source license such as the Apache 2.0 License.

use std::path::Path;
use std::result;
use std::str::FromStr;

use regex::Regex;
use clap::{App, AppSettings, Arg};
use url::Url;
use hcore::crypto::keys::PairType;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

pub fn get() -> App<'static, 'static> {
    let alias_apply = sub_config_apply()
                          .about("Alias for 'config apply'")
                          .aliases(&["ap", "app", "appl"])
                          .setting(AppSettings::Hidden);
    let alias_install = sub_package_install()
                            .about("Alias for 'pkg install'")
                            .aliases(&["i", "in", "ins", "inst", "insta", "instal"])
                            .setting(AppSettings::Hidden);
    let alias_start = alias_start().aliases(&["st", "sta", "star"]);

    clap_app!(hab =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand artifact =>
            (about: "Commands relating to Habitat artifacts")
            (aliases: &["ar", "art", "arti", "artif", "artifa", "artifac"])
            (@setting ArgRequiredElseHelp)
            (@subcommand upload =>
                (about: "Uploads a local Habitat artifact to a Depot")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                 "Use a specific Depot URL")
                (@arg ARTIFACT: +required +multiple {file_exists}
                 "One or more paths to a Habitat artifact \
                 (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand sign =>
                (about: "Signs a archive file with with an origin key, creating a Habitat artifact")
                (aliases: &["s", "si", "sig"])
                (@arg ORIGIN: --origin +takes_value
                 "Origin key used to create signature")
                (@arg SOURCE: +required {file_exists}
                 "A path to an archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)")
                (@arg ARTIFACT: +required
                 "The path to the generated Habitat artifact \
                 (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand verify =>
                (about: "Verifies a Habitat artifact with an origin key")
                (aliases: &["v", "ve", "ver", "veri", "verif"])
                (@arg ARTIFACT: +required {file_exists}
                 "A path to a Habitat artifact \
                 (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand hash=>
                (about: "Generate a BLAKE2b hash for a file")
                (aliases: &["ha", "has"])
                (@arg SOURCE: +required {file_exists}
                 "A path to any local file")
            )
        )
        (@subcommand config =>
            (about: "Commands relating to Habitat runtime config")
            (aliases: &["c", "co", "con", "conf", "confi"])
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
                 "Target service group for this injection (ex: redis.default)")
                (@arg FILE: +required {file_exists} "Path to local file on disk")
                (@arg VERSION_NUMBER: +required
                   "A version number (positive integer) for this configuration (ex: 42)")
                (@arg ORG: --org +takes_value "Name of service organization")
                (@arg USER: +takes_value)
                (@arg PEER: -p --peer +takes_value
                 "A comma-delimited list of one or more Habitat Supervisor peers to infect \
                 (default: 127.0.0.1:9634)")
                (@arg RING: -r --ring +takes_value
                 "Ring key name, which will encrypt communication messages")
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
                         "Use a specific Depot URL")
                 )
                 (@subcommand export =>
                        (about: "Outputs the latest origin key contents to stdout")
                        (aliases: &["e", "ex", "exp", "expo", "expor"])
                        (@arg ORIGIN: +required +takes_value)
                        (@arg PAIR_TYPE: -t --type +takes_value +required {valid_pair_type}
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
                        (about: "Upload a public origin key to the depot")
                        (aliases: &["u", "up", "upl", "uplo", "uploa"])
                        (@arg FILE: +required {file_exists}
                         "Path to a local public origin key file on disk")
                        (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                         "Use a specific Depot URL")
                 )
            )
        )
        (@subcommand service =>
            (about: "Commands relating to Habitat services")
            (aliases: &["se", "ser", "serv", "servi", "servic"])
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                 (about: "Commands relating to Habitat service keys")
                 (aliases: &["k", "ke"])
                 (@setting ArgRequiredElseHelp)
                 (@subcommand generate =>
                        (about: "Generates a Habitat service key")
                        (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                        (@arg SERVICE_GROUP: +required +takes_value {valid_service_group})
                        (@arg ORG: "The service organization")
                 )
            )
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
                        (@arg USER: +required +takes_value)
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
                   (@arg BINARY: +required +takes_value
                    "The command to symlink (ex: bash)")
                   (@arg DEST_DIR: -d --dest +takes_value
                    "Sets the destination directory (default: /bin)")
            )
            (subcommand: sub_package_build())
            (@subcommand exec =>
                   (about: "Executes a command using the 'PATH' context of an installed package")
                   (aliases: &["e", "ex", "exe"])
                   (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                   (@arg CMD: +required +takes_value
                    "The command to execute (ex: ls)")
                   (@arg ARGS: +takes_value +multiple
                    "Arguments to the command (ex: -l /tmp)")
            )
            (subcommand: sub_package_install().aliases(&["i", "in", "ins", "inst", "insta", "instal"]))
            (@subcommand path =>
                   (about: "Prints the path to a specific installed release of a package")
                   (aliases: &["p", "pa", "pat"])
                   (@arg PKG_IDENT: +required +takes_value
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
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
                        (@arg RING: +required +takes_value)
                 )
                 (@subcommand import =>
                        (about: "Reads a stdin stream containing ring key contents and writes \
                         the key to disk")
                        (aliases: &["i", "im", "imp", "impo", "impor"])
                 )
                 (@subcommand generate =>
                        (about: "Generates a Habitat ring key")
                        (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                        (@arg RING: +required +takes_value)
                 )
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

        (subcommand: alias_apply)
        (subcommand: alias_install)
        (subcommand: alias_start)
        (after_help: "ALIASES:\
             \n    apply       Alias for: 'config apply'\
             \n    install     Alias for: 'pkg install'\
             \n    start       Alias for: 'sup start'\
             \n"
        )
    )
}

fn sub_package_build() -> App<'static, 'static> {
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
        sub.arg( Arg::with_name("REUSE")
            .help("Reuses a previous Studio for the build (default: clean up before building)")
            .short("R")
            .long("reuse"))
    } else {
        sub
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_package_install() -> App<'static, 'static> {
    clap_app!(@subcommand install =>
        (about: "Installs a Habitat package from a Depot or locally from a Habitat artifact")
        (@arg DEPOT_URL: -u --url +takes_value {valid_url} "Use a specific Depot URL")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +multiple
         "One or more Habitat package identifiers (ex: acme/redis) and/or paths to a \
         Habitat artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_config_apply() -> App<'static, 'static> {
    clap_app!(@subcommand apply =>
        (about: "Applies a configuration to a group of Habitat Supervisors")
        (@arg PEER: -p --peer +takes_value
         "A comma-delimited list of one or more Habitat Supervisor peers to infect \
         (default: 127.0.0.1:9634)")
        (@arg RING: -r --ring +takes_value
         "Ring key name, which will encrypt communication messages")
        (@arg SERVICE_GROUP: +required {valid_service_group}
         "Target service group for this injection (ex: redis.default)")
        (@arg VERSION_NUMBER: +required
         "A version number (positive integer) for this configuration (ex: 42)")
        (@arg FILE: {file_exists_or_stdin}
         "Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)")
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn alias_start() -> App<'static, 'static> {
    clap_app!(@subcommand start =>
        (about: "Starts a Habitat-supervised service")
        (@setting Hidden)
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
    if val == "-" {
        Ok(())
    } else {
        file_exists(val)
    }
}

fn valid_pair_type(val: String) -> result::Result<(), String> {
    match PairType::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(format!("PAIR_TYPE: {} is invalid, must be one of (public, secret)",
                        &val))
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
