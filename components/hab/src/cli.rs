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

use clap::{App, AppSettings, Arg, ArgGroup, SubCommand};
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

    App::new("hab")
        .about("\"A Habitat is the natural environment for your services\" - Alan Turing")
        .version(super::VERSION)
        .author("\nAuthors: The Habitat Maintainers <humans@habitat.sh>\n")
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("cli")
            .about("Commands relating to Habitat runtime config")
            .aliases(&["cl"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(sub_cli_setup().aliases(&["s", "se", "set", "setu"]))
            .subcommand(sub_cli_completers().aliases(&["c", "co", "com", "comp"]))
        )
        .subcommand(SubCommand::with_name("config")
            .about("Commands relating to Habitat runtime config")
            .aliases(&["co", "con", "conf", "confi"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(sub_config_apply().aliases(&["a", "ap", "app", "appl"]))
        )
        .subcommand(SubCommand::with_name("file")
            .about("Commands relating to Habitat files")
            .aliases(&["f", "fi", "fil"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("upload")
                .about("Upload a file to the Supervisor ring.")
                .aliases(&["u", "up", "upl", "uplo", "uploa"])
                .arg(Arg::with_name("SERVICE_GROUP")
                    .required(true)
                    .takes_value(true)
                    .validator(valid_service_group)
                    .help("Target service group (ex: redis.default)")
                )
                .arg(Arg::with_name("FILE")
                    .required(true)
                    .validator(file_exists)
                    .help("Path to local file on disk")
                )
                .arg(Arg::with_name("VERSION_NUMBER")
                    .required(true)
                    .help("A version number (positive integer) for this configuration (ex: 42)")
                )
                .arg(Arg::with_name("ORG")
                    .long("org")
                    .takes_value(true)
                    .help("Name of service organization")
                )
                .arg(Arg::with_name("USER")
                    .takes_value(true)
                    .help("Name of the user key")
                )
                .arg(Arg::with_name("PEER")
                    .short("p")
                    .long("peer")
                    .takes_value(true)
                    .help("A comma-delimited list of one or more Habitat Supervisor peers to infect \
                    (default: 127.0.0.1:9638)")
                )
                .arg(Arg::with_name("RING")
                    .short("r")
                    .long("ring")
                    .takes_value(true)
                    .help("Ring key name, which will encrypt communication messages")
                )
            )
        )
        .subcommand(SubCommand::with_name("bldr")
            .about("Commands relating to Habitat Builder")
            .aliases(&["b", "bl", "bld"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("job")
                .about("Commands relating to Habitat Builder jobs")
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("start")
                    .about("Schedule a build job or group of jobs")
                    .aliases(&["s", "st", "sta", "star"])
                    .arg(Arg::with_name("PKG_IDENT")
                        .required(true)
                        .takes_value(true)
                        .help("The origin and name of the package to schedule a job for (eg: core/redis)"))
                    .arg(bldr_url())
                    .arg(bldr_auth_token())
                    .arg(Arg::with_name("GROUP")
                        .short("g")
                        .long("group")
                        .help("Schedule jobs for this package and all of its reverse dependencies")
                    )
                )
                .subcommand(SubCommand::with_name("cancel")
                    .about("Cancel a build job group and any in-progress builds")
                    .aliases(&["c", "ca", "can", "cance", "cancel"])
                    .arg(bldr_group_id())
                    .arg(bldr_url())
                    .arg(bldr_auth_token())
                )
                .subcommand(SubCommand::with_name("promote")
                    .about("Promote packages from a completed build job to a specified channel")
                    .aliases(&["p", "pr", "pro", "prom", "promo", "promot"])
                    .arg(bldr_group_id())
                    .arg(pkg_channel(true, true, "The target channel name"))
                    .arg(origin(false, true, "Limit the promotable packages to the specified origin"))
                    .arg(Arg::with_name("INTERACTIVE")
                        .short("i")
                        .long("interactive")
                        .help("Allow editing the list of promotable packages")
                    )
                    .arg(Arg::with_name("VERBOSE")
                        .short("v")
                        .long("verbose")
                        .help("Show full list of promotable packages")
                    )
                    .arg(bldr_url())
                    .arg(bldr_auth_token())
                )
                .subcommand(SubCommand::with_name("demote")
                    .about("Demote packages from a completed build job to a specified channel")
                    .aliases(&["d", "de", "dem", "demo", "demot"])
                    .arg(bldr_group_id())
                    .arg(pkg_channel(true, true, "The target channel name"))
                    .arg(origin(false, true, "Limit the demotable packages to the specified origin"))
                    .arg(Arg::with_name("INTERACTIVE")
                        .short("i")
                        .long("interactive")
                        .help("Allow editing the list of demotable packages")
                    )
                    .arg(Arg::with_name("VERBOSE")
                        .short("v")
                        .long("verbose")
                        .help("Show full list of demotable packages")
                    )
                    .arg(bldr_url())
                    .arg(bldr_auth_token())
                )
                .subcommand(SubCommand::with_name("status")
                    .group(ArgGroup::with_name("status")
                        .required(true)
                        .args(&["ORIGIN", "GROUP_ID"])
                    )
                    .about("Get the status of a job group")
                    .aliases(&["stat", "statu"])
                    .arg(bldr_url())
                    .arg(bldr_group_id())
                    .arg(origin(false, true, "You can see the status of every group in an origin by providing this value"))
                )
                .subcommand(SubCommand::with_name("encrypt")
                    .about("Reads a stdin stream containing plain text and outputs an encrypted representation")
                    .aliases(&["e", "en", "enc", "encr", "encry"])
                    .arg(bldr_url())
                )
                .subcommand(SubCommand::with_name("channel")
                    .about("Commands relating to Habitat Builder channels")
                    .aliases(&["c", "ch", "cha", "chan", "chann", "channe"])
                    .setting(AppSettings::ArgRequiredElseHelp)
                    .subcommand(SubCommand::with_name("create")
                        .about("Creates a new channel")
                        .aliases(&["c", "cr", "cre", "crea", "creat"])
                        .arg(bldr_url())
                        .arg(pkg_channel(true, true, "The channel name"))
                        .arg(origin(false, true,
                            "Sets the origin to which the channel will belong. Default is from 'HAB_ORIGIN' or cli.toml")
                        )
                    )
                    .subcommand(SubCommand::with_name("destroy")
                        .about("Destroys a channel")
                        .aliases(&["d", "de", "des", "dest", "destr", "destro"])
                        .arg(bldr_url())
                        .arg(pkg_channel(true, true, "The channel name"))
                        .arg(origin(false, true,
                            "Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml")
                        )
                    )
                    .subcommand(SubCommand::with_name("list")
                        .about("Lists origin channels")
                        .aliases(&["l", "li", "lis"])
                        .arg(bldr_url())
                        .arg(origin(false, true,
                            "The origin for which channels will be listed. Default is from 'HAB_ORIGIN' or cli.toml")
                        )
                    )
                )
            )
        )
        .subcommand(SubCommand::with_name("origin")
            .about("Commands relating to Habitat origin keys")
            .aliases(&["o", "or", "ori", "orig", "origi"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("key")
                .about("Commands relating to Habitat origin key maintenance")
                .aliases(&["k", "ke"])
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("download")
                    .about("Download origin key(s) to HAB_CACHE_KEY_PATH")
                    .aliases(&["d", "do", "dow", "down", "downl", "downlo", "downloa"])
                    .arg(origin(true, false, "The origin name"))
                    .arg(Arg::with_name("REVISION")
                        .help("The key revision")
                    )
                    .arg(bldr_url())
                    .arg(Arg::with_name("WITH_SECRET")
                        .short("s")
                        .long("secret")
                        .help("Download secret key instead of public key")
                    )
                    .arg(bldr_auth_token())
                )
                .subcommand(SubCommand::with_name("export")
                    .about("Outputs the latest origin key contents to stdout")
                    .aliases(&["e", "ex", "exp", "expo", "expor"])
                    .arg(origin(true, true, "origin to export key from"))
                    .arg(Arg::with_name("PAIR_TYPE")
                        .short("t")
                        .long("type")
                        .takes_value(true)
                        .validator(valid_pair_type)
                        .help("Export either the 'public' or 'secret' key")
                    )
                )
                .subcommand(SubCommand::with_name("generate")
                    .about("Generates a Habitat origin key")
                    .aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    .arg(origin(false, false, "The origin name"))
                )
                .subcommand(SubCommand::with_name("import")
                    .about("Reads a stdin stream containing a public or secret origin key contents and writes the key to disk")
                    .aliases(&["i", "im", "imp", "impo", "impor"])
                )
                .subcommand(SubCommand::with_name("upload")
                    .about("Upload origin keys to Builder")
                    .aliases(&["u", "up", "upl", "uplo", "uploa"])
                    .group(ArgGroup::with_name("upload")
                        .required(true)
                        .args(&["ORIGIN", "PUBLIC_FILE"])
                    )
                    .arg(origin(true, false, "The origin name"))
                    .arg(Arg::with_name("PUBLIC_FILE")
                        .long("pubfile")
                        .takes_value(true)
                        .validator(file_exists)
                        .help("Path to a local public origin key file on disk")
                    )
                    .arg(Arg::with_name("WITH_SECRET")
                        .short("s")
                        .long("secret")
                        .takes_value(true)
                        .conflicts_with("PUBLIC_FILE")
                        .help("Upload secret key in addition to the public key")
                    )
                    .arg(Arg::with_name("SECRET_FILE")
                        .long("secfile")
                        .takes_value(true)
                        .validator(file_exists)
                        .conflicts_with("ORIGIN")
                        .help("Path to a local secret origin key file on disk")
                    )
                    .arg(bldr_url())
                    .arg(bldr_auth_token()))
            )
        )
        .subcommand(SubCommand::with_name("pkg")
            .about("Commands relating to Habitat packages")
            .aliases(&["p", "pk", "package"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("binds")
                .about("Displays the binds for a service")
                .arg(pkg_ident(None))
            )
            .subcommand(SubCommand::with_name("binlink")
                .about("Creates a binlink for a package binary in a common 'PATH' location")
                .aliases(&["bi", "bin", "binl", "binli", "binlin"])
                .arg(pkg_ident(None))
                .arg(Arg::with_name("BINARY")
                    .takes_value(true)
                    .help("The command to binlink (ex: bash)")
                )
                .arg(Arg::with_name("DEST_DIR")
                    .short("d")
                    .long("dest")
                    .takes_value(true)
                    .help("Sets the destination directory (default: /bin)")
                )
                .arg(Arg::with_name("FORCE")
                    .short("f")
                    .long("force")
                    .help("Overwrite existing binlinks")
                )
            )
            .subcommand(SubCommand::with_name("config")
                .about("Displays the default configuration options for a service")
                .aliases(&["p", "pk", "package"])
                .arg(pkg_ident(None))
            )
            .subcommand(sub_pkg_build())
            .subcommand(SubCommand::with_name("env")
                .about("Prints the runtime environment of a specific installed package")
                .arg(pkg_ident(None))
            )
            .subcommand(SubCommand::with_name("exec")
                .about("Executes a command using the 'PATH' context of an installed package")
                .aliases(&["exe"])
                .arg(pkg_ident(None))
                .arg(Arg::with_name("CMD")
                    .required(true)
                    .takes_value(true)
                    .help("The command to execute (ex: ls)")
                )
                .arg(Arg::with_name("ARGS")
                    .takes_value(true)
                    .multiple(true)
                    .help("Arguments to the command (ex: -l /tmp)")
                )
            )
            .subcommand(SubCommand::with_name("export")
                .about("Exports the package to the specified format")
                .aliases(&["exp"])
                .arg(Arg::with_name("FORMAT")
                    .required(true)
                    .takes_value(true)
                    .help("The export format (ex: aci, cf, docker, kubernetes, mesos, or tar)")
                )
                .arg(pkg_ident(Some("A package identifier (ex: core/redis, core/busybox-static/1.42.2) or \
                    filepath to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)"))
                )
                .arg(bldr_url())
                .arg(pkg_channel(false, true, "Retrieve the container's package from the specified release channel (default: stable)"))
            )
            .subcommand(SubCommand::with_name("hash")
                .about("Generates a blake2b hashsum from a target at any given filepath")
                .aliases(&["ha", "has"])
                .arg(Arg::with_name("SOURCE")
                    .takes_value(true)
                    .validator(file_exists)
                    .help("A filepath of the target")
                )
            )
            .subcommand(sub_pkg_install()
                .aliases(&["i", "in", "ins", "inst", "insta", "instal"])
            )
            .subcommand(SubCommand::with_name("path")
                .about("Prints the path to a specific installed release of a package")
                .aliases(&["p", "pa", "pat"])
                .arg(pkg_ident(None))
            )
            .subcommand(SubCommand::with_name("provides")
                .about("Search installed Habitat packages for a given file")
                .arg(Arg::with_name("FILE")
                    .required(true)
                    .takes_value(true)
                    .help("File name to find")
                )
                .arg(Arg::with_name("FULL_RELEASES")
                    .short("r")
                    .help("Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)")
                )
                .arg(Arg::with_name("FULL_PATHS")
                    .short("p")
                    .help("Show full path to file")
                )
            )
            .subcommand(SubCommand::with_name("search")
                .about("Search for a package in Builder")
                .arg(Arg::with_name("SEARCH_TERM")
                    .required(true)
                    .takes_value(true)
                    .help("Search term")
                )
                .arg(bldr_url())
                .arg(bldr_auth_token())
            )
            .subcommand(SubCommand::with_name("sign")
                .about("Signs an archive with an origin key, generating a Habitat Artifact")
                .aliases(&["s", "si", "sig"])
                .arg(origin(false, true, "Origin key used to create signature"))
                .arg(Arg::with_name("SOURCE")
                    .required(true)
                    .validator(file_exists)
                    .help("A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)")
                )
                .arg(Arg::with_name("DEST")
                    .required(true)
                    .help("The destination path to the signed Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                )
            )
            .subcommand(SubCommand::with_name("upload")
                .about("Uploads a local Habitat Artifact to Builder")
                .aliases(&["u", "up", "upl", "uplo", "uploa"])
                .arg(bldr_url())
                .arg(bldr_auth_token())
                .arg(pkg_channel(false, true, "Additional release channel to upload package to. \
                     Packages are always uploaded to `unstable`, regardless \
                     of the value of this option. (default: none)")
                )
                .arg(Arg::with_name("HART_FILE")
                    .required(true)
                    .multiple(true)
                    .validator(file_exists)
                    .help("One or more filepaths to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                )
            )
            .subcommand(SubCommand::with_name("promote")
                .about("Promote a package to a specified channel")
                .aliases(&["pr", "pro", "promo", "promot"])
                .arg(bldr_url())
                .arg(pkg_ident(None))
                .arg(pkg_channel(true, true, "Promote to the specified release channel"))
                .arg(bldr_auth_token())
            )
            .subcommand(SubCommand::with_name("demote")
                .about("Demote a package from a specified channel")
                .aliases(&["de", "dem", "demo", "demot"])
                .arg(bldr_url())
                .arg(pkg_ident(None))
                .arg(pkg_channel(true, true, "Demote from the specified release channel"))
                .arg(bldr_auth_token())
            )
            .subcommand(SubCommand::with_name("channels")
                .about("Find out what channels a package belongs to")
                .aliases(&["ch", "cha", "chan", "chann", "channe", "channel"])
                .arg(bldr_url())
                .arg(pkg_ident(None))
                .arg(bldr_auth_token())
            )
            .subcommand(SubCommand::with_name("verify")
                .about("Verifies a Habitat Artifact with an origin key")
                .aliases(&["v", "ve", "ver", "veri", "verif"])
                .arg(Arg::with_name("SORUCE")
                    .required(true)
                    .validator(file_exists)
                    .help("A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                )
            )
            .subcommand(SubCommand::with_name("header")
                .about("Returns the Habitat Artifact header")
                .aliases(&["hea", "head", "heade", "header"])
                .setting(AppSettings::Hidden)
                .arg(Arg::with_name("SORUCE")
                    .required(true)
                    .validator(file_exists)
                    .help("A path to a Habitat Artifact \
                    (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
                )
            )
        )
        .subcommand(SubCommand::with_name("plan")
            .about("Commands relating to plans and other app-specific configuration.")
            .aliases(&["pl", "pla"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("init")
                .about("Generates common package specific configuration files. Executing without \
                    argument will create a `habitat` directory in your current folder for the \
                    plan. If `PKG_NAME` is specified it will create a folder with that name. \
                    Environment variables (those starting with 'pkg_') that are set will be used \
                    in the generated plan")
                .aliases(&["i", "in", "ini"])
                .arg(Arg::with_name("PKG_NAME")
                    .takes_value(true)
                    .help("Name for the new app")
                )
                .arg(origin(false, true, "Origin for the new app"))
                .arg(Arg::with_name("WITH_DOCS")
                    .long("with-docs")
                    .help("Include plan options documentation")
                )
                .arg(Arg::with_name("WITH_CALLBACKS")
                    .long("with-callbacks")
                    .help("Include callback functions in template")
                )
                .arg(Arg::with_name("WITH_ALL")
                    .long("with-all")
                    .help("Generate omnibus plan with all available plan options")
                )
                .arg(Arg::with_name("WINDOWS")
                    .long("windows")
                    .help("Use a Windows Powershell plan template")
                )
                .arg(Arg::with_name("SCAFFOLDING")
                    .short("s")
                    .long("scaffolding")
                    .takes_value(true)
                    .help("Specify explicit Scaffolding for your app (ex: node, ruby)")
                )
            )
        )
        .subcommand(SubCommand::with_name("ring")
            .about("Commands relating to Habitat rings")
            .aliases(&["r", "ri", "rin"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("key")
                .about("Commands relating to Habitat ring keys")
                .aliases(&["k", "ke"])
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("export")
                    .about("Outputs the latest ring key contents to stdout")
                    .aliases(&["e", "ex", "exp", "expo", "expor"])
                    .arg(Arg::with_name("RING")
                        .required(true)
                        .takes_value(true)
                        .help("Ring key name")
                    )
                )
                .subcommand(SubCommand::with_name("import")
                    .about("Reads a stdin stream containing ring key contents and writes \
                    the key to disk")
                    .aliases(&["i", "im", "imp", "impo", "impor"])
                )
                .subcommand(SubCommand::with_name("generate")
                    .about("Generates a Habitat ring key")
                    .aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    .arg(Arg::with_name("RING")
                        .required(true)
                        .takes_value(true)
                        .help("Ring key name")
                    )
                )
            )
        )
        .subcommand(SubCommand::with_name("svc")
            .about("Commands relating to Habitat services")
            .aliases(&["sv", "ser", "serv", "service"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("key")
                .about("Commands relating to Habitat service keys")
                .aliases(&["k", "ke"])
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("generate")
                    .about("Generates a Habitat service key")
                    .aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    .arg(Arg::with_name("SERVICE_GROUP")
                        .required(true)
                        .takes_value(true)
                        .validator(valid_service_group)
                        .help("Target service group (ex: redis.default)")
                    )
                    .arg(Arg::with_name("ORG")
                        .help("The service organization")
                    )
                )
            )
            .subcommand(SubCommand::with_name("load")
                .about("Load a service to be started and supervised by Habitat from a package or \
                    artifact. Services started in this manner will persist through Supervisor \
                    restarts.")
                .setting(AppSettings::Hidden)
            )
            .subcommand(SubCommand::with_name("unload")
                .about("Unload a persistent or transient service started by the Habitat \
                    Supervisor. If the Supervisor is running when the service is unloaded the \
                    service will be stopped.")
                .setting(AppSettings::Hidden)
            )
            .subcommand(SubCommand::with_name("start")
                .about("Start a loaded, but stopped, Habitat service or a transient service from \
                    a package or artifact. If the Habitat Supervisor is not already running this \
                    will additionally start one for you.")
                .setting(AppSettings::Hidden)
            )
            .subcommand(SubCommand::with_name("stop")
                .about("Stop a running Habitat service.")
                .setting(AppSettings::Hidden)
            )
            .after_help("\nALIASES:\
                \n    load       Alias for: 'sup load'\
                \n    unload     Alias for: 'sup unload'\
                \n    start      Alias for: 'sup start'\
                \n    stop       Alias for: 'sup stop'\
                \n    status     Alias for: 'sup status'\
                \n")
        )
        .subcommand(SubCommand::with_name("studio")
            .about("Commands relating to Habitat Studios")
            .aliases(&["stu", "stud", "studi"])
        )
        .subcommand(SubCommand::with_name("sup")
            .about("Commands relating to the Habitat Supervisor")
            .aliases(&["su"])
        )
        .subcommand(SubCommand::with_name("user")
            .about("Commands relating to Habitat users")
            .aliases(&["u", "us", "use"])
            .setting(AppSettings::ArgRequiredElseHelp)
            .subcommand(SubCommand::with_name("key")
                .about("Commands relating to Habitat user keys")
                .aliases(&["k", "ke"])
                .setting(AppSettings::ArgRequiredElseHelp)
                .subcommand(SubCommand::with_name("generate")
                    .about("Generates a Habitat user key")
                    .aliases(&["g", "ge", "gen", "gene", "gener", "genera", "generat"])
                    .arg(Arg::with_name("USER")
                        .required(true)
                        .takes_value(true)
                        .help("Name of the user key")
                    )
                )
            )
        )
        .subcommand(alias_apply)
        .subcommand(alias_install)
        .subcommand(alias_run())
        .subcommand(alias_setup)
        .subcommand(alias_start())
        .subcommand(alias_stop())
        .subcommand(alias_term())
        .after_help("\nALIASES:\
            \n    apply      Alias for: 'config apply'\
            \n    install    Alias for: 'pkg install'\
            \n    run        Alias for: 'sup run'\
            \n    setup      Alias for: 'cli setup'\
            \n    start      Alias for: 'svc start'\
            \n    stop       Alias for: 'svc stop'\
            \n    term       Alias for: 'sup term'\
            \n")
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

fn bldr_url() -> Arg<'static, 'static> {
    Arg::with_name("BLDR_URL")
        .short("u")
        .long("url")
        .takes_value(true)
        .validator(valid_url)
        .help(
            "Specify an alternate Builder endpoint. If not specified, the value will \
             be taken from the HAB_BLDR_URL environment variable if defined. (default: \
             https://bldr.habitat.sh)",
        )
}

fn bldr_group_id() -> Arg<'static, 'static> {
    Arg::with_name("GROUP_ID")
        .required(true)
        .takes_value(true)
        .help(
        "The job group id that was returned from \"hab bldr job start\" (ex: 771100000000000000)",
    )
}

fn bldr_auth_token() -> Arg<'static, 'static> {
    Arg::with_name("AUTH_TOKEN")
        .short("z")
        .long("auth")
        .takes_value(true)
        .help("Authentication token for Builder")
}

fn pkg_channel(
    required: bool,
    takes_value: bool,
    help_text: &'static str,
) -> Arg<'static, 'static> {
    Arg::with_name("CHANNEL")
        .short("c")
        .long("channel")
        .takes_value(takes_value)
        .required(required)
        .help(help_text)
}

fn origin(required: bool, takes_value: bool, help_text: &'static str) -> Arg<'static, 'static> {
    Arg::with_name("ORIGIN")
        .short("o")
        .long("origin")
        .required(required)
        .takes_value(takes_value)
        .help(help_text)
}

fn pkg_ident(help_text: Option<&'static str>) -> Arg<'static, 'static> {
    let arg = Arg::with_name("PKG_IDENT").required(true).takes_value(true);

    match help_text {
        Some(text) => arg.help(text),
        None => arg.help("A package identifier (ex: core/redis, core/busybox-static/1.42.2)"),
    }
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
                .help("Reuses a previous Studio for the build (default: clean up before building)")
                .short("R")
                .long("reuse"),
        ).arg(
            Arg::with_name("DOCKER")
                .help(
                    "Uses a Dockerized Studio for the build (default: Studio uses a chroot on \
                     linux)",
                )
                .short("D")
                .long("docker"),
        )
    } else if cfg!(target_os = "windows") {
        sub.arg(
            Arg::with_name("WINDOWS")
                .help("Use a Windows Studio instead of a Docker Studio")
                .short("w")
                .long("windows"),
        )
    } else {
        sub
    }
}

fn sub_pkg_install() -> App<'static, 'static> {
    clap_app!(@subcommand install =>
        (about: "Installs a Habitat package from Builder or locally from a Habitat Artifact")
        (@arg BLDR_URL: --url -u +takes_value {valid_url}
            "Specify an alternate Builder endpoint. If not specified, the value will \
                         be taken from the HAB_BLDR_URL environment variable if defined. (default: \
                         https://bldr.habitat.sh)")
        (@arg CHANNEL: --channel -c +takes_value
            "Install from the specified release channel (default: stable)")
        (@arg PKG_IDENT_OR_ARTIFACT: +required +multiple
            "One or more Habitat package identifiers (ex: acme/redis) and/or filepaths \
            to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)")
        (@arg BINLINK: -b --binlink "Binlink all binaries from installed package(s)")
        (@arg FORCE: -f --force "Overwrite existing binlinks")
        (@arg AUTH_TOKEN: -z --auth +takes_value "Authentication token for Builder")
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
        Err(_) => Err(format!(
            "PAIR_TYPE: {} is invalid, must be one of (public, secret)",
            &val
        )),
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
