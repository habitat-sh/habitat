+++
title = "knife download"
draft = false

aliases = ["/knife_download.html", "/knife_download/"]

[menu]
  [menu.workstation]
    title = "knife download"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_download.md knife download"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_download.md)

{{% knife_download_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife download [PATTERN...] (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`--chef-repo-path PATH`

:   The path to the chef-repo. This setting will override the default
    path to the chef-repo. Default: same value as specified by
    `chef_repo_path` in client.rb.

`--concurrency`

:   The number of allowed concurrent connections. Default: `10`.

`--cookbook-version VERSION`

:   The version of a cookbook to download.

`-n`, `--dry-run`

:   Take no action and only print out results. Default: `false`.

`--[no-]diff`

:   Download only new and modified files. Set to `false` to download all
    files. Default: `--diff`.

`--[no-]force`

:   Use `--force` to download files even when the file on the hard drive
    is identical to the object on the server (role, cookbook, etc.). By
    default, files are compared to see if they have equivalent content,
    and local files are only overwritten if they are different. Default:
    `--no-force`.

`--[no-]purge`

:   Use `--purge` to delete local files and directories that do not
    exist on the Chef Infra Server. By default, if a role, cookbook,
    etc. does not exist on the Chef Infra Server, the local file for
    said role is left alone and NOT deleted. Default: `--no-purge`.

`--[no-]recurse`

:   Use `--no-recurse` to disable downloading a directory recursively.
    Default: `--recurse`.

`--repo-mode MODE`

:   The layout of the local chef-repo. Possible values: `static`,
    `everything`, or `hosted_everything`. Use `static` for just roles,
    environments, cookbooks, and data bags. By default, `everything` and
    `hosted_everything` are dynamically selected depending on the server
    type. Default: `everything` / `hosted_everything`.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**Download the entire chef-repo**

To download the entire chef-repo from the Chef Infra Server, browse to
the top level of the chef-repo and enter:

``` bash
knife download /
```

**Download the /cookbooks directory**

To download the `cookbooks/` directory from the Chef Infra Server,
browse to the top level of the chef-repo and enter:

``` bash
knife download cookbooks
```

or from anywhere in the chef-repo, enter:

``` bash
knife download /cookbooks
```

**Download the /environments directory**

To download the `environments/` directory from the Chef Infra Server,
browse to the top level of the chef-repo and enter:

``` bash
knife download environments
```

or from anywhere in the chef-repo, enter:

``` bash
knife download /environments
```

**Download an environment**

To download an environment named "production" from the Chef Infra
Server, browse to the top level of the chef-repo and enter:

``` bash
knife download environments/production.json
```

or from the `environments/` directory, enter:

``` bash
knife download production.json
```

**Download the /roles directory**

To download the `roles/` directory from the Chef Infra Server, browse to
the top level of the chef-repo and enter:

``` bash
knife download roles
```

or from anywhere in the chef-repo, enter:

``` bash
knife download /roles
```

**Download cookbooks and roles**

To download all cookbooks that start with "apache" and belong to the
"webserver" role, browse to the top level of the chef-repo and enter:

``` bash
knife download cookbooks/apache\* roles/webserver.json
```

**Download data bags**

To download all data bags from the Chef Infra Server, browse to the top
level of the chef-repo and enter:

``` bash
knife download data_bags
```

or from anywhere in the chef-repo, enter:

``` bash
knife download /data_bags
```
