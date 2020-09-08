+++
title = "knife delete"
draft = false

aliases = ["/knife_delete.html", "/knife_delete/"]

[menu]
  [menu.workstation]
    title = "knife delete"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_delete.md knife delete"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_delete.md)

{{% knife_delete_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife delete [PATTERN...] (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`--both`

:   Delete both local and remote copies of an object. Default: `false`.

`--chef-repo-path PATH`

:   The path to the chef-repo. This setting will override the default
    path to the chef-repo. Default: same value as specified by
    `chef_repo_path` in client.rb.

`--concurrency`

:   The number of allowed concurrent connections. Default: `10`.

`--local`

:   Delete only the local copy of an object. A remote copy will not be
    deleted. Default: `false`.

`-r`, `--[no-]recurse`

:   Use `--recurse` to delete directories recursively. Default:
    `--no-recurse`.

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

None.
