+++
title = "knife recipe list"
draft = false

aliases = ["/knife_recipe_list.html", "/knife_recipe_list/"]

[menu]
  [menu.workstation]
    title = "knife recipe list"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_recipe_list.md knife recipe list"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_recipe_list.md)

{{% knife_recipe_list_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife recipe list REGEX
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This command does not have any specific options.

## Examples

The following examples show how to use this knife subcommand:

**View a list of recipes**

To view a list of recipes:

``` bash
knife recipe list 'couchdb::*'
```

to return:

``` bash
couchdb::main_monitors
couchdb::master
couchdb::default
couchdb::org_cleanup
```
