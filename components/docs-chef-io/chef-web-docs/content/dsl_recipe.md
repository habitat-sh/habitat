+++
title = "About the Recipe DSL"
draft = false

aliases = ["/dsl_recipe.html"]

[menu]
  [menu.infra]
    title = "DSL Overview"
    identifier = "chef_infra/cookbook_reference/recipe_dsl/dsl_recipe.md DSL Overview"
    parent = "chef_infra/cookbook_reference/recipe_dsl"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/dsl_recipe.md)

{{% dsl_recipe_summary %}}

Because the Recipe DSL is a Ruby DSL, anything that can be done using
Ruby can also be done in a recipe or custom resource, including `if` and
`case` statements, using the `include?` Ruby method, including recipes
in recipes, and checking for dependencies. See the [Ruby
Guide](/ruby/) for further information on built-in Ruby
functionality.

## Include Recipes

{{% cookbooks_recipe_include_in_recipe %}}

### Reload Attributes

{{% cookbooks_attribute_file_reload_from_recipe %}}

## Recipe DSL Methods

The Recipe DSL provides support for using attributes, data bags (and
encrypted data), and search results in a recipe, as well as four helper
methods that can be used to check for a node's platform from the recipe
to ensure that specific actions are taken for specific platforms. The
helper methods are:

-   `platform?`
-   `platform_family?`
-   `value_for_platform`
-   `value_for_platform_family`

### attribute?

Use the `attribute?` method to ensure that certain actions only execute
in the presence of a particular node attribute. The `attribute?` method
will return true if one of the listed node attributes matches a node
attribute that is detected by Ohai during every Chef Infra Client run.

The syntax for the `attribute?` method is as follows:

``` ruby
attribute?('name_of_attribute')
```

For example:

``` ruby
if node.attribute?('ipaddress')
  # the node has an ipaddress
end
```

### cookbook_name

Use the `cookbook_name` method to return the name of a cookbook.

The syntax for the `cookbook_name` method is as follows:

``` ruby
cookbook_name
```

This method is often used as part of a log entry. For example:

``` ruby
Chef::Log.info('I am a message from the #{recipe_name} recipe in the #{cookbook_name} cookbook.')
```

### data_bag

{{% data_bag %}}

Use the `data_bag` method to get a list of the contents of a data bag.

The syntax for the `data_bag` method is as follows:

``` ruby
data_bag(bag_name)
```

**Examples**

The following example shows how the `data_bag` method can be used in a
recipe.

**Get a data bag, and then iterate through each data bag item**

{{% dsl_recipe_data_bag %}}

### data_bag_item

{{% data_bag %}}

The `data_bag_item` method can be used in a recipe to get the contents
of a data bag item.

The syntax for the `data_bag_item` method is as follows:

``` ruby
data_bag_item(bag_name, item, secret)
```

where `secret` is the secret used to load an encrypted data bag. If
`secret` is not specified, Chef Infra Client looks for a secret at the
path specified by the `encrypted_data_bag_secret` setting in the
client.rb file.

**Examples**

The following examples show how the `data_bag_item` method can be used
in a recipe.

**Get a data bag, and then iterate through each data bag item**

{{% dsl_recipe_data_bag %}}

**Use the contents of a data bag in a recipe**

The following example shows how to use the `data_bag` and
`data_bag_item` methods in a recipe, also using a data bag named
`sea-power`):

``` ruby
package 'sea-power' do
  action :install
end

directory node['sea-power']['base_path'] do
  # attributes for owner, group, mode
end

gale_warnings = data_bag('sea-power').map do |viking_north|
  data_bag_item('sea-power', viking_north)['source']
end

template '/etc/seattle/power.list' do
  source 'seattle-power.erb'
  # attributes for owner, group, mode
  variables(
    :base_path => node['sea-power']['base_path'],
    # more variables
    :repo_location => gale_warnings
  )
end
```

For a more complete version of the previous example, see the default
recipe in the <https://github.com/hw-cookbooks/apt-mirror> community
cookbook.

### declare_resource

Use the `declare_resource` method to instantiate a resource and then add
it to the resource collection.

The syntax for the `declare_resource` method is as follows:

``` ruby
declare_resource(:resource_type, 'resource_name', resource_attrs_block)
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.
-   `resource_attrs_block` is a block in which properties of the
    instantiated resource are declared.

For example:

``` ruby
declare_resource(:file, '/x/y.txy', caller[0]) do
  action :delete
end
```

is equivalent to:

``` ruby
file '/x/y.txt' do
  action :delete
end
```

### delete_resource

Use the `delete_resource` method to find a resource in the resource
collection, and then delete it.

The syntax for the `delete_resource` method is as follows:

``` ruby
delete_resource(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
delete_resource(:template, '/x/y.erb')
```

## delete_resource!


Use the `delete_resource!` method to find a resource in the resource
collection, and then delete it. If the resource is not found, an
exception is returned.

The syntax for the `delete_resource!` method is as follows:

``` ruby
delete_resource!(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
delete_resource!(:file, '/x/file.txt')
```

### edit_resource

Use the `edit_resource` method to:

-   Find a resource in the resource collection, and then edit it.
-   Define a resource block. If a resource block with the same name
    exists in the resource collection, it will be updated with the
    contents of the resource block defined by the `edit_resource`
    method. If a resource block does not exist in the resource
    collection, it will be created.

The syntax for the `edit_resource` method is as follows:

``` ruby
edit_resource(:resource_type, 'resource_name', resource_attrs_block)
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.
-   `resource_attrs_block` is a block in which properties of the
    instantiated resource are declared.

For example:

``` ruby
edit_resource(:template, '/x/y.txy') do
  cookbook 'cookbook_name'
end
```

and a resource block:

``` ruby
edit_resource(:template, '/etc/aliases') do
  source 'aliases.erb'
  cookbook 'aliases'
  variables({:aliases => {} })
  notifies :run, 'execute[newaliases]'
end
```

## edit_resource!

Use the `edit_resource!` method to:

-   Find a resource in the resource collection, and then edit it.
-   Define a resource block. If a resource with the same name exists in
    the resource collection, its properties will be updated with the
    contents of the resource block defined by the `edit_resource`
    method.

In both cases, if the resource is not found, an exception is returned.

The syntax for the `edit_resource!` method is as follows:

``` ruby
edit_resource!(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.
-   `resource_attrs_block` is a block in which properties of the
    instantiated resource are declared.

For example:

``` ruby
edit_resource!(:file, '/x/y.rst')
```

### find_resource

Use the `find_resource` method to:

-   Find a resource in the resource collection.
-   Define a resource block. If a resource block with the same name
    exists in the resource collection, it will be returned. If a
    resource block does not exist in the resource collection, it will be
    created.

The syntax for the `find_resource` method is as follows:

``` ruby
find_resource(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
find_resource(:template, '/x/y.txy')
```

and a resource block:

``` ruby
find_resource(:template, '/etc/seapower') do
  source 'seapower.erb'
  cookbook 'seapower'
  variables({:seapower => {} })
  notifies :run, 'execute[newseapower]'
end
```

## find_resource!

Use the `find_resource!` method to find a resource in the resource
collection. If the resource is not found, an exception is returned.

The syntax for the `find_resource!` method is as follows:

``` ruby
find_resource!(:resource_type, 'resource_name')
```

where:

-   `:resource_type` is the resource type, such as `:file` (for the
    **file** resource), `:template` (for the **template** resource), and
    so on. Any resource available to Chef may be declared.
-   `resource_name` the property that is the default name of the
    resource, typically the string that appears in the
    `resource 'name' do` block of a resource (but not always); see the
    Syntax section for the resource to be declared to verify the default
    name property.

For example:

``` ruby
find_resource!(:template, '/x/y.erb')
```

### platform?

Use the `platform?` method to ensure that certain actions are run for
specific platform. The `platform?` method will return true if one of the
listed parameters matches the `node['platform']` attribute that is
detected by Ohai during every Chef Infra Client run.

The syntax for the `platform?` method is as follows:

``` ruby
platform?('parameter', 'parameter')
```

where:

-   `parameter` is a comma-separated list, each specifying a platform,
    such as Red Hat, CentOS, or Fedora
-   `platform?` method is typically used with an `if`, `elsif`, or
    `case` statement that contains Ruby code that is specific for the
    platform, if detected

#### Parameters

The following parameters can be used with this method:

<table>
<colgroup>
<col style="width: 16%" />
<col style="width: 83%" />
</colgroup>
<thead>
<tr class="header">
<th>Parameter</th>
<th>Platforms</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>aix</code></td>
<td>AIX. All platform variants of AIX return <code>aix</code>.</td>
</tr>
<tr class="even">
<td><code>amazon</code></td>
<td>Amazon Linux</td>
</tr>
<tr class="odd">
<td><code>arch</code></td>
<td>Arch Linux</td>
</tr>
<tr class="even">
<td><code>debian</code></td>
<td>Debian</td>
</tr>
<tr class="odd">
<td><code>fedora</code></td>
<td>Fedora</td>
</tr>
<tr class="even">
<td><code>freebsd</code></td>
<td>FreeBSD. All platform variants of FreeBSD return <code>freebsd</code>.</td>
</tr>
<tr class="odd">
<td><code>gentoo</code></td>
<td>Gentoo</td>
</tr>
<tr class="even">
<td><code>mac_os_x</code></td>
<td>macOS</td>
</tr>
<tr class="odd">
<td><code>netbsd</code></td>
<td>NetBSD. All platform variants of NetBSD return <code>netbsd</code>.</td>
</tr>
<tr class="even">
<td><code>openbsd</code></td>
<td>OpenBSD. All platform variants of OpenBSD return <code>openbsd</code>.</td>
</tr>
<tr class="odd">
<td><code>opensuseleap</code></td>
<td>openSUSE leap</td>
</tr>
<tr class="even">
<td><code>slackware</code></td>
<td>Slackware</td>
</tr>
<tr class="odd">
<td><code>solaris</code></td>
<td>Solaris. For Solaris-related platforms, the <code>platform_family</code> method does not support the Solaris platform family and will default back to <code>platform_family = platform</code>. For example, if the platform is OmniOS, the <code>platform_family</code> is <code>omnios</code>, if the platform is SmartOS, the <code>platform_family</code> is <code>smartos</code>, and so on. All platform variants of Solaris return <code>solaris</code>.</td>
</tr>
<tr class="even">
<td><code>suse</code></td>
<td>SUSE Enterprise Linux Server.</td>
</tr>
<tr class="odd">
<td><code>ubuntu</code></td>
<td>Ubuntu Linux.</td>
</tr>
<tr class="even">
<td><code>windows</code></td>
<td>Microsoft Windows. All platform variants of Microsoft Windows return <code>windows</code>.</td>
</tr>
</tbody>
</table>

{{< note >}}

Ohai collects platform information at the start of a Chef Infra Client
run and stores that information in the `node['platform']` attribute.

{{< /note >}}

For example:

``` ruby
platform?('debian')
```

or:

``` ruby
platform?('redhat', 'debian')
```

#### Examples

The following example shows how the `platform?` method can be used in a
recipe.

**Use an if statement with the platform recipe DSL method**

{{% resource_ruby_block_if_statement_use_with_platform %}}

### platform_family?

Use the `platform_family?` method to ensure that certain actions are run
for specific platform family. The `platform_family?` method will return
true if one of the listed parameters matches the
`node['platform_family']` attribute that is detected by Ohai during
every Chef Infra Client run.

The syntax for the `platform_family?` method is as follows:

``` ruby
platform_family?('parameter', 'parameter')
```

where:

-   `'parameter'` is a comma-separated list, each specifying a platform
    family, such as Debian, or Red Hat Enterprise Linux
-   `platform_family?` method is typically used with an `if`, `elsif`,
    or `case` statement that contains Ruby code that is specific for the
    platform family, if detected

For example:

``` ruby
if platform_family?('rhel')
  # do RHEL things
end
```

or:

``` ruby
if platform_family?('debian', 'rhel')
  # do things on debian and rhel families
end
```

For example:

``` ruby
platform_family?('gentoo')
```

or:

``` ruby
platform_family?('slackware', 'suse', 'arch')
```

{{< note >}}

`platform_family?` will default to `platform?` when `platform_family?`
is not explicitly defined.

{{< /note >}}

#### Examples

The following examples show how the `platform_family?` method can be
used in a recipe.

**Use a specific binary for a specific platform**

{{< readFile_shortcode file="resource_remote_file_use_platform_family.md" >}}

### reboot_pending?

Use the `reboot_pending?` method to test if a node needs a reboot, or is
expected to reboot. `reboot_pending?` returns `true` when the node needs
a reboot.

The syntax for the `reboot_pending?` method is as follows:

``` ruby
reboot_pending?
```

### recipe_name

Use the `recipe_name` method to return the name of a recipe.

The syntax for the `recipe_name` method is as follows:

``` ruby
recipe_name
```

This method is often used as part of a log entry. For example:

``` ruby
Chef::Log.info('I am a message from the #{recipe_name} recipe in the #{cookbook_name} cookbook.')
```

### resources

Use the `resources` method to look up a resource in the resource
collection. The `resources` method returns the value for the resource
that it finds in the resource collection. The preferred syntax for the
`resources` method is as follows:

``` ruby
resources('resource_type[resource_name]')
```

but the following syntax can also be used:

``` ruby
resources(:resource_type => 'resource_name')
```

where in either approach `resource_type` is the name of a resource and
`resource_name` is the name of a resource that can be configured by Chef
Infra Client.

The `resources` method can be used to modify a resource later on in a
recipe. For example:

``` ruby
file '/etc/hosts' do
  content '127.0.0.1 localhost.localdomain localhost'
end
```

and then later in the same recipe, or elsewhere:

``` ruby
f = resources('file[/etc/hosts]')
f.mode '0644'
```

where `file` is the type of resource, `/etc/hosts` is the name, and
`f.mode` is used to set the `mode` property on the **file** resource.

### search

{{% search %}}

Use the `search` method to perform a search query against the Chef Infra
Server from within a recipe.

The syntax for the `search` method is as follows:

``` ruby
search(:index, 'query')
```

where:

-   `:index` is of name of the index on the Chef Infra Server against
    which the search query will run: `:client`, `:data_bag_name`,
    `:environment`, `:node`, and `:role`
-   `'query'` is a valid search query against an object on the Chef
    Infra Server (see below for more information about how to build the
    query)

For example, using the results of a search query within a variable:

``` ruby
webservers = search(:node, 'role:webserver')
```

and then using the results of that query to populate a template:

``` ruby
template '/tmp/list_of_webservers' do
  source 'list_of_webservers.erb'
  variables(:webservers => webservers)
end
```

#### :filter_result

{{% dsl_recipe_method_search_filter_result %}}

#### Query Syntax

{{% search_query_syntax %}}

**Keys**

{{% search_key %}}

**Nested Fields**

{{% search_key_nested %}}

**Patterns**

{{% search_pattern %}}

**Exact Match**

{{% search_pattern_exact %}}

**Wildcard Match**

{{% search_pattern_wildcard %}}

**Range Match**

{{% search_pattern_range %}}

**Fuzzy Match**

{{% search_pattern_fuzzy %}}

**Operators**

{{% search_boolean_operators %}}

**Special Characters**

{{% search_special_characters %}}

#### Examples

The following examples show how the `search` method can be used in a
recipe.

**Use the search recipe DSL method to find users**

{{< readFile_shortcode file="resource_execute_use_search_dsl_method.md" >}}

### shell_out

The `shell_out` method can be used to run a command against the node,
and then display the output to the console when the log level is set to
`debug`.

The syntax for the `shell_out` method is as follows:

``` ruby
shell_out(command_args)
```

where `command_args` is the command that is run against the node.

## shell_out!

The `shell_out!` method can be used to run a command against the node,
display the output to the console when the log level is set to `debug`,
and then raise an error when the method returns `false`.

The syntax for the `shell_out!` method is as follows:

``` ruby
shell_out!(command_args)
```

where `command_args` is the command that is run against the node. This
method will return `true` or `false`.

### tag, tagged?, untag

{{% chef_tags %}}

{{% cookbooks_recipe_tags %}}

### value_for_platform

Use the `value_for_platform` method in a recipe to select a value based
on the `node['platform']` and `node['platform_version']` attributes.
These values are detected by Ohai during every Chef Infra Client run.

The syntax for the `value_for_platform` method is as follows:

``` ruby
value_for_platform( ['platform', ...] => { 'version' => 'value' } )
```

where:

-   `'platform', ...` is a comma-separated list of platforms, such as
    Red Hat, openSUSE, or Fedora
-   `version` specifies the version of that platform
-   Version constraints---`>`, `<`, `>=`, `<=`, `~>`---may be used with
    `version`; an exception is raised if two version constraints match;
    an exact match will always take precedence over a match made from a
    version constraint
-   `value` specifies the value that will be used if the node's platform
    matches the `value_for_platform` method

When each value only has a single platform, use the following syntax:

``` ruby
value_for_platform(
  'platform' => { 'version' => 'value' },
  'platform' => { 'version' => 'value' },
  'platform' => 'value'
)
```

When each value has more than one platform, the syntax changes to:

``` ruby
value_for_platform(
  ['platform', 'platform', ... ] => {
    'version' => 'value'
  },
)
```

#### Operators

{{% cookbooks_version_constraints_operators %}}

#### Examples

The following example will set `package_name` to `httpd` for the Red Hat
platform and to `apache2` for the Debian platform:

``` ruby
package_name = value_for_platform(
  ['centos', 'redhat', 'suse', 'fedora' ] => {
    'default' => 'httpd'
  },
  ['ubuntu', 'debian'] => {
    'default' => 'apache2'
  }
)
```

The following example will set `package` to `apache-couchdb` for OpenBSD
platforms, `dev-db/couchdb` for Gentoo platforms, and `couchdb` for all
other platforms:

``` ruby
package = value_for_platform(
  'openbsd' => { 'default' => 'apache-couchdb' },
  'gentoo' => { 'default' => 'dev-db/couchdb' },
  'default' => 'couchdb'
)
```

The following example shows using version constraints to specify a value
based on the version:

``` ruby
value_for_platform(
  'os1' => { '< 1.0' => 'less than 1.0',
             '~> 2.0' => 'version 2.x',
             '>= 3.0' => 'greater than or equal to version 3.0',
             '3.0.1' => '3.0.1 will always use this value' }
)
```

### value_for_platform_family

Use the `value_for_platform_family` method in a recipe to select a value
based on the `node['platform_family']` attribute. This value is detected
by Ohai during every Chef Infra Client run.

The syntax for the `value_for_platform_family` method is as follows:

``` ruby
value_for_platform_family( 'platform_family' => 'value', ... )
```

where:

-   `'platform_family' => 'value', ...` is a comma-separated list of
    platforms, such as Fedora, openSUSE, or Red Hat Enterprise Linux
-   `value` specifies the value that will be used if the node's platform
    family matches the `value_for_platform_family` method

When each value only has a single platform, use the following syntax:

``` ruby
value_for_platform_family(
  'platform_family' => 'value',
  'platform_family' => 'value',
  'platform_family' => 'value'
)
```

When each value has more than one platform, the syntax changes to:

``` ruby
value_for_platform_family(
  ['platform_family', 'platform_family', 'platform_family', 'platform_family' ] => 'value',
  ['platform_family', 'platform_family'] => 'value',
  'default' => 'value'
)
```

The following example will set `package` to `httpd-devel` for the Red
Hat Enterprise Linux, Fedora, and openSUSE platforms and to
`apache2-dev` for the Debian platform:

``` ruby
package = value_for_platform_family(
  ['rhel', 'fedora', 'suse'] => 'httpd-devel',
    'debian' => 'apache2-dev'
)
```

### with_run_context

Use the `with_run_context` method to define a block that has a pointer
to a location in the `run_context` hierarchy. Resources in recipes
always run at the root of the `run_context` hierarchy, whereas custom
resources and notification blocks always build a child `run_context`
which contains their sub-resources.

The syntax for the `with_run_context` method is as follows:

``` ruby
with_run_context :type do
  # some arbitrary pure Ruby stuff goes here
end
```

where `:type` may be one of the following:

-   `:root` runs the block as part of the root `run_context` hierarchy
-   `:parent` runs the block as part of the parent process in the
    `run_context` hierarchy

For example:

``` ruby
action :run do
  with_run_context :root do
    edit_resource(:my_thing, "accumulated state") do
      action :nothing
      my_array_property << accumulate_some_stuff
    end
  end
  log "kick it off" do
    notifies :run, "my_thing[accumulated state]", :delayed
  end
end
```

## Windows Platform

{{% dsl_recipe_method_windows_methods %}}

{{< note >}}

{{% notes_dsl_recipe_order_for_windows_methods %}}

{{< /note >}}

### registry_data_exists?

{{% dsl_recipe_method_registry_data_exists %}}

### registry_get_subkeys

{{% dsl_recipe_method_registry_get_subkeys %}}

### registry_get_values

{{% dsl_recipe_method_registry_get_values %}}

### registry_has_subkeys?

{{% dsl_recipe_method_registry_has_subkeys %}}

### registry_key_exists?

{{% dsl_recipe_method_registry_key_exists %}}

### registry_value_exists?

{{% dsl_recipe_method_registry_value_exists %}}

## Log Entries

{{% ruby_style_basics_chef_log %}}

The following examples show using `Chef::Log` entries in a recipe.

{{% ruby_class_chef_log_fatal %}}

{{% ruby_class_chef_log_multiple %}}
