---
title: Habitat CLI reference
---

# Habitat command-line interface (CLI) reference

The commands and sub-commands for the Habitat CLI (`hab`) are listed below.

- [hab](#hab)
- [hab cli](#hab-cli)
- [hab config](#hab-config)
- [hab file](#hab-file)
- [hab origin](#hab-origin)
- [hab pkg](#hab-pkg)
- [hab ring](#hab-ring)
- [hab service](#hab-service)
- [hab studio](#hab-studio)
- [hab sup](#hab-sup)
- [hab user](#hab-user)

<h2 id="hab" class="anchor">hab</h2>

The main program that allows you to sign and upload packages, start Habitat services, and other related functions through various subcommands.

**USAGE**

    hab [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**SUBCOMMANDS**

    config      Commands relating to Habitat runtime config
    file        Commands relating to Habitat files
    help        Prints this message or the help of the given subcommand(s)
    origin      Commands relating to Habitat origin keys
    pkg         Commands relating to Habitat packages
    ring        Commands relating to Habitat rings
    service     Commands relating to Habitat services
    studio      Commands relating to Habitat Studios
    sup         Commands relating to the Habitat Supervisor
    user        Commands relating to Habitat users

**ALIASES**

    apply       Alias for: 'config apply'
    install     Alias for: 'pkg install'
    start       Alias for: 'sup start'

***

<h2 id="hab-cli" class="anchor">hab cli</h2>
Subcommand for changing the behavior of the command-line interface.

**USAGE**

    hab cli [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    setup    Performs initial configuration for the command-line interface.

***

<h2 id="hab-config" class="anchor">hab config</h2>
Subcommand for applying configuration changes to a service group.

**USAGE**

    hab config [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    apply    Applies a configuration to a group of Habitat
             Supervisors
    help     Prints this message or the help message of the given
             subcommand(s)

***

<h2 id="hab-file" class="anchor">hab file</h2>
Subcommand for uploading files to a service group.

**USAGE**

    hab file [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help      Prints this message or the help of the given subcommand(s)
    upload    Upload a file to the supervisor ring.


<h2 id="hab-origin" class="anchor">hab origin</h2>
Subcommand that performs key maintenance for origin keys.

**USAGE**

    hab origin [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help    Prints this message or the help message of the
            given subcommand(s)
    key     Commands relating to Habitat origin key maintenance

***

<h2 id="hab-pkg" class="anchor">hab pkg</h2>
Subcommand that allows you to build or install local or remote packages.

**USAGE**

    hab pkg [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    binlink    Creates a symlink for a package binary in a common 'PATH'
               location
    build      Builds a Plan using a Studio
    exec       Executes a command using the 'PATH' context of an installed
               package
    export     Exports the package to the specified format
    hash       Generate a Habitat packaging hash for a file
    help       Prints this message or the help of the given subcommand(s)
    install    Installs a Habitat package from a Depot or locally from a
               Habitat artifact
    path       Prints the path to a specific installed release of a package
    sign       Signs a archive file with with an origin key, creating a Habitat artifact
    upload     Uploads a local Habitat artifact to a depot
    verify     Verifies a Habitat package with an origin key

***

<h2 id="hab-ring" class="anchor">hab ring</h2>
Subcommand for managing the keys that supervisors use when passing encrypted messages to each other in a ring.

**USAGE**

    hab ring [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help    Prints this message or the help message of the given subcommand(s)
    key     Commands relating to Habitat ring keys

***

<h2 id="hab-service" class="anchor">hab service</h2>
Subcommand for managing Habitat service group keys.

**USAGE**

    hab service [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help    Prints this message or the help message of the given subcommand(s)
    key     Commands relating to Habitat service keys

***

<h2 id="hab-studio" class="anchor">hab studio</h2>
Helps you to build packages inside a studio environment.

**USAGE**

USAGE:
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

    hab-sup [FLAGS] [SUBCOMMAND]

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

<h2 id="hab-user" class="anchor">hab user</h2>
Subcommand for managing Habitat user keys.

**USAGE**

    hab user [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**
    help    Prints this message or the help message of the given subcommand(s)
    key     Commands relating to Habitat user keys
