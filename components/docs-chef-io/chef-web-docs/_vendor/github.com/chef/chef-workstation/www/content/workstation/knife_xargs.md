+++
title = "knife xargs"
draft = false

aliases = ["/knife_xargs.html", "/knife_xargs/"]

[menu]
  [menu.workstation]
    title = "knife xargs"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_xargs.md knife xargs"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_xargs.md)

{{% knife_xargs_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife xargs [PATTERN...] (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`-0`

:   Default: `false`.

`--chef-repo-path PATH`

:   The path to the chef-repo. This setting will override the default
    path to the chef-repo. Default: same value as specified by
    `chef_repo_path` in client.rb.

`--concurrency`

:   The number of allowed concurrent connections. Default: `10`.

`--[no-]diff`

:   Show a diff when a file changes. Default: `--diff`.

`--dry-run`

:   Prevent changes from being uploaded to the Chef Infra Server.
    Default: `false`.

`--[no-]force`

:   Force the upload of files even if they haven't been changed.
    Default: `--no-force`.

`-I REPLACE_STRING`, `--replace REPLACE_STRING`

:   Define a string that is to be used to replace all occurrences of a
    file name. Default: `nil`.

`-J REPLACE_STRING`, `--replace-first REPLACE_STRING`

:   Define a string that is to be used to replace the first occurrence
    of a file name. Default: `nil`.

`--local`

:   Build or execute a command line against a local file. Set to `false`
    to build or execute against a remote file. Default: `false`.

`-n MAX_ARGS`, `--max-args MAX_ARGS`

:   The maximum number of arguments per command line. Default: `nil`.

`-p [PATTERN...]`, `--pattern [PATTERN...]`

:   One (or more) patterns for a command line. If this option is not
    specified, a list of patterns may be expected on standard input.
    Default: `nil`.

`--repo-mode MODE`

:   The layout of the local chef-repo. Possible values: `static`,
    `everything`, or `hosted_everything`. Use `static` for just roles,
    environments, cookbooks, and data bags. By default, `everything` and
    `hosted_everything` are dynamically selected depending on the server
    type. Default value: `default`.

`-s LENGTH`, `--max-chars LENGTH`

:   The maximum size (in characters) for a command line. Default: `nil`.

`-t`

:   Run the print command on the command line. Default: `nil`.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**Find, and then replace data**

The following example will go through all nodes on the server, and then
replace the word `foobar` with `baz`:

``` bash
knife xargs --pattern /nodes/* "perl -i -pe 's/foobar/baz'"
```

**Use output of knife list and Perl**

The following examples show various ways of listing all nodes on the
server, and then using Perl to replace `grantmc` with `gmc`:

``` bash
knife list 'nodes/*' | knife xargs "perl -i -pe 's/grantmc/gmc'"
```

or without quotes and the backslash escaped:

``` bash
knife list /nodes/\* | knife xargs "perl -i -pe 's/grantmc/gmc'"
```

or by using the `--pattern` option:

``` bash
knife xargs --pattern '/nodes.*' "perl -i -pe 's/grantmc/gmc'"
```

**View security groups data**

The following example shows how to display the content of all groups on
the server:

``` bash
knife xargs --pattern '/groups/*' cat
```

and will return something like:

``` javascript
{
  "name": "4bd14db60aasdfb10f525400cdde21",
  "users": [
    "grantmc"
  ]
}{
  "name": "62c4e268e15fasdasc525400cd944b",
  "users": [
    "robertf"
  ]
}{
  "name": "admins",
  "users": [
    "grantmc",
    "robertf"
  ]
}{
  "name": "billing-admins",
  "users": [
    "dtek"
  ]
}{
  "name": "clients",
  "clients": [
    "12345",
    "67890",
  ]
}{
  "name": "users",
  "users": [
    "grantmc"
    "robertf"
    "dtek"
  ],
  "groups": [
    "4bd14db60aasdfb10f525400cdde21",
    "62c4e268e15fasdasc525400cd944b"
  ]
}
```
