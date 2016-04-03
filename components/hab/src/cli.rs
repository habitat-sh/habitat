// Copyright:: Copyright (c) 2015-2016 Chef Software, Inc.
//
// The terms of the Evaluation Agreement (Bldr) between Chef Software Inc. and the party accessing
// this file ("Licensee") apply to Licensee's use of the Software until such time that the Software
// is made available under an open source license such as the Apache 2.0 License.

use clap::{App, AppSettings};
use url::Url;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn get() -> App<'static, 'static> {
    let alias_install = sub_pkg_install()
                            .about("Alias for 'pkg install'")
                            .setting(AppSettings::Hidden);

    clap_app!(hab =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting VersionlessSubcommands)
        (@setting ArgRequiredElseHelp)
        (@subcommand arch =>
            (about: "Runs Habitat package archive commands")
            (@subcommand upload =>
                (about: "[STUB] Uploads a local package archive to a depot")
            )
        )
        (@subcommand pkg =>
            (about: "Runs Habitat package commands")
            (subcommand: sub_pkg_install())
        )
        (@subcommand sup =>
            (about: "Runs Habitat supervisor commands")
        )
        (subcommand: alias_install)
        (subcommand: alias_start())
        (after_help: "ALIASES:\
             \n    install     Alias for: 'pkg install'\
             \n    start       Alias for: 'sup start'\
             \n"
        )
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn sub_pkg_install() -> App<'static, 'static> {
    let valid_url = |url| {
        let url = String::from(url);
        match Url::parse(&url) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("URL: '{}' is not valid", &url))
        }
    };

    clap_app!(@subcommand install =>
        (about: "Installs a package from a repo (or locally from an archive...)")
        (@arg REPO_URL: -u --url +takes_value {valid_url} "Use a specific package repo URL")
        (@arg PKG_IDENT: +required "A package identifier (ex: chef/redis)")
    )
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn alias_start() -> App<'static, 'static> {
    clap_app!(@subcommand start =>
        (about: "Starts a Habitat-supervised service")
        (@setting Hidden)
    )
}
