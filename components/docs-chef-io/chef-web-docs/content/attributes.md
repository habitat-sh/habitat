+++
title = "About Attributes"
draft = false

aliases = ["/attributes.html"]

[menu]
  [menu.infra]
    title = "Attributes"
    identifier = "chef_infra/cookbook_reference/attributes.md Attributes"
    parent = "chef_infra/cookbook_reference"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/attributes.md)

{{% node_attribute %}}

So how does Chef Infra Client determine which value should be applied?
Keep reading to learn more about how attributes work, including more
about the types of attributes, where attributes are saved, and how Chef
Infra Client chooses which attribute to apply.

## Attribute Persistence

{{% node_attribute_persistence %}}

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

## Attribute Sources

Attributes are provided to Chef Infra Client from the following
locations:

-   JSON files passed via the `chef-client -j`
-   Nodes (collected by Ohai at the start of each Chef Infra Client run)
-   Attribute files (in cookbooks)
-   Recipes (in cookbooks)
-   Environments
-   Roles
-   Policyfiles

Notes:

-   Many attributes are maintained in the chef-repo for Policyfiles,
    environments, roles, and cookbooks (attribute files and recipes)
-   Many attributes are collected by Ohai on each individual node at the
    start of every Chef Infra Client run
-   The attributes that are maintained in the chef-repo are uploaded to
    the Chef Infra Server from the workstation, periodically
-   Chef Infra Client will pull down the node object from the Chef Infra
    Server and then reset all the attributes except `normal`. The node
    object will contain the attribute data from the previous Chef Infra
    Client run including attributes set with JSON files via `-j`.
-   Chef Infra Client will update the cookbooks on the node (if
    required), which updates the attributes contained in attribute files
    and recipes
-   Chef Infra Client will update the role and environment data (if
    required)
-   Chef Infra Client will rebuild the attribute list and apply
    attribute precedence while configuring the node
-   Chef Infra Client pushes the node object to the Chef Infra Server at
    the end of a Chef Infra Client run; the updated node object on the
    Chef Infra Server is then indexed for search and is stored until the
    next Chef Infra Client run

### Automatic (Ohai)

{{% ohai_automatic_attribute %}}

{{% ohai_attribute_list %}}

### Attribute Files

An attribute file is located in the `attributes/` sub-directory for a
cookbook. When a cookbook is run against a node, the attributes
contained in all attribute files are evaluated in the context of the
node object. Node methods (when present) are used to set attribute
values on a node. For example, the `apache2` cookbook contains an
attribute file called `default.rb`, which contains the following
attributes:

``` ruby
default['apache']['dir']          = '/etc/apache2'
default['apache']['listen_ports'] = [ '80','443' ]
```

The use of the node object (`node`) is implicit in the previous example;
the following example defines the node object itself as part of the
attribute:

``` ruby
node.default['apache']['dir']          = '/etc/apache2'
node.default['apache']['listen_ports'] = [ '80','443' ]
```

### Attribute Evaluation Order

{{% node_attribute_evaluation_order %}}

#### Use Attribute Files

{{% node_attribute_when_to_use %}}

{{% node_attribute_when_to_use_unless_variants %}}

**File Methods**

{{% cookbooks_attribute_file_methods %}}

**attribute?**

A useful method that is related to attributes is the `attribute?`
method. This method will check for the existence of an attribute, so
that processing can be done in an attributes file or recipe, but only if
a specific attribute exists.

Using `attribute?()` in an attributes file:

``` ruby
if attribute?('ec2')
  # ... set stuff related to EC2
end
```

Using `attribute?()` in a recipe:

``` ruby
if node.attribute?('ec2')
  # ... do stuff on EC2 nodes
end
```

### Recipes

{{% cookbooks_recipe %}}

{{% cookbooks_attribute %}}

### Roles

{{% role %}}

{{% role_attribute %}}

### Environments

{{% environment %}}

{{% environment_attribute %}}

## Attribute Precedence

{{% node_attribute_precedence %}}

### Blacklist Attributes

{{% node_attribute_blacklist %}}

### Whitelist Attributes

{{% node_attribute_whitelist %}}

### Examples

The following examples are listed from low to high precedence.

**Default attribute in /attributes/default.rb**

``` ruby
default['apache']['dir'] = '/etc/apache2'
```

**Default attribute in node object in recipe**

``` ruby
node.default['apache']['dir'] = '/etc/apache2'
```

**Default attribute in /environments/environment_name.rb**

``` ruby
default_attributes({ 'apache' => {'dir' => '/etc/apache2'}})
```

**Default attribute in /roles/role_name.rb**

``` ruby
default_attributes({ 'apache' => {'dir' => '/etc/apache2'}})
```

**Normal attribute set as a cookbook attribute**

``` ruby
set['apache']['dir'] = '/etc/apache2'
normal['apache']['dir'] = '/etc/apache2'  #set is an alias of normal.
```

**Normal attribute set in a recipe**

``` ruby
node.normal['apache']['dir'] = '/etc/apache2'
```

**Override attribute in /attributes/default.rb**

``` ruby
override['apache']['dir'] = '/etc/apache2'
```

**Override attribute in /roles/role_name.rb**

``` ruby
override_attributes({ 'apache' => {'dir' => '/etc/apache2'}})
```

**Override attribute in /environments/environment_name.rb**

``` ruby
override_attributes({ 'apache' => {'dir' => '/etc/apache2'}})
```

**Override attribute in a node object (from a recipe)**

``` ruby
node.override['apache']['dir'] = '/etc/apache2'
```

**Ensure that a default attribute has precedence over other attributes**

When a default attribute is set like this:

``` ruby
default['attribute'] = 'value'
```

any value set by a role or an environment will replace it. To prevent
this value from being replaced, use the `force_default` attribute
precedence:

``` ruby
force_default['attribute'] = 'I will crush you, role or environment attribute'
```

or:

``` ruby
default!['attribute'] = "The '!' means I win!"
```

**Ensure that an override attribute has precedence over other
attributes**

When an override attribute is set like this:

``` ruby
override['attribute'] = 'value'
```

any value set by a role or an environment will replace it. To prevent
this value from being replaced, use the `force_override` attribute
precedence:

``` ruby
force_override['attribute'] = 'I will crush you, role or environment attribute'
```

or:

``` ruby
override!['attribute'] = "The '!' means I win!"
```

## Change Attributes

Attribute precedence levels may be:

-   Removed for a specific, named attribute precedence level.
-   Removed for all attribute precedence levels.
-   Fully assigned attributes.

### Remove Precedence Level

A specific attribute precedence level for default, normal, and override
attributes may be removed by using one of the following syntax patterns.

For default attributes:

-   `node.rm_default('foo', 'bar')`

For normal attributes:

-   `node.rm_normal('foo', 'bar')`

For override attributes:

-   `node.rm_override('foo', 'bar')`

These patterns return the computed value of the key being deleted for
the specified precedence level.

#### Examples

The following examples show how to remove a specific, named attribute
precedence level.

**Delete a default value when only default values exist**

Given the following code structure under `'foo'`:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

And some role attributes:

``` ruby
# Please don't ever do this in real code :)
node.role_default['foo']['bar']['thing'] = 'otherstuff'
```

And a force attribute:

``` ruby
node.force_default['foo']['bar']['thing'] = 'allthestuff'
```

When the default attribute precedence `node['foo']['bar']` is removed:

``` ruby
node.rm_default('foo', 'bar') #=> {'baz' => 52, 'thing' => 'allthestuff'}
```

What is left under `'foo'` is only `'bat'`:

``` ruby
node.attributes.combined_default['foo'] #=> {'bat' => { 'things' => [5,6] } }
```

**Delete default without touching higher precedence attributes**

Given the following code structure:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

And some role attributes:

``` ruby
# Please don't ever do this in real code :)
node.role_default['foo']['bar']['thing'] = 'otherstuff'
```

And a force attribute:

``` ruby
node.force_default['foo']['bar']['thing'] = 'allthestuff'
```

And also some override attributes:

``` ruby
node.override['foo']['bar']['baz'] = 99
```

Same delete as before:

``` ruby
node.rm_default('foo', 'bar') #=> { 'baz' => 52, 'thing' => 'allthestuff' }
```

The other attribute precedence levels are unaffected:

``` ruby
node.attributes.combined_override['foo'] #=> { 'bar' => {'baz' => 99} }
node['foo'] #=> { 'bar' => {'baz' => 99}, 'bat' => { 'things' => [5,6] }
```

**Delete override without touching lower precedence attributes**

Given the following code structure, which has an override attribute:

``` ruby
node.override['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

with a single default value:

``` ruby
node.default['foo']['bar']['baz'] = 11
```

and a force at each attribute precedence:

``` ruby
node.force_default['foo']['bar']['baz'] = 55
node.force_override['foo']['bar']['baz'] = 99
```

Delete the override:

``` ruby
node.rm_override('foo', 'bar') #=> { 'baz' => 99, 'thing' => 'stuff' }
```

The other attribute precedence levels are unaffected:

``` ruby
node.attributes.combined_default['foo'] #=> { 'bar' => {'baz' => 55} }
```

**Non-existent key deletes return nil**

``` ruby
node.rm_default("no", "such", "thing") #=> nil
```

### Remove All Levels

All attribute precedence levels may be removed by using the following
syntax pattern:

-   `node.rm('foo', 'bar')`

{{< note >}}

Using `node['foo'].delete('bar')` will throw an exception that points to
the new API.

{{< /note >}}

#### Examples

The following examples show how to remove all attribute precedence
levels.

**Delete all attribute precedence levels**

Given the following code structure:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
    'things' => [5, 6],
  },
}
```

With override attributes:

``` ruby
node.override['foo']['bar']['baz'] = 999
```

Removing the `'bar'` key returns the computed value:

``` ruby
node.rm('foo', 'bar') #=> {'baz' => 999, 'thing' => 'stuff'}
```

Looking at `'foo'`, all that's left is the `'bat'` entry:

``` ruby
node['foo'] #=> {'bat' => { 'things' => [5,6] } }
```

**Non-existent key deletes return nil**

``` ruby
node.rm_default("no", "such", "thing") #=> nil
```

### Full Assignment

Use `!` to clear out the key for the named attribute precedence level,
and then complete the write by using one of the following syntax
patterns:

-   `node.default!['foo']['bar'] = {...}`
-   `node.force_default!['foo']['bar'] = {...}`
-   `node.normal!['foo']['bar'] = {...}`
-   `node.override!['foo']['bar'] = {...}`
-   `node.force_override!['foo']['bar'] = {...}`

#### Examples

The following examples show how to remove all attribute precedence
levels.

**Just one component**

Given the following code structure:

``` ruby
node.default['foo']['bar'] = {'a' => 'b'}
node.default!['foo']['bar'] = {'c' => 'd'}
```

The `'!'` caused the entire 'bar' key to be overwritten:

``` ruby
node['foo'] #=> {'bar' => {'c' => 'd'}
```

**Multiple components; one "after"**

Given the following code structure:

``` ruby
node.default['foo']['bar'] = {'a' => 'b'}
# Please don't ever do this in real code :)
node.role_default['foo']['bar'] = {'c' => 'd'}
node.default!['foo']['bar'] = {'d' => 'e'}
```

The `'!'` write overwrote the "cookbook-default" value of `'bar'`, but
since role data is later in the resolution list, it was unaffected:

``` ruby
node['foo'] #=> {'bar' => {'c' => 'd', 'd' => 'e'}
```

**Multiple components; all "before"**

Given the following code structure:

``` ruby
node.default['foo']['bar'] = {'a' => 'b'}
# Please don't ever do this in real code :)
node.role_default['foo']['bar'] = {'c' => 'd'}
node.force_default!['foo']['bar'] = {'d' => 'e'}
```

With `force_default!` there is no other data under `'bar'`:

``` ruby
node['foo'] #=> {'bar' => {'d' => 'e'}
```

**Multiple precedence levels**

Given the following code structure:

``` ruby
node.default['foo'] = {
  'bar' => {
    'baz' => 52,
    'thing' => 'stuff',
  },
  'bat' => {
   'things' => [5, 6],
  },
}
```

And some attributes:

``` ruby
# Please don't ever do this in real code :)
node.role_default['foo']['bar']['baz'] = 55
node.force_default['foo']['bar']['baz'] = 66
```

And other precedence levels:

``` ruby
node.normal['foo']['bar']['baz'] = 88
node.override['foo']['bar']['baz'] = 99
```

With a full assignment:

``` ruby
node.default!['foo']['bar'] = {}
```

Role default and force default are left in default, plus other
precedence levels:

``` ruby
node.attributes.combined_default['foo'] #=> {'bar' => {'baz' => 66}, 'bat'=>{'things'=>[5, 6]}}
node.attributes.normal['foo'] #=> {'bar' => {'baz' => 88}}
node.attributes.combined_override['foo'] #=> {'bar' => {'baz' => 99}}
node['foo']['bar'] #=> {'baz' => 99}
```

If `force_default!` is written:

``` ruby
node.force_default!['foo']['bar'] = {}
```

the difference is:

``` ruby
node.attributes.combined_default['foo'] #=> {'bat'=>{'things'=>[5, 6]}, 'bar' => {}}
node.attributes.normal['foo'] #=> {'bar' => {'baz' => 88}}
node.attributes.combined_override['foo'] #=> {'bar' => {'baz' => 99}}
node['foo']['bar'] #=> {'baz' => 99}
```

## About Deep Merge

Attributes are typically defined in cookbooks, recipes, roles, and
environments. These attributes are rolled-up to the node level during a
Chef Infra Client run. A recipe can store attribute values using a
multi-level hash or array.

For example, a group of attributes for web servers might be:

``` ruby
override_attributes(
  :apache => {
    :listen_ports => [ 80 ],
    :prefork => {
      :startservers => 20,
      :minspareservers => 20,
      :maxspareservers => 40
    }
  }
)
```

But what if all of the web servers are not the same? What if some of the
web servers required a single attribute to have a different value? You
could store these settings in two locations, once just like the
preceding example and once just like the following:

``` ruby
override_attributes(
  :apache => {
    :listen_ports => [ 80 ],
    :prefork => {
      :startservers => 30,
      :minspareservers => 20,
      :maxspareservers => 40
    }
  }
)
```

But that is not very efficient, especially because most of them are
identical. The deep merge capabilities of Chef Infra Client allows
attributes to be layered across cookbooks, recipes, roles, and
environments. This allows an attribute to be reused across nodes, making
use of default attributes set at the cookbook level, but also providing
a way for certain attributes (with a higher attribute precedence) to be
applied only when they are supposed to be.

For example, a role named `baseline.rb`:

``` ruby
name "baseline"
description "The most basic role for all configurations"
run_list "recipe[baseline]"

override_attributes(
  :apache => {
    :listen_ports => [ 80 ],
    :prefork => {
      :startservers => 20,
      :minspareservers => 20,
      :maxspareservers => 40
    }
  }
)
```

and then a role named `web.rb`:

``` ruby
name 'web'
description 'Web server config'
run_list 'role[baseline]'

override_attributes(
  :apache => {
    :prefork => {
      :startservers => 30
    }
  }
)
```

Both of these files are similar because they share the same structure.
When an attribute value is a hash, that data is merged. When an
attribute value is an array, if the attribute precedence levels are the
same, then that data is merged. If the attribute value precedence levels
in an array are different, then that data is replaced. For all other
value types (such as strings, integers, etc.), that data is replaced.

For example, the `web.rb` references the `baseline.rb` role. The
`web.rb` file only provides a value for one attribute: `:startservers`.
When Chef Infra Client compares these attributes, the deep merge feature
will ensure that `:startservers` (and its value of `30`) will be applied
to any node for which the `web.rb` attribute structure should be
applied.

This approach will allow a recipe like this:

``` ruby
include_recipe 'apache2'
Chef::Log.info(node['apache']['prefork'].to_hash)
```

and a `run_list` like this:

``` ruby
run_list/web.json
{
  "run_list": [ "role[web]" ]
}
```

to produce results like this:

``` ruby
[Tue, 16 Aug 2011 14:44:26 -0700] INFO:
         {
           "startservers"=>30,
           "minspareservers"=>20,
           "maxspareservers"=>40,
           "serverlimit"=>400,
           "maxclients"=>400,
           "maxrequestsperchild"=>10000
         }
```

Even though the `web.rb` file does not contain attributes and values for
`minspareservers`, `maxspareservers`, `serverlimit`, `maxclients`, and
`maxrequestsperchild`, the deep merge capabilities pulled them in.

The following sections show how the logic works for using deep merge to
perform substitutions and additions of attributes.

### Substitution

The following examples show how the logic works for substituting an
existing string using a hash:

    role_or_environment 1 { :x => '1', :y => '2' }
    +
    role_or_environment 2 { :y => '3' }
    =
    { :x => '1', :y => '3' }

For substituting an existing boolean using a hash:

    role_or_environment 1 { :x => true, :y => false }
    +
    role_or_environment 2 { :y => true }
    =
    { :x => true, :y => true }

For substituting an array with a hash:

    role_or_environment 1 [ '1', '2', '3' ]
    +
    role_or_environment 2 { :x => '1' , :y => '2' }
    =
    { :x => '1', :y => '2' }

When items cannot be merged through substitution, the original data is
overwritten.

### Addition

The following examples show how the logic works for adding a string
using a hash:

    role_or_environment 1 { :x => '1', :y => '2' }
    +
    role_or_environment 2 { :z => '3' }
    =
    { :x => '1', :y => '2', :z => '3' }

For adding a string using an array:

    role_or_environment 1 [ '1', '2' ]
    +
    role_or_environment 2 [ '3' ]
    =
    [ '1', '2', '3' ]

For adding a string using a multi-level hash:

    role_or_environment 1 { :x => { :y => '2' } }
    +
    role_or_environment 2 { :x => { :z => '3' } }
    =
    { :x => { :y => '2', :z => '3' } }

For adding a string using a multi-level array:

    role_or_environment 1 [ [ 1, 2 ] ]
    +
    role_or_environment 2 [ [ 3 ] ]
    =
    [ [ 1, 2 ], [ 3 ] ]
