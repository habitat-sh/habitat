+++
title = "An Overview of Chef Infra"
draft = false

aliases = ["/chef_overview.html"]

[menu]
  [menu.infra]
    title = "Chef Infra Overview"
    identifier = "chef_infra/getting_started/chef_overview.md Chef Infra Overview"
    parent = "chef_infra/getting_started"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_overview.md)

{{% chef %}}

-   **Chef Workstation** is the location where users interact with Chef
    Infra. With Chef Workstation, users can author and test
    [cookbooks](/cookbooks/) using tools such as [Test
    Kitchen](/workstation/kitchen/) and interact with the Chef Infra Server
    using the [knife](/workstation/knife/) and [chef](/ctl_chef/) command
    line tools.
-   **Chef Infra Client nodes** are the machines that are managed by
    Chef Infra. The Chef Infra Client is installed on each node and is
    used to configure the node to its desired state.
-   **Chef Infra Server** acts as [a hub for configuration
    data](/server_overview/). Chef Infra Server stores cookbooks,
    the policies that are applied to nodes, and metadata that describes
    each registered node that is being managed by Chef. Nodes use the
    Chef Infra Client to ask the Chef Infra Server for configuration
    details, such as recipes, templates, and file distributions.

## Chef Infra Components

The following diagram shows the relationships between the various
elements of Chef Infra, including the nodes, the server, and the
workstation. These elements work together to provide the Chef Infra
Client the information and instruction that it needs so that it can do
its job. As you are reviewing the rest of this topic, use the icons in
the tables to refer back to this image.

<img src="/images/chef_overview.svg" class="align-center" width="600" alt="image" />

Chef Infra has the following major components:

<table>
<colgroup>
<col style="width: 20%" />
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
<td><p><img src="/images/icon_workstation.svg" class="align-center" width="100" alt="image" /></p>
<p><img src="/images/icon_cookbook.svg" class="align-center" width="100" alt="image" /></p>
<p><img src="/images/icon_ruby.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>One (or more) workstations are configured to allow users to author, test, and maintain cookbooks.</p>
<p>Workstation systems run the Chef Workstation package which includes tools such as Chef Infra Client, Chef InSpec, Test Kitchen, ChefSpec, Cookstyle, and other tools necessary for developing and testing your infrastructure with Chef products.</p>
<p>Cookbooks are uploaded to the Chef Infra Server from these workstations. Some cookbooks are custom to the organization and others are based on community cookbooks available from the Chef Supermarket.</p>
<p>Ruby is the programming language that is the authoring syntax for cookbooks. Most recipes are simple patterns (blocks that define properties and values that map to specific configuration items like packages, files, services, templates, and users. The full power of Ruby is available for when you need a programming language.</p></td>
</tr>
<tr class="even">
<td><p><img src="/images/icon_node.svg" class="align-center" width="100" alt="image" /></p>
<p><img src="/images/icon_chef_client.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>{{< readFile_shortcode file="node.md" >}}</p>
<p>Chef Infra Client is installed on each node that is managed with Chef Infra. Chef Infra Client configures the node locally by performing the tasks specified in the run-list. Chef Infra Client will also pull down any required configuration data from the Chef Infra Server during a Chef Infra Client run.</p></td>
</tr>
<tr class="odd">
<td><p><img src="/images/icon_chef_server.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>The Chef Infra Server acts as a hub of information. Cookbooks and policy settings are uploaded to the Chef Infra Server by users from workstations. (Policy settings may also be maintained from the Chef Infra Server itself, via the Chef management console web user interface.)</p>
<p>The Chef Infra Client accesses the Chef Infra Server from the node on which it's installed to get configuration data, performs searches of historical Chef Infra Client run data, and then pulls down the necessary configuration data. After a Chef Infra Client run is finished, the Chef Infra Client uploads updated run data to the Chef Infra Server.</p></td>
</tr>
<tr class="even">
<td><img src="/images/icon_chef_supermarket.svg" class="align-center" width="100" alt="image" /></td>
<td>Chef Supermarket is the location in which community cookbooks are shared and managed. Cookbooks that are part of the Chef Supermarket may be used by any Chef user. How community cookbooks are used varies from organization to organization.</td>
</tr>
</tbody>
</table>

Chef Infra Client run reporting, compliance reporting, high availability
configurations, and Chef Infra Server replication are available as part
of Chef Automate.

The following sections discuss these elements (and their various
components) in more detail.

## Workstations

A workstation is your local computer running Chef Workstation that you
use to author cookbooks, interact with the Chef Infra Server, and
interact with nodes.

The workstation is where users do most of their work, including:

- Developing and testing cookbooks
- Keeping the Chef Infra repository synchronized with version source control
- Configuring organizational by including defining and applying Policyfiles or Policy Groups
- Interacting with nodes, as (or when) required, such as performing a bootstrap operation

### Chef Workstation Components and Tools

Some important tools and components of Chef Workstation include:

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
<td><img src="/images/icon_devkit.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="chef_workstation.md" >}}</td>
</tr>
<tr class="even">
<td><p><img src="/images/icon_ctl_chef.svg" class="align-center" width="100" alt="image" /></p>
<p><img src="/images/icon_ctl_knife.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>Chef Workstation includes important command-line tools:</p>
<ul>
<li>Chef Infra: Use the chef command-line tool to work with items in a chef-repo, which is the primary location in which cookbooks are authored, tested, and maintained, and from which policy is uploaded to the Chef Infra Server</li>
<li>Knife: Use the knife command-line tool to interact with nodes or work with objects on the Chef Infra Server</li>
<li>Chef Infra Client: an agent that configures your nodes</li>
<li>Test Kitchen: a testing harness for rapid validation of Chef code</li>
<li>Chef InSpec: Chef's open source security &amp; compliance automation framework</li>
<li>chef-run: a tool for running ad-hoc tasks</li>
<li>Chef Workstation App: for updating and managing your chef tools</li>
</ul></td>
</tr>
<tr class="odd">
<td><p><img src="/images/icon_repository.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>The chef-repo is the repository structure in which cookbooks are authored, tested, and maintained:</p>
<ul>
<li>Cookbooks contain recipes, attributes, custom resources, libraries, files, templates, tests, and metadata</li>
<li>The chef-repo should be synchronized with a version control system (such as git), and then managed as if it were source code</li>
</ul>
<p>The directory structure within the chef-repo varies. Some organizations prefer to keep all of their cookbooks in a single chef-repo, while other organizations prefer to use a chef-repo for every cookbook.</p></td>
</tr>
<tr class="even">
<td><img src="/images/icon_kitchen.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="test_kitchen.md" >}}</td>
</tr>
<tr class="odd">
<td><img src="/images/icon_chefspec.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="chefspec_summary.md" >}}</td>
</tr>
</tbody>
</table>

## Cookbooks

{{% cookbooks_summary %}}

The Chef Infra Client uses Ruby as its reference language for creating
cookbooks and defining recipes, with an extended DSL for specific
resources. A reasonable set of resources are available to the Chef Infra
Client, enough to support many of the most common infrastructure
automation scenarios; however, this DSL can also be extended when
additional resources and capabilities are required.

### Components

Cookbooks are comprised of the following components:

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
<td><img src="/images/icon_cookbook_attributes.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="cookbooks_attribute.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_cookbook_files.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="resource_cookbook_file_summary.md" >}}</td>
</tr>
<tr class="odd">
<td><img src="/images/icon_cookbook_libraries.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="libraries_summary.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_cookbook_metadata.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="cookbooks_metadata.md" >}}</td>
</tr>
<tr class="odd">
<td><p><img src="/images/icon_cookbook_recipes.svg" class="align-center" width="100" alt="image" /></p>
<p><img src="/images/icon_recipe_dsl.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>{{< readFile_shortcode file="cookbooks_recipe.md" >}}</p>
<p>The Chef Infra Client will run a recipe only when asked. When the Chef Infra Client runs the same recipe more than once, the results will be the same system state each time. When a recipe is run against a system, but nothing has changed on either the system or in the recipe, the Chef Infra Client won't change anything.</p>
<p>{{< readFile_shortcode file="dsl_recipe_summary.md" >}}</p></td>
</tr>
<tr class="even">
<td><p><img src="/images/icon_cookbook_resources.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>{{< readFile_shortcode file="resources_common.md" >}}</p>
<p>Chef has <a href="/resources/">many built-in resources</a> that cover all of the most common actions across all of the most common platforms. You can <a href="/custom_resources/">build your own resources</a> to handle any situation that isn't covered by a built-in resource.</p></td>
</tr>
<tr class="odd">
<td><img src="/images/icon_cookbook_templates.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="template.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_cookbook_tests.svg" class="align-center" width="100" alt="image" /></td>
<td>Testing cookbooks improves the quality of those cookbooks by ensuring they are doing what they are supposed to do and that they are authored in a consistent manner. Unit and integration testing validates the recipes in cookbooks. Syntax testing---often called linting---validates the quality of the code itself. The following tools are popular tools used for testing Chef recipes: Test Kitchen, ChefSpec, and Cookstyle.</td>
</tr>
</tbody>
</table>

## Nodes

{{% node %}}

### Node Types

{{% node_types %}}

### Chef on Nodes

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

## The Chef Infra Server

{{% chef_server %}}

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Feature</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><img src="/images/icon_search.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="search.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_manage.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="chef_manager.md" >}}</td>
</tr>
<tr class="odd">
<td><img src="/images/icon_data_bags.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="data_bag.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_policy.svg" class="align-center" width="100" alt="image" /></td>
<td>Policy defines how business and operational requirements, processes, and production workflows map to objects that are stored on the Chef Infra Server. Policy objects on the Chef Infra Server include roles, environments, and cookbook versions.</td>
</tr>
</tbody>
</table>

### Policy

{{% policy_summary %}}

Some important aspects of policy include:

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Feature</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><img src="/images/icon_roles.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="role.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_environments.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="environment.md" >}}</td>
</tr>
<tr class="odd">
<td><img src="/images/icon_cookbook_versions.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="cookbooks_version.md" >}}</td>
</tr>
<tr class="even">
<td><img src="/images/icon_run_lists.svg" class="align-center" width="100" alt="image" /></td>
<td>{{< readFile_shortcode file="node_run_list.md" >}}</td>
</tr>
</tbody>
</table>

## Conclusion

Chef is a thin DSL (domain-specific language) built on top of Ruby. This
approach allows Chef to provide just enough abstraction to make
reasoning about your infrastructure easy. Chef includes a built-in
taxonomy of all the basic resources one might configure on a system,
plus a defined mechanism to extend that taxonomy using the full power of
the Ruby language. Ruby was chosen because it provides the flexibility
to use both the simple built-in taxonomy, as well as being able to
handle any customization path your organization requires.
