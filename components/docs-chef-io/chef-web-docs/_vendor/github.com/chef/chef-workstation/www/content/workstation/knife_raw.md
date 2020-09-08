+++
title = "knife raw"
draft = false

aliases = ["/knife_raw.html", "/knife_raw/"]

[menu]
  [menu.workstation]
    title = "knife raw"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_raw.md knife raw"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_raw.md)

{{% knife_raw_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife raw REQUEST_PATH (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`-i FILE`, `--input FILE`

:   The name of a file to be used with the `PUT` or a `POST` request.

`--[no-]pretty`

:   Use `--no-pretty` to disable pretty-print output for JSON. Default:
    `--pretty`.

`-m METHOD`, `--method METHOD`

:   The request method: `DELETE`, `GET`, `POST`, or `PUT`. Default
    value: `GET`.

`--proxy-auth`

:   Enable proxy authentication to the Chef Infra Server web user
    interface. Default value: `false`.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**View a client**

To view information about a client:

``` bash
knife raw /clients/<client_name>
```

**View a node**

To view information about a node:

``` bash
knife raw /nodes/<node_name>
```

**Delete a data bag**

To delete a data bag, enter a command similar to:

``` bash
knife raw -m DELETE /data/foo
```

to return something similar to:

``` bash
{
  "name":"foo",
  "json_class":"Chef::DataBag",
  "chef_type":"data_bag"
}
```

**Delete a role**

To delete a role, enter a command similar to:

``` bash
knife raw -m DELETE /roles/role-foo.json
```

to return something similar to:

``` bash
{
  "name":"role-foo",
  "json_class":"Chef::Role",
  "chef_type":"role"
}
```
