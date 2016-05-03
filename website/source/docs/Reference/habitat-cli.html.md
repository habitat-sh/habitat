---
title: Habitat CLI reference
---

# Habitat command-line interface (CLI) reference
The commands and sub-commands for the Habitat CLI tools are listed below:

- [hab](#hab)
- [hab-artifact](#hab-artifact)
- [hab-bpm](#hab-bpm)
- [hab-depot](#hab-depot)
- [hab-origin](#hab-origin)
- [hab-pkg](#hab-pkg)
- [hab-rumor](#hab-rumor)
- [hab-studio](#hab-studio)
- [hab-sup](#hab-sup)

## hab
The main program that allows you to sign and upload packages, start Habitat services, and other related functions through various subcommands.

**USAGE**

    hab [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**SUBCOMMANDS**

    artifact    Commands relating to Habitat artifacts
    config      Commands relating to Habitat runtime config
    help        Prints this message or the help message of the given subcommand(s)
    origin      Commands relating to Habitat origin keys
    pkg         Commands relating to Habitat packages
    ring        Commands relating to Habitat rings
    service     Commands relating to Habitat services
    sup         Commands relating to the Habitat Supervisor
    user        Commands relating to Habitat users

**ALIASES**

    apply       Alias for: 'config apply'
    install     Alias for: 'pkg install'
    start       Alias for: 'sup start'

***

## hab-artifact
Subcommand used for signing and uploading packages.

**USAGE**

    hab artifact [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    hash      Generate a BLAKE2b hash for a file
    help      Prints this message or the help message of the given subcommand(s)
    sign      Signs a archive file with with an origin key, creating a Habitat artifact
    upload    Uploads a local package artifact to a depot
    verify    Verifies a Habitat artifact with an origin key

***

## hab-bpm
Package manager for Habitat. Mostly used to download and install packages.

**USAGE**

    $program [COMMON_FLAGS] <SUBCOMMAND> [ARG ..]

**COMMON FLAGS**

    -h  Prints this message
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information

**SUBCOMMANDS**

    binlink   Creates a symlink for a package binary in a common 'PATH' location
    exec      Executes a command using the 'PATH' context of an installed package
    help      Prints this message
    install   Installs a package
    pkgpath   Prints the path to a package
    version   Prints version information

**ENVIRONMENT VARIABLES**

    QUIET     Prints less output (\`-q' flag takes precedence)
    VERBOSE   Prints more verbose output (\`-v' flag takes precedence)

**SUBCOMMAND HELP**

    $program <SUBCOMMAND> -h

***

## hab-config
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

## hab-depot
Creates a new local depot for packages to be uploaded to and downloaded from. Most users should use the public depot for their package management.

**USAGE**

    hab-depot [FLAGS] [OPTIONS] [SUBCOMMAND]

**FLAGS**

    -h, --help       Prints help information
    -V, --version    Prints version information

**OPTIONS**

    -p, --path <path>    Filepath to service storage for the Depot service

**SUBCOMMANDS**

    help      With no arguments it prints this message, otherwise it prints help information about other subcommands
    repair    Verify and repair data integrity of the package Depot
    start     Run a Habitat package Depot
    view      Creates or lists views in the package Depot

***

## hab-origin
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

## hab-pkg
Subcommand that allows you to install local or remote packages.

**USAGE**

    hab pkg [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help       Prints this message or the help message of the given subcommand(s)
    install    Installs a Habitat package from a Depot or locally from a Habitat artifact

***

## hab-ring
Subcommand for managing the keys that supervisors use when passing encrypted messages to each other in a ring.

**USAGE**

    hab ring [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help    Prints this message or the help message of the given subcommand(s)
    key     Commands relating to Habitat ring keys

***

## hab-service
Subcommand for managing Habitat service keys.

**USAGE**

    hab service [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**

    help    Prints this message or the help message of the given subcommand(s)
    key     Commands relating to Habitat service keys

***

## hab-studio
Helps you to build packages inside or outside of a studio environment.

**USAGE**

    $program [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]

**FLAGS**

    -h  Prints this message
    -n  Do not mount the source path into the Studio (default: mount the path)
    -q  Prints less output for better use in scripts
    -v  Prints more verbose output
    -V  Prints version information

**OPTIONS**

    -r <STUDIO_ROOT>  Sets a Studio root (default: /opt/studio)
    -s <SRC_PATH>     Sets the source path (default: \$PWD)
    -t <STUDIO_TYPE>  Sets a Studio type when creating (default: default)
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

    NO_SRC_PATH   If set, do not mount source path (\`-n' flag takes precedence)
    QUIET         Prints less output (\`-q' flag takes precedence)
    SRC_PATH      Sets the source path (\`-s' option takes precedence)
    STUDIO_ROOT   Sets a Studio root (\`-r' option takes precedence)
    STUDIO_TYPE   Sets a Studio type when creating (\`-t' option takes precedence)
    STUDIOS_HOME  Sets a home path for all Studios (default: /opt/studios)
    VERBOSE       Prints more verbose output (\`-v' flag takes precedence)

**EXAMPLES**

    # Create a new default Studio
    $program new

    # Enter the default Studio
    $program enter

    # Run a command in the default Studio
    $program run wget --version

    # Destroy the default Studio
    $program rm

    # Create and enter a busybox type Studio with a custom root
    $program -r /opt/slim -t busybox enter

    # Run a command in the slim Studio, showing only the command output
    $program -q -r /opt/slim run busybox ls -l /

    # Verbosely destroy the slim Studio
    $program -v -r /opt/slim rm

***

## hab-sup
Supervisor that starts and manages the package payload of a Habitat service.

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

## hab-user
Subcommand for managing Habitat user keys.

**USAGE**

    hab user [FLAGS] [SUBCOMMAND]

**FLAGS**

    -h, --help    Prints help information

**SUBCOMMANDS**
    help    Prints this message or the help message of the given subcommand(s)
    key     Commands relating to Habitat user keys
