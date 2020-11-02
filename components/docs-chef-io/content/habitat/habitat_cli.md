+++
title = "hab CLI Reference"
draft= false

aliases = ["/habitat/habitat-cli/"]

[menu]
  [menu.habitat]
    title = "hab CLI Reference"
    identifier = "habitat/reference/habitat-cli CLI Reference"
    parent = "habitat/reference"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/habitat_cli.md)

<!-- This is a generated file, do not edit it directly. See https://github.com/habitat-sh/habitat/blob/master/www/scripts/generate-cli-docs.js -->

Chef Habitat Command-Line Interface (CLI) Reference

The commands for the Chef Habitat CLI (`hab`) are listed below.

| Applies to Version | Last Updated |
| ------- | ------------ |
| hab 1.6.175/20201026161911 (linux) | 28 Oct 2020 |

## hab

**USAGE**

```
hab <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**ALIASES**

```
apply      Alias for: 'config apply'
install    Alias for: 'pkg install'
run        Alias for: 'sup run'
setup      Alias for: 'cli setup'
start      Alias for: 'svc start'
stop       Alias for: 'svc stop'
term       Alias for: 'sup term'
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab bldr]({{< relref "#hab-bldr" >}}) | Commands relating to Habitat Builder |
| [hab cli]({{< relref "#hab-cli" >}}) | Commands relating to Habitat runtime config |
| [hab config]({{< relref "#hab-config" >}}) | Commands relating to a Service's runtime config |
| [hab file]({{< relref "#hab-file" >}}) | Commands relating to Habitat files |
| [hab license]({{< relref "#hab-license" >}}) | Commands relating to Habitat license agreements |
| [hab origin]({{< relref "#hab-origin" >}}) | Commands relating to Habitat Builder origins |
| [hab pkg]({{< relref "#hab-pkg" >}}) | Commands relating to Habitat packages |
| [hab plan]({{< relref "#hab-plan" >}}) | Commands relating to plans and other app-specific configuration |
| [hab ring]({{< relref "#hab-ring" >}}) | Commands relating to Habitat rings |
| [hab studio]({{< relref "#hab-studio" >}}) | Commands relating to Habitat Studios |
| [hab sup]({{< relref "#hab-sup" >}}) | The Habitat Supervisor |
| [hab supportbundle]({{< relref "#hab-supportbundle" >}}) | Create a tarball of Habitat Supervisor data to send to support |
| [hab svc]({{< relref "#hab-svc" >}}) | Commands relating to Habitat services |
| [hab user]({{< relref "#hab-user" >}}) | Commands relating to Habitat users |
---

## hab bldr

Commands relating to Habitat Builder

**USAGE**

```
hab bldr <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab bldr channel]({{< relref "#hab-bldr-channel" >}}) | Commands relating to Habitat Builder channels |
| [hab bldr job]({{< relref "#hab-bldr-job" >}}) | Commands relating to Habitat Builder jobs |
---

### hab bldr channel

Commands relating to Habitat Builder channels

**USAGE**

```
hab bldr channel <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab bldr channel create]({{< relref "#hab-bldr-channel-create" >}}) | Creates a new channel |
| [hab bldr channel demote]({{< relref "#hab-bldr-channel-demote" >}}) | Atomically demotes selected packages in a target channel |
| [hab bldr channel destroy]({{< relref "#hab-bldr-channel-destroy" >}}) | Destroys a channel |
| [hab bldr channel list]({{< relref "#hab-bldr-channel-list" >}}) | Lists origin channels |
| [hab bldr channel promote]({{< relref "#hab-bldr-channel-promote" >}}) | Atomically promotes all packages in channel |
---

### hab bldr channel create

Creates a new channel

**USAGE**

```
hab bldr channel create [OPTIONS] <CHANNEL>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-u, --url <BLDR_URL>     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>    Sets the origin to which the channel will belong. Default is from 'HAB_ORIGIN' or cli.toml
```

**ARGS**

```
<CHANNEL>    The channel name
```

### hab bldr channel demote

Atomically demotes selected packages in a target channel

**USAGE**

```
hab bldr channel demote [OPTIONS] <SOURCE_CHANNEL> <TARGET_CHANNEL> --origin <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      The origin for the channels. Default is from 'HAB_ORIGIN' or cli.toml
```

**ARGS**

```
<SOURCE_CHANNEL>    The channel from which all packages will be selected for demotion
<TARGET_CHANNEL>    The channel selected packages will be removed from
```

### hab bldr channel destroy

Destroys a channel

**USAGE**

```
hab bldr channel destroy [OPTIONS] <CHANNEL>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-u, --url <BLDR_URL>     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>    Sets the origin to which the channel belongs. Default is from 'HAB_ORIGIN' or cli.toml
```

**ARGS**

```
<CHANNEL>    The channel name
```

### hab bldr channel list

Lists origin channels

**USAGE**

```
hab bldr channel list [OPTIONS] [ORIGIN]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-u, --url <BLDR_URL>    Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>    The origin for which channels will be listed. Default is from 'HAB_ORIGIN' or cli.
```

### hab bldr channel promote

Atomically promotes all packages in channel

**USAGE**

```
hab bldr channel promote [OPTIONS] <SOURCE_CHANNEL> <TARGET_CHANNEL> --origin <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      The origin for the channels. Default is from 'HAB_ORIGIN' or cli.toml
```

**ARGS**

```
<SOURCE_CHANNEL>    The channel from which all packages will be selected for promotion
<TARGET_CHANNEL>    The channel to which packages will be promoted
```

### hab bldr job

Commands relating to Habitat Builder jobs

**USAGE**

```
hab bldr job <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab bldr job cancel]({{< relref "#hab-bldr-job-cancel" >}}) | Cancel a build job group and any in-progress builds |
| [hab bldr job demote]({{< relref "#hab-bldr-job-demote" >}}) | Demote packages from a completed build job from a specified channel |
| [hab bldr job promote]({{< relref "#hab-bldr-job-promote" >}}) | Promote packages from a completed build job to a specified channel |
| [hab bldr job start]({{< relref "#hab-bldr-job-start" >}}) | Schedule a build job or group of jobs |
| [hab bldr job status]({{< relref "#hab-bldr-job-status" >}}) | Get the status of one or more job groups |
---

### hab bldr job cancel

Cancel a build job group and any in-progress builds

**USAGE**

```
hab bldr job cancel [FLAGS] [OPTIONS] <GROUP_ID>
```

**FLAGS**

```
-f, --force      Don't prompt for confirmation
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<GROUP_ID>    The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
```

### hab bldr job demote

Demote packages from a completed build job from a specified channel

**USAGE**

```
hab bldr job demote [FLAGS] [OPTIONS] <GROUP_ID> <CHANNEL>
```

**FLAGS**

```
-i, --interactive    Allow editing the list of demotable packages
-h, --help           Prints help information
-V, --version        Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      Limit the demotable packages to the specified origin
```

**ARGS**

```
<GROUP_ID>    The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
<CHANNEL>     The name of the channel to demote from
```

### hab bldr job promote

Promote packages from a completed build job to a specified channel

**USAGE**

```
hab bldr job promote [FLAGS] [OPTIONS] <GROUP_ID> <CHANNEL>
```

**FLAGS**

```
-i, --interactive    Allow editing the list of promotable packages
-h, --help           Prints help information
-V, --version        Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      Limit the promotable packages to the specified origin
```

**ARGS**

```
<GROUP_ID>    The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
<CHANNEL>     The target channel name
```

### hab bldr job start

Schedule a build job or group of jobs

**USAGE**

```
hab bldr job start [FLAGS] [OPTIONS] <PKG_IDENT> [PKG_TARGET]
```

**FLAGS**

```
-g, --group      Schedule jobs for this package and all of its reverse dependencies
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<PKG_IDENT>     A package identifier (ex: core/redis, core/busybox-static/1.42.2)
<PKG_TARGET>    A package target (ex: x86_64-windows) (default: system appropriate target) [env: HAB_PACKAGE_TARGET=]
```

### hab bldr job status

Get the status of one or more job groups

**USAGE**

```
hab bldr job status [FLAGS] [OPTIONS] <GROUP_ID|--origin <ORIGIN>>
```

**FLAGS**

```
-s, --showjobs    Show the status of all build jobs for a retrieved job group
-h, --help        Prints help information
-V, --version     Prints version information
```

**OPTIONS**

```
-u, --url <BLDR_URL>     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-l, --limit <LIMIT>      Limit how many job groups to retrieve, ordered by most recent (default: 10)
-o, --origin <ORIGIN>    Show the status of recent job groups created in this origin (default: 10 most recent)
```

**ARGS**

```
<GROUP_ID>    The job group id that was returned from "hab bldr job start" (ex: 771100000000000000)
```

## hab cli

Commands relating to Habitat runtime config

**USAGE**

```
hab cli <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab cli completers]({{< relref "#hab-cli-completers" >}}) | Creates command-line completers for your shell |
| [hab cli setup]({{< relref "#hab-cli-setup" >}}) | Sets up the CLI with reasonable defaults |
---

### hab cli completers

Creates command-line completers for your shell

**USAGE**

```
hab cli completers --shell <SHELL>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-s, --shell <SHELL>    The name of the shell you want to generate the command-completion [possible values: Bash, Fish, Zsh, PowerShell]
```

### hab cli setup

Sets up the CLI with reasonable defaults

**USAGE**

```
hab cli setup [OPTIONS]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

## hab config

Commands relating to a Service's runtime config

**USAGE**

```
hab config <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab config apply]({{< relref "#hab-config-apply" >}}) | Sets a configuration to be shared by members of a Service Group |
| [hab config show]({{< relref "#hab-config-show" >}}) | Displays the default configuration options for a service |
---

### hab config apply

Sets a configuration to be shared by members of a Service Group

**USAGE**

```
hab config apply [OPTIONS] <SERVICE_GROUP> <VERSION_NUMBER> [FILE]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
-r, --remote-sup <REMOTE_SUP>            Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
-u, --user <USER>                        Name of a user key to use for encryption
```

**ARGS**

```
<SERVICE_GROUP>     Target service group service.group[@organization] (ex: redis.default or foo.default@bazcorp)
<VERSION_NUMBER>    A version number (positive integer) for this configuration (ex: 42)
<FILE>              Path to local file on disk (ex: /tmp/config.toml, default: <stdin>)
```

### hab config show

Displays the default configuration options for a service

**USAGE**

```
hab config show [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP>    Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

## hab file

Commands relating to Habitat files

**USAGE**

```
hab file <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab file upload]({{< relref "#hab-file-upload" >}}) | Uploads a file to be shared between members of a Service Group |
---

### hab file upload

Uploads a file to be shared between members of a Service Group

**USAGE**

```
hab file upload [OPTIONS] <SERVICE_GROUP> <VERSION_NUMBER> <FILE>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
-r, --remote-sup <REMOTE_SUP>            Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
-u, --user <USER>                        Name of the user key
```

**ARGS**

```
<SERVICE_GROUP>     Target service group service.group[@organization] (ex: redis.default or foo.default@bazcorp)
<VERSION_NUMBER>    A version number (positive integer) for this configuration (ex: 42)
<FILE>              Path to local file on disk
```

## hab license

Commands relating to Habitat license agreements

**USAGE**

```
hab license <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab license accept]({{< relref "#hab-license-accept" >}}) | Accept the Chef Binary Distribution Agreement without prompting |
---

### hab license accept

Accept the Chef Binary Distribution Agreement without prompting

**USAGE**

```
hab license accept
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

## hab origin

Commands relating to Habitat Builder origins

**USAGE**

```
hab origin <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab origin create]({{< relref "#hab-origin-create" >}}) | Creates a new Builder origin |
| [hab origin delete]({{< relref "#hab-origin-delete" >}}) | Removes an unused/empty origin |
| [hab origin depart]({{< relref "#hab-origin-depart" >}}) | Departs membership from selected origin |
| [hab origin info]({{< relref "#hab-origin-info" >}}) | Displays general information about an origin |
| [hab origin invitations]({{< relref "#hab-origin-invitations" >}}) | Manage origin member invitations |
| [hab origin key]({{< relref "#hab-origin-key" >}}) | Commands relating to Habitat origin key maintenance |
| [hab origin secret]({{< relref "#hab-origin-secret" >}}) | Commands related to secret management |
| [hab origin transfer]({{< relref "#hab-origin-transfer" >}}) | Transfers ownership of an origin to another member of that origin |
---

### hab origin create

Creates a new Builder origin

**USAGE**

```
hab origin create [OPTIONS] <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>    The origin to be created
```

### hab origin delete

Removes an unused/empty origin

**USAGE**

```
hab origin delete [OPTIONS] <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>    The origin name
```

### hab origin depart

Departs membership from selected origin

**USAGE**

```
hab origin depart [OPTIONS] <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>    The origin name
```

### hab origin info

Displays general information about an origin

**USAGE**

```
hab origin info [FLAGS] [OPTIONS] <ORIGIN>
```

**FLAGS**

```
-j, --json       Output will be rendered in json
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>    The origin name to be queried
```

### hab origin invitations

Manage origin member invitations

**USAGE**

```
hab origin invitations <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab origin invitations accept]({{< relref "#hab-origin-invitations-accept" >}}) | Accept an origin member invitation |
| [hab origin invitations ignore]({{< relref "#hab-origin-invitations-ignore" >}}) | Ignore an origin member invitation |
| [hab origin invitations list]({{< relref "#hab-origin-invitations-list" >}}) | List origin invitations sent to your account |
| [hab origin invitations pending]({{< relref "#hab-origin-invitations-pending" >}}) | List pending invitations for a particular origin. Requires that you are the origin owner |
| [hab origin invitations rescind]({{< relref "#hab-origin-invitations-rescind" >}}) | Rescind an existing origin member invitation |
| [hab origin invitations send]({{< relref "#hab-origin-invitations-send" >}}) | Send an origin member invitation |
---

### hab origin invitations accept

Accept an origin member invitation

**USAGE**

```
hab origin invitations accept [OPTIONS] <ORIGIN> <INVITATION_ID>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>           The origin name the invitation applies to
<INVITATION_ID>    The id of the invitation to accept
```

### hab origin invitations ignore

Ignore an origin member invitation

**USAGE**

```
hab origin invitations ignore [OPTIONS] <ORIGIN> <INVITATION_ID>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>           The origin name the invitation applies to
<INVITATION_ID>    The id of the invitation to ignore
```

### hab origin invitations list

List origin invitations sent to your account

**USAGE**

```
hab origin invitations list [OPTIONS]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

### hab origin invitations pending

List pending invitations for a particular origin. Requires that you are the origin owner

**USAGE**

```
hab origin invitations pending [OPTIONS] <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>    The name of the origin you wish to list invitations for
```

### hab origin invitations rescind

Rescind an existing origin member invitation

**USAGE**

```
hab origin invitations rescind [OPTIONS] <ORIGIN> <INVITATION_ID>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>           The origin name the invitation applies to
<INVITATION_ID>    The id of the invitation to rescind
```

### hab origin invitations send

Send an origin member invitation

**USAGE**

```
hab origin invitations send [OPTIONS] <ORIGIN> <INVITEE_ACCOUNT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>             The origin name the invitation applies to
<INVITEE_ACCOUNT>    The account name to invite into the origin
```

### hab origin key

Commands relating to Habitat origin key maintenance

**USAGE**

```
hab origin key <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab origin key download]({{< relref "#hab-origin-key-download" >}}) | Download origin key(s) |
| [hab origin key export]({{< relref "#hab-origin-key-export" >}}) | Outputs the latest origin key contents to stdout |
| [hab origin key generate]({{< relref "#hab-origin-key-generate" >}}) | Generates a Habitat origin key pair |
| [hab origin key import]({{< relref "#hab-origin-key-import" >}}) | Reads a stdin stream containing a public or private origin key contents and writes the key to disk |
| [hab origin key upload]({{< relref "#hab-origin-key-upload" >}}) | Upload origin keys to Builder |
---

### hab origin key download

Download origin key(s)

**USAGE**

```
hab origin key download [FLAGS] [OPTIONS] <ORIGIN> [REVISION]
```

**FLAGS**

```
-e, --encryption    Download public encryption key instead of origin public key
-s, --secret        Download origin private key instead of origin public key
-h, --help          Prints help information
-V, --version       Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>                  Authentication token for Builder (required for downloading origin private keys)
-u, --url <BLDR_URL>                     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    --cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<ORIGIN>      The origin name
<REVISION>    The origin key revision
```

### hab origin key export

Outputs the latest origin key contents to stdout

**USAGE**

```
hab origin key export [OPTIONS] <ORIGIN>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
-t, --type <KEY_TYPE>                    Export either the 'public' or 'secret' key. The 'secret' key is the origin private key
```

**ARGS**

```
<ORIGIN>    The origin name
```

### hab origin key generate

Generates a Habitat origin key pair

**USAGE**

```
hab origin key generate [OPTIONS] [ORIGIN]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<ORIGIN>    The origin name
```

### hab origin key import

Reads a stdin stream containing a public or private origin key contents and writes the key to disk

**USAGE**

```
hab origin key import [OPTIONS]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

### hab origin key upload

Upload origin keys to Builder

**USAGE**

```
hab origin key upload [FLAGS] [OPTIONS] <ORIGIN|--pubfile <PUBLIC_FILE>>
```

**FLAGS**

```
-s, --secret     Upload origin private key in addition to the public key
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>                  Authentication token for Builder
-u, --url <BLDR_URL>                     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    --cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
    --pubfile <PUBLIC_FILE>              Path to a local public origin key file on disk
    --secfile <SECRET_FILE>              Path to a local origin private key file on disk
```

**ARGS**

```
<ORIGIN>    The origin name
```

### hab origin rbac

Role Based Access Control for origin members

**USAGE**

```
hab origin rbac <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab origin secret delete]({{< relref "#hab-origin-secret-delete" >}}) | Delete a secret for your origin |
| [hab origin secret list]({{< relref "#hab-origin-secret-list" >}}) | List all secrets for your origin |
| [hab origin secret upload]({{< relref "#hab-origin-secret-upload" >}}) | Create and upload a secret for your origin |
---

### hab origin rbac set

Change an origin member's role

**USAGE**

```
hab origin rbac set [FLAGS] [OPTIONS] <MEMBER_ACCOUNT> <ROLE> --origin <ORIGIN>
```

**FLAGS**

```
-n, --no-prompt    Do not prompt for confirmation
-h, --help         Prints help information
-V, --version      Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      The Builder origin name to target
```

**ARGS**

```
<MEMBER_ACCOUNT>    The account name whose role will be changed
<ROLE>              The role name to enforce for the member account [possible values: readonly_member, member, maintainer, administrator, owner]
```

### hab origin rbac show

Display an origin member's current role

**USAGE**

```
hab origin rbac show [FLAGS] [OPTIONS] <MEMBER_ACCOUNT> --origin <ORIGIN>
```

**FLAGS**

```
-j, --json       Output will be rendered in json
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      The Builder origin name to target
```

**ARGS**

```
<MEMBER_ACCOUNT>    The account name of the role to display
```

### hab origin secret

Commands related to secret management

**USAGE**

```
hab origin secret <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab origin secret delete](#hab-origin-secret-delete) | Delete a secret for your origin |
| [hab origin secret list](#hab-origin-secret-list) | List all secrets for your origin |
| [hab origin secret upload](#hab-origin-secret-upload) | Create and upload a secret for your origin |
---

### hab origin secret delete

Delete a secret for your origin

**USAGE**

```
hab origin secret delete [OPTIONS] <KEY_NAME>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      The origin for which the secret will be deleted. Default is from 'HAB_ORIGIN' or cli.toml
```

**ARGS**

```
<KEY_NAME>    The name of the variable key to be injected into the studio
```

### hab origin secret list

List all secrets for your origin

**USAGE**

```
hab origin secret list [OPTIONS]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-o, --origin <ORIGIN>      The origin for which secrets will be listed. Default is from 'HAB_ORIGIN' or cli.toml
```

### hab origin secret upload

Create and upload a secret for your origin

**USAGE**

```
hab origin secret upload [OPTIONS] <KEY_NAME> <SECRET>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>                  Authentication token for Builder
-u, --url <BLDR_URL>                     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    --cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
-o, --origin <ORIGIN>                    The origin for which the secret will be uploaded. Default is from HAB_ORIGIN' or cli.toml
```

**ARGS**

```
<KEY_NAME>    The name of the variable key to be injected into the studio. Ex: KEY="some_value"
<SECRET>      The contents of the variable to be injected into the studio
```

### hab origin transfer

Transfers ownership of an origin to another member of that origin

**USAGE**

```
hab origin transfer [OPTIONS] <ORIGIN> <NEW_OWNER_ACCOUNT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<ORIGIN>               The origin name
<NEW_OWNER_ACCOUNT>    The account name of the new origin owner
```

## hab pkg

Commands relating to Habitat packages

**USAGE**

```
hab pkg <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab pkg binds]({{< relref "#hab-pkg-binds" >}}) | Displays the binds for a service |
| [hab pkg binlink]({{< relref "#hab-pkg-binlink" >}}) | Creates a binlink for a package binary in a common 'PATH' location |
| [hab pkg build]({{< relref "#hab-pkg-build" >}}) | Builds a Plan using a Studio |
| [hab pkg bulkupload]({{< relref "#hab-pkg-bulkupload" >}}) | Bulk Uploads Habitat Artifacts to a Depot from a local directory |
| [hab pkg channels]({{< relref "#hab-pkg-channels" >}}) | Find out what channels a package belongs to |
| [hab pkg config]({{< relref "#hab-pkg-config" >}}) | Displays the default configuration options for a service |
| [hab pkg delete]({{< relref "#hab-pkg-delete" >}}) | Removes a package from Builder |
| [hab pkg demote]({{< relref "#hab-pkg-demote" >}}) | Demote a package from a specified channel |
| [hab pkg dependencies]({{< relref "#hab-pkg-dependencies" >}}) | Returns the Habitat Artifact dependencies. By default it will return the direct dependencies of the package |
| [hab pkg download]({{< relref "#hab-pkg-download" >}}) | Download Habitat artifacts (including dependencies and keys) from Builder |
| [hab pkg env]({{< relref "#hab-pkg-env" >}}) | Prints the runtime environment of a specific installed package |
| [hab pkg exec]({{< relref "#hab-pkg-exec" >}}) | Executes a command using the 'PATH' context of an installed package |
| [hab pkg export]({{< relref "#hab-pkg-export" >}}) | Exports the package to the specified format |
| [hab pkg hash]({{< relref "#hab-pkg-hash" >}}) | Generates a blake2b hashsum from a target at any given filepath |
| [hab pkg info]({{< relref "#hab-pkg-info" >}}) | Returns the Habitat Artifact information |
| [hab pkg install]({{< relref "#hab-pkg-install" >}}) | Installs a Habitat package from Builder or locally from a Habitat Artifact |
| [hab pkg list]({{< relref "#hab-pkg-list" >}}) | List all versions of installed packages |
| [hab pkg path]({{< relref "#hab-pkg-path" >}}) | Prints the path to a specific installed release of a package |
| [hab pkg promote]({{< relref "#hab-pkg-promote" >}}) | Promote a package to a specified channel |
| [hab pkg provides]({{< relref "#hab-pkg-provides" >}}) | Search installed Habitat packages for a given file |
| [hab pkg search]({{< relref "#hab-pkg-search" >}}) | Search for a package in Builder |
| [hab pkg sign]({{< relref "#hab-pkg-sign" >}}) | Signs an archive with an origin key, generating a Habitat Artifact |
| [hab pkg uninstall]({{< relref "#hab-pkg-uninstall" >}}) | Safely uninstall a package and dependencies from the local filesystem |
| [hab pkg upload]({{< relref "#hab-pkg-upload" >}}) | Uploads a local Habitat Artifact to Builder |
| [hab pkg verify]({{< relref "#hab-pkg-verify" >}}) | Verifies a Habitat Artifact with an origin key |
---

### hab pkg binds

Displays the binds for a service

**USAGE**

```
hab pkg binds <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg binlink

Creates a binlink for a package binary in a common 'PATH' location

**USAGE**

```
hab pkg binlink [FLAGS] [OPTIONS] <PKG_IDENT> [BINARY]
```

**FLAGS**

```
-f, --force      Overwrite existing binlinks
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-d, --dest <DEST_DIR>    Sets the destination directory [env: HAB_BINLINK_DIR=]  [default: /bin]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
<BINARY>       The command to binlink (ex: bash)
```

### hab pkg build

Builds a Plan using a Studio

**USAGE**

```
hab pkg build [FLAGS] [OPTIONS] <PLAN_CONTEXT>
```

**FLAGS**

```
-D, --docker     Uses a Dockerized Studio for the build
-R, --reuse      Reuses a previous Studio for the build (default: clean up before building)
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
-k, --keys <HAB_ORIGIN_KEYS>             Installs secret origin keys (ex: "unicorn", "acme,other,acme-ops")
-r, --root <HAB_STUDIO_ROOT>             Sets the Studio root (default: /hab/studios/<DIR_NAME>)
-s, --src <SRC_PATH>                     Sets the source path (default: $PWD)
```

**ARGS**

```
<PLAN_CONTEXT>    A directory containing a plan file or a habitat/ directory which contains the plan file
```

### hab pkg bulkupload

Bulk Uploads Habitat Artifacts to a Depot from a local directory

**USAGE**

```
hab pkg bulkupload [FLAGS] [OPTIONS] <UPLOAD_DIRECTORY>
```

**FLAGS**

```
--auto-build             Enable auto-build for all packages in this upload. Only applicable to SaaS Builder
    --auto-create-origins    Skip the confirmation prompt and automatically create origins that do not exist in the target Builder
    --force                  Skip checking availability of package and force uploads, potentially overwriting a stored copy of a package
-h, --help                   Prints help information
-V, --version                Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-c, --channel <CHANNEL>    Optional additional release channel to upload package to. Packages are always uploaded to unstable, regardless of the value of this option
```

**ARGS**

```
<UPLOAD_DIRECTORY>    Directory Path from which artifacts will be uploaded
```

### hab pkg channels

Find out what channels a package belongs to

**USAGE**

```
hab pkg channels [OPTIONS] <PKG_IDENT> [PKG_TARGET]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<PKG_IDENT>     A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
<PKG_TARGET>    A package target (ex: x86_64-windows) (default: system appropriate target) [env: HAB_PACKAGE_TARGET=]
```

### hab pkg config

Displays the default configuration options for a service

**USAGE**

```
hab pkg config <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```


**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg delete

Removes a package from Builder

**USAGE**

```
hab pkg delete [OPTIONS] <PKG_IDENT> [PKG_TARGET]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<PKG_IDENT>     A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
<PKG_TARGET>    A package target (ex: x86_64-windows) (default: system appropriate target) [env: HAB_PACKAGE_TARGET=]
```

### hab pkg demote

Demote a package from a specified channel

**USAGE**

```
hab pkg demote [OPTIONS] <PKG_IDENT> <CHANNEL> [PKG_TARGET]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<PKG_IDENT>     A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
<CHANNEL>       Demote from the specified release channel
<PKG_TARGET>    A package target (ex: x86_64-windows) (default: system appropriate target) [env: HAB_PACKAGE_TARGET=]
```

### hab pkg dependencies

Returns the Habitat Artifact dependencies. By default it will return the direct dependencies of the package

**USAGE**

```
hab pkg dependencies [FLAGS] <PKG_IDENT>
```

**FLAGS**

```
-r, --reverse       Show packages which are dependant on this one
-t, --transitive    Show transitive dependencies
-h, --help          Prints help information
-V, --version       Prints version information
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg download

Download Habitat artifacts (including dependencies and keys) from Builder

**USAGE**

```
hab pkg download [FLAGS] [OPTIONS] [--] [PKG_IDENT]...
```

**FLAGS**

```
--ignore-missing-seeds    Ignore packages specified that are not present on the target Builder
    --verify                  Verify package integrity after download (Warning: this can be slow)
-h, --help                    Prints help information
-V, --version                 Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>                          Authentication token for Builder
-u, --url <BLDR_URL> Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-c, --channel <CHANNEL> Download from the specified release channel. Overridden if channel is specified in toml file [env: HAB_BLDR_CHANNEL=]  [default: stable]
    --download-directory <DOWNLOAD_DIRECTORY>    The path to store downloaded artifacts
    --file <PKG_IDENT_FILE>... File with newline separated package identifiers, or TOML file (ending with .toml extension)

-t, --target <PKG_TARGET> Target architecture to fetch. E.g. x86_64-linux. Overridden if architecture is specified in toml file
```

**ARGS**

```
<PKG_IDENT>...    One or more Habitat package identifiers (ex: acme/redis)
```

### hab pkg env

Prints the runtime environment of a specific installed package

**USAGE**

```
hab pkg env <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg exec

Executes a command using the 'PATH' context of an installed package

**USAGE**

```
hab pkg exec <PKG_IDENT> <CMD> [ARGS]...
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
<CMD>          The command to execute (ex: ls)
<ARGS>...      Arguments to the command
```

### hab pkg export

Exports the package to the specified format

**USAGE**

```
hab pkg export <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab pkg export cf](#hab-pkg-export-cf) | Cloud Foundry exporter |
| [hab pkg export container](#hab-pkg-export-container) | Container exporter |
| [hab pkg export mesos](#hab-pkg-export-mesos) | Mesos exporter |
| [hab pkg export tar](#hab-pkg-export-tar) | Tar exporter |
---

### hab pkg hash

Generates a blake2b hashsum from a target at any given filepath

**USAGE**

```
hab pkg hash [SOURCE]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<SOURCE>    A filepath of the target
```

### hab pkg info

Returns the Habitat Artifact information

**USAGE**

```
hab pkg info [FLAGS] <SOURCE>
```

**FLAGS**

```
-j, --json       Output will be rendered in json. (Includes extended metadata)
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<SOURCE>    A path to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
```

### hab pkg install

Installs a Habitat package from Builder or locally from a Habitat Artifact

**USAGE**

```
hab pkg install [FLAGS] [OPTIONS] <PKG_IDENT_OR_ARTIFACT>...
```

**FLAGS**

```
-b, --binlink                Binlink all binaries from installed package(s) into BINLINK_DIR
-f, --force                  Overwrite existing binlinks
    --ignore-install-hook    Do not run any install hooks
-h, --help                   Prints help information
-V, --version                Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>            Authentication token for Builder
    --binlink-dir <BINLINK_DIR>    Binlink all binaries from installed package(s) into BINLINK_DIR [env: HAB_BINLINK_DIR=]  [default: /bin]
-u, --url <BLDR_URL>               Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-c, --channel <CHANNEL>            Install from the specified release channel [env: HAB_BLDR_CHANNEL=]  [default: stable]
```

**ARGS**

```
<PKG_IDENT_OR_ARTIFACT>...    One or more Habitat package identifiers (ex: acme/redis) and/or filepaths to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
```

### hab pkg list

List all versions of installed packages

**USAGE**

```
hab pkg list [OPTIONS] <--all|--origin <ORIGIN>|PKG_IDENT>
```

**FLAGS**

```
-a, --all        List all installed packages
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-o, --origin <ORIGIN>    An origin to list
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg path

Prints the path to a specific installed release of a package

**USAGE**

```
hab pkg path <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg promote

Promote a package to a specified channel

**USAGE**

```
hab pkg promote [OPTIONS] <PKG_IDENT> <CHANNEL> [PKG_TARGET]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
```

**ARGS**

```
<PKG_IDENT>     A fully qualified package identifier (ex: core/busybox-static/1.42.2/20170513215502)
<CHANNEL>       Promote to the specified release channel
<PKG_TARGET>    A package target (ex: x86_64-windows) (default: system appropriate target) [env: HAB_PACKAGE_TARGET=]
```

### hab pkg provides

Search installed Habitat packages for a given file

**USAGE**

```
hab pkg provides [FLAGS] <FILE>
```

**FLAGS**

```
-p               Show full path to file
-r               Show fully qualified package names (ex: core/busybox-static/1.24.2/20160708162350)
-h, --help       Prints help information
-V, --version    Prints version information
```

**ARGS**

```
<FILE>    File name to find
```

### hab pkg search

Search for a package in Builder

**USAGE**

```
hab pkg search [OPTIONS] <SEARCH_TERM>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>    Authentication token for Builder
-u, --url <BLDR_URL>       Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
-l, --limit <LIMIT>        Limit how many packages to retrieve [default: 50]
```

**ARGS**

```
<SEARCH_TERM>    Search term
```

### hab pkg sign

Signs an archive with an origin key, generating a Habitat Artifact

**USAGE**

```
hab pkg sign [OPTIONS] <SOURCE> <DEST>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
    --origin <ORIGIN>                    Origin key used to create signature
```

**ARGS**

```
<SOURCE>    A path to a source archive file (ex: /home/acme-redis-3.0.7-21120102031201.tar.xz)
<DEST>      The destination path to the signed Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201- x86_64-linux.hart)
```

### hab pkg uninstall

Safely uninstall a package and dependencies from the local filesystem

**USAGE**

```
hab pkg uninstall [FLAGS] [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-d, --dryrun                   Just show what would be uninstalled, don't actually do it
    --ignore-uninstall-hook    Do not run any uninstall hooks
    --no-deps                  Don't uninstall dependencies
-h, --help                     Prints help information
-V, --version                  Prints version information
```

**OPTIONS**

```
--exclude <EXCLUDE>...         Identifier of one or more packages that should not be uninstalled. (ex: core/redis, core/busybox-static/1.42.2/21120102031201)
    --keep-latest <KEEP_LATEST>    Only keep this number of latest packages uninstalling all others
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab pkg upload

Uploads a local Habitat Artifact to Builder

**USAGE**

```
hab pkg upload [FLAGS] [OPTIONS] <HART_FILE>...
```

**FLAGS**

```
--force       Skips checking availability of package and force uploads, potentially overwriting a stored copy of a package. (default: false)
    --no-build    Disable auto-build for all packages in this upload
-h, --help        Prints help information
-V, --version     Prints version information
```

**OPTIONS**

```
-z, --auth <AUTH_TOKEN>                  Authentication token for Builder
-u, --url <BLDR_URL>                     Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    --cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
-c, --channel <CHANNEL>                  Optional additional release channel to upload package to. Packages are always uploaded to unstable, regardless of the value of this option
```

**ARGS**

```
<HART_FILE>...    One or more filepaths to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64- linux.hart)
```

### hab pkg verify

Verifies a Habitat Artifact with an origin key

**USAGE**

```
hab pkg verify [OPTIONS] <SOURCE>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<SOURCE>    A path to a Habitat Artifact (ex: /home/acme-redis-3.0.7-21120102031201-x86_64-linux.hart)
```

## hab plan

Commands relating to plans and other app-specific configuration

**USAGE**

```
hab plan <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab plan init]({{< relref "#hab-plan-init" >}}) | Generates common package specific configuration files. Executing without argument will create a habitat directory in your current folder for the plan. If PKG_NAME is specified it will create a folder with that name. Environment variables (those starting with 'pkg_') that are set will be used in the generated plan |
| [hab plan render]({{< relref "#hab-plan-render" >}}) | Renders plan config files |
---

### hab plan init

Generates common package specific configuration files. Executing without argument will create a habitat directory in

**USAGE**

```
hab plan init [FLAGS] [OPTIONS] [PKG_NAME]
```

**FLAGS**

```
-m, --min        Create a minimal plan file
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-o, --origin <ORIGIN>              Origin for the new app
-s, --scaffolding <SCAFFOLDING>    Specify explicit Scaffolding for your app (ex: node, ruby)
```

**ARGS**

```
<PKG_NAME>    Name for the new app
```

### hab plan render

Renders plan config files

**USAGE**

```
hab plan render [FLAGS] [OPTIONS] <TEMPLATE_PATH>
```

**FLAGS**

```
-n, --no-render    Don't write anything to disk, ignores --render-dir
-p, --print        Prints config to STDOUT
-q, --quiet        Don't print any helper messages.  When used with --print will only print config file
-h, --help         Prints help information
-V, --version      Prints version information
```

**OPTIONS**

```
-d, --default-toml <DEFAULT_TOML>    Path to default.toml [default: ./default.toml]
-m, --mock-data <MOCK_DATA>          Path to json file with mock data for template, defaults to none
-r, --render-dir <RENDER_DIR>        Path to render templates [default: ./results]
-u, --user-toml <USER_TOML>          Path to user.toml, defaults to none
```

**ARGS**

```
<TEMPLATE_PATH>    Path to config to render
```

## hab ring

Commands relating to Habitat rings

**USAGE**

```
hab ring <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab ring key]({{< relref "#hab-ring-key" >}}) | Commands relating to Habitat ring keys |
---

### hab ring key

Commands relating to Habitat ring keys

**USAGE**

```
hab ring key <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab ring key export]({{< relref "#hab-ring-key-export" >}}) | Outputs the latest ring key contents to stdout |
| [hab ring key generate]({{< relref "#hab-ring-key-generate" >}}) | Generates a Habitat ring key |
| [hab ring key import]({{< relref "#hab-ring-key-import" >}}) | Reads a stdin stream containing ring key contents and writes the key to disk |
---

### hab ring key export

Outputs the latest ring key contents to stdout

**USAGE**

```
hab ring key export [OPTIONS] <RING>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<RING>    Ring key name
```

### hab ring key generate

Generates a Habitat ring key

**USAGE**

```
hab ring key generate [OPTIONS] <RING>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<RING>    Ring key name
```

### hab ring key import

Reads a stdin stream containing ring key contents and writes the key to disk

**USAGE**

```
hab ring key import [OPTIONS]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

## hab studio

**USAGE**

```
hab studio [FLAGS] [OPTIONS] <SUBCOMMAND> [ARG ..]
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab studio build]({{< relref "#hab-studio-build" >}}) | Build using a Studio |
| [hab studio enter]({{< relref "#hab-studio-enter" >}}) | Interactively enter a Studio |
| [hab studio new]({{< relref "#hab-studio-new" >}}) | Creates a new Studio |
| [hab studio rm]({{< relref "#hab-studio-rm" >}}) | Destroys a Studio |
| [hab studio run]({{< relref "#hab-studio-run" >}}) | Run a command in a Studio |
| [hab studio version]({{< relref "#hab-studio-version" >}}) | Prints version information |
---

### hab studio build

**USAGE**

```
hab studio [COMMON_FLAGS] [COMMON_OPTIONS] build [FLAGS] [PLAN_DIR]
```

**FLAGS**

```
-R  Reuse a previous Studio state (default: clean up before building)
```

### hab studio enter

**USAGE**

```
hab studio [COMMON_FLAGS] [COMMON_OPTIONS] enter
```

### hab studio new

**USAGE**

```
hab studio [COMMON_FLAGS] [COMMON_OPTIONS] new
```

### hab studio rm

**USAGE**

```
hab studio [COMMON_FLAGS] [COMMON_OPTIONS] rm
```

### hab studio run

**USAGE**

```
hab studio [COMMON_FLAGS] [COMMON_OPTIONS] run [CMD] [ARG ..]
```

### hab studio version


## hab sup

**USAGE**

```
hab sup <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab sup bash]({{< relref "#hab-sup-bash" >}}) | Start an interactive Bash-like shell |
| [hab sup depart]({{< relref "#hab-sup-depart" >}}) | Depart a Supervisor from the gossip ring; kicking and banning the target from joining again with the same member-id |
| [hab sup run]({{< relref "#hab-sup-run" >}}) | Run the Habitat Supervisor |
| [hab sup secret]({{< relref "#hab-sup-secret" >}}) | Commands relating to a Habitat Supervisor's Control Gateway secret |
| [hab sup sh]({{< relref "#hab-sup-sh" >}}) | Start an interactive Bourne-like shell |
| [hab sup status]({{< relref "#hab-sup-status" >}}) | Query the status of Habitat services |
| [hab sup term]({{< relref "#hab-sup-term" >}}) | Gracefully terminate the Habitat Supervisor and all of its running services |
---

### hab sup bash

Start an interactive Bash-like shell

**USAGE**

```
hab sup bash
```

**FLAGS**

```
-h, --help    Prints help information
```

### hab sup depart

Depart a Supervisor from the gossip ring; kicking and banning the target from joining again with the same member-id

**USAGE**

```
hab sup depart [OPTIONS] <MEMBER_ID>
```

**FLAGS**

```
-h, --help    Prints help information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP>    Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
```

**ARGS**

```
<MEMBER_ID>    The member-id of the Supervisor to depart
```

### hab sup restart

Restart a Supervisor without restarting its services

**USAGE**

```
hab sup restart [OPTIONS]
```

**FLAGS**

```
-h, --help    Prints help information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP>    Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
```

### hab sup run

Run the Habitat Supervisor

**USAGE**

```
hab sup run [FLAGS] [OPTIONS] [--] [PKG_IDENT_OR_ARTIFACT]
```

**FLAGS**

```
-A, --auto-update          Enable automatic updates for the Supervisor itself
    --generate-config      Generate a TOML config
-D, --http-disable         Disable the HTTP Gateway completely
    --json-logging         Use structured JSON logging for the Supervisor
    --local-gossip-mode    Start the supervisor in local mode
    --no-color             Turn ANSI color off
-I, --permanent-peer       Make this Supervisor a permanent peer
-v                         Verbose output showing file and line/column numbers
-h, --help                 Prints help information
```

**OPTIONS**

```
--auto-update-period <AUTO_UPDATE_PERIOD> The period of time in seconds between Supervisor update checks [default: 60]

    --bind <BIND>... One or more service groups to bind to a configuration

    --binding-mode <BINDING_MODE> Governs how the presence or absence of binds affects service startup [default: strict]  [possible values: strict, relaxed]
-u, --url <BLDR_URL> Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    --cache-key-path <CACHE_KEY_PATH> Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]

    --ca-certs <CA_CERT_FILE> The CA certificate for HTTP Gateway TLS encryption

    --certs <CERT_FILE> The server certificates for HTTP Gateway TLS encryption

    --channel <CHANNEL> Receive updates from the specified release channel [default: stable]

    --config-files <CONFIG_FILES>...                                       Paths to config files to read
    --config-from <CONFIG_FROM> Use the package config from this path rather than the package itself

    --ctl-client-ca-certificate <CTL_CLIENT_CA_CERTIFICATE> Enable client authentication for the control gateway and set the certificate authority to use when authenticating the client [default: /hab/cache/keys/ctl]
    --ctl-server-certificate <CTL_SERVER_CERTIFICATE> The control gateway server's TLS certificate [default: /hab/cache/keys/ctl]

    --ctl-server-key <CTL_SERVER_KEY> Enable TLS for the control gateway and set the server's private key [default: /hab/cache/keys/ctl]

    --event-meta <EVENT_META>... An arbitrary key-value pair to add to each event generated by this Supervisor

    --event-stream-application <EVENT_STREAM_APPLICATION> The name of the application for event stream purposes

    --event-stream-connect-timeout <EVENT_STREAM_CONNECT_TIMEOUT> Event stream connection timeout before exiting the Supervisor [env: HAB_EVENT_STREAM_CONNECT_TIMEOUT=] default: 0]
    --event-stream-environment <EVENT_STREAM_ENVIRONMENT> The name of the environment for event stream purposes

    --event-stream-server-certificate <EVENT_STREAM_SERVER_CERTIFICATE> The path to Chef Automate's event stream certificate used to establish a TLS connection

    --event-stream-site <EVENT_STREAM_SITE> The name of the site where this Supervisor is running for event stream purposes

    --event-stream-token <EVENT_STREAM_TOKEN> The authentication token for connecting the event stream to Chef Automate [env: HAB_AUTOMATE_AUTH_TOKEN=]

    --event-stream-url <EVENT_STREAM_URL> The event stream connection url used to send events to Chef Automate

    --group <GROUP> The service group with shared config and topology [default: default]

-i, --health-check-interval <HEALTH_CHECK_INTERVAL> The interval in seconds on which to run health checks [default: 30]

    --keep-latest-packages <KEEP_LATEST_PACKAGES> Automatically cleanup old packages [env: HAB_KEEP_LATEST_PACKAGES=]

    --key <KEY_FILE> The private key for HTTP Gateway TLS encryption

    --listen-ctl <LISTEN_CTL> The listen address for the Control Gateway [env: HAB_LISTEN_CTL=]  [default: 127.0.0.1:9632]

    --listen-gossip <LISTEN_GOSSIP> The listen address for the Gossip Gateway [env: HAB_LISTEN_GOSSIP=]  [default: 0.0.0.0:9638]

    --listen-http <LISTEN_HTTP> The listen address for the HTTP Gateway [env: HAB_LISTEN_HTTP=]  [default: 0.0.0.0:9631]

    --org <ORGANIZATION> The organization the Supervisor and its services are part of

    --peer <PEER>... The listen address of one or more initial peers (IP[:PORT])

    --peer-watch-file <PEER_WATCH_FILE> Watch this file for connecting to the ring

-r, --ring <RING> The name of the ring used by the Supervisor when running with wire encryption [env: HAB_RING=]

    --service-update-period <SERVICE_UPDATE_PERIOD> The period of time in seconds between service update checks [default: 60]

    --shutdown-timeout <SHUTDOWN_TIMEOUT> The delay in seconds after sending the shutdown signal to wait before killing the service process

-s, --strategy <STRATEGY> The update strategy [default: none]  [possible values: none, at-once, rolling]

    --sys-ip-address <SYS_IP_ADDRESS> The IPv4 address to use as the sys.ip template variable

-t, --topology <TOPOLOGY> Service topology [possible values: standalone, leader]

    --update-condition <UPDATE_CONDITION> The condition dictating when this service should update [default: latest]  [possible values: latest, track- channel]
```

**ARGS**

```
<PKG_IDENT_OR_ARTIFACT>    Load a Habitat package as part of the Supervisor startup
```

### hab sup secret

Commands relating to a Habitat Supervisor's Control Gateway secret

**USAGE**

```
hab sup secret <SUBCOMMAND>
```

**FLAGS**

```
-h, --help    Prints help information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab sup secret generate]({{< relref "#hab-sup-secret-generate" >}}) | Generate a secret key to use as a Supervisor's Control Gateway secret |
---

### hab sup secret generate

Generate a secret key to use as a Supervisor's Control Gateway secret

**USAGE**

```
hab sup secret generate
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

### hab sup secret generate-tls

Generate a private key and certificate for the Supervisor's Control Gateway TLS connection

**USAGE**

```
hab sup secret generate-tls [OPTIONS] --subject-alternative-name <subject-alternative-name>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--path <path> The directory to store the generated private key and certificate [default: /hab/cache/keys/ctl]

    --subject-alternative-name <subject-alternative-name> The DNS name to use in the certificates subject alternative name extension
```

### hab sup sh

Start an interactive Bourne-like shell

**USAGE**

```
hab sup sh
```

**FLAGS**

```
-h, --help    Prints help information
```

### hab sup status

Query the status of Habitat services

**USAGE**

```
hab sup status [OPTIONS] [PKG_IDENT]
```

**FLAGS**

```
-h, --help    Prints help information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP>    Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab sup term

Gracefully terminate the Habitat Supervisor and all of its running services

**USAGE**

```
hab sup term
```

**FLAGS**

```
-h, --help    Prints help information
```

## hab supportbundle

Create a tarball of Habitat Supervisor data to send to support

**USAGE**

```
hab supportbundle
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```


## hab svc

Commands relating to Habitat services

**USAGE**

```
hab svc <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab svc key]({{< relref "#hab-svc-key" >}}) | Commands relating to Habitat service keys |
| [hab svc load]({{< relref "#hab-svc-load" >}}) | Load a service to be started and supervised by Habitat from a package identifier. If an installed package doesn't satisfy the given package identifier, a suitable package will be installed from Builder |
| [hab svc start]({{< relref "#hab-svc-start" >}}) | Start a loaded, but stopped, Habitat service |
| [hab svc status]({{< relref "#hab-svc-status" >}}) | Query the status of Habitat services |
| [hab svc stop]({{< relref "#hab-svc-stop" >}}) | Stop a running Habitat service |
| [hab svc unload]({{< relref "#hab-svc-unload" >}}) | Unload a service loaded by the Habitat Supervisor. If the service is running it will additionally be stopped |
---

### hab svc key

Commands relating to Habitat service keys

**USAGE**

```
hab svc key <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab svc key generate]({{< relref "#hab-svc-key-generate" >}}) | Generates a Habitat service key |
---

### hab svc key generate

Generates a Habitat service key

**USAGE**

```
hab svc key generate [OPTIONS] <SERVICE_GROUP> [ORG]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<SERVICE_GROUP>    Target service group service.group[@organization] (ex: redis.default or foo.default@bazcorp)
<ORG>              The service organization
```

### hab svc load

Load a service to be started and supervised by Habitat from a package identifier. If an installed package doesn't

**USAGE**

```
hab svc load [FLAGS] [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-f, --force      Load or reload an already loaded service. If the service was previously loaded and running this operation will also restart the service
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--bind <BIND>...                                   One or more service groups to bind to a configuration
    --binding-mode <BINDING_MODE> Governs how the presence or absence of binds affects service startup [default: strict]  [possible values: strict, relaxed]
-u, --url <BLDR_URL> Specify an alternate Builder endpoint. If not specified, the value will be taken from the HAB_BLDR_URL environment variable if defined. (default: https://bldr.habitat.sh)
    --channel <CHANNEL> Receive updates from the specified release channel [default: stable]

    --config-from <CONFIG_FROM> Use the package config from this path rather than the package itself

    --group <GROUP> The service group with shared config and topology [default: default]

-i, --health-check-interval <HEALTH_CHECK_INTERVAL> The interval in seconds on which to run health checks [default: 30]

-r, --remote-sup <REMOTE_SUP> Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]

    --shutdown-timeout <SHUTDOWN_TIMEOUT> The delay in seconds after sending the shutdown signal to wait before killing the service process

-s, --strategy <STRATEGY> The update strategy [default: none]  [possible values: none, at-once, rolling]

-t, --topology <TOPOLOGY>                              Service topology [possible values: standalone, leader]
    --update-condition <UPDATE_CONDITION> The condition dictating when this service should update [default: latest]  [possible values: latest, track- channel]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab svc start

Start a loaded, but stopped, Habitat service

**USAGE**

```
hab svc start [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP>    Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab svc status

Query the status of Habitat services

**USAGE**

```
hab svc status [OPTIONS] [PKG_IDENT]
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP>    Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab svc stop

Stop a running Habitat service

**USAGE**

```
hab svc stop [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP> Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]

    --shutdown-timeout <SHUTDOWN_TIMEOUT> The delay in seconds after sending the shutdown signal to wait before killing the service process
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab svc unload

Unload a service loaded by the Habitat Supervisor. If the service is running it will additionally be stopped

**USAGE**

```
hab svc unload [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
-r, --remote-sup <REMOTE_SUP> Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]

    --shutdown-timeout <SHUTDOWN_TIMEOUT> The delay in seconds after sending the shutdown signal to wait before killing the service process
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

### hab svc update

Update how the Supervisor manages an already-running service. Depending on the given changes, they may be able to be

**USAGE**

```
hab svc update [OPTIONS] <PKG_IDENT>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--bind <BIND>...                                   One or more service groups to bind to a configuration
    --binding-mode <BINDING_MODE> Governs how the presence or absence of binds affects service startup [possible values: strict, relaxed]

-u, --url <BLDR_URL>                                   Specify an alternate Builder endpoint
    --channel <CHANNEL>                                Receive updates from the specified release channel
    --group <GROUP>                                    The service group with shared config and topology
-i, --health-check-interval <HEALTH_CHECK_INTERVAL>    The interval in seconds on which to run health checks
-r, --remote-sup <REMOTE_SUP> Address to a remote Supervisor's Control Gateway [default: 127.0.0.1:9632]

    --shutdown-timeout <SHUTDOWN_TIMEOUT> The delay in seconds after sending the shutdown signal to wait before killing the service process

-s, --strategy <STRATEGY>                              The update strategy [possible values: none, at-once, rolling]
-t, --topology <TOPOLOGY>                              Service topology [possible values: standalone, leader]
    --update-condition <UPDATE_CONDITION> The condition dictating when this service should update [possible values: latest, track-channel]
```

**ARGS**

```
<PKG_IDENT>    A package identifier (ex: core/redis, core/busybox-static/1.42.2)
```

## hab user

Commands relating to Habitat users

**USAGE**

```
hab user <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab user key]({{< relref "#hab-user-key" >}}) | Commands relating to Habitat user keys |
---

### hab user key

Commands relating to Habitat user keys

**USAGE**

```
hab user key <SUBCOMMAND>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**SUBCOMMANDS**

| Command | Description |
| ------- | ----------- |
| [hab user key generate]({{< relref "#hab-user-key-generate" >}}) | Generates a Habitat user key |
---

### hab user key generate

Generates a Habitat user key

**USAGE**

```
hab user key generate [OPTIONS] <USER>
```

**FLAGS**

```
-h, --help       Prints help information
-V, --version    Prints version information
```

**OPTIONS**

```
--cache-key-path <CACHE_KEY_PATH>    Cache for creating and searching for encryption keys [env: HAB_CACHE_KEY_PATH=]  [default: /hab/cache/keys]
```

**ARGS**

```
<USER>    Name of the user key
```
