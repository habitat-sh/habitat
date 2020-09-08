+++
title = "knife status"
draft = false

aliases = ["/knife_status.html", "/knife_status/"]

[menu]
  [menu.workstation]
    title = "knife status"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_status.md knife status"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_status.md)

{{% knife_status_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife status (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`QUERY`

:   The search query used to identify a list of items on a Chef Infra
    Server. This option uses the same syntax as the `search` subcommand.

`--hide-by-mins`

:   Hide nodes that have performed a successful Chef Infra Client run
    within the last specified number of minutes. The number of minutes
    to hide is provided as an integer, such as `--hide-by-mins 10`.

`-l`, `--long`

:   Display all attributes in the output and show the output as JSON.

`-m`, `--medium`

:   Display normal attributes in the output and to show the output as
    JSON.

`-r RUN_LIST`, `--run-list RUN_LIST`

:   A comma-separated list of roles and/or recipes to be applied.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**View status, include run-lists**

{{% knife_status_include_run_lists %}}

**View status using a time range**

To show the status of nodes on which Chef Infra Client did not run
successfully within the past hour, enter:

``` bash
knife status --hide-by-mins 60
```

to return something like:

``` bash
422492 hours ago, runner-1-432.lxc, centos 6.8.
27 hours ago, union-3-432.lxc, centos 7.3.1611.
```

**View status using a query**

{{% knife_status_returned_by_query %}}

**View status for all nodes**

To view the status of all nodes in the organization, enter:

``` bash
knife status
```

to return something like:

``` bash
20 hours ago, dev-vm.chisamore.com, ubuntu 10.04, dev-vm.chisamore.com, 10.66.44.126
3 hours ago, i-225f954f, ubuntu 10.04, ec2-67-202-63-102.compute-1.amazonaws.com, 67.202.63.102
3 hours ago, i-a45298c9, ubuntu 10.04, ec2-174-129-127-206.compute-1.amazonaws.com, 174.129.127.206
3 hours ago, i-5272a43f, ubuntu 10.04, ec2-184-73-9-250.compute-1.amazonaws.com, 184.73.9.250
3 hours ago, i-226ca64f, ubuntu 10.04, ec2-75-101-240-230.compute-1.amazonaws.com, 75.101.240.230
3 hours ago, i-f65c969b, ubuntu 10.04, ec2-184-73-60-141.compute-1.amazonaws.com, 184.73.60.141
```
