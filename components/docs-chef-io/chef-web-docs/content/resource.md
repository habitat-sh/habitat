+++
title = "About Resources"
draft = false

aliases = ["/resource.html"]

[menu]
  [menu.infra]
    title = "About Resources"
    identifier = "chef_infra/cookbook_reference/resources/resource.md About Resources"
    parent = "chef_infra/cookbook_reference/resources"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/resource.md)

{{% resources_common %}}

## Resource Syntax

A resource is a Ruby block with four components: a type, a name, one (or
more) properties (with values), and one (or more) actions. The syntax
for a resource is like this:

``` ruby
type 'name' do
   attribute 'value'
   action :type_of_action
end
```

Every resource has its own set of actions and properties. Most
properties have default values. Some properties are available to all
resources, for example those used to send notifications to other
resources and guards that help ensure that some resources are
idempotent.

For example, a resource that is used to install a tar.gz package for
version 1.16.1 may look something like this:

``` ruby
package 'tar' do
  version '1.16.1'
  action :install
end
```

All actions have a default value. Only non-default behaviors of actions
and properties need to be specified. For example, the **package**
resource's default action is `:install` and the name of the package
defaults to the `name` of the resource. Therefore, it is possible to
write a resource block that installs the latest tar.gz package like
this:

``` ruby
package 'tar'
```

and a resource block that installs a tar.gz package for version 1.6.1
like this:

``` ruby
package 'tar' do
  version '1.16.1'
end
```

In both cases, Chef Infra Client will use the default action
(`:install`) to install the `tar` package.

## Additional Information

See these guides for additional information about resources:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Topic</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="/resource_common/">Common Properties</a></td>
<td>Provides a detailed list of the common properties that are available in all resources.</td>
</tr>
<tr class="even">
<td><a href="/resources/">Resource Reference</a></td>
<td>A reference guide that lists both the common and individual options available to every resource that is bundled into Chef.</td>
</tr>
<tr class="odd">
<td><a href="/custom_resources/">Custom Resources</a></td>
<td>Shows you how to create your own Chef resources.</td>
</tr>
</tbody>
</table>
