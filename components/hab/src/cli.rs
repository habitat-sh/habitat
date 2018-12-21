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

use crate::command::studio;
use clap::{App,
           AppSettings,
           Arg};
use habitat_common::{cli::{BINLINK_DIR_ENVVAR,
                           DEFAULT_BINLINK_DIR,
                           GOSSIP_DEFAULT_ADDR,
                           GOSSIP_LISTEN_ADDRESS_ENVVAR,
                           LISTEN_CTL_DEFAULT_ADDR_STRING,
                           LISTEN_HTTP_ADDRESS_ENVVAR,
                           LISTEN_HTTP_DEFAULT_ADDR,
                           PACKAGE_TARGET_ENVVAR,
                           RING_ENVVAR,
                           RING_KEY_ENVVAR},
                     types::ListenCtlAddr,
                     FeatureFlag};
use habitat_core::{crypto::{keys::PairType,
                            CACHE_KEY_PATH_ENV_VAR},
                   env::Config,
                   fs::CACHE_KEY_PATH,
                   package::{ident,
                             Identifiable,
                             PackageIdent,
                             PackageTarget},
                   service::{HealthCheckInterval,
                             ServiceGroup},
                   ChannelIdent};
use habitat_sup_protocol;
use std::{net::SocketAddr,
          path::Path,
          result,
          str::FromStr};
use url::Url;

pub fn get(feature_flags: FeatureFlag) -> App<'static, 'static> {
    let alias_apply = sub_config_apply().about("Alias for 'config apply'")
                                        .aliases(&["ap", "app", "appl"])
                                        .setting(AppSettings::Hidden);
    let alias_install =
        sub_pkg_install(feature_flags).about("Alias for 'pkg install'")
                                      .aliases(&["i", "in", "ins", "inst", "insta", "instal"])
                                      .setting(AppSettings::Hidden);
    let alias_setup = sub_cli_setup().about("Alias for 'cli setup'")
                                     .aliases(&["set", "setu"])
                                     .setting(AppSettings::Hidden);
    let alias_start = sub_svc_start().about("Alias for 'svc start'")
                                     .aliases(&["sta", "star"])
                                     .setting(AppSettings::Hidden);
    let alias_stop = sub_svc_stop().about("Alias for 'svc stop'")
                                   .aliases(&["sto"])
                                   .setting(AppSettings::Hidden);

    clap_app!(hab =>
        (about: "\"A Habitat is the natural environment for your services\" - Alan Turing")
        (version: super::VERSION)
        (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        (@setting GlobalVersion)
        (@setting ArgRequiredElseHelp)
        (@subcommand license =>
            (about: "Commands relating to Habitat license agreements")
            (@setting ArgRequiredElseHelp)
            (@subcommand accept =>
                (about: "Accept the Chef Binary Distribution Agreement without prompting"))
        )
        (@subcommand cli =>
            (about: "Commands relating to Habitat runtime config")
            (aliases: &["cl"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_cli_setup().aliases(&["s", "se", "set", "setu"]))
            (subcommand: sub_cli_completers().aliases(&["c", "co", "com", "comp"]))
        )
        (@subcommand config =>
            (about: "Commands relating to a Service's runtime config")
            (aliases: &["co", "con", "conf", "confi"])
            (@setting ArgRequiredElseHelp)
            (subcommand: sub_config_apply().aliases(&["ap", "app", "appl"]))
            (@subcommand show =>
                (about: "Displays the default configuration options for a service")
                (aliases: &["sh", "sho"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
                    "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
            )
        )
        (@subcommand file =>
            (about: "Commands relating to Habitat files")
            (aliases: &["f", "fi", "fil"])
            (@setting ArgRequiredElseHelp)
            (@subcommand upload =>
                (about: "Uploads a file to be shared between members of a Service Group")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg SERVICE_GROUP: +required +takes_value {valid_service_group}
                    "Target service group service.group[@organization] (ex: redis.default or foo.default@bazcorp)")
                (@arg VERSION_NUMBER: +required
                    "A version number (positive integer) for this configuration (ex: 42)")
                (@arg FILE: +required {file_exists} "Path to local file on disk")
                (@arg USER: -u --user +takes_value "Name of the user key")
                (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
                    "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
                (arg: arg_cache_key_path("Path to search for encryption keys. \
                    Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                    directory otherwise."))
            )
        )
        (@subcommand bldr =>
            (about: "Commands relating to Habitat Builder")
            (aliases: &["b", "bl", "bld"])
            (@setting ArgRequiredElseHelp)
            (@subcommand job =>
                (about: "Commands relating to Habitat Builder jobs")
                (aliases: &["j", "jo"])
                (@setting ArgRequiredElseHelp)
                (@subcommand start =>
                    (about: "Schedule a build job or group of jobs")
                    (aliases: &["s", "st", "sta", "star"])
                    (@arg PKG_IDENT: +required +takes_value {valid_ident}
                        "The origin and name of the package to schedule a job for (eg: core/redis)")
                    (arg: arg_target())
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the cli.toml or HAB_BLDR_URL environment variable if defined. \
                         (default: https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                    (@arg GROUP: -g --group "Schedule jobs for this package and all of its reverse \
                        dependencies")
                )
                (@subcommand cancel =>
                    (about: "Cancel a build job group and any in-progress builds")
                    (aliases: &["c", "ca", "can", "cance", "cancel"])
                    (@arg GROUP_ID: +required +takes_value
                        "The job group id that was returned from \"hab bldr job start\" \
                        (ex: 771100000000000000)")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg FORCE: -f --force
                     "Don't prompt for confirmation")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                )
                (@subcommand promote =>
                    (about: "Promote packages from a completed build job to a specified channel")
                    (aliases: &["p", "pr", "pro", "prom", "promo", "promot"])
                    (@arg GROUP_ID: +required +takes_value
                        "The job id that was returned from \"hab bldr job start\" \
                        (ex: 771100000000000000)")
                    (@arg CHANNEL: +takes_value +required "The target channel name")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "Limit the promotable packages to the specified origin")
                    (@arg INTERACTIVE: -i --interactive
                        "Allow editing the list of promotable packages")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                )
                (@subcommand demote =>
                    (about: "Demote packages from a completed build job to a specified channel")
                    (aliases: &["d", "de", "dem", "demo", "demot"])
                    (@arg GROUP_ID: +required +takes_value
                        "The job id that was returned from \"hab bldr start\" \
                        (ex: 771100000000000000)")
                    (@arg CHANNEL: +takes_value +required "The target channel name")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "Limit the demotable packages to the specified origin")
                    (@arg INTERACTIVE: -i --interactive
                        "Allow editing the list of demotable packages")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                )
                (@subcommand status =>
                    (about: "Get the status of one or more job groups")
                    (aliases: &["stat", "statu"])
                    (@group status =>
                        (@attributes +required)
                        (@arg GROUP_ID: +takes_value
                            "The group id that was returned from \"hab bldr job start\" \
                            (ex: 771100000000000000)")
                        (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                            "Show the status of recent job groups created in this origin \
                            (default: 10 most recent)")
                    )
                    (@arg LIMIT: -l --limit +takes_value {valid_numeric::<usize>}
                        "Limit how many job groups to retrieve, ordered by most recent \
                        (default: 10)")
                    (@arg SHOW_JOBS: -s --showjobs
                        "Show the status of all build jobs for a retrieved job group")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                )
            )
            (@subcommand channel =>
                (about: "Commands relating to Habitat Builder channels")
                (aliases: &["c", "ch", "cha", "chan", "chann", "channe"])
                (@setting ArgRequiredElseHelp)
                (@subcommand create =>
                    (about: "Creates a new channel")
                    (aliases: &["c", "cr", "cre", "crea", "creat"])
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg CHANNEL: +required + takes_value "The channel name")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "Sets the origin to which the channel will belong. Default is from \
                        'HAB_ORIGIN' or cli.toml")
                )
                (@subcommand destroy =>
                    (about: "Destroys a channel")
                    (aliases: &["d", "de", "des", "dest", "destr", "destro"])
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg CHANNEL: +required + takes_value "The channel name")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN'\
                        or cli.toml")
                )
                (@subcommand list =>
                    (about: "Lists origin channels")
                    (aliases: &["l", "li", "lis"])
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg ORIGIN: +takes_value {valid_origin}
                        "The origin for which channels will be listed. Default is from 'HAB_ORIGIN'\
                        or cli.toml")
                )
            )
        )
        (@subcommand origin =>
            (about: "Commands relating to Habitat origin keys")
            (aliases: &["o", "or", "ori", "orig", "origi"])
            (@setting ArgRequiredElseHelp)
            (@subcommand delete =>
                (about: "Removes an unused/empty origin")
                (aliases: &["del", "dele"])
                (@arg ORIGIN: +required {valid_origin} "The origin name")
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                     "Specify an alternate Builder endpoint. If not specified, the value will \
                     be taken from the `HAB_BLDR_URL environment variable if defined. (default: \
                     https://bldr.habitat.sh)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand key =>
                (about: "Commands relating to Habitat origin key maintenance")
                (aliases: &["k", "ke"])
                (@setting ArgRequiredElseHelp)
                (@subcommand download =>
                    (about: "Download origin key(s)")
                    (aliases: &["d", "do", "dow", "down", "downl", "downlo", "downloa"])
                    (arg: arg_cache_key_path("Path to download keys to. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                    (@arg ORIGIN: +required {valid_origin} "The origin name" )
                    (@arg REVISION: "The key revision")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg WITH_SECRET: -s --secret
                        "Download secret signing key instead of public signing key")
                    (@arg WITH_ENCRYPTION: -e --encryption
                        "Download public encryption key instead of public signing key")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder \
                        (required for downloading secret keys)")
                )
                (@subcommand export =>
                    (about: "Outputs the latest origin key contents to stdout")
                    (aliases: &["e", "ex", "exp", "expo", "expor"])
                    (@arg ORIGIN: +required +takes_value {valid_origin})
                    (@arg PAIR_TYPE: -t --type +takes_value {valid_pair_type}
                        "Export either the 'public' or 'secret' key")
                    (arg: arg_cache_key_path("Path to export keys from. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
                (@subcommand generate =>
                    (about: "Generates a Habitat origin key")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg ORIGIN: {valid_origin} "The origin name")
                    (arg: arg_cache_key_path("Path to store generated keys. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))

                )
                (@subcommand import =>
                    (about: "Reads a stdin stream containing a public or secret origin key \
                        contents and writes the key to disk")
                    (aliases: &["i", "im", "imp", "impo", "impor"])
                    (arg: arg_cache_key_path("Path to import keys to. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
                (@subcommand upload =>
                    (@group upload =>
                        (@attributes +required)
                        (@arg ORIGIN: {valid_origin} "The origin name")
                        (@arg PUBLIC_FILE: --pubfile +takes_value {file_exists}
                            "Path to a local public origin key file on disk")
                    )
                    (about: "Upload origin keys to Builder")
                    (aliases: &["u", "up", "upl", "uplo", "uploa"])
                    (arg: arg_cache_key_path("Path to upload keys from. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                    (@arg WITH_SECRET: -s --secret conflicts_with[PUBLIC_FILE]
                        "Upload secret key in addition to the public key")
                    (@arg SECRET_FILE: --secfile +takes_value {file_exists} conflicts_with[ORIGIN]
                        "Path to a local secret origin key file on disk")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                )
            )
            (@subcommand secret =>
                (about: "Commands related to secret management")
                (@setting ArgRequiredElseHelp)
                (@subcommand upload =>
                    (about: "Create and upload a secret for your origin.")
                    (@arg KEY_NAME: +required +takes_value
                        "The name of the variable key to be injected into the studio. \
                        Ex: KEY=\"some_value\"")
                    (@arg SECRET: +required +takes_value
                        "The contents of the variable to be injected into the studio.")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "The origin for which the secret will be uploaded. Default is from \
                        'HAB_ORIGIN' or cli.toml")
                    (arg: arg_cache_key_path("Path to public encryption key. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
                (@subcommand delete =>
                    (about: "Delete a secret for your origin")
                    (@arg KEY_NAME: +required +takes_value
                        "The name of the variable key to be injected into the studio.")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "The origin for which the secret will be deleted. Default is from \
                        'HAB_ORIGIN' or cli.toml")
                )
                (@subcommand list =>
                    (about: "List all secrets for your origin")
                    (@arg BLDR_URL: -u --url +takes_value {valid_url}
                        "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
                    (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                        "The origin for which secrets will be listed. Default is from 'HAB_ORIGIN' \
                        or cli.toml")
                )
            )
        )
        (@subcommand pkg =>
            (about: "Commands relating to Habitat packages")
            (aliases: &["p", "pk", "package"])
            (@setting ArgRequiredElseHelp)
            (@subcommand binds =>
                (about: "Displays the binds for a service")
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-statis/1.42.2)")
            )
            (@subcommand binlink =>
                (about: "Creates a binlink for a package binary in a common 'PATH' location")
                (aliases: &["bi", "bin", "binl", "binli", "binlin"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
                (@arg BINARY: +takes_value
                    "The command to binlink (ex: bash)")
                (@arg DEST_DIR: -d --dest +takes_value {non_empty} env(BINLINK_DIR_ENVVAR) default_value(DEFAULT_BINLINK_DIR)
                    "Sets the destination directory")
                (@arg FORCE: -f --force "Overwrite existing binlinks")
            )
            (@subcommand config =>
                (about: "Displays the default configuration options for a service")
                (aliases: &["conf", "cfg"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (subcommand: sub_pkg_build())
            (@subcommand env =>
                (about: "Prints the runtime environment of a specific installed package")
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (@subcommand exec =>
                (about: "Executes a command using the 'PATH' context of an installed package")
                (aliases: &["exe"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
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
                    "The export format (ex: aci, cf, docker, kubernetes, mesos, or tar)")
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2) or \
                    filepath to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                (@arg BLDR_URL: -u --url +takes_value {valid_url}
                    "Specify an alternate Builder endpoint. If not specified, the value will \
                     be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                     https://bldr.habitat.sh)")
                (@arg CHANNEL: --channel -c +takes_value default_value[stable] env(ChannelIdent::ENVVAR)
                    "Retrieve the container's package from the specified release channel")
            )
            (@subcommand hash =>
                (about: "Generates a blake2b hashsum from a target at any given filepath")
                (aliases: &["ha", "has"])
                (@arg SOURCE: +takes_value {file_exists} "A filepath of the target")
            )
            (subcommand: sub_pkg_install(feature_flags).aliases(
                &["i", "in", "ins", "inst", "insta", "instal"]))
            (@subcommand path =>
                (about: "Prints the path to a specific installed release of a package")
                (aliases: &["p", "pa", "pat"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
            )
            (@subcommand list =>
                (about: "List all versions of installed packages")
                (aliases: &["li"])
                (@group prefix =>
                    (@attributes +required)
                    (@arg ALL: -a --all
                            "List all installed packages")
                    (@arg ORIGIN: -o --origin +takes_value {valid_origin}
                            "An origin to list")
                    (@arg PKG_IDENT: +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2).")
                )

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
                (about: "Search for a package in Builder")
                (@arg SEARCH_TERM: +required +takes_value "Search term")
                (@arg BLDR_URL: -u --url +takes_value {valid_url} "Specify an alternate Builder \
                    endpoint. If not specified, the value will be taken from the HAB_BLDR_URL \
                    environment variable if defined. (default: https://bldr.habitat.sh)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand sign =>
                (about: "Signs an archive with an origin key, generating a Habitat Artifact")
                (aliases: &["s", "si", "sig"])
                (@arg ORIGIN: --origin +takes_value {valid_origin} "Origin key used to create signature")
                (@arg SOURCE: +required {file_exists}
                    "A path to a source archive file \
                    (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)")
                (@arg DEST: +required
                    "The destination path to the signed Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                (arg: arg_cache_key_path("Path to search for origin keys. \
                    Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                    directory otherwise."))
            )
            (@subcommand uninstall =>
                (about: "Safely uninstall a package and dependencies from the local filesystem")
                (aliases: &["un", "unin"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2/21120102031201)")
                (@arg DRYRUN: -d --dryrun "Just show what would be uninstalled, don't actually do it")
                (@arg EXCLUDE: --exclude +takes_value +multiple {valid_ident}
                    "Identifier of one or more packages that should not be uninstalled. \
                    (ex: core/redis, core/busybox-static/1.42.2/21120102031201)")
                (@arg NO_DEPS: --("no-deps") "Don't uninstall dependencies")
            )
            (@subcommand upload =>
                (about: "Uploads a local Habitat Artifact to Builder")
                (aliases: &["u", "up", "upl", "uplo", "uploa"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url} "Specify an alternate Builder \
                    endpoint. If not specified, the value will be taken from the HAB_BLDR_URL \
                    environment variable if defined. (default: https://bldr.habitat.sh)")
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
                (@arg CHANNEL: --channel -c +takes_value
                    "Optional additional release channel to upload package to. \
                     Packages are always uploaded to `unstable`, regardless \
                     of the value of this option.")
                (@arg FORCE: --force "Skips checking availability of package and \
                    force uploads, potentially overwriting a stored copy of a package. \
                    (default: false)")
                (@arg HART_FILE: +required +multiple {file_exists}
                    "One or more filepaths to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                (arg: arg_cache_key_path("Path to search for public origin keys to upload. \
                    Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                    directory otherwise."))
            )
            (@subcommand delete =>
                (about: "Removes a package from Builder")
                (aliases: &["del", "dele"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url} "Specify an alternate Builder \
                    endpoint. If not specified, the value will be taken from the HAB_BLDR_URL \
                    environment variable if defined. (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value {valid_fully_qualified_ident} "A fully qualified package identifier \
                    (ex: core/busybox-static/1.42.2/20170513215502)")
                (arg: arg_target())
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand promote =>
                (about: "Promote a package to a specified channel")
                (aliases: &["pr", "pro", "promo", "promot"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url} "Specify an alternate Builder \
                    endpoint. If not specified, the value will be taken from the HAB_BLDR_URL \
                    environment variable if defined. (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value {valid_fully_qualified_ident} "A fully qualified package identifier \
                    (ex: core/busybox-static/1.42.2/20170513215502)")
                (@arg CHANNEL: +required +takes_value "Promote to the specified release channel")
                (arg: arg_target())
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand demote =>
                (about: "Demote a package from a specified channel")
                (aliases: &["de", "dem", "demo", "demot"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url} "Specify an alternate Builder \
                    endpoint. If not specified, the value will be taken from the HAB_BLDR_URL \
                    environment variable if defined. (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value {valid_fully_qualified_ident} "A fully qualified package identifier \
                    (ex: core/busybox-static/1.42.2/20170513215502)")
                (@arg CHANNEL: +required +takes_value "Demote from the specified release channel")
                (arg: arg_target())
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand channels =>
                (about: "Find out what channels a package belongs to")
                (aliases: &["ch", "cha", "chan", "chann", "channe", "channel"])
                (@arg BLDR_URL: -u --url +takes_value {valid_url} "Specify an alternate Builder \
                    endpoint. If not specified, the value will be taken from the HAB_BLDR_URL \
                    environment variable if defined. (default: https://bldr.habitat.sh)")
                (@arg PKG_IDENT: +required +takes_value {valid_fully_qualified_ident} "A fully qualified package identifier \
                    (ex: core/busybox-static/1.42.2/20170513215502)")
                (arg: arg_target())
                (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
            )
            (@subcommand verify =>
                (about: "Verifies a Habitat Artifact with an origin key")
                (aliases: &["v", "ve", "ver", "veri", "verif"])
                (@arg SOURCE: +required {file_exists} "A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                (arg: arg_cache_key_path("Path to search for public origin keys for verification. \
                    Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                    directory otherwise."))
            )
            (@subcommand header =>
                (about: "Returns the Habitat Artifact header")
                (aliases: &["hea", "head", "heade", "header"])
                (@setting Hidden)
                (@arg SOURCE: +required {file_exists} "A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand info =>
                (about: "Returns the Habitat Artifact information")
                (aliases: &["inf", "info"])
                (@arg TO_JSON: -j --json "Output will be rendered in json")
                (@arg SOURCE: +required {file_exists} "A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
            )
            (@subcommand dependencies =>
                (about: "Returns the Habitat Artifact dependencies. By default it will return \
                    the direct dependencies of the package")
                (aliases: &["dep", "deps"])
                (@arg TRANSITIVE: -t --transitive "Show transitive dependencies")
                (@arg REVERSE: -r --reverse "Show packages which are dependant on this one")
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A package identifier (ex: core/redis, core/busybox-static/1.42.2)")
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
                (@arg ORIGIN: --origin -o +takes_value {valid_origin} "Origin for the new app")
                (@arg WITH_DOCS: --("with-docs") "Include plan options documentation")
                (@arg WITH_CALLBACKS: --("with-callbacks")
                    "Include callback functions in template")
                (@arg WITH_ALL: --("with-all")
                    "Generate omnibus plan with all available plan options")
                (@arg WINDOWS: --("windows")
                    "Use a Windows Powershell plan template")
                (@arg SCAFFOLDING: --scaffolding -s +takes_value
                    "Specify explicit Scaffolding for your app (ex: node, ruby)")
            )
            (@subcommand render =>
                (about: "Renders plan config files")
                (aliases: &["r", "re", "ren", "rend", "rende"])
                (@arg TEMPLATE_PATH: +required {file_exists} "Path to config to render")
                (@arg DEFAULT_TOML: -d --("default-toml") +takes_value default_value("./default.toml") "Path to default.toml")
                (@arg USER_TOML: -u --("user-toml") +takes_value "Path to user.toml, defaults to none")
                (@arg MOCK_DATA: -m --("mock-data") +takes_value "Path to json file with mock data for template, defaults to none")
                (@arg PRINT: -p --("print") "Prints config to STDOUT")
                (@arg RENDER_DIR: -r --("render-dir") +takes_value default_value("./results") "Path to render templates")
                (@arg NO_RENDER: -n --("no-render") "Don't write anything to disk, ignores --render-dir")
                (@arg QUIET: -q --("no-verbose") --quiet
                    "Don't print any helper messages.  When used with `--print` will only print config file")
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
                    (arg: arg_cache_key_path("Path to search for keys. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
                (@subcommand import =>
                    (about: "Reads a stdin stream containing ring key contents and writes \
                    the key to disk")
                    (aliases: &["i", "im", "imp", "impo", "impor"])
                    (arg: arg_cache_key_path("Path to store imported keys. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
                (@subcommand generate =>
                    (about: "Generates a Habitat ring key")
                    (aliases: &["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    (@arg RING: +required +takes_value "Ring key name")
                    (arg: arg_cache_key_path("Path to store generated keys. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
            )
        )
        (subcommand: sup_commands())
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
                        "Target service group service.group[@organization] (ex: redis.default or foo.default@bazcorp)")
                    (@arg ORG: "The service organization")
                    (arg: arg_cache_key_path("Path to store generated keys. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
            )
            (subcommand: sub_svc_load().aliases(&["l", "lo", "loa"]))
            (subcommand: sub_svc_start().aliases(&["star"]))
            (subcommand: sub_svc_status().aliases(&["stat", "statu"]))
            (subcommand: sub_svc_stop().aliases(&["sto"]))
            (@subcommand unload =>
                (about: "Unload a service loaded by the Habitat Supervisor. If the service is \
                    running it will additionally be stopped.")
                (aliases: &["u", "un", "unl", "unlo", "unloa"])
                (@arg PKG_IDENT: +required +takes_value {valid_ident}
                    "A Habitat package identifier (ex: core/redis)")
                (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
                    "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
            )
        )
        (@subcommand studio =>
            (about: "Commands relating to Habitat Studios")
            (aliases: &["stu", "stud", "studi"])
        )
        (@subcommand supportbundle =>
            (about: "Create a tarball of Habitat Supervisor data to send to support")
            (aliases: &["supp", "suppo", "suppor", "support-bundle"])
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
                    (arg: arg_cache_key_path("Path to store generated keys. \
                        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
                        directory otherwise."))
                )
            )
        )
        (subcommand: alias_apply)
        (subcommand: alias_install)
        (subcommand: alias_run())
        (subcommand: alias_setup)
        (subcommand: alias_start)
        (subcommand: alias_stop)
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

fn alias_term() -> App<'static, 'static> {
    clap_app!(@subcommand term =>
        (about: "Gracefully terminate the Habitat Supervisor and all of its running services")
        (@setting Hidden)
    )
}

fn sub_cli_setup() -> App<'static, 'static> {
    clap_app!(@subcommand setup =>
    (about: "Sets up the CLI with reasonable defaults.")
    (arg: arg_cache_key_path("Path to search for or create keys in. \
        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
        directory otherwise."))
    )
}

pub fn sup_commands() -> App<'static, 'static> {
    // Define all of the `hab sup *` subcommands in one place.
    // This removes the need to duplicate this in `hab-sup`.
    // The 'sup' App name here is significant for the `hab` binary as it
    // is inserted as a named subcommand. For the `hab-sup` binary, it is
    // the top-level App name (not a named subcommand) and therefore is not
    // significant since we override `usage` below.
    clap_app!(("sup") =>
    (about: "The Habitat Supervisor")
    (version: super::VERSION)
    (author: "\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
    // set custom usage string, otherwise the binary
    // is displayed as the clap_app name, which may or may not be different.
    // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
    (usage: "hab sup <SUBCOMMAND>")
    (@setting VersionlessSubcommands)
    (@setting SubcommandRequiredElseHelp)
    (subcommand: sub_sup_bash().aliases(&["b", "ba", "bas"]))
    (subcommand: sub_sup_depart().aliases(&["d", "de", "dep", "depa", "depart"]))
    (subcommand: sub_sup_run().aliases(&["r", "ru"]))
    (subcommand: sub_sup_secret().aliases(&["sec", "secr"]))
    (subcommand: sub_sup_sh().aliases(&[]))
    (subcommand: sub_svc_status().aliases(&["stat", "statu"]))
    (subcommand: sub_sup_term().aliases(&["ter"]))
    )
}

fn sub_cli_completers() -> App<'static, 'static> {
    let sub = clap_app!(@subcommand completers =>
        (about: "Creates command-line completers for your shell."));

    let supported_shells = ["bash", "fish", "zsh", "powershell"];

    // The clap_app! macro above is great but does not support the ability to specify a range of
    // possible values. We wanted to fail here with an unsupported shell instead of pushing off a
    // bad value to clap.

    sub.arg(Arg::with_name("SHELL").help("The name of the shell you want to generate the \
                                          command-completion. Supported Shells: bash, fish, zsh, \
                                          powershell")
                                   .short("s")
                                   .long("shell")
                                   .required(true)
                                   .takes_value(true)
                                   .possible_values(&supported_shells))
}

// We need a default_value so that the argument can be required and validated. We hide the
// default because it's a special value that will be internally mapped according to the
// user type. This is to allow us to apply consistent validation to the env var override.
fn arg_cache_key_path(help_text: &'static str) -> Arg<'static, 'static> {
    Arg::with_name("CACHE_KEY_PATH").long("cache-key-path")
                                    .required(true)
                                    .validator(non_empty)
                                    .env(CACHE_KEY_PATH_ENV_VAR)
                                    .default_value(CACHE_KEY_PATH)
                                    .hide_default_value(true)
                                    .help(&help_text)
}

fn arg_target() -> Arg<'static, 'static> {
    Arg::with_name("PKG_TARGET").required(false)
                                .takes_value(true)
                                .validator(valid_target)
                                .env(PACKAGE_TARGET_ENVVAR)
                                .help("A package target (ex: x86_64-windows) (default: system \
                                       appropriate target")
}

fn sub_pkg_build() -> App<'static, 'static> {
    let mut sub = clap_app!(@subcommand build =>
    (about: "Builds a Plan using a Studio")
    (@arg HAB_ORIGIN_KEYS: -k --keys +takes_value
        "Installs secret origin keys (ex: \"unicorn\", \"acme,other,acme-ops\")")
    (@arg HAB_STUDIO_ROOT: -r --root +takes_value
        "Sets the Studio root (default: /hab/studios/<DIR_NAME>)")
    (@arg SRC_PATH: -s --src +takes_value
        "Sets the source path (default: $PWD)")
    (@arg PLAN_CONTEXT: +required +takes_value
        "A directory containing a plan file \
        or a `habitat/` directory which contains the plan file")
    (arg: arg_cache_key_path("Path to search for origin keys. \
        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
        directory otherwise."))
    );
    // Only a truly native/local Studio can be reused--the Docker implementation will always be
    // ephemeral
    if studio::native_studio_support() {
        sub = sub.arg(Arg::with_name("REUSE").help("Reuses a previous Studio for the build \
                                                    (default: clean up before building)")
                                             .short("R")
                                             .long("reuse"))
                 .arg(Arg::with_name("DOCKER").help("Uses a Dockerized Studio for the build")
                                              .short("D")
                                              .long("docker"));
    }

    if cfg!(windows) {
        sub = sub.arg(Arg::with_name("WINDOWS").help("Use a local Windows Studio instead of a \
                                                      Docker Studio")
                                               .short("w")
                                               .long("windows"));
    }

    sub
}

fn sub_pkg_install(feature_flags: FeatureFlag) -> App<'static, 'static> {
    let mut sub = clap_app!(@subcommand install =>
        (about: "Installs a Habitat package from Builder or locally from a Habitat Artifact")
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
        (@arg CHANNEL: --channel -c +takes_value default_value[stable] env(ChannelIdent::ENVVAR)
            "Install from the specified release channel")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +multiple
            "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths \
            to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
        (@arg BINLINK: -b --binlink +takes_value {non_empty} env(BINLINK_DIR_ENVVAR)
            default_value(DEFAULT_BINLINK_DIR) "Binlink all binaries from installed package(s)")
        (@arg FORCE: -f --force "Overwrite existing binlinks")
        (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
    );
    if feature_flags.contains(FeatureFlag::OFFLINE_INSTALL) {
        sub = sub.arg(Arg::with_name("OFFLINE").help("Install packages in offline mode")
                                               .long("offline"));
    };
    if feature_flags.contains(FeatureFlag::IGNORE_LOCAL) {
        sub = sub.arg(Arg::with_name("IGNORE_LOCAL").help("Do not use locally-installed \
                                                           packages when a corresponding \
                                                           package cannot be installed from \
                                                           Builder")
                                                    .long("ignore-local"));
    };
    if feature_flags.contains(FeatureFlag::INSTALL_HOOK) {
        sub = sub.arg(Arg::with_name("IGNORE_INSTALL_HOOK").help("Do not run any install hooks")
                                                           .long("ignore-install-hook"));
    };
    sub
}

fn sub_config_apply() -> App<'static, 'static> {
    clap_app!(@subcommand apply =>
    (about: "Sets a configuration to be shared by members of a Service Group")
    (@arg SERVICE_GROUP: +required {valid_service_group}
        "Target service group service.group[@organization] (ex: redis.default or foo.default@bazcorp)")
    (@arg VERSION_NUMBER: +required
        "A version number (positive integer) for this configuration (ex: 42)")
    (@arg FILE: {file_exists_or_stdin}
        "Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)")
    (@arg USER: -u --user +takes_value "Name of a user key to use for encryption")
    (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
        "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
    (arg: arg_cache_key_path("Path to search for encryption keys. \
        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
        directory otherwise."))
    )
}

// the following sup related functions are
// public due to their utilization in `hab-sup`
// for consistency, all supervisor related clap subcommands are defined in this module
pub fn sub_sup_depart() -> App<'static, 'static> {
    clap_app!(@subcommand depart =>
        (about: "Depart a Supervisor from the gossip ring; kicking and banning the target \
            from joining again with the same member-id")
        (@arg MEMBER_ID: +required +takes_value "The member-id of the Supervisor to depart")
        (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
            "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
    )
}

pub fn sub_sup_secret() -> App<'static, 'static> {
    clap_app!(@subcommand secret =>
        (about: "Commands relating to a Habitat Supervisor's Control Gateway secret")
        (@setting ArgRequiredElseHelp)
        (@subcommand generate =>
            (about: "Generate a secret key to use as a Supervisor's Control Gateway secret")
        )
    )
}

pub fn sub_sup_bash() -> App<'static, 'static> {
    clap_app!(@subcommand bash =>
        (about: "Start an interactive Bash-like shell")
        // set custom usage string, otherwise the binary
        // is displayed confusingly as `hab-sup`
        // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
        (usage: "hab sup bash")
    )
}

pub fn sub_sup_run() -> App<'static, 'static> {
    clap_app!(@subcommand run =>
    (about: "Run the Habitat Supervisor")
    // set custom usage string, otherwise the binary
    // is displayed confusingly as `hab-sup`
    // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
    (usage: "hab sup run [FLAGS] [OPTIONS] [--] [PKG_IDENT_OR_ARTIFACT]")
    (@arg LISTEN_GOSSIP: --("listen-gossip") env(GOSSIP_LISTEN_ADDRESS_ENVVAR) default_value(&GOSSIP_DEFAULT_ADDR) {valid_socket_addr}
        "The listen address for the Gossip System Gateway.")
    (@arg LOCAL_GOSSIP_MODE: --("local-gossip-mode") conflicts_with("LISTEN_GOSSIP") conflicts_with("PEER") conflicts_with("PEER_WATCH_FILE")
        "Start the supervisor in local mode.")
    (@arg LISTEN_HTTP: --("listen-http") env(LISTEN_HTTP_ADDRESS_ENVVAR) default_value(&LISTEN_HTTP_DEFAULT_ADDR) {valid_socket_addr}
        "The listen address for the HTTP Gateway.")
    (@arg HTTP_DISABLE: --("http-disable") -D
        "Disable the HTTP Gateway completely [default: false]")
    (@arg LISTEN_CTL: --("listen-ctl") env(ListenCtlAddr::ENVVAR) default_value(&LISTEN_CTL_DEFAULT_ADDR_STRING) {valid_socket_addr}
        "The listen address for the Control Gateway. If not specified, the value will \
        be taken from the HAB_LISTEN_CTL environment variable if defined. [default: 127.0.0.1:9632]")
    (@arg ORGANIZATION: --org +takes_value
        "The organization that the Supervisor and its subsequent services are part of.")
    (@arg PEER: --peer +takes_value +multiple
        "The listen address of one or more initial peers (IP[:PORT])")
    (@arg PERMANENT_PEER: --("permanent-peer") -I "If this Supervisor is a permanent peer")
    (@arg PEER_WATCH_FILE: --("peer-watch-file") +takes_value conflicts_with("PEER")
        "Watch this file for connecting to the ring"
    )
    (arg: arg_cache_key_path("Path to search for encryption keys. \
        Default value is hab/cache/keys if root and .hab/cache/keys under the home \
        directory otherwise."))
    (@arg RING: --ring -r env(RING_ENVVAR) conflicts_with("RING_KEY") {non_empty}
        "The name of the ring used by the Supervisor when running with wire encryption. \
         (ex: hab sup run --ring myring)")
    (@arg RING_KEY: --("ring-key") env(RING_KEY_ENVVAR) conflicts_with("RING") +hidden {non_empty}
        "The contents of the ring key when running with wire encryption. \
             (Note: This option is explicitly undocumented and for testing purposes only. Do not use it in a production system. Use the corresponding environment variable instead.)
             (ex: hab sup run --ring-key 'SYM-SEC-1 \
              foo-20181113185935 \

                  GCrBOW6CCN75LMl0j2V5QqQ6nNzWm6and9hkKBSUFPI=')")
    (@arg CHANNEL: --channel +takes_value default_value[stable]
        "Receive Supervisor updates from the specified release channel")
    (@arg BLDR_URL: -u --url +takes_value {valid_url}
        "Specify an alternate Builder endpoint. If not specified, the value will \
         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
         https://bldr.habitat.sh)")

    (@arg CONFIG_DIR: --("config-from") +takes_value {dir_exists}
        "Use package config from this path, rather than the package itself")
    (@arg AUTO_UPDATE: --("auto-update") -A "Enable automatic updates for the Supervisor \
        itself")
    (@arg KEY_FILE: --key +takes_value {file_exists} requires[CERT_FILE]
        "Used for enabling TLS for the HTTP gateway. Read private key from KEY_FILE. \
         This should be a RSA private key or PKCS8-encoded private key, in PEM format.")
    (@arg CERT_FILE: --certs +takes_value {file_exists} requires[KEY_FILE]
        "Used for enabling TLS for the HTTP gateway. Read server certificates from CERT_FILE. \
         This should contain PEM-format certificates in the right order (the first certificate \
         should certify KEY_FILE, the last should be a root CA).")
    (@arg CA_CERT_FILE: --("ca-certs") +takes_value {file_exists} requires[CERT_FILE] requires[KEY_FILE]
        "Used for enabling client-authentication with TLS for the HTTP gateway. Read CA certificate from CA_CERT_FILE. \
         This should contain PEM-format certificate that can be used to validate client requests.")
    // === Optional arguments to additionally load an initial service for the Supervisor
    (@arg PKG_IDENT_OR_ARTIFACT: +takes_value "Load the given Habitat package as part of \
        the Supervisor startup specified by a package identifier \
        (ex: core/redis) or filepath to a Habitat Artifact \
        (ex: /home/core-redis-3.0.7-21120102031201-x86_64-linux.hart).")
    (@arg APPLICATION: --application -a +takes_value requires[ENVIRONMENT]
        "Application name; [default: not set].")
    (@arg ENVIRONMENT: --environment -e +takes_value requires[APPLICATION]
        "Environment name; [default: not set].")
    (@arg GROUP: --group +takes_value
        "The service group; shared config and topology [default: default].")
    (@arg TOPOLOGY: --topology -t +takes_value possible_value[standalone leader]
        "Service topology; [default: none]")
    (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
        "The update strategy; [default: none] [values: none, at-once, rolling]")
    (@arg BIND: --bind +takes_value +multiple
        "One or more service groups to bind to a configuration")
    (@arg BINDING_MODE: --("binding-mode") +takes_value {valid_binding_mode}
        "Governs how the presence or absence of binds affects service startup. `strict` blocks \
         startup until all binds are present. [default: strict] [values: relaxed, strict]")
    (@arg VERBOSE: -v "Verbose output; shows file and line/column numbers")
    (@arg NO_COLOR: --("no-color") "Turn ANSI color off")
    (@arg JSON: --("json-logging") "Use structured JSON logging for the Supervisor. \
        Implies NO_COLOR")
    (@arg HEALTH_CHECK_INTERVAL: --("health-check-interval") -i +takes_value {valid_health_check_interval}
        "The interval (seconds) on which to run health checks [default: 30]")
    )
}

pub fn sub_sup_sh() -> App<'static, 'static> {
    clap_app!(@subcommand sh =>
        (about: "Start an interactive Bourne-like shell")
        // set custom usage string, otherwise the binary
        // is displayed confusingly as `hab-sup`
        // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
        (usage: "hab sup sh")
    )
}

pub fn sub_sup_term() -> App<'static, 'static> {
    clap_app!(@subcommand term =>
        (about: "Gracefully terminate the Habitat Supervisor and all of its running services")
        // set custom usage string, otherwise the binary
        // is displayed confusingly as `hab-sup`
        // see: https://github.com/kbknapp/clap-rs/blob/2724ec5399c500b12a1a24d356f4090f4816f5e2/src/app/mod.rs#L373-L394
        (usage: "hab sup term [OPTIONS]")
    )
}

fn sub_svc_start() -> App<'static, 'static> {
    clap_app!(@subcommand start =>
        (about: "Start a loaded, but stopped, Habitat service.")
        (@arg PKG_IDENT: +required +takes_value {valid_ident}
            "A Habitat package identifier (ex: core/redis)")
        (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
            "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
    )
}

// `hab svc status` is the canonical location for this command, but we
// have historically used `hab sup status` as an alias.
pub fn sub_svc_status() -> App<'static, 'static> {
    clap_app!(@subcommand status =>
        (about: "Query the status of Habitat services.")
        (@arg PKG_IDENT: +takes_value {valid_ident} "A Habitat package identifier (ex: core/redis)")
        (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
        "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
    )
}

fn sub_svc_stop() -> App<'static, 'static> {
    clap_app!(@subcommand stop =>
        (about: "Stop a running Habitat service.")
        (@arg PKG_IDENT: +required +takes_value {valid_ident}
            "A Habitat package identifier (ex: core/redis)")
        (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
            "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
    )
}

fn sub_svc_load() -> App<'static, 'static> {
    let mut sub = clap_app!(@subcommand load =>
        (about: "Load a service to be started and supervised by Habitat from a package \
            identifier. If an installed package doesn't satisfy the given package \
            identifier, a suitable package will be installed from Builder.")
        (@arg PKG_IDENT: +required +takes_value {valid_ident}
            "A Habitat package identifier (ex: core/redis)")
        (@arg APPLICATION: --application -a +takes_value requires[ENVIRONMENT]
            "Application name; [default: not set].")
        (@arg ENVIRONMENT: --environment -e +takes_value requires[APPLICATION]
            "Environment name; [default: not set].")
        (@arg CHANNEL: --channel +takes_value default_value[stable]
            "Receive package updates from the specified release channel")
        (@arg GROUP: --group +takes_value
            "The service group; shared config and topology [default: default].")
        (@arg BLDR_URL: -u --url +takes_value {valid_url}
            "Specify an alternate Builder endpoint. If not specified, the value will \
             be taken from the HAB_BLDR_URL environment variable if defined. (default: \
             https://bldr.habitat.sh)")
        (@arg TOPOLOGY: --topology -t +takes_value possible_value[standalone leader]
            "Service topology; [default: none]")
        (@arg STRATEGY: --strategy -s +takes_value {valid_update_strategy}
            "The update strategy; [default: none] [values: none, at-once, rolling]")
        (@arg BIND: --bind +takes_value +multiple
            "One or more service groups to bind to a configuration")
        (@arg BINDING_MODE: --("binding-mode") +takes_value {valid_binding_mode}
             "Governs how the presence or absence of binds affects service startup. `strict` blocks \
              startup until all binds are present. [default: strict] [values: relaxed, strict]")
        (@arg FORCE: --force -f "Load or reload an already loaded service. If the service \
            was previously loaded and running this operation will also restart the service")
        (@arg REMOTE_SUP: --("remote-sup") -r +takes_value
            "Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]")
        (@arg HEALTH_CHECK_INTERVAL: --("health-check-interval") -i +takes_value {valid_health_check_interval}
            "The interval (seconds) on which to run health checks [default: 30]")
    );

    if cfg!(windows) {
        sub = sub.arg(Arg::with_name("PASSWORD").long("password")
                                                .takes_value(true)
                                                .help("Password of the service user"));
    }

    sub
}

// CLAP Validation Functions
////////////////////////////////////////////////////////////////////////

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_binding_mode(val: String) -> result::Result<(), String> {
    match habitat_sup_protocol::types::BindingMode::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Binding mode: '{}' is not valid", &val)),
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_pair_type(val: String) -> result::Result<(), String> {
    match PairType::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(format!("PAIR_TYPE: {} is invalid, must be one of \
                         (public, secret)",
                        &val))
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_service_group(val: String) -> result::Result<(), String> {
    ServiceGroup::validate(&val).map_err(|e| e.to_string())
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
fn valid_socket_addr(val: String) -> result::Result<(), String> {
    match SocketAddr::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err("Socket address should include both IP and port, eg: '0.0.0.0:9700'".to_string())
        }
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
fn valid_numeric<T: FromStr>(val: String) -> result::Result<(), String> {
    match val.parse::<T>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("'{}' is not a valid number", &val)),
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_health_check_interval(val: String) -> result::Result<(), String> {
    match HealthCheckInterval::from_str(&val) {
        Ok(_) => Ok(()),
        Err(e) => {
            Err(format!("'{}' is not a valid value for health check \
                         interval: {}",
                        val, e))
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_update_strategy(val: String) -> result::Result<(), String> {
    match habitat_sup_protocol::types::UpdateStrategy::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => Err(format!("Update strategy: '{}' is not valid", &val)),
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_ident(val: String) -> result::Result<(), String> {
    match PackageIdent::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(format!("'{}' is not valid. Package identifiers have the \
                         form origin/name[/version[/release]]",
                        &val))
        }
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn valid_target(val: String) -> result::Result<(), String> {
    match PackageTarget::from_str(&val) {
        Ok(_) => Ok(()),
        Err(_) => {
            Err(format!("'{}' is not valid. A valid target is in the form \
                         architecture-platform (example: x86_64-linux)",
                        &val))
        }
    }
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
fn valid_origin(val: String) -> result::Result<(), String> {
    if ident::is_valid_origin_name(&val) {
        Ok(())
    } else {
        Err(format!("'{}' is not valid. A valid origin contains a-z, \
                     0-9, and _ or - after the first character",
                    &val))
    }
}

#[allow(clippy::needless_pass_by_value)] // Signature required by CLAP
fn non_empty(val: String) -> result::Result<(), String> {
    if val.is_empty() {
        Err("must not be empty (check env overrides)".to_string())
    } else {
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    fn no_feature_flags() -> FeatureFlag { FeatureFlag::empty() }

    use super::*;

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
}
