---
title: Habitat CLI reference
---

# Habitat command-line interface (CLI) reference

The commands and sub-commands for the Habitat CLI (`hab`) are listed below.

- [hab](#hab)
- [hab cli setup](#hab-cli-setup)
- [hab config apply](#hab-config-apply)
- [hab file upload](#hab-file-upload)
- [hab origin key download](#hab-origin-key-download)
- [hab origin key export](#hab-origin-key-export)
- [hab origin key generate](#hab-origin-key-generate)
- [hab origin key import](#hab-origin-key-import)
- [hab origin key upload](#hab-origin-key-upload)
- [hab pkg binlink](#hab-pkg-binlink)
- [hab pkg build](#hab-pkg-build)
- [hab pkg exec](#hab-pkg-exec)
- [hab pkg export](#hab-pkg-export)
- [hab pkg hash](#hab-pkg-hash)
- [hab pkg install](#hab-pkg-install)
- [hab pkg path](#hab-pkg-path)
- [hab pkg provides](#hab-pkg-provides)
- [hab pkg sign](#hab-pkg-sign)
- [hab pkg upload](#hab-pkg-upload)
- [hab pkg verify](#hab-pkg-verify)
- [hab plan init](#hab-plan-init)
- [hab ring key export](#hab-ring-key-export)
- [hab ring key generate](#hab-ring-key-generate)
- [hab ring key import](#hab-ring-key-import)
- [hab service key generate](#hab-service-key-generate)
- [hab studio](#hab-studio)
- [hab sup](#hab-sup)
- [hab user key generate](#hab-user-key-generate)

<h2 id="hab" class="anchor">hab</h2>

The main program that allows you to sign and upload packages, start Habitat services, and other related functions through various subcommands.

**USAGE**

    hab [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**SUBCOMMANDS**

    cli        Commands relating to Habitat runtime config
    config     Commands relating to Habitat runtime config
    file       Commands relating to Habitat files
    help       Prints this message or the help of the given subcommand(s)
    origin     Commands relating to Habitat origin keys
    pkg        Commands relating to Habitat packages
    ring       Commands relating to Habitat rings
    service    Commands relating to Habitat services
    studio     Commands relating to Habitat Studios
    sup        Commands relating to the Habitat Supervisor
    user       Commands relating to Habitat users

**ALIASES**

    apply      Alias for: 'config apply'
    install    Alias for: 'pkg install'
    setup      Alias for: 'cli setup'
    start      Alias for: 'sup start'

***

<h2 id="hab-cli-setup" class="anchor">hab cli setup</h2>
Interatively setup the CLI with reasonable defaults.

**USAGE**

    hab cli setup

<h2 id="hab-config-apply" class="anchor">hab config apply</h2>
Applies configuration to a group of Habitat supervisors.

**USAGE**

     hab config apply [FLAGS] [OPTIONS] <SERVICE_GROUP> <VERSION_NUMBER> [ARGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

        --org <ORG>      Name of service organization
    -p, --peer <PEER>    A comma-delimited list of one or more Habitat Supervisor peers to
                         communicate with (default: 127.0.0.1:9638)
    -r, --ring <RING>    Ring key name, which will encrypt communication messages

**ARGS**

    <SERVICE_GROUP>     Target service group (ex: redis.default)
    <VERSION_NUMBER>    A version number (positive integer) for this configuration (ex: 42)
    <FILE>              Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)

<h2 id="hab-file-upload" class="anchor">hab file upload</h2>
Upload a file to a supervisor ring.

**USAGE**

    hab file upload [FLAGS] [OPTIONS] <SERVICE_GROUP> <FILE> <VERSION_NUMBER> [ARGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

        --org <ORG>      Name of service organization
    -p, --peer <PEER>    A comma-delimited list of one or more Habitat Supervisor peers to infect
                         (default: 127.0.0.1:9638)
    -r, --ring <RING>    Ring key name, which will encrypt communication messages

**ARGS**

    <SERVICE_GROUP>     Target service group (ex: redis.default)
    <FILE>              Path to local file on disk
    <VERSION_NUMBER>    A version number (positive integer) for this configuration (ex: 42)
    <USER>              Name of the user key

<h2 id="hab-origin-key-download" class="anchor">hab origin key download</h2>
Download origin key(s) to `HAB_CACHE_KEY_PATH`

**USAGE**

    hab origin key download [FLAGS] [OPTIONS] <ORIGIN> [ARGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -u, --url <DEPOT_URL>    Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS**

    <ORIGIN>      The origin name
    <REVISION>    The key revision

***

<h2 id="hab-origin-key-export" class="anchor">hab origin key export</h2>
Outputs the latest origin key contents to stdout

**USAGE**

    hab origin key export [FLAGS] <ORIGIN> --type <PAIR_TYPE>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -t, --type <PAIR_TYPE>    Export either the `public' or `secret' key (default: public)

**ARGS**

    <ORIGIN>

<h2 id="hab-origin-key-generate" class="anchor">hab origin key generate</h2>
Generates a Habitat origin key

**USAGE**

    hab origin key generate [FLAGS] [ARGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <ORIGIN>    The origin name

<h2 id="hab-origin-key-import" class="anchor">hab origin key import</h2>
Reads a stdin stream containing a public or secret origin key contents and writes the key to disk

**USAGE**

    hab origin key import [FLAGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

<h2 id="hab-origin-key-upload" class="anchor">hab origin key upload</h2>
Upload origin keys to the depot

**USAGE**

    hab origin key upload [FLAGS] [OPTIONS] <ORIGIN|--pubfile <PUBLIC_FILE>>

**FLAGS**

    -s, --secret     Upload secret key in addition to the public key
    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -z, --auth <AUTH_TOKEN>        Authentication token for the Depot
    -u, --url <DEPOT_URL>          Use a specific Depot URL (ex: http://depot.example.com/v1/depot)
        --pubfile <PUBLIC_FILE>    Path to a local public origin key file on disk
        --secfile <SECRET_FILE>    Path to a local secret origin key file on disk

**ARGS**

    <ORIGIN>    The origin name

<h2 id="hab-pkg-binlink" class="anchor">hab pkg binlink</h2>
Creates a symlink for a package binary in a common 'PATH' location

**USAGE**

    hab pkg binlink [FLAGS] [OPTIONS] <PKG_IDENT> <BINARY>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -d, --dest <DEST_DIR>    Sets the destination directory (default: /bin)

**ARGS**

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
    <BINARY>       The command to symlink (ex: bash)  If no binary is specified, all binaries from the specified package will be symlinked.

<h2 id="hab-pkg-build" class="anchor">hab pkg build</h2>
Builds a Plan using a Studio

**USAGE**

    hab pkg build [FLAGS] [OPTIONS] <PLAN_CONTEXT>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -k, --keys <HAB_ORIGIN_KEYS>    Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
    -r, --root <HAB_STUDIO_ROOT>    Sets the Studio root (default: /hab/studios/<DIR_NAME>)
    -s, --src <SRC_PATH>            Sets the source path (default: $PWD)

**ARGS**

    <PLAN_CONTEXT>    A directory containing a `plan.sh` file or a `habitat/` directory which contains
                      the `plan.sh` file

<h2 id="hab-pkg-exec" class="anchor">hab pkg exec</h2>
Executes a command using the 'PATH' context of an installed package

**USAGE**

    hab pkg exec [FLAGS] <PKG_IDENT> <CMD> [ARGS]

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

    hab pkg export [FLAGS] <FORMAT> <PKG_IDENT>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <FORMAT>       The export format (ex: docker, aci)
    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-pkg-hash" class="anchor">hab pkg hash</h2>
Generates a blake2b hashsum from a target at any given filepath

**USAGE**

    hab pkg hash [FLAGS] <SOURCE>

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

    -u, --url <DEPOT_URL>    Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS**

    <PKG_IDENT_OR_ARTIFACT>...    One or more Habitat package identifiers (ex: acme/redis) and/or
                                  filepaths to a Habitat Artifact (ex:
                                  /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-pkg-path" class="anchor">hab pkg path</h2>
Prints the path to a specific installed release of a package

**USAGE**

    hab pkg path [FLAGS] <PKG_IDENT>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)

<h2 id="hab-pkg-provides" class="anchor">hab pkg provides</h2>
Search installed Habitat packages for a given file.

**USAGE**

    hab pkg provides [FLAGS] <FILE>

**FLAGS**

    -p               Show full path to file
    -r               Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <FILE>    File name to find

<h2 id="hab-pkg-sign" class="anchor">hab pkg sign</h2>
Signs an archive with an origin key, generating a Habitat Artifact

**USAGE**

    hab pkg sign [FLAGS] [OPTIONS] <SOURCE> <DEST>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

        --origin <ORIGIN>    Origin key used to create signature

**ARGS**

    <SOURCE>    A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
    <DEST>      The destination path to the signed Habitat Artifact (ex:
                /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-pkg-upload" class="anchor">hab pkg upload</h2>
Uploads a local Habitat Artifact to a Depot

**USAGE**

    hab pkg upload [FLAGS] [OPTIONS] <HART_FILE>...

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -z, --auth <AUTH_TOKEN>    Authentication token for the Depot
    -u, --url <DEPOT_URL>      Use a specific Depot URL (ex: http://depot.example.com/v1/depot)

**ARGS**

    <HART_FILE>...    One or more filepaths to a Habitat Artifact (ex:
                      /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-pkg-verify" class="anchor">hab pkg verify</h2>
Verifies a Habitat Artifact with an origin key

**USAGE**

    hab pkg verify [FLAGS] <SOURCE>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <SOURCE>    A path to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)

<h2 id="hab-plan-init" class="anchor">hab plan init</h2>
Generates common package specific configuration files. Executing
without argument will create a `habitat` directory in your current
folder for the plan. If `PKG_NAME` is specified it will create a
folder with that name. Environment variables (those starting with
`'pkg_'`) that are set will be used in the generated plan

**USAGE**

    hab plan init [FLAGS] [OPTIONS] [PKG_NAME]

**FLAGS**

    -f, --nocallbacks    Do not include callback functions in
                         template
    -h, --help           Prints help information
    -V, --version        Prints version information

**OPTIONS**

    -o, --origin <ORIGIN>    Origin for the new app

**ARGS**

    <PKG_NAME>    Name for the new app.

<h2 id="hab-ring-key-export" class="anchor">hab ring key export</h2>
Outputs the latest ring key contents to stdout

**USAGE**

    hab ring key export [FLAGS] <RING>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <RING>           Ring key name

<h2 id="hab-ring-key-generate" class="anchor">hab ring key generate</h2>
Generates a Habitat ring key

**USAGE**

    hab ring key generate [FLAGS] <RING>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <RING>           Ring key name

<h2 id="hab-ring-key-import" class="anchor">hab ring key import</h2>
Reads a stdin stream containing ring key contents and writes the key to disk

**USAGE**

    hab ring key import [FLAGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

<h2 id="hab-service-key-generate" class="anchor">hab service key generate</h2>
Generates a Habitat service key

**USAGE**

    hab service key generate [FLAGS] <SERVICE_GROUP> [ARGS]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <SERVICE_GROUP>    Target service group (ex: redis.default)
    <ORG>              The service organization

<h2 id="hab-studio" class="anchor">hab studio</h2>
Helps you to build packages inside a studio environment.

**USAGE**

    hab studio [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

**FLAGS**

    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information

**OPTIONS**

    -k <HAB_ORIGIN_KEYS>  Installs secret origin keys (default:$HAB_ORIGIN )
    -r <HAB_STUDIO_ROOT>  Sets a Studio root (default: /hab/studios/<DIR_NAME>)
    -s <SRC_PATH>         Sets the source path (default: $PWD)
    -t <STUDIO_TYPE>      Sets a Studio type when creating (default: default)
                          Valid types: [default baseimage busybox stage1]

**SUBCOMMANDS**

    build     Build using a Studio
    enter     Interactively enter a Studio
    help      Prints this message
    new       Creates a new Studio
    rm        Destroys a Studio
    run       Run a command in a Studio
    version   Prints version information

**ENVIRONMENT VARIABLES**

    HAB_ORIGIN        Propagates this variable into any studios
    HAB_ORIGIN_KEYS   Installs secret keys (`-k' option overrides)
    HAB_STUDIOS_HOME  Sets a home path for all Studios (default: /hab/studios)
    HAB_STUDIO_ROOT   Sets a Studio root (`-r' option overrides)
    NO_SRC_PATH       If set, do not mount source path (`-n' flag overrides)
    QUIET             Prints less output (`-q' flag overrides)
    SRC_PATH          Sets the source path (`-s' option overrides)
    STUDIO_TYPE       Sets a Studio type when creating (`-t' option overrides)
    VERBOSE           Prints more verbose output (`-v' flag overrides)

***

<h2 id="hab-sup" class="anchor">hab sup</h2>
Supervisor that starts and manages the software in a Habitat service.

**USAGE**

    hab sup [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help        Prints help information
        --no-color    Turn ANSI color off :(
    -V, --version     Prints version information
    -v                Verbose output; shows line numbers

**SUBCOMMANDS**

    config                  Print the default.toml for a given package
    help                    Prints this message
    sh                      Start an interactive shell
    start                   Start a Habitat-supervised service from a package

***

<h2 id="hab-user-key-generate" class="anchor">hab user key generate</h2>
Generates a Habitat user key

**USAGE**

    hab user key generate [FLAGS] <USER>

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**ARGS**

    <USER>           Name of the user key
