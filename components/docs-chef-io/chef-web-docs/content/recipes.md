+++
title = "About Recipes"
draft = false

aliases = ["/recipes.html"]

[menu]
  [menu.infra]
    title = "About Recipes"
    identifier = "chef_infra/cookbook_reference/recipes/recipes.md About Recipes"
    parent = "chef_infra/cookbook_reference/recipes"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/recipes.md)

{{% cookbooks_recipe %}}

## Recipe Attributes

{{% cookbooks_attribute %}}

{{< note >}}

{{% notes_see_attributes_overview %}}

{{< /note >}}

### Attribute Types

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

### Attribute Persistence

{{% node_attribute_persistence %}}

### Attribute Precedence

{{% node_attribute_precedence %}}

### Blacklist Attributes

{{% node_attribute_blacklist %}}

#### Whitelist Attributes

{{% node_attribute_whitelist %}}

## File Methods

{{% cookbooks_attribute_file_methods %}}

## Environment Variables

{{% environment_variables_summary %}}

{{< readFile_shortcode file="environment_variables_access_resource_attributes.md" >}}

## Work with Recipes

The following sections show approaches to working with recipes.

### Use Data Bags

{{% data_bag %}}

The contents of a data bag can be loaded into a recipe. For example, a
data bag named `apps` and a data bag item named `my_app`:

``` javascript
{
  "id": "my_app",
  "repository": "git://github.com/company/my_app.git"
}
```

can be accessed in a recipe, like this:

``` ruby
my_bag = data_bag_item('apps', 'my_app')
```

The data bag item's keys and values can be accessed with a Hash:

``` ruby
my_bag['repository'] #=> 'git://github.com/company/my_app.git'
```

#### Secret Keys

{{% data_bag_encryption_secret_key %}}

#### Store Keys on Nodes

An encryption key can also be stored in an alternate file on the nodes
that need it and specify the path location to the file inside an
attribute; however, `EncryptedDataBagItem.load` expects to see the
actual secret as the third argument, rather than a path to the secret
file. In this case, you can use `EncryptedDataBagItem.load_secret` to
slurp the secret file contents and then pass them:

``` ruby
# inside your attribute file:
# default[:mysql][:secretpath] = 'C:\\chef\\any_secret_filename'
#
# inside your recipe:
# look for secret in file pointed to by mysql attribute :secretpath
mysql_secret = Chef::EncryptedDataBagItem.load_secret('#{node['mysql']['secretpath']}')
mysql_creds = Chef::EncryptedDataBagItem.load('passwords', 'mysql', mysql_secret)
mysql_creds['pass'] # will be decrypted
```

### Assign Dependencies

If a cookbook has a dependency on a recipe that is located in another
cookbook, that dependency must be declared in the metadata.rb file for
that cookbook using the `depends` keyword.

{{< note >}}

Declaring cookbook dependencies is not required with chef-solo.

{{< /note >}}

For example, if the following recipe is included in a cookbook named
`my_app`:

``` ruby
include_recipe 'apache2::mod_ssl'
```

Then the metadata.rb file for that cookbook would have:

``` ruby
depends 'apache2'
```

### Include Recipes

{{% cookbooks_recipe_include_in_recipe %}}

### Reload Attributes

{{% cookbooks_attribute_file_reload_from_recipe %}}

### Use Ruby

Anything that can be done with Ruby can be used within a recipe, such as
expressions (if, unless, etc.), case statements, loop statements,
arrays, hashes, and variables. In Ruby, the conditionals `nil` and
`false` are false; every other conditional is `true`.

#### Assign a value

A variable uses an equals sign (`=`) to assign a value.

To assign a value to a variable:

``` ruby
package_name = 'apache2'
```

#### Use Case Statement

A case statement can be used to compare an expression, and then execute
the code that matches.

To select a package name based on platform:

``` ruby
package 'apache2' do
  case node['platform']
  when 'centos', 'redhat', 'fedora', 'suse'
    package_name 'httpd'
  when 'debian', 'ubuntu'
    package_name 'apache2'
  when 'arch'
    package_name 'apache'
  end
  action :install
end
```

#### Check Conditions

An if expression can be used to check for conditions (true or false).

To check for condition only for Debian and Ubuntu platforms:

``` ruby
if platform?('debian', 'ubuntu')
  # do something if node['platform'] is debian or ubuntu
else
  # do other stuff
end
```

#### Execute Conditions

An unless expression can be used to execute code when a condition
returns a false value (effectively, an unless expression is the opposite
of an if statement).

To use an expression to execute when a condition returns a false value:

``` ruby
unless node['platform_version'] == '5.0'
  # do stuff on everything but 5.0
end
```

#### Loop over Array

A loop statement is used to execute a block of code one (or more) times.
A loop statement is created when `.each` is added to an expression that
defines an array or a hash. An array is an integer-indexed collection of
objects. Each element in an array can be associated with and referred to
by an index.

To loop over an array of package names by platform:

``` ruby
['apache2', 'apache2-mpm'].each do |p|
  package p
end
```

#### Loop over Hash

A hash is a collection of key-value pairs. Indexing for a hash is done
using arbitrary keys of any object (as opposed to the indexing done by
an array). The syntax for a hash is: `key => "value"`.

To loop over a hash of gem package names:

``` ruby
{ 'fog' => '0.6.0', 'highline' => '1.6.0' }.each do |g, v|
  gem_package g do
    version v
  end
end
```

### Apply to Run-lists

A recipe must be assigned to a run-list using the appropriate name, as
defined by the cookbook directory and namespace. For example, a cookbook
directory has the following structure:

    cookbooks/
      apache2/
        recipes/
          default.rb
          mod_ssl.rb

There are two recipes: a default recipe (that has the same name as the
cookbook) and a recipe named `mod_ssl`. The syntax that applies a recipe
to a run-list is similar to:

``` ruby
{
  'run_list': [
  'recipe[cookbook_name::default_recipe]',
  'recipe[cookbook_name::recipe_name]'
  ]
}
```

where `::default_recipe` is implied (and does not need to be specified).
On a node, these recipes can be assigned to a node's run-list similar
to:

``` ruby
{
  'run_list': [
  'recipe[apache2]',
  'recipe[apache2::mod_ssl]'
  ]
}
```

#### Chef Infra Server

Use knife to add a recipe to the run-list for a node. For example:

``` bash
knife node run list add NODENAME "recipe[apache2]"
```

More than one recipe can be added:

``` bash
% knife node run list add NODENAME "recipe[apache2],recipe[mysql],role[ssh]"
```

which creates a run-list similar to:

``` ruby
run_list:
   recipe[apache2]
   recipe[mysql]
   role[ssh]
```

#### chef-solo

Use a JSON file to pass run-list details to chef-solo as long as the
cookbook in which the recipe is located is available to the system on
which chef-solo is running. For example, a file named `dna.json`
contains the following details:

``` none
{
  "run_list": ["recipe[apache2]"]
}
```

To add the run-list to the node, enter the following:

``` bash
sudo chef-solo -j /etc/chef/dna.json
```

### Use Search Results

{{% search %}}

The results of a search query can be loaded into a recipe. For example,
a very simple search query (in a recipe) might look like this:

``` ruby
search(:node, 'attribute:value')
```

A search query can be assigned to variables and then used elsewhere in a
recipe. For example, to search for all nodes that have a role assignment
named `webserver`, and then render a template which includes those role
assignments:

``` ruby
webservers = search(:node, 'role:webserver')

template '/tmp/list_of_webservers' do
  source 'list_of_webservers.erb'
  variables(webservers: webservers)
end
```

### Use Tags

{{% chef_tags %}}

{{% cookbooks_recipe_tags %}}

### End Chef Infra Client Run

Sometimes it may be necessary to stop processing a recipe and/or stop
processing the entire Chef Infra Client run. There are a few ways to do
this:

-   Use the `return` keyword to stop processing a recipe based on a
    condition, but continue processing a Chef Infra Client run
-   Use the `raise` keyword to stop a Chef Infra Client run by
    triggering an unhandled exception
-   Use a `rescue` block in Ruby code
-   Use an [exception handler](/handlers/)
-   Use `Chef::Application.fatal!` to log a fatal message to the logger
    and `STDERR`, and then stop a Chef Infra Client run

The following sections show various approaches to ending a Chef Infra
Client run.

#### return Keyword

The `return` keyword can be used to stop processing a recipe based on a
condition, but continue processing a Chef Infra Client run. For example:

``` ruby
file '/tmp/name_of_file' do
  action :create
end

return if platform?('windows')

package 'name_of_package' do
  action :install
end
```

where `platform?('windows')` is the condition set on the `return`
keyword. When the condition is met, stop processing the recipe. This
approach is useful when there is no need to continue processing, such as
when a package cannot be installed. In this situation, it's OK for a
recipe to stop processing.

#### fail/raise Keywords

In certain situations it may be useful to stop a Chef Infra Client run
entirely by using an unhandled exception. The `raise` and `fail`
keywords can be used to stop a Chef Infra Client run in both the compile
and execute phases.

{{< note >}}

Both `raise` and `fail` behave the same way when triggering unhandled
exceptions and may be used interchangeably.

{{< /note >}}

Use these keywords in a recipe---but outside of any resource blocks---to
trigger an unhandled exception during the compile phase. For example:

``` ruby
file '/tmp/name_of_file' do
  action :create
end

raise "message" if platform?('windows')

package 'name_of_package' do
  action :install
end
```

where `platform?('windows')` is the condition that will trigger the
unhandled exception.

Use these keywords in the **ruby_block** resource to trigger an
unhandled exception during the execute phase. For example:

``` ruby
ruby_block "name" do
  block do
    # Ruby code with a condition, e.g. if ::File.exist?(::File.join(path, "/tmp"))
    fail "message"  # e.g. "Ordering issue with file path, expected foo"
  end
end
```

Use these keywords in a class. For example:

``` ruby
class CustomError < StandardError; end
```

and then later on:

``` ruby
def custom_error
  raise CustomError, "error message"
end
```

or:

``` ruby
def custom_error
  fail CustomError, "error message"
end
```

#### Rescue Blocks

Since recipes are written in Ruby, they can be written to attempt to
handle error conditions using the `rescue` block.

For example:

``` ruby
begin
  dater = data_bag_item(:basket, 'flowers')
rescue Net::HTTPClientException
  # maybe some retry code here?
  raise 'message_to_be_raised'
end
```

where `data_bag_item` makes an HTTP request to the Chef Infra Server to
get a data bag item named `flowers`. If there is a problem, the request
will return a `Net::HTTPClientException`. The `rescue` block can be used
to try to retry or otherwise handle the situation. If the `rescue` block
is unable to handle the situation, then the `raise` keyword is used to
specify the message to be raised.

#### Fatal Messages

A Chef Infra Client run is stopped after a fatal message is sent to the
logger and `STDERR`. For example:

``` ruby
Chef::Application.fatal!("log_message", error_code) if condition
```

where `condition` defines when a `"log_message"` and an `error_code` are
sent to the logger and `STDERR`, after which Chef Infra Client will
exit. The `error_code` itself is arbitrary and is assigned by the
individual who writes the code that triggers the fatal message.
Assigning an error code is optional, but they can be useful during log
file analysis.

This approach is used within Chef Infra Client itself to help ensure
consistent messaging around certain behaviors. That said, this approach
is not recommended for use within recipes and cookbooks and should only
be used when the other approaches are not applicable.

{{< note >}}

This approach should be used carefully when Chef Infra Client is run as
a daemonized service. Some services---such as a runit service---should
restart, but others---such as an init.d services---likely will not.

{{< /note >}}

### node.run_state

Use `node.run_state` to stash transient data during a Chef Infra Client
run. This data may be passed between resources, and then evaluated
during the execution phase. `run_state` is an empty Hash that is always
discarded at the end of a Chef Infra Client run.

For example, the following recipe will install the Apache web server,
randomly choose PHP or Perl as the scripting language, and then install
that scripting language:

``` ruby
package 'httpd' do
  action :install
end

ruby_block 'randomly_choose_language' do
  block do
    if Random.rand > 0.5
      node.run_state['scripting_language'] = 'php'
    else
      node.run_state['scripting_language'] = 'perl'
    end
  end
end

package 'scripting_language' do
  package_name lazy { node.run_state['scripting_language'] }
  action :install
end
```

where:

-   The **ruby_block** resource declares a `block` of Ruby code that is
    run during the execution phase of a Chef Infra Client run
-   The `if` statement randomly chooses PHP or Perl, saving the choice
    to `node.run_state['scripting_language']`
-   When the **package** resource has to install the package for the
    scripting language, it looks up the scripting language and uses the
    one defined in `node.run_state['scripting_language']`
-   `lazy {}` ensures that the **package** resource evaluates this
    during the execution phase of a Chef Infra Client run (as opposed to
    during the compile phase)

When this recipe runs, Chef Infra Client will print something like the
following:

``` bash
* ruby_block[randomly_choose_language] action run
 - execute the ruby block randomly_choose_language

* package[scripting_language] action install
 - install version 5.3.3-27.el6_5 of package php
```
