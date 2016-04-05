// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use std::path::Path;
use std::result;

use clap::{App, AppSettings};
use url::Url;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get() -> App<'static, 'static> {
    let alias_inject = sub_rumor_inject()
                           .about("Alias for 'rumor inject'")
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
        (@subcommand archive =>
            (about: "Runs Habitat package archive commands")
            (@subcommand upload =>
                (about: "Uploads a local package archive to a depot")
                (@arg DEPOT_URL: -u --url +takes_value {valid_url}
                 "Use a specific package depot URL")
                (@arg ARCHIVE: +required {file_exists}
                 "A path to an archive file (ex: /home/chef-redis-3.0.7-21120102031201.hab)")
            )
        )
        (@subcommand pkg =>
            (about: "Runs Habitat package commands")
            (subcommand: sub_package_install())
        )
        (@subcommand rumor =>
            (about: "Runs Habitat rumor commands")
            (subcommand: sub_rumor_inject())
        )
        (@subcommand sup =>
            (about: "Runs Habitat supervisor commands")
        )
        (subcommand: alias_inject)
        (subcommand: alias_install)
        (subcommand: alias_start())
        (after_help: "ALIASES:\
             \n    inject      Alias for: 'rumor inject'\
             \n    install     Alias for: 'pkg install'\
             \n    start       Alias for: 'sup start'\
             \n"
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_package_install() -> App<'static, 'static> {
    clap_app!(@subcommand install =>
        (about: "Installs a package from a repo or locally from an archive file")
        (@arg REPO_URL: -u --url +takes_value {valid_url} "Use a specific package repo URL")
        (@arg PKG_IDENT_OR_ARCHIVE: +required "A package identifier (ex: chef/redis) \
         or path to an archive file (ex: /home/chef-redis-3.0.7-21120102031201.hab)")
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_rumor_inject() -> App<'static, 'static> {
    clap_app!(@subcommand inject =>
        (about: "Injects a configuration or configuration file into a group of Supervisors")
        (@arg PEERS: -p --peers +takes_value
         "A comma-delimited list of one or more gossip peers to infect (default: 127.0.0.1:9634)")
        (@arg SERVICE_GROUP: +required
         "Target service group for this injection (ex: redis.default)")
        (@arg VERSION_NUMBER: +required
         "A version number (integer) for this configuration (ex: 42)")
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

fn valid_url(val: String) -> result::Result<(), String> {
    match Url::parse(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("URL: '{}' is not valid", &val)),
    }
}
