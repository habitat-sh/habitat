+++
title = "knife edit"
draft = false

aliases = ["/knife_edit.html", "/knife_edit/"]

[menu]
  [menu.workstation]
    title = "knife edit"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_edit.md knife edit"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_edit.md)

{{% knife_edit_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife edit (options)
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

`--local`

:   Show files in the local chef-repo instead of a remote location.
    Default: `false`.

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

**Remove a user from /groups/admins.json**

{{% knife_edit_admin_users %}}
