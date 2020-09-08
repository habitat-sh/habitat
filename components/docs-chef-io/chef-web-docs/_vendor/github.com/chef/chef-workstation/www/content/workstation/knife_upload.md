+++
title = "knife upload"
draft = false

aliases = ["/knife_upload.html", "/knife_upload/"]

[menu]
  [menu.workstation]
    title = "knife upload"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_upload.md knife upload"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_upload.md)

{{% knife_upload_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife upload [PATTERN...] (options)
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

`--[no-]diff`

:   Upload only new and modified files. Set to `false` to upload all
    files. Default: `true`.

`--[no-]force`

:   Use `--force` to upload roles, cookbooks, etc. even if the file in
    the directory is identical (by default, no `POST` or `PUT` is
    performed unless an actual change would be made). Default:
    `--no-force`.

`--[no-]freeze`

:   Require changes to a cookbook be included as a new version. Only the
    `--force` option can override this setting. Default: `false`.

`-n`, `--dry-run`

:   Take no action and only print out results. Default: `false`.

`--[no-]purge`

:   Use `--purge` to delete roles, cookbooks, etc. from the Chef Infra
    Server if their corresponding files do not exist in the chef-repo.
    By default, such objects are left alone and NOT purged. Default:
    `--no-purge`.

`--[no-]recurse`

:   Use `--no-recurse` to disable uploading a directory recursively.
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

**Upload the entire chef-repo**

Browse to the top level of the chef-repo and enter:

``` bash
knife upload .
```

or from anywhere in the chef-repo, enter:

``` bash
knife upload /
```

to upload all cookbooks and data bags, plus all roles and enviroments
that are stored as JSON data. (Roles and environments stored as Ruby
data will not be uploaded.)

**Upload the /cookbooks directory**

Browse to the top level of the chef-repo and enter:

``` bash
knife upload cookbooks
```

or from anywhere in the chef-repo, enter:

``` bash
knife upload /cookbooks
```

**Upload the /environments directory**

Browse to the top level of the chef-repo and enter:

``` bash
knife upload environments
```

or from anywhere in the chef-repo, enter:

``` bash
knife upload /environments
```

to upload all enviroments that are stored as JSON data. (Environments
stored as Ruby data will not be uploaded.)

**Upload a single environment**

Browse to the top level of the chef-repo and enter:

``` bash
knife upload environments/production.json
```

or from the `environments/` directory, enter:

``` bash
knife upload production.json
```

**Upload the /roles directory**

Browse to the top level of the chef-repo and enter:

``` bash
knife upload roles
```

or from anywhere in the chef-repo, enter:

``` bash
knife upload /roles
```

to upload all roles that are stored as JSON data. (Roles stored as Ruby
data will not be uploaded.)

**Upload cookbooks and roles**

Browse to the top level of the chef-repo and enter:

``` bash
knife upload cookbooks/apache\* roles/webserver.json
```

**Use output of knife deps to pass command to knife upload**

``` bash
knife upload `knife deps nodes/*.json`
```
