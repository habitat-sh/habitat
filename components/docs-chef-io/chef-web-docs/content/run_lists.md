+++
title = "About Run-lists"
draft = false

aliases = ["/run_lists.html"]

[menu]
  [menu.infra]
    title = "Run-lists"
    identifier = "chef_infra/concepts/policy/run_lists.md Run-lists"
    parent = "chef_infra/concepts/policy"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/run_lists.md)

{{% node_run_list %}}

## Run-list Format

{{% node_run_list_format %}}

### Empty Run-lists

{{% node_run_list_empty %}}

## Knife Commands

The following knife commands may be used to manage run-lists on the Chef
Infra Server.

### Quotes, Windows

{{% knife_common_windows_quotes %}}

#### Import-Module chef

{{% knife_common_windows_quotes_module %}}

### run_list add

{{% knife_node_run_list_add %}}

{{% node_run_list_format %}}

#### Syntax

{{% knife_node_run_list_add_syntax %}}

#### Options

{{% knife_node_run_list_add_options %}}

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

#### Examples

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

### run_list remove

{{% knife_node_run_list_remove %}}

#### Syntax

{{% knife_node_run_list_remove_syntax %}}

#### Options

This command does not have any specific options.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

#### Examples

The following examples show how to use this knife subcommand:

**Remove a role**

{{% knife_node_run_list_remove_role %}}

**Remove a run-list**

{{% knife_node_run_list_remove_run_list %}}

### run_list set

{{% knife_node_run_list_set %}}

#### Syntax

{{% knife_node_run_list_set_syntax %}}

#### Options

This command does not have any specific options.

#### Examples

None.

### status

The following examples show how to use the `knife status` subcommand to
verify the status of run-lists.

**View status, include run-lists**

{{% knife_status_include_run_lists %}}

**View status using a query**

{{% knife_status_returned_by_query %}}

## Management Console

The following sections describe how to manage run-lists when using the
Chef management console.

### Add Recipe

{{% manage_webui_node_run_list_add_role_or_recipe %}}

### Add Role

{{% manage_webui_node_run_list_add_role_or_recipe %}}

### Edit Node

{{% manage_webui_node_run_list_edit %}}

### Edit Role

{{% manage_webui_policy_role_edit_run_list %}}

### Remove Recipe

{{% manage_webui_node_run_list_remove_role_or_recipe %}}

### Remove Role

{{% manage_webui_node_run_list_remove_role_or_recipe %}}

### View Current

{{% manage_webui_node_run_list_view_current %}}

### View Node

To view all of the nodes:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node.

4.  Select the **Details** tab.

5.  The run-list for the node appears under the **Run List** header:

    ![image](/images/step_manage_webui_nodes_view_run_list.png)

## Run-lists, Applied

A run-list will tell Chef Infra Client what to do when bootstrapping
that node for the first time, and then how to configure that node on
every subsequent Chef Infra Client run.

### Bootstrap Operations

{{% install_chef_client %}}

{{% chef_client_bootstrap_node %}}

{{% chef_client_bootstrap_stages %}}

### The Chef Infra Client Run

{{% chef_client_run %}}

### Attribute Evaluation Order

{{% node_attribute_evaluation_order %}}
