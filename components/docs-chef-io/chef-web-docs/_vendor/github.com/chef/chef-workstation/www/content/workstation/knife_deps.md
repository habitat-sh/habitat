+++
title = "knife deps"
draft = false

aliases = ["/knife_deps.html", "/knife_deps/"]

[menu]
  [menu.workstation]
    title = "knife deps"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_deps.md knife deps"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_deps.md)

{{% knife_deps_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife deps (options)
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

`--[no-]recurse`

:   Use `--recurse` to list dependencies recursively. This option can
    only be used when `--tree` is set to `true`. Default:
    `--no-recurse`.

`--remote`

:   Determine dependencies from objects located on the Chef Infra Server
    instead of in the local chef-repo. Default: `false`.

`--repo-mode MODE`

:   The layout of the local chef-repo. Possible values: `static`,
    `everything`, or `hosted_everything`. Use `static` for just roles,
    environments, cookbooks, and data bags. By default, `everything` and
    `hosted_everything` are dynamically selected depending on the server
    type. Default: `everything` / `hosted_everything`.

`--tree`

:   Show dependencies in a visual tree structure (including duplicates,
    if they exist). Default: `false`.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**Find dependencies for a node**

``` bash
knife deps nodes/node_name.json
```

**Find dependencies for a role**

``` bash
knife deps roles/role_name.json
```

**Find dependencies for a cookbook**

``` bash
knife deps cookbooks/cookbook_name.json
```

**Find dependencies for an environment**

``` bash
knife deps environments/environment_name.json
```

**Find dependencies for a combination of nodes, roles, and so on**

To find the dependencies for a combination of nodes, cookbooks, roles,
and/or environments:

``` bash
knife deps cookbooks/git.json cookbooks/github.json roles/base.json environments/desert.json nodes/mynode.json
```

**Use a wildcard**

A wildcard can be used to return all of the child nodes. For example,
all of the environments:

``` bash
knife deps environments/*.json
```

**Return as tree**

Use the `--tree` option to view the results with structure:

``` bash
knife deps roles/webserver.json
```

to return something like:

``` none
roles/webserver.json
  roles/base.json
    cookbooks/github
      cookbooks/git
    cookbooks/users
  cookbooks/apache2
```

**Pass knife deps output to knife upload**

The output of `knife deps` can be passed to `knife upload`:

``` bash
knife upload `knife deps nodes/*.json
```

**Pass knife deps output to knife xargs**

The output of `knife deps` can be passed to `knife xargs`:

``` bash
knife deps nodes/*.json | xargs knife upload
```
