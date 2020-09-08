+++
title = "knife node"
draft = false

aliases = ["/knife_node.html", "/knife_node/"]

[menu]
  [menu.workstation]
    title = "knife node"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_node.md knife node"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_node.md)

{{% node %}}

{{% knife_node_summary %}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## bulk delete

Use the `bulk delete` argument to delete one or more nodes that match a
pattern defined by a regular expression. The regular expression must be
within quotes and not be surrounded by forward slashes (/).

### Syntax

This argument has the following syntax:

``` bash
knife node bulk delete REGEX
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Bulk delete nodes**

Use a regular expression to define the pattern used to bulk delete
nodes:

``` bash
knife node bulk delete "^[0-9]{3}$"
```

Type `Y` to confirm a deletion.

## create

Use the `create` argument to add a node to the Chef Infra Server. Node
data is stored as JSON on the Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife node create NODE_NAME
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Create a node**

To add a node named `node1`, enter:

``` bash
knife node create node1
```

In the \$EDITOR enter the node data in JSON:

``` javascript
{
   "normal": {
   },
   "name": "foobar",
   "override": {
   },
   "default": {
   },
   "json_class": "Chef::Node",
   "automatic": {
   },
   "run_list": [
      "recipe[zsh]",
      "role[webserver]"
   ],
   "chef_type": "node"
}
```

When finished, save it.

## delete

Use the `delete` argument to delete a node from the Chef Infra Server.
If using Chef Client 12.17 or later, you can delete multiple nodes using
this subcommand.

{{< note >}}

Deleting a node will not delete any corresponding API clients.

{{< /note >}}

### Syntax

This argument has the following syntax:

``` bash
knife node delete NODE_NAME
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Delete a node**

To delete a node named `node1`, enter:

``` bash
knife node delete node1
```

## edit

Use the `edit` argument to edit the details of a node on a Chef Infra
Server. Node data is stored as JSON on the Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife node edit NODE_NAME (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Display a node in the \$EDITOR. By default, attributes that are
    default, override, or automatic, are not shown.

### Examples

The following examples show how to use this knife subcommand:

**Edit a node**

To edit the data for a node named `node1`, enter:

``` bash
knife node edit node1 -a
```

Update the role data in JSON:

``` javascript
{
   "normal": {
   },
   "name": "node1",
   "override": {
   },
   "default": {
   },
   "json_class": "Chef::Node",
   "automatic": {
   },
   "run_list": [
      "recipe[devops]",
      "role[webserver]"
   ],
   "chef_type": "node"
}
```

When finished, save it.

## environment set

Use the `environment set` argument to set the environment for a node
without editing the node object.

### Syntax

This argument has the following syntax:

``` bash
knife node environment_set NODE_NAME ENVIRONMENT_NAME (options)
```

### Options

This command does not have any specific options.

### Examples

None.

## from file

Use the `from file` argument to create a node using existing node data
as a template.

### Syntax

This argument has the following syntax:

``` bash
knife node from file FILE
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Create a node using a JSON file**

To add a node using data contained in a JSON file:

``` bash
knife node from file "PATH_TO_JSON_FILE"
```

## list

Use the `list` argument to view all of the nodes that exist on a Chef
Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife node list (options)
```

### Options

This argument has the following options:

`-w`, `--with-uri`

:   Show the corresponding URIs.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**View a list of nodes**

To verify the list of nodes that are registered with the Chef Infra
Server, enter:

``` bash
knife node list
```

to return something similar to:

``` bash
i-12345678
rs-123456
```

## policy set

Use the `policy set` argument to set the policy group and policy name
for a node.

### Syntax

This argument has the following syntax:

``` bash
knife node policy set NODE POLICY_GROUP POLICY_NAME
```

### Examples

Set the policy group and policy name for a node named `test-node`:

``` bash
knife node policy set test-node 'test-group' 'test-name'
```

## run_list add

{{% node_run_list %}}

{{% knife_node_run_list_add %}}

{{% node_run_list_format %}}

### Syntax

{{% knife_node_run_list_add_syntax %}}

{{< warning >}}

{{% knife_common_windows_quotes %}}

{{< /warning >}}

{{< note >}}

{{% knife_common_windows_quotes_module %}}

{{< /note >}}

### Options

{{% knife_node_run_list_add_options %}}

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Add a role**

{{% knife_node_run_list_add_role %}}

**Add roles and recipes**

{{% knife_node_run_list_add_roles_and_recipes %}}

**Add a recipe with a FQDN**

{{% knife_node_run_list_add_recipe_with_fqdn %}}

**Add a recipe with a cookbook**

{{% knife_node_run_list_add_recipe_with_cookbook %}}

**Add the default recipe**

{{% knife_node_run_list_add_default_recipe %}}

## run_list remove

{{% knife_node_run_list_remove %}}

### Syntax

{{% knife_node_run_list_remove_syntax %}}

### Options

This command does not have any specific options.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Remove a role**

{{% knife_node_run_list_remove_role %}}

**Remove a run-list**

{{% knife_node_run_list_remove_run_list %}}

## run_list set

{{% knife_node_run_list_set %}}

### Syntax

{{% knife_node_run_list_set_syntax %}}

{{< warning >}}

{{% knife_common_windows_quotes %}}

{{< /warning >}}

{{< note >}}

{{% knife_common_windows_quotes_module %}}

{{< /note >}}

### Options

This command does not have any specific options.

### Examples

None.

## show

Use the `show` argument to display information about a node.

### Syntax

This argument has the following syntax:

``` bash
knife node show NODE_NAME (options)
```

### Options

This argument has the following options:

`-a ATTR`, `--attribute ATTR`

:   The attribute (or attributes) to show.

`-F json`, `--format=json`

:   Display output as JSON.

`-l`, `--long`

:   Display all attributes in the output.

`-m`, `--medium`

:   Display normal attributes in the output.

`-r`, `--run-list`

:   Show only the run-list.

### Examples

The following examples show how to use this knife subcommand:

**Show all data about nodes**

To view all data for a node named `build`, enter:

``` bash
knife node show build
```

to return:

``` bash
Node Name:   build
Environment: _default
FQDN:
IP:
Run List:
Roles:
Recipes:
Platform:
```

**Show basic information about nodes**

To show basic information about a node, truncated and nicely formatted:

``` bash
knife node show NODE_NAME
```

**Show all data about nodes, truncated**

To show all information about a node, nicely formatted:

``` bash
knife node show -l NODE_NAME
```

**Show attributes**

To list a single node attribute:

``` bash
knife node show NODE_NAME -a ATTRIBUTE_NAME
```

where `ATTRIBUTE_NAME` is something like `kernel` or `platform`.

To list a nested attribute:

``` bash
knife node show NODE_NAME -a ATTRIBUTE_NAME.NESTED_ATTRIBUTE_NAME
```

where `ATTRIBUTE_NAME` is something like `kernel` and
`NESTED_ATTRIBUTE_NAME` is something like `machine`.

**Show the FQDN**

To view the FQDN for a node named `i-12345678`, enter:

``` bash
knife node show i-12345678 -a fqdn
```

to return:

``` bash
fqdn: ip-10-251-75-20.ec2.internal
```

**Show a run-list**

To view the run-list for a node named `dev`, enter:

``` bash
knife node show dev -r
```

**Show as JSON data**

To view information in JSON format, use the `-F` common option; use a
command like this for a node named `devops`:

``` bash
knife node show devops -F json
```

Other formats available include `text`, `yaml`, and `pp`.

**Show as raw JSON data**

To view node information in raw JSON, use the `-l` or `--long` option:

``` bash
knife node show -l -F json NODE_NAME
```

and/or:

``` bash
knife node show -l --format=json NODE_NAME
```
