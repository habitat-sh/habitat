+++
title = "About Cookbooks"
draft = false

aliases = ["/cookbooks.html"]

[menu]
  [menu.infra]
    title = "About Cookbooks"
    identifier = "chef_infra/cookbook_reference/cookbooks.md About Cookbooks"
    parent = "chef_infra/cookbook_reference"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/cookbooks.md)

{{% cookbooks_summary %}}

Chef Infra Client uses Ruby as its reference language for creating
cookbooks and defining recipes, with an extended DSL for specific
resources. Chef Infra Client provides a reasonable set of resources,
enough to support many of the most common infrastructure automation
scenarios; however, this DSL can also be extended when additional
resources and capabilities are required.

Chef Infra Client will run a recipe only when asked. When Chef Infra
Client runs the same recipe more than once, the results will be the same
system state each time. When a recipe is run against a system, but
nothing has changed on either the system or in the recipe, Chef Infra
Client won't change anything.

## Components

A cookbook is comprised of recipes and other optional components as
files or directories.

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 8%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Component</th>
<th>File/Directory Name</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="/recipes/">Recipes</a></td>
<td>recipes/</td>
<td>{{< readFile_shortcode file="cookbooks_recipe.md" >}}</td>
</tr>
<tr class="even">
<td><a href="/attributes/">Attributes</a></td>
<td>attributes/</td>
<td>{{< readFile_shortcode file="cookbooks_attribute.md" >}}</td>
</tr>
<tr class="odd">
<td><a href="/files/">Files</a></td>
<td>files/</td>
<td>A file distribution is a specific type of resource that tells a cookbook how to distribute files, including by node, by platform, or by file version.</td>
</tr>
<tr class="even">
<td><a href="/libraries/">Libraries</a></td>
<td>libraries/</td>
<td>A library allows the use of arbitrary Ruby code in a cookbook, either as a way to extend the Chef Infra Client language or to implement a new class.</td>
</tr>
<tr class="odd">
<td><a href="/custom_resources/">Custom Resources</a></td>
<td>resources/</td>
<td>A custom resource is an abstract approach for defining a set of actions and (for each action) a set of properties and validation parameters.</td>
</tr>
<tr class="even">
<td><a href="/templates/">Templates</a></td>
<td>templates/</td>
<td>A template is a file written in markup language that uses Ruby statements to solve complex configuration scenarios.</td>
</tr>
<tr class="odd">
<td><a href="/ohai_custom/">Ohai Plugins</a></td>
<td>ohai/</td>
<td>Custom Ohai plugins can be written to load additional information about your nodes to be used in recipes. This requires Chef Infra Server 12.18.14 or later.</td>
</tr>
<tr class="even">
<td><a href="/config_rb_metadata/">Metadata</a></td>
<td>metadata.rb</td>
<td>This file contains information about the cookbook such as the cookbook name, description, and <a href="/cookbook_versioning/">version</a>.</td>
</tr>
</tbody>
</table>

## Community Cookbooks

Chef maintains a large collection of cookbooks. In addition, there are
thousands of cookbooks created and maintained by the community:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Components</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="https://github.com/chef-cookbooks">Cookbooks Maintained by Chef</a></td>
<td>Chef maintains a collection of cookbooks that are widely used by the community.</td>
</tr>
<tr class="even">
<td><a href="https://supermarket.chef.io/cookbooks">Cookbooks Maintained by the Community</a></td>
<td>The community has authored thousands of cookbooks, ranging from niche cookbooks that are used by only a few organizations to cookbooks that are some of the most popular and are used by nearly everyone.</td>
</tr>
</tbody>
</table>
