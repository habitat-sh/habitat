+++
title = "Configure Chef Workstation"
draft = false

[menu]
  [menu.workstation]
    title = "Configure Chef Workstation"
    identifier = "chef_workstation/config.md Configure Chef Workstation"
    parent = "chef_workstation"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/config.md)


# Configuration

Chef Workstation App and `chef-run` configuration is managed in `config.toml`.
Chef Workstation will create `config.toml` the first time you use `chef-run`,
if one does not already exist.  To configure other tools, see their
respective pages found in the toolbar under Chef Workstation Tools.

## Default location

Chef Workstation looks for the `config.toml` in a default location.

### Windows

* Powershell: `$env:USERPROFILE\.chef-workstation\config.toml`
* cmd.exe: `%USERPROFILE%\.chef-workstation\config.toml`

### Linux and Mac

`/home/$USER/.chef-workstation/config.toml`

## Settings

### Telemetry

Configure telemetry behaviors for Chef Workstation components.

#### Example

```toml
[telemetry]
enable = true
dev = false
```
#### enable

Description
: When `true`, anonymous usage data and bug reports are sent to Chef. See Chef's [Privacy Statement]({{< ref "privacy.md" >}}) for the type and usage of gathered data.

Value
: `true`, `false`

Default
: `true`

Used by
: `chef-run`

`CHEF_TELEMETRY_OPT_OUT`
: When set to any value, `chef-run` will not capture or send telemetry data.

#### dev

Description
: When set to any value, `chef-run` will not capture or send telemetry data. Only set this if you have access to Chef's internal QA environment - otherwise the telemetry data will not be successfully captured by Chef. If you have access to Chef's internal QA environment, if `dev` and `enable` are both `true`, anonymous data is reported to Chef's QA environment.

Values
: `true`, `false`

Default
: `false`

Used by
: `chef-run`, `Chef Workstation App`

### Log

Control logging level and location.

#### Example

```toml
[log]
level = "debug"
location = "C:\Users\username\chef-workstation.log"
```

#### level

Description
: Determines what messages are logged from locally-run Chef Workstation commands to the to the local log file.

Values
: `"debug"`, `"warn"`, `"info"`, `"error"`, `"fatal"`

Default
: `"warn"`

Used by
: `chef-run`

#### location

Description
: The location of the local Chef Workstation log file.

Values
: Value must be a valid, writable file path.

Default
: `"$USERHOME/.chef-workstation/logs/default.log"

Used by
: `chef-run`

### Cache

Configure caching options.

#### Example

```toml
[cache]
path = "/home/users/username/.cache/chef-workstation"
```

#### path

Description
: The base path used to store cached cookbooks and downloads.

Default
: `$USERHOME/.chef-workstation/cache`

Values
: This must reference a valid, writable directory.

Used by
: `chef-run`

### Connection

Control default connection behaviors.

#### Example

```toml
[connection]
default_protocol = "winrm"
default_user = "username"
```

#### default_protocol

Description
: Default protocol for connecting to target hosts.

Values
: `"ssh"`, `"winrm"`

Default
: `"ssh"`

Used by
: `chef-run`

CLI flag
: `--protocol PROTO`

#### default_user

Description
: Default username for target host authentication.

Values
: A username that exists on the target hosts.

Default
: `root` (Linux),  `administrator` (Windows)

Used by
: `chef-run`

CLI flag
: `--user USERNAME`

### Connection.winrm

Control connection behaviors for the WinRM protocol.

#### Example

```toml
[connection.winrm]
ssl = true
ssl_verify = false
```

#### ssl

Description
: Enable SSL for WinRM session encryption

Values
: `true`, `false`

Default
: `false`

Used by
: `chef-run`

CLI flag
: `--[no]-ssl`

#### ssl_verify

Description
:Intended for use in testing environments that use self-signed certificates on Windows nodes.

Default
: `true`

Values
: `true`, `false`

Used by
: `chef-run`

CLI flag
: --ssl-[no]-verify

### Chef

Configure remote Chef running on instances.

#### Example

```toml
[chef]
trusted_certs_dir = "/home/username/mytrustedcerts"
cookbook_repo_paths = [
  "/home/username/cookbooks",
  "/var/chef/cookbooks"
]
```

#### trusted_certs_dir

Description
: Describes where to find Chef's trusted certificates. Used to ensure trusted certs are provided to the `chef-client` run on target nodes.

Values
: A directory containing the trusted certificates for use in the Chef ecosystem.

Default
:  Look first for `.chef/config.rb` and use that value if provided; otherwise `"/opt/chef-workstation/embedded/ssl/certs/"`

Used by
: `chef-run`

#### cookbook_repo_paths

Description
: Path or paths to use for cookbook resolution. When multiple cookbook paths are provided and a cookbook exists in more than one of them, the cookbook found in the last-most directory will be used. Considering the example, when resolving cookbook `mycb`: if `mycb` existed in both `/home/username/cookbooks` and `/var/chef/cookbooks`, `mycb` in `/var/chef/cookbooks` will be used.

Values
: A string referencing a valid cookbook path, or an array of such strings.  See example for syntax.

Default
: `cookbook_path` value from `.chef/config.rb`, otherwise not found

Used by
: `chef-run`

CLI flag
: `--cookbook-repo-paths PATH1,PATH2,..PATHn`

### Updates

Control the behavior of automatic update checking for Chef Workstation.

#### Example

```toml
[updates]
enable = true
channel = "current"
```

#### enable

Description
: Enable update checking for Chef Workstation updates.

Values
: `true`, `false`

Default
: `true`

Used by
: Chef Workstation App

#### channel

Description
: Set the update channel to use when checking for Chef Workstation updates. `"stable"` is the recommended value. Switch to `"current"` is not guaranteed to be stable, and should only be used if you are comfortable with the risks associated.

Values
: `"stable"`, `"current"`

Default
: `"stable"`

Used by
: Chef Workstation App

### Data_collector

Configure reporting of `chef-client` runs triggered via `chef-run`.

#### Example

```toml
[data_collector]
url = "https://1.1.1.1/data-collector/v0/"
token = "ABCDEF0123456789"
```

#### url

Description
: URL of an Automate [data collection](https://automate.chef.io/docs/data-collection/) endpoint.  This URL is provided to the target host, allowing them to report in to Automate when `chef-run` is used to converge the targets. A valid token generated by automate is required.

Values
: A valid automate data collector endpoint.

Default
: not set.

Used by
: `chef-run`

#### token

Description
: An Automate [API token](https://automate.chef.io/docs/api-tokens/#creating-a-standard-api-token), used on target host to authenticate to the `url` provided.

Values
: A valid token generated by Automate.

Default
: not set.

Used by
: `chef-run`

### Dev

These options are intended for development and troubleshooting of Chef Workstation tools. Their usage is not supported and is subject to change.

#### Example

```toml
[dev]
spinner = false
```

#### spinner

Description
: Use animated spinners and progress indicators in the terminal output.

Values
: `true`, `false`

Default
: `true`

Used by
: `chef-run`

### Features

Enable and disable experimental features for Chef Workstation.

#### Example

```toml
[features]
example = true
```

Description
: List of experimental features. Boolean. Default: none. Enable the feature with `feature = true` and disable with `feature = false`.  `example = true` enables one feature, which is the `example` feature. You can also enable or disable any feature from the command line using an environment variable. For example, setting `CHEF_FEAT_EXAMPLE=true` from the command line enables the `example` feature for the duration of your terminal session.

Values
: `name = true`, `name = false`

Used by
: The entire Chef Workstation ecosystem.
