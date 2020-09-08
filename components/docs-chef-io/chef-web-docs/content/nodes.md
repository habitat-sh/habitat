+++
title = "About Nodes"
draft = false

aliases = ["/nodes.html"]

[menu]
  [menu.infra]
    title = "Nodes"
    identifier = "chef_infra/concepts/nodes.md Nodes"
    parent = "chef_infra/concepts"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/nodes.md)

{{% node %}}

{{% node_types %}}

The key components of nodes that are under management by Chef include:

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Component</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p><img src="/images/icon_chef_client.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>{{< readFile_shortcode file="chef_client_summary.md" >}}</p>
<p>{{< readFile_shortcode file="security_key_pairs_chef_client.md" >}}</p></td>
</tr>
<tr class="even">
<td><img src="/images/icon_ohai.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="ohai_summary.md" >}}</td>
</tr>
</tbody>
</table>

## About Run-lists

{{% node_run_list %}}

### Run-list Format

{{% node_run_list_format %}}

### Empty Run-lists

{{% node_run_list_empty %}}

#### About Node Names

The name of a node is required as part of the authentication process to
the Chef Infra Server. The name of each node must be unique within an
organization, but otherwise can be any string that matches the following
regular expression:

    /^[\-[:alnum:]_:.]+$/

The name of a node can be obtained from the `node_name` attribute in the
client.rb file or by allowing Ohai to collect this data during a Chef
Infra Client run. When Ohai collects this data during a Chef Infra
Client run, it uses the node's FQDN, which is always unique within an
organization, as the name of the node.

Using the FQDN as the node name, and then allowing Ohai to collect this
information during each Chef Infra Client run, is the recommended
approach and the easiest way to ensure that the names of all nodes
across the organization are unique.

#### Manage Nodes

There are several ways to manage nodes directly: via knife, Chef
Automate, or by using command-line tools that are specific to Chef Infra
Client.

-   knife can be used to create, edit, view, list, tag, and delete
    nodes.
-   knife plug-ins can be used to create, edit, and manage nodes that
    are located on cloud providers.
-   Chef Automate can be used to create, edit, view, list, tag, and
    delete nodes. In addition, node attributes can be modified and nodes
    can be moved between environments.
-   Chef Infra Client can be used to manage node data using the command
    line and JSON files. Each JSON file contains a hash, the elements of
    which are added as node attributes. In addition, the `run_list`
    setting allows roles and/or recipes to be added to the node.
-   chef-solo can be used to manage node data using the command line and
    JSON files. Each JSON file contains a hash, the elements of which
    are added as node attributes. In addition, the `run_list` setting
    allows roles and/or recipes to be added to the node.
-   The command line can also be used to edit JSON files and files that
    are related to third-party services, such as Amazon EC2, where the
    JSON files can contain per-instance metadata that is stored in a
    file on-disk and then read by chef-solo or Chef Infra Client as
    required.

#### Node Objects

For Chef Infra Client, two important aspects of nodes are groups of
attributes and run-lists. An attribute is a specific piece of data about
the node, such as a network interface, a file system, the number of
clients a service running on a node is capable of accepting, and so on.
A run-list is an ordered list of recipes and/or roles that are run in an
exact order. The node object consists of the run-list and node
attributes, which is a JSON file that is stored on the Chef Infra
Server. Chef Infra Client gets a copy of the node object from the Chef
Infra Server during each Chef Infra Client run and places an updated
copy on the Chef Infra Server at the end of each Chef Infra Client run.

{{% node_attribute %}}

#### Attributes

{{% node_attribute_when_to_use %}}

{{% node_attribute_when_to_use_unless_variants %}}

{{< note >}}

{{% notes_see_attributes_overview %}}

{{< /note >}}

## Attribute Types

{{% node_attribute_type %}}

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Attribute Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>default</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_default.md" >}}</td>
</tr>
<tr class="even">
<td><code>force_default</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_force_default.md" >}}</td>
</tr>
<tr class="odd">
<td><code>normal</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_normal.md" >}}</td>
</tr>
<tr class="even">
<td><code>override</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_override.md" >}}</td>
</tr>
<tr class="odd">
<td><code>force_override</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_force_override.md" >}}</td>
</tr>
<tr class="even">
<td><code>automatic</code></td>
<td>{{< readFile_shortcode file="node_attribute_type_automatic.md" >}}</td>
</tr>
</tbody>
</table>

## Attribute Persistence

{{% node_attribute_persistence %}}

## Attribute Precedence

{{% node_attribute_precedence %}}

## Blacklist Attributes

{{% node_attribute_blacklist %}}

### Whitelist Attributes

{{% node_attribute_whitelist %}}
