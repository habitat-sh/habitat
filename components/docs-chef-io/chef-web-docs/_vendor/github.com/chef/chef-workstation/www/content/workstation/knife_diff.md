+++
title = "knife diff"
draft = false

aliases = ["/knife_diff.html", "/knife_diff/"]

[menu]
  [menu.workstation]
    title = "knife diff"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_diff.md knife diff"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_diff.md)

{{% knife_diff_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife diff [PATTERN...] (options)
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

`--cookbook-version VERSION`

:   The version of a cookbook to download.

`--concurrency`

:   The number of allowed concurrent connections. Default: `10`.

`--diff-filter=[(A|D|M|T)...[*]]`

:   Select only files that have been added (`A`), deleted (`D`),
    modified (`M`), and/or have had their type changed (`T`). Any
    combination of filter characters may be used, including no filter
    characters. Use `*` to select all paths if a file matches other
    criteria in the comparison. Default value: `nil`.

`--name-only`

:   Show only the names of modified files.

`--name-status`

:   Show only the names of files with a status of `Added`, `Deleted`,
    `Modified`, or `Type Changed`.

`--no-recurse`

:   Use `--no-recurse` to disable listing a directory recursively.
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

**Compare files that contain JSON data**

To compare the `base.json` role to a `webserver.json` role, enter:

``` bash
knife diff roles/base.json roles/webserver.json
```

**Compare the chef-repo and the server**

To compare the differences between the local chef-repo and the files
that are on the Chef Infra Server, enter:

``` bash
knife diff
```

**Compare, then return results**

To diff a node named `node-lb` and then only return files that have been
added, deleted, modified, or changed, enter:

``` bash
knife diff --name-status node-lb
```

to return something like:

``` bash
node-lb/recipes/eip.rb
node-lb/recipes/heartbeat-int.rb
node-lb/templates/default/corpsite.conf.erb
node-lb/files/default/wildcard.node.com.crt
node-lb/files/default/wildcard.node.com.crt-2009
node-lb/files/default/wildcard.node.com.key
node-lb/.gitignore
node-lb/Rakefile
```
