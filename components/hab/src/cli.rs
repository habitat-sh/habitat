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
use clap::{App, AppSettings};
use url::Url;
use hcore::crypto::keys::PairType;

const VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/VERSION"));

pub fn get() -> App<'static, 'static> {
    let alias_apply = sub_config_apply()
                          .about("Alias for 'config apply'")
                          .setting(AppSettings::Hidden);
    let alias_install = sub_package_install()
                            .about("Alias for 'pkg install'")
                            .setting(AppSettings::Hidden);

    clap_app!(hab =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand artifact =>
            (about: "Commands relating to Habitat artifacts")
            (@setting ArgRequiredElseHelp)
            (@subcommand upload =>
                (about: "Uploads a local Habitat artifact to a Depot")
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                 "Use a specific Depot URL")
                (@arg ARTIFACT: +required {file_exists}
                 "A path to a Habitat artifact \
                 (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand sign =>
                (about: "Signs a archive file with with an origin key, creating a Habitat artifact")
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
                (@arg ARTIFACT: +required {file_exists}
                 "A path to a Habitat artifact \
                 (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand hash=>
                (about: "Generate a BLAKE2b hash for a file")
                (@arg SOURCE: +required {file_exists}
                 "A path to any local file")
            )

        )
        (@subcommand config =>
            (about: "Commands relating to Habitat runtime config")
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_config_apply())
        )
        (@subcommand file =>
            (about: "Commands relating to Habitat files")
            (@setting ArgRequiredElseHelp)
            (@subcommand upload =>
                (about: "Upload a file to the supervisor ring.")
                (@arg SERVICE_GROUP: +required +takes_value {valid_service_group}
                 "Target service group for this injection (ex: redis.default)")
                (@arg FILE: +required {file_exists} "Path to local file on disk")
                (@arg VERSION_NUMBER: +required
                   "A version number (positive integer) for this configuration (ex: 42)")
                (@arg ORG: --org +takes_value "Name of service organization")
                (@arg USER: +takes_value)
                (@arg PEERS: -p --peers +takes_value
                 "A comma-delimited list of one or more Habitat Supervisor peers to infect \
                 (default: 127.0.0.1:9634)")
                (@arg RING: -r --ring +takes_value
                 "Ring key name, which will encrypt communication messages")
            )
        )
        (@subcommand origin =>
            (about: "Commands relating to Habitat origin keys")
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                 (about: "Commands relating to Habitat origin key maintenance")
                 (@setting ArgRequiredElseHelp)
                 (@subcommand download =>
                        (about: "Download origin key(s) to HAB_CACHE_KEY_PATH")
                        (@arg ORIGIN: +required "The origin name")
                        (@arg REVISION: "The key revision")
                        (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                         "Use a specific Depot URL")
                 )
                 (@subcommand export =>
                        (about: "Outputs the latest origin key contents to stdout")
                        (@arg ORIGIN: +required +takes_value)
                        (@arg PAIR_TYPE: -t --type +takes_value +required {valid_pair_type}
                         "Export either the `public' or `secret' key")
                 )
                 (@subcommand generate =>
                        (about: "Generates a Habitat origin key")
                        (@arg ORIGIN: "The origin name")
                 )
                 (@subcommand import =>
                        (about: "Reads a stdin stream containing a public or secret origin key \
                         contents and writes the key to disk")
                 )
                 (@subcommand upload =>
                        (about: "Upload a public origin key to the depot")
                        (@arg FILE: +required {file_exists}
                         "Path to a local public origin key file on disk")
                        (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                         "Use a specific Depot URL")
                 )
            )
        )
        (@subcommand service =>
            (about: "Commands relating to Habitat services")
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                 (about: "Commands relating to Habitat service keys")
                 (@setting ArgRequiredElseHelp)
                 (@subcommand generate =>
                        (about: "Generates a Habitat service key")
                        (@arg SERVICE_GROUP: +required +takes_value {valid_service_group})
                        (@arg ORG: "The service organization")
                 )
            )
        )
        (@subcommand user =>
            (about: "Commands relating to Habitat users")
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                 (about: "Commands relating to Habitat user keys")
                 (@setting ArgRequiredElseHelp)
                 (@subcommand generate =>
                        (about: "Generates a Habitat user key")
                        (@arg USER: +required +takes_value)
                 )
            )
        )
        (@subcommand pkg =>
            (about: "Commands relating to Habitat packages")
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_package_install())
             (@subcommand path =>
                    (about: "Prints the path to a specific installed release of a package")
                    (@arg PKG_IDENT: +required +takes_value
                     "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
             )
        )
        (@subcommand ring =>
            (about: "Commands relating to Habitat rings")
            (@setting ArgRequiredElseHelp)
            (@subcommand key =>
                 (about: "Commands relating to Habitat ring keys")
                 (@setting ArgRequiredElseHelp)
                 (@subcommand export =>
                        (about: "Outputs the latest ring key contents to stdout")
                        (@arg RING: +required +takes_value)
                 )
                 (@subcommand import =>
                        (about: "Reads a stdin stream containing ring key contents and writes \
                         the key to disk")
                 )
                 (@subcommand generate =>
                        (about: "Generates a Habitat ring key")
                        (@arg RING: +required +takes_value)
                 )
            )
        )
        (@subcommand sup =>
            (about: "Commands relating to the Habitat Supervisor")
        )

        (subcommand: alias_apply)
        (subcommand: alias_install)
        (subcommand: alias_start())
        (after_help: "ALIASES:\
             \n    apply       Alias for: 'config apply'\
             \n    install     Alias for: 'pkg install'\
             \n    start       Alias for: 'sup start'\
             \n"
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_package_install() -> App<'static, 'static> {
    clap_app!(@subcommand install =>
        (about: "Installs a Habitat package from a Depot or locally from a Habitat artifact")
        (@arg DEPOT_URL: -u --url +takes_value {valid_url} "Use a specific Depot URL")
        (@arg PKG_IDENT_OR_ARTIFACT: +required "A Habitat package identifier (ex: acme/redis) \
         or path to a Habitat artifact \
         (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_config_apply() -> App<'static, 'static> {
    clap_app!(@subcommand apply =>
        (about: "Applies a configuration to a group of Habitat Supervisors")
        (@arg PEERS: -p --peers +takes_value
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
