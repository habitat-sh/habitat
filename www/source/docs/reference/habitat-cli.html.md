---
title: Habitat CLI reference
---

# Habitat command-line interface (CLI) reference

The commands for the Habitat CLI (`hab`) are listed below. This document is not currently auto-updating, so the results of `hab --help` should be considered a better source for a complete list of commands for the time being.

- [hab](#hab)
- [hab bldr](#hab-bldr)
- [hab cli](#hab-cli)
- [hab config apply](#hab-config-apply)
- [hab file upload](#hab-file-upload)
- [hab job](#hab-job)
- [hab origin key](#hab-origin-key)
- [hab pkg](#hab-pkg)
- [hab plan init](#hab-plan-init)
- [hab ring key](#hab-ring-key)
- [hab studio](#hab-studio)
- [hab sup](#hab-sup)
- [hab svc key generate](#hab-svc)
- [hab user key generate](#hab-user-key-generate)   

<h2 id="hab" class="anchor">hab</h2>

The main program that allows you to sign and upload packages, start Habitat services, and other related functions through various subcommands.

**USAGE**

    hab [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**SUBCOMMANDS**

    bldr      Commands relating to Habitat build service
    cli       Commands relating to Habitat runtime config
    config    Commands relating to Habitat runtime config
    file      Commands relating to Habitat files
    help      Prints this message or the help of the given subcommand(s)
    job       Commands relating to build job control
    origin    Commands relating to Habitat origin keys
    pkg       Commands relating to Habitat packages
    plan      Commands relating to plans and other app-specific configuration.
    ring      Commands relating to Habitat rings
    studio    Commands relating to Habitat Studios
    sup       Commands relating to the Habitat Supervisor
    svc       Commands relating to Habitat services
    user      Commands relating to Habitat users

**ALIASES**

    apply      Alias for: 'config apply'
    install    Alias for: 'pkg install'
    run        Alias for: 'sup run'
    setup      Alias for: 'cli setup'
    start      Alias for: 'svc start'
    stop       Alias for: 'svc stop'
    term       Alias for: 'sup term'
***

<h2 id="hab-bldr" class="anchor">hab bldr</h2>
Commands relating to Habitat build service.

**USAGE**

    hab bldr [SUBCOMMAND]

**FLAGS** 

    -h, --help    Prints help information

**SUBCOMMANDS**

    encrypt  Reads a stdin stream containing plain text and outputs an encrypted representation
    help     Prints this message or the help of the given subcommand(s)

**Read More:**

- [hab bldr encrypt](#hab-bldr-encrypt)

<h2 id="hab-bldr-encrypt" class="anchor">hab bldr encrypt</h2>
Reads a stdin stream containing plain text and outputs an encrypted representation.

**USAGE** 

    hab bldr encrypt [OPTIONS]  

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -u, --url <DEPOT_URL>    Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

<h2 id="hab-cli" class="anchor">hab cli</h2>
Commands relating to Habitat runtime config.

**USAGE** 

    hab cli [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS** 

    completers    Creates command-line completers for your shell.
    help          Prints this message or the help of the given subcommand(s)
    setup         Sets up the CLI with reasonable defaults.

**Read More** 

- [hab cli completers](#hab-cli-completers)
- [hab cli setup](#hab-cli-setup)

<h2 id="hab-cli-completers" class="anchor">hab cli completers</h2>
Creates command-line completers for your shell.

**USAGE**

    hab cli completers --shell <SHELL>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -s, --shell <SHELL>    The name of the shell you want to generate the command-completion. Supported Shells: bash, fish, zsh, powershell [values: bash, fish, zsh, powershell]

<h2 id="hab-cli-setup" class="anchor">hab cli setup</h2>
Sets up the CLI with reasonable defaults.

**USAGE** 

    hab cli setup 

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

<h2 id="hab-config-apply" class="anchor">hab config apply</h2>
Applies a configuration to a group of Habitat Supervisors.

**USAGE** 

    hab config apply [OPTIONS] <SERVICE_GROUP> <VERSION_NUMBER> [FILE]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

        --org <ORG>      Name of service organization to use for encryption
    -p, --peer <PEER>    A comma-delimited list of one or more Habitat Supervisor peers (default: 127.0.0.1:9638)
    -r, --ring <RING>    Ring key name, which will encrypt communication messages
    -u, --user <USER>    Name of a user key to use for encryption

**ARGS** 

    <SERVICE_GROUP>     Target service group (ex: redis.default)
    <VERSION_NUMBER>    A version number (positive integer) for this configuration (ex: 42)
    <FILE>              Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)

**Read More**: 

- <a href="/docs/run-packages-apply-config-updates">Knowledge Article: Configuration Updates</a>

<h2 id="hab-file-upload" class="anchor">hab file</h2>
Upload a file to the Supervisor ring.

**USAGE** 

    hab file upload [OPTIONS] <SERVICE_GROUP> <VERSION_NUMBER> <FILE>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

        --org  <ORG>     Name of service organization
    -p, --peer <PEER>    A comma-delimited list of one or more Habitat Supervisor peers (default: 127.0.0.1:9638)
    -r, --ring <RING>    Ring key name, which will encrypt communication messages
    -u, --user <USER>    Name of the user key

**ARGS** 

    <SERVICE_GROUP>     Target service group (ex: redis.default)
    <VERSION_NUMBER>    A version number (positive integer) for this
                        configuration (ex: 42)
    <FILE>              Path to local file on disk

<h2 id="hab-job" class="anchor">hab job</h2>
Commands relating to build job control.

**USAGE** 

    hab job [SUBCOMMAND]

**FLAGS** 

    -h, --help    Prints help information

**SUBCOMMANDS** 

    help       Prints this message or the help of the given subcommand(s)
    promote    Promote every package in a job group to a specified channel
    start      Schedule a job or group of jobs

**Read More** 

- [hab job promote](#hab-job-promote)
- [hab job start](#hab-job-start)

<h2 id="hab-job-promote" class="anchor">hab job promote</h2>
Promote every package in a job group to a specified channel.

**USAGE** 

    hab job promote [OPTIONS] <GROUP_ID> <CHANNEL>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -z, --auth <AUTH_TOKEN>    Authentication token for the Depot
    -u, --url <DEPOT_URL>      Use a specific Depot URL (ex:http://depot.example.com/v1/depot)

<h2 id="hab-origin-key" class="anchor">hab origin key</h2>
Commands relating to Habitat origin key maintenance.

**USAGE** 

    hab origin key [SUBCOMMAND]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**SUBCOMMANDS** 

    download    Download origin key(s) to HAB_CACHE_KEY_PATH
    export      Outputs the latest origin key contents to stdout
    generate    Generates a Habitat origin key
    help        Prints this message or the help of the given subcommand(s)
    import      Reads a stdin stream containing a public or secret origin key contents and writes the key to disk
    upload      Upload origin keys to the depot

**Read More:** 

- [hab origin key download](#hab-origin-key-download)
- [hab origin key export](#hab-origin-key-export)
- [hab origin key generate](#hab-origin-key-generate)
- [hab origin key import](#hab-origin-key-import)
- [hab origin key upload](#hab-origin-key-upload)
- <a href="/docs/share-packages-overview">Knowledge Article: Sharing Packages</a>
- <a href="/docs/concepts-keys">Knowledge Article: Keys</a>


<h2 id="hab-origin-key-download" class="anchor">hab origin key download</h2>
Download origin key(s) to HAB_CACHE_KEY_PATH

**USAGE** 

    hab origin key download [OPTIONS] <ORIGIN> [REVISION]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -u, --url <DEPOT_URL>    Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS** 

    <ORIGIN>      The origin name
    <REVISION>    The key revision

<h2 id="hab-origin-key-export" class="anchor">hab origin key download</h2>
Outputs the latest origin key contents to stdout.

**USAGE** 

    hab origin key export [OPTIONS] <ORIGIN>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -t, --type <PAIR_TYPE>    Export either the `public' or `secret' key

**ARGS** 

    <ORIGIN>

<h2 id="hab-origin-key-generate" class="anchor">hab origin key download</h2>
Generates a Habitat origin key.

**USAGE** 

    hab origin key generate [ORIGIN]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <ORIGIN>    The origin name

<h2 id="hab-origin-key-import" class="anchor">hab origin key download</h2>
Reads a stdin stream containing a public or secret origin key contents and writes the key to disk.

**USAGE** 

    hab origin key import

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

<h2 id="hab-origin-key-upload" class="anchor">hab origin key download</h2>
Upload origin keys to the depot.

**USAGE** 

    hab origin key upload [FLAGS] [OPTIONS] <ORIGIN|--pubfile <PUBLIC_FILE>>

**FLAGS** 

    -s, --secret     Upload secret key in addition to the public key
    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -z, --auth <AUTH_TOKEN>     Authentication token for the Depot
    -u, --url <DEPOT_URL>       Use a specific Depot URL (ex: http://depot.example.com/v1/depot)
        --pubfile <PUBLIC_FILE> Path to a local public origin key file on disk
        --secfile <SECRET_FILE> Path to a local secret origin key file on disk

**ARGS** 

    <ORIGIN>    The origin name

<h2 id="hab-pkg" class="anchor">hab pkg</h2>
Commands relating to Habitat packages.

**USAGE**     

    hab pkg [SUBCOMMAND]

**FLAGS** 

    -h, --help    Prints help information

**SUBCOMMANDS** 

    binlink     Creates a symlink for a package binary in a common 'PATH' location
    build       Builds a Plan using a Studio
    channels    Find out what channels a package belongs to
    config      Displays the default configuration options for a service
    demote      Demote a package from a specified channel
    env         Prints the runtime environment of a specific installed package
    exec        Executes a command using the 'PATH' context of an installed package
    export      Exports the package to the specified format
    hash        Generates a blake2b hashsum from a target at any given filepath
    help        Prints this message or the help of the given subcommand(s)
    install     Installs a Habitat package from a Depot or locally from a Habitat Artifact
    path        Prints the path to a specific installed release of a package
    promote     Promote a package to a specified channel
    provides    Search installed Habitat packages for a given file
    search      Search for a package on a Depot
    sign        Signs an archive with an origin key, generating a Habitat Artifact
    upload      Uploads a local Habitat Artifact to a Depot
    verify      Verifies a Habitat Artifact with an origin key

**Read More** 

- <a href="/docs/create-packages-build">Knowledge Article: Create and Build Packages</a>
- [hab pkg binlink](#hab-pkg-binlink) Creates a symlink for a package binary in a common 'PATH' location
- [hab pkg build](#hab-pkg-build) Builds a Plan using a Studio
- [hab pkg channels](#hab-pkg-channels) Find out what channels a package belongs to
- [hab pkg config](#hab-pkg-config) Displays the default configuration options for a service
- [hab pkg demote](#hab-pkg-demote) Demote a package from a specified channel
- [hab pkg env](#hab-pkg-env) Prints the runtime environment of a specific installed package
- [hab pkg exec](#hab-pkg-exec) Executes a command using the 'PATH' context of an installed package
- [hab pkg export](#hab-pkg-export) Exports the package to the specified format
- [hab pkg hash](#hab-pkg-hash) Generates a blake2b hashsum from a target at any given filepath
- [hab pkg install](#hab-pkg-install) Installs a Habitat package from a Depot or locally from a Habitat Artifact
- [hab pkg path](#hab-pkg-path) Prints the path to a specific installed release of a package
- [hab pkg promote](#hab-pkg-promote) Promote a package to a specified channel
- [hab pkg provides](#hab-pkg-provides) Search installed Habitat packages for a given file
- [hab pkg search](#hab-pkg-provides) Search for a package on a Depot
- [hab pkg sign](#hab-pkg-sign) Signs an archive with an origin key, generating a Habitat Artifact
- [hab pkg upload](#hab-pkg-upload) Uploads a local Habitat Artifact to a Depot
- [hab pkg verify](#hab-pkg-verify) Verifies a Habitat Artifact with an origin key

<h2 id="hab-pkg-binlink" class="anchor">hab pkg binlink</h2>
Creates a symlink for a package binary in a common 'PATH' location

**USAGE** 

    hab pkg binlink [OPTIONS] <PKG_IDENT> [BINARY]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -d, --dest <DEST_DIR>    Sets the destination directory (default: /bin)

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-
                   static/1.42.2)
    <BINARY>       The command to symlink (ex: bash)

<h2 id="hab-pkg-build" class="anchor">hab pkg build</h2>
Builds a Plan using a Studio

**USAGE** 

    hab pkg build [FLAGS] [OPTIONS] <PLAN_CONTEXT>

**FLAGS** 

    -D, --docker     Uses a Dockerized Studio for the build (default: Studio uses a chroot on linux)
    -R, --reuse      Reuses a previous Studio for the build (default: clean up before building)
    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -k, --keys <HAB_ORIGIN_KEYS> Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    -r, --root <HAB_STUDIO_ROOT> Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    -s, --src <SRC_PATH>         Sets the source path (default: $PWD)

**ARGS** 

    <PLAN_CONTEXT>    A directory containing a `plan.sh` file or a
                      `habitat/` directory which contains the `plan.sh` file

<h2 id="hab-pkg-channels" class="anchor">hab pkg channels</h2>
Find out what channels a package belongs to

**USAGE** 

    hab pkg channels [OPTIONS] <PKG_IDENT>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -u, --url <DEPOT_URL>    Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS** 

    <PKG_IDENT>    A fully qualified package identifier (ex: core/redis/3.2.1/20160729052715)

<h2 id="hab-pkg-config" class="anchor">hab pkg config</h2>
Displays the default configuration options for a service

**USAGE** 

    hab pkg config <PKG_IDENT>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-pkg-demote" class="anchor">hab pkg demote</h2>
Demote a package from a specified channel

**USAGE** 

    hab pkg demote [OPTIONS] <PKG_IDENT> <CHANNEL>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -z, --auth <AUTH_TOKEN>    Authentication token for the Depot
    -u, --url <DEPOT_URL>      Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    <CHANNEL>      Demote from the specified release channel

<h2 id="hab-pkg-env" class="anchor">hab pkg env</h2>
Prints the runtime environment of a specific installed package

**USAGE** 

    hab pkg env <PKG_IDENT>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-pkg-exec" class="anchor">hab pkg exec</h2>
Executes a command using the 'PATH' context of an installed package

**USAGE** 

    hab pkg exec <PKG_IDENT> <CMD> [ARGS]...

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    <CMD>          The command to execute (ex: ls)
    <ARGS>...      Arguments to the command (ex: -l /tmp)

<h2 id="hab-pkg-export" class="anchor">hab pkg export</h2>
Exports the package to the specified format

**USAGE** 

    hab pkg export [OPTIONS] <FORMAT> <PKG_IDENT>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -c, --channel <CHANNEL>    Retrieve the container's package from the specified release channel (default: stable)
    -u, --url <DEPOT_URL>      Retrieve the container's package from the specified Depot (default: https://bldr.habitat.sh/v1/depot)

**ARGS** 

    <FORMAT>       The export format (docker, aci, mesos, or tar)
    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-pkg-hash" class="anchor">hab pkg hash</h2>
Generates a blake2b hashsum from a target at any given filepath

**USAGE** 

    hab pkg hash [SOURCE]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <SOURCE>    A filepath of the target

<h2 id="hab-pkg-install" class="anchor">hab pkg install</h2>
Installs a Habitat package from a Depot or locally from a Habitat Artifact

**USAGE** 

    hab pkg install [FLAGS] [OPTIONS] <PKG_IDENT_OR_ARTIFACT>...

**FLAGS** 

    -b, --binlink    Binlink all binaries from installed package(s)
    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -c, --channel <CHANNEL>    Install from the specified release channel (default: stable)
    -u, --url <DEPOT_URL>      Use a specific Depot URL (default: https://bldr.habitat.sh/v1/depot)

**ARGS** 

    <PKG_IDENT_OR_ARTIFACT>...   
        One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-pkg-path" class="anchor">hab pkg path</h2>
Prints the path to a specific installed release of a package

**USAGE** 

    hab pkg path <PKG_IDENT>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-pkg-promote" class="anchor">hab pkg promote</h2>
Promote a package to a specified channel

**USAGE** 

    hab pkg promote [OPTIONS] <PKG_IDENT> <CHANNEL>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -z, --auth <AUTH_TOKEN>    Authentication token for the Depot
    -u, --url <DEPOT_URL>      Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    <CHANNEL>      Promote to the specified release channel

<h2 id="hab-pkg-provides" class="anchor">hab pkg provides</h2>
Search installed Habitat packages for a given file

**USAGE** 

    hab pkg provides [FLAGS] <FILE>

**FLAGS** 

    -p               Show full path to file
    -r               Show fully qualified package names (ex: core/busybox-
                     static/1.24.2/20160708162350)
    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <FILE>    File name to find

<h2 id="hab-pkg-search" class="anchor">hab pkg search</h2>
Search for a package on a Depot

**USAGE** 

    hab pkg search [OPTIONS] <SEARCH_TERM>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -u, --url <DEPOT_URL>    Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS** 
    <SEARCH_TERM>    Search term

<h2 id="hab-pkg-sign" class="anchor">hab pkg sign</h2>
Signs an archive with an origin key, generating a Habitat Artifact

**USAGE** 

    hab pkg sign [OPTIONS] <SOURCE> <DEST>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

        --origin <ORIGIN>    Origin key used to create signature

**ARGS** 

    <SOURCE>    A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    <DEST>      The destination path to the signed Habitat Artifact (ex:/home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-pkg-upload" class="anchor">hab pkg upload</h2>
Uploads a local Habitat Artifact to a Depot

**USAGE** 

    hab pkg upload [OPTIONS] <HART_FILE>...

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS** 

    -z, --auth <AUTH_TOKEN>    Authentication token for the Depot
    -c, --channel <CHANNEL>    Additional release channel to upload package to. Packages are always uploaded to `unstable`, regardless of the   value of this option. (default: none)
    -u, --url <DEPOT_URL>      Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS** 

    <HART_FILE>...    One or more filepaths to a Habitat Artifact (ex: /home/acme-redis-
                      3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-pkg-verify" class="anchor">hab pkg verify</h2>
Verifies a Habitat Artifact with an origin key

**USAGE** 

    hab pkg verify <SOURCE>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <SOURCE>    A path to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-plan-init" class="anchor">hab plan init</h2>
Generates common package specific configuration files. Executing without argument will create
a `habitat` directory in your current folder for the plan. If `PKG_NAME` is specified it will
create a folder with that name. Environment variables (those starting with 'pkg_') that are
set will be used in the generated plan.

**USAGE** 

    hab plan init [FLAGS] [OPTIONS] [PKG_NAME]

**FLAGS** 

        --with-all          Generate omnibus plan with all available plan options
        --with-callbacks    Include callback functions in template
        --with-docs         Include plan options documentation
    -h, --help              Prints help information
    -V, --version           Prints version information

**OPTIONS** 

    -o, --origin <ORIGIN>              Origin for the new app
    -s, --scaffolding <SCAFFOLDING>    Specify explicit scaffolding type for your app (ruby, node, go)

**ARGS** 

    <PKG_NAME>    Name for the new app

**Scaffolding Options** 

You can specify which scaffolding you want to use via the `hab plan init -s` command. 
Current fully supported scaffoldings are accessed via the keyword `ruby`, `node`, or `go`.

<h2 id="hab-ring-key" class="anchor">hab ring key</h2>
Commands relating to Habitat ring keys

**USAGE** 

    hab ring key [SUBCOMMAND]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**SUBCOMMANDS** 

    export      Outputs the latest ring key contents to stdout
    generate    Generates a Habitat ring key
    help        Prints this message or the help of the given subcommand(s)
    import      Reads a stdin stream containing ring key contents and writes the key to disk

**Read More:**

- [hab ring key export](#hab-ring-key-export)
- [hab ring key generate](#hab-ring-key-generate)
- [hab ring key import](#hab-ring-key-import)
- <a href="/docs/run-packages-security">Knowledge Article: Supervisor Security</a>
- <a href="/docs/concepts-keys">Knowledge Article: Keys</a>

<h2 id="hab-ring-key-export" class="anchor">hab ring key export</h2>
Outputs the latest ring key contents to stdout

**USAGE** 

    hab ring key export <RING>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <RING>    Ring key name

<h2 id="hab-ring-key-generate" class="anchor">hab ring key generate</h2>
Generates a Habitat ring key

**USAGE** 

    hab ring key generate <RING>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <RING>    Ring key name

<h2 id="hab ring key import" class="anchor">hab ring key import</h2>
Reads a stdin stream containing ring key contents and writes the key to disk

**USAGE**

    hab ring key import

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

<h2 id="hab-studio" class="anchor">hab studio</h2>

**USAGE** 

    hab studio [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

**COMMON FLAGS** 

    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -N  Do not mount the source artifact cache path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information
    -D  Use a docker studio instead of a chroot studio (only available on Linux)
    -w  Use a Windows studio instead of a docker studio (only available on Windows)

**COMMON OPTIONS** 

    -a <ARTIFACT_PATH>    Sets the source artifact cache path (default: /hab/cache/artifacts)
    -k <HAB_ORIGIN_KEYS>  Installs secret origin keys (default:$HAB_ORIGIN )
    -r <HAB_STUDIO_ROOT>  Sets a Studio root (default: /hab/studios/<DIR_NAME>)
    -s <SRC_PATH>         Sets the source path (default: $PWD)
    -t <STUDIO_TYPE>      Sets a Studio type when creating (default: default) Valid types: [default baseimage busybox stage1]

**SUBCOMMANDS** 

    build     Build using a Studio
    enter     Interactively enter a Studio
    help      Prints this message
    new       Creates a new Studio
    rm        Destroys a Studio
    run       Run a command in a Studio
    version   Prints version information

**SUBCOMMAND HELP** 

    hab studio <SUBCOMMAND> -h

**ENVIRONMENT VARIABLES** 

    ARTIFACT_PATH       Sets the source artifact cache path (`-a' option overrides)
    HAB_NOCOLORING      Disables text coloring mode despite TERM capabilities
    HAB_NONINTERACTIVE  Disables interactive progress bars despite tty
    HAB_ORIGIN          Propagates this variable into any studios
    HAB_ORIGIN_KEYS     Installs secret keys (`-k' option overrides)
    HAB_STUDIOS_HOME    Sets a home path for all Studios (default: /hab/studios)
    HAB_STUDIO_ROOT     Sets a Studio root (`-r' option overrides)
    NO_ARTIFACT_PATH    If set, do not mount the source artifact cache path (`-N' flag overrides)
    NO_SRC_PATH         If set, do not mount the source path (`-n' flag overrides)
    QUIET               Prints less output (`-q' flag overrides)
    SRC_PATH            Sets the source path (`-s' option overrides)
    STUDIO_TYPE         Sets a Studio type when creating (`-t' option overrides)
    VERBOSE             Prints more verbose output (`-v' flag overrides)
    http_proxy          Sets an http_proxy environment variable inside the Studio
    https_proxy         Sets an https_proxy environment variable inside the Studio
    no_proxy            Sets a no_proxy environment variable inside the Studio

**EXAMPLES** 

    # Create a new default Studio
    hab studio new

    # Enter the default Studio
    hab studio enter

    # Run a command in the default Studio
    hab studio run wget --version

    # Destroy the default Studio
    hab studio rm

    # Create and enter a busybox type Studio with a custom root
    hab studio -r /opt/slim -t busybox enter

    # Run a command in the slim Studio, showing only the command output
    hab studio -q -r /opt/slim run busybox ls -l /

    # Verbosely destroy the slim Studio
    hab studio -v -r /opt/slim rm

**Read More**

- <a href="/docs/concepts-studio">Knowledge Article: Studio</a>
- <a href="/docs/reference/environment-vars">Knowledge Article: Environment Variables</a>
- <a href="/blog/2017/07/Hab-Studio-Artifact-Caching/">Blog: Habitat Studio Artifact Caching</a>
- [hab studio build](#hab-studio-build)
- [hab studio enter](#hab-studio-enter)
- [hab studio new](#hab-studio-new)
- [hab studio rm](#hab-studio-rm)
- [hab studio run](#hab-studio-run)

<h2 id="hab-studio-build" class="anchor">hab studio build</h2>
Execute a build using a Studio.

**USAGE** 

    hab studio [COMMON_FLAGS] [COMMON_OPTIONS] build [FLAGS] [PLAN_DIR]

**FLAGS** 

    -R  Reuse a previous Studio state (default: clean up before building)

**EXAMPLES** 

    # Build a Redis plan
    hab-studio build plans/redis

    # Reuse previous Studio for a build
    hab-studio build -R plans/glibc

<h2 id="hab-studio-enter" class="anchor">hab studio enter</h2>
Interactively enter a Studio.

**USAGE** 

    hab studio [COMMON_FLAGS] [COMMON_OPTIONS] enter

<h2 id="hab-studio-new" class="anchor">hab studio new</h2>
Create a new Studio.

**USAGE** 

     hab studio [COMMON_FLAGS] [COMMON_OPTIONS] new

<h2 id="hab-studio-rm" class="anchor">hab studio rm</h2>
Destroy a Studio.

**USAGE** 

    hab studio [COMMON_FLAGS] [COMMON_OPTIONS] rm

<h2 id="hab-studio-run" class="anchor">hab studio run</h2>
Run a command in a Studio 

**USAGE** 

    hab studio [COMMON_FLAGS] [COMMON_OPTIONS] run [CMD] [ARG ..]

**ARGUMENTS** 

    CMD     Command to run in the Studio
    ARG     Arguments to the command

**EXAMPLE**

    hab-studio run wget --version

<h2 id="hab-sup" class="anchor">hab sup</h2>
The Habitat Supervisor

**USAGE** 

hab sup [FLAGS] <SUBCOMMAND>

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information
    -V, --version     Prints version information

**SUBCOMMANDS** 

    bash      Start an interactive Bash-like shell
    config    Displays the default configuration options for a service
    help      Prints this message or the help of the given subcommand(s)
    load      Load a service to be started and supervised by Habitat from a package or artifact. Services started in this manner will persist through Supervisor restarts.
    run       Run the Habitat Supervisor
    sh        Start an interactive Bourne-like shell
    start     Start a loaded, but stopped, Habitat service or a transient service from a package or artifact. If the Habitat Supervisor is not already running this will additionally start one for you.
    status    Query the status of Habitat services.
    stop      Stop a running Habitat service.
    term      Gracefully terminate the Habitat Supervisor and all of it's running services
    unload    Unload a persistent or transient service started by the Habitat supervisor. If the Supervisor is running when the service is unloaded the service will be stopped.

**Read More:** 

- <a href="/docs/concepts-supervisor">Knowledge Article: Supervisor</a>
- <a href="/docs/reference/log-keys">Knowledge Article: Habitat Supervisor Log Key Reference</a>
- <a href="/docs/run-packages-multiple-services">Knowledge Article: Run Multiple Services with one Supervisor</a>
- <a href="/docs/internals-supervisors">Developer Documentation: Supervisor Internals</a>
- <a href="/blog/2017/04/Multi-Service-Supervision">Blog: Multi-Service Supervision</a>
- [hab sup bash](#hab-sup-bash)
- [hab sup config](#hab-sup-config)
- [hab sup load](#hab-sup-load)
- [hab sup run](#hab-sup-run)
- [hab sup sh](#hab-sup-sh)
- [hab sup start](#hab-sup-start)
- [hab sup status](#hab-sup-status)
- [hab sup stop](#hab-sup-stop)
- [hab sup term](#hab-sup-term)
- [hab sup unload](#hab-sup-unload)

<h2 id="hab-sup-bash" class="anchor">hab sup bash</h2>
Start an interactive Bash-like shell

**USAGE** 

    hab sup bash [FLAGS]

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

<h2 id="hab-sup-config" class="anchor">hab sup config</h2>
Displays the default configuration options for a service

**USAGE** 

    hab sup config [FLAGS] <PKG_IDENT>

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

**ARGS** 

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-sup-load" class="anchor">hab sup load</h2>
Load a service to be started and supervised by Habitat from a package or artifact. Services started in this manner will persist through Supervisor restarts.

**USAGE** 

    hab sup load [FLAGS] [OPTIONS] <PKG_IDENT>

**FLAGS** 

    -f, --force       Load or reload an already loaded service. If the service was previously
                      loaded and running this operation will also restart the service
        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

**OPTIONS** 

    -a, --application <APPLICATION>    Application name; [default: not set].
        --bind <BIND>...               One or more service groups to bind to a configuration
        --channel <CHANNEL>
            Receive package updates from the specified release channel [default: stable]

    -u, --url <DEPOT_URL>
            Receive package updates from the Depot at the specified URL [default:
            https://bldr.habitat.sh/v1/depot]
    -e, --environment <ENVIRONMENT>    Environment name; [default: not set].
        --group <GROUP>
            The service group; shared config and topology [default: default].

        --override-name <NAME>
            The name for the state directory if there is more than one Supervisor running
            [default: default]
    -s, --strategy <STRATEGY>
            The update strategy; [default: none] [values: none, at-once, rolling]

    -t, --topology <TOPOLOGY>          Service topology; [default: none]

**ARGS** 

    <PKG_IDENT>    A Habitat package identifier (ex: core/redis)

<h2 id="hab-sup-run" class="anchor">hab sup run</h2>
Run the Habitat Supervisor

**USAGE** 

    hab sup run [FLAGS] [OPTIONS]

**FLAGS** 

    -A, --auto-update       Enable automatic updates for the Supervisor itself
        --no-color          Turn ANSI color off
    -I, --permanent-peer    If this Supervisor is a permanent peer
    -v                      Verbose output; shows line numbers
    -h, --help              Prints help information

**OPTIONS** 

        --channel <CHANNEL>
            Receive Supervisor updates from the specified release channel [default: stable]

    -u, --url <DEPOT_URL>
            Receive Supervisor updates from the Depot at the specified URL [default:
            https://bldr.habitat.sh/v1/depot]
    -n, --events <EVENTS>
            Name of the service group running a Habitat EventSrv to forward supervisor and
            service event data to
        --listen-gossip <LISTEN_GOSSIP>
            The listen address for the gossip system [default: 0.0.0.0:9638]

        --listen-http <LISTEN_HTTP>
            The listen address for the HTTP gateway [default: 0.0.0.0:9631]

        --override-name <NAME>
            The name of the Supervisor if launching more than one [default: default]

        --org <ORGANIZATION>
            The organization that the supervisor and it's subsequent services are part of
            [default: default]
        --peer <PEER>...                   The listen address of an initial peer (IP[:PORT])
    -r, --ring <RING>                      Ring key name

<h2 id="hab-sup-sh" class="anchor">hab sup sh</h2>
Start an interactive Bourne-like shell

**USAGE** 

    hab sup sh [FLAGS]

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

<h2 id="hab-sup-start" class="anchor">hab sup start</h2>
Start a loaded, but stopped, Habitat service or a transient service from a package or artifact. If the Habitat Supervisor is not already running this will additionally start one for you.

**USAGE** 

    hab sup start [FLAGS] [OPTIONS] <PKG_IDENT_OR_ARTIFACT>

**FLAGS** 

    -A, --auto-update       Enable automatic updates for the Supervisor itself
        --no-color          Turn ANSI color off
    -I, --permanent-peer    If this Supervisor is a permanent peer
    -v                      Verbose output; shows line numbers
    -h, --help              Prints help information

**OPTIONS** 

    -a, --application <APPLICATION>        Application name; [default: not set].
        --bind <BIND>...
            One or more service groups to bind to a configuration

        --channel <CHANNEL>
            Receive package updates from the specified release channel [default: stable]

        --config-from <CONFIG_DIR>
            Use package config from this path, rather than the package itself

    -u, --url <DEPOT_URL>
            Receive package updates from the Depot at the specified URL [default:
            https://bldr.habitat.sh/v1/depot]
    -e, --environment <ENVIRONMENT>        Environment name; [default: not set].
    -n, --events <EVENTS>
            Name of the service group running a Habitat EventSrv to forward supervisor and
            service event data to
        --group <GROUP>
            The service group; shared config and topology [default: default]

        --listen-gossip <LISTEN_GOSSIP>
            The listen address for the gossip system [default: 0.0.0.0:9638]

        --listen-http <LISTEN_HTTP>
            The listen address for the HTTP gateway [default: 0.0.0.0:9631]

        --override-name <NAME>
            The name for the state directory if launching more than one Supervisor [default:
            default]
        --org <ORGANIZATION>
            The organization that the supervisor and it's subsequent services are part of
            [default: default]
        --peer <PEER>...                   The listen address of an initial peer (IP[:PORT])
    -r, --ring <RING>                      Ring key name
    -s, --strategy <STRATEGY>
            The update strategy; [default: none] [values: none, at-once, rolling]

    -t, --topology <TOPOLOGY>              Service topology; [default: none]

**ARGS** 

    <PKG_IDENT_OR_ARTIFACT>    A Habitat package identifier (ex: core/redis) or filepath
                               to a Habitat Artifact (ex: /home/core-redis-3.0.7-
                               21120102031201-x86_64-linux.hart)

<h2 id="hab-sup-status" class="anchor">hab sup status</h2>
Query the status of Habitat services.

**USAGE** 

    hab sup status [FLAGS] [OPTIONS] [PKG_IDENT]

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

**OPTIONS** 

        --override-name <NAME>    The name for the state directory if there is more than one
                                  Supervisor running [default: default]

**ARGS** 

    <PKG_IDENT>    A Habitat package identifier (ex: core/redis)

<h2 id="hab-sup-stop" class="anchor">hab sup stop</h2>
Stop a running Habitat service.

**USAGE** 

    hab sup stop [FLAGS] [OPTIONS] <PKG_IDENT>

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

**OPTIONS** 

        --override-name <NAME>    The name for the state directory if there is more than one Supervisor running [default: default]

**ARGS** 

    <PKG_IDENT>    A Habitat package identifier (ex: core/redis)

<h2 id="hab-sup-term" class="anchor">hab sup term</h2>
Gracefully terminate the Habitat Supervisor and all of it's running services

**USAGE** 

    hab sup term [FLAGS] [OPTIONS]

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

**OPTIONS** 

        --override-name <NAME>    The name of the Supervisor if more than one is running [default: default]

<h2 id="hab-sup-unload" class="anchor">hab sup unload</h2>
Unload a persistent or transient service started by the Habitat supervisor. If the Supervisor is running when the service is unloaded the service will be stopped.

**USAGE** 

    hab sup unload [FLAGS] [OPTIONS] <PKG_IDENT>

**FLAGS** 

        --no-color    Turn ANSI color off
    -v                Verbose output; shows line numbers
    -h, --help        Prints help information

**OPTIONS** 

        --override-name <NAME>    The name for the state directory if there is more than one
                                  Supervisor running [default: default]

**ARGS** 

    <PKG_IDENT>    A Habitat package identifier (ex: core/redis)                                  

<h2 id="hab-svc-key-generate" class="anchor">hab svc key generate</h2>
Generates a Habitat service key

**USAGE** 

    hab svc key generate <SERVICE_GROUP> [ORG]

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <SERVICE_GROUP>    Target service group (ex: redis.default)
    <ORG>              The service organization

**Read More:** 

- <a href="/docs/run-packages-security">Knowledge Article: Supervisor Security</a>
- <a href="docs/concepts-keys">Knowledge Article: Keys</a>

<h2 id="hab-user-key-generate" class="anchor">hab user key generate</h2>
Generates a Habitat user key

**USAGE** 

    hab user key generate <USER>

**FLAGS** 

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS** 

    <USER>    Name of the user key
