+++
title = "knife configure"
draft = false

aliases = ["/knife_configure.html", "/knife_configure/"]

[menu]
  [menu.workstation]
    title = "knife configure"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_configure.md knife configure"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_configure.md)

{{% knife_configure_summary %}}

## Syntax

This subcommand has the following syntax when creating a config.rb file:

``` bash
knife configure (options)
```

and the following syntax when creating a client.rb file:

``` bash
knife configure client DIRECTORY
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options for use when configuring a
config.rb file:

`--admin-client-name NAME`

:   The name of the client, typically the name of the admin client.

`--admin-client-key PATH`

:   The path to the private key used by the client, typically a file
    named `admin.pem`.

`-i`, `--initial`

:   Create a API client, typically an administrator client on a
    freshly-installed Chef Infra Server.

`-r REPO`, `--repository REPO`

:   The path to the chef-repo.

`--validation-client-name NAME`

:   The name of the validation client, typically a client named
    chef-validator.

`--validation-key PATH`

:   The path to the validation key used by the client, typically a file
    named chef-validator.pem.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**Configure config.rb**

``` bash
knife configure
```

**Configure client.rb**

``` bash
knife configure client '/directory'
```
