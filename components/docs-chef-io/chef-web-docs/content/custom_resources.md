+++
title = "Custom Resources"
draft = false

aliases = ["/custom_resources.html"]

[menu]
  [menu.infra]
    title = "Custom Resources"
    identifier = "chef_infra/cookbook_reference/resources/custom_resources.md Custom Resources"
    parent = "chef_infra/cookbook_reference/resources"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/custom_resources.md)

A custom resource:

-   Is a simple extension of Chef Infra Client that adds your own resources
-   Is implemented and shipped as part of a cookbook
-   Follows easy, repeatable syntax patterns
-   Effectively leverages resources that are built into Chef Infra Client and/or custom Ruby code
-   Is reusable in the same way as resources that are built into Chef Infra Client

For example, Chef Infra Client includes built-in resources to manage
files, packages, templates, and services, but it does not include a
resource that manages websites.

## Syntax

A custom resource is defined as a Ruby file and is located in a
cookbook's `/resources` directory. This file:

-   Declares the properties of the custom resource
-   Loads current state of properties, if the resource already exists
-   Defines each action the custom resource may take

The syntax for a custom resource is. For example:

``` ruby
property :property_name, RubyType, default: 'value'

action :action_name do
 # a mix of built-in Chef resources and Ruby
end

action :another_action_name do
 # a mix of built-in Chef resources and Ruby
end
```

where the first action listed is the default action.

{{< warning >}}

Do not use existing keywords from the Chef Infra Client resource system
in a custom resource, like "name". For example,
`property :property_name` in the following invalid syntax:
`property :name, String, default: 'thename'`.

{{< /warning >}}

### Example

This example `site` utilizes Chef's built-in `file`, `service` and
`package` resources, and includes `:create` and `:delete` actions. Since
it uses built-in Chef Infra Client resources, besides defining the
property and actions, the code is very similar to that of a recipe.

``` ruby
property :homepage, String, default: '<h1>Hello world!</h1>'

action :create do
  package 'httpd'

  service 'httpd' do
    action [:enable, :start]
  end

  file '/var/www/html/index.html' do
    content new_resource.homepage
  end
end

action :delete do
  package 'httpd' do
    action :remove
  end
end
```

where

-   `homepage` is a property that sets the default HTML for the
    `index.html` file with a default value of `'<h1>Hello world!</h1>'`
-   the `action` block uses the built-in collection of resources to tell
    Chef Infra Client how to install Apache, start the service, and then
    create the contents of the file located at
    `/var/www/html/index.html`
-   `action :create` is the default resource, because it is listed
    first; `action :delete` must be called specifically (because it is
    not the default action)

Once written, the custom resource may be used in a recipe just like any
of the resources that are built into Chef Infra Client. The resource
gets its name from the cookbook and from the file name in the
`/resources` directory, with an underscore (`_`) separating them. For
example, a cookbook named `exampleco` with a custom resource named
`site.rb` is used in a recipe like this:

``` ruby
exampleco_site 'httpd' do
  homepage '<h1>Welcome to the Example Co. website!</h1>'
end
```

and to delete the exampleco website, do the following:

``` ruby
exampleco_site 'httpd' do
  action :delete
end
```

## Scenario: website Resource

Create a resource that configures Apache httpd for Red Hat Enterprise
Linux 7 and CentOS 7.

This scenario covers the following:

1.  Defining a cookbook named `website`
2.  Defining two properties
3.  Defining an action
4.  For the action, defining the steps to configure the system using resources that are built into Chef Infra
5.  Creating two templates that support the custom resource
6.  Adding the resource to a recipe

### Create a Cookbook

This article assumes that a cookbook directory named `website` exists in
a chef-repo with (at least) the following directories:

``` text
/website
  /recipes
  /resources
  /templates
```

You may use a cookbook that already exists or you may create a new
cookbook.

See /ctl_chef.html for more information about how to use the `chef`
command-line tool that is packaged with Chef Workstation to build the
chef-repo, plus related cookbook sub-directories.

### Objectives

Define a custom resource!

A custom resource typically contains:

-   A list of defined custom properties (property values are specified
    in recipes)
-   At least one action (actions tell Chef Infra Client what to do)
-   For each action, use a collection of resources that are built into
    Chef Infra Client to define the steps required to complete the
    action

#### What is needed?

This custom resource requires:

-   Two template files
-   Two properties
-   An action that defines all of the steps necessary to create the
    website

### Define Properties

Custom properties are defined in the resource. This custom resource
needs two:

-   `instance_name`
-   `port`

These properties are defined as variables in the `httpd.conf.erb` file.
A **template** block in recipes will tell Chef Infra Client how to apply
these variables.

In the custom resource, add the following custom properties:

``` ruby
property :instance_name, String, name_property: true
property :port, Integer, required: true
```

where

-   `String` and `Integer` are Ruby types (all custom properties must
    have an assigned Ruby type)
-   `name_property: true` allows the value for this property to be equal
    to the `'name'` of the resource block

The `instance_name` property is then used within the custom resource in
many locations, including defining paths to configuration files,
services, and virtual hosts.

### Define Actions

Each custom resource must have at least one action that is defined
within an `action` block:

``` ruby
action :create do
  # the steps that define the action
end
```

where `:create` is a value that may be assigned to the `action` property
for when this resource is used in a recipe.

For example, the `action` appears as a property when this custom
resource is used in a recipe:

``` ruby
custom_resource 'name' do
  # some properties
  action :create
end
```

### Define Resource

Use the **package**, **template** (two times), **directory**, and
**service** resources to define the `website` resource. Remember: order
matters!

#### package

Use the **package** resource to install httpd:

``` ruby
package 'httpd' do
  action :install
end
```

#### template, httpd.service

Use the **template** resource to create an `httpd.service` on the node
based on the `httpd.service.erb` template located in the cookbook:

``` ruby
template "/lib/systemd/system/httpd-#{new_resource.instance_name}.service" do
  source 'httpd.service.erb'
  variables(
    instance_name: new_resource.instance_name
  )
  action :create
end
```

where

-   `source` gets the `httpd.service.erb` template from this cookbook
-   `variables` assigns the `instance_name` property to a variable in
    the template

#### template, httpd.conf

Use the **template** resource to configure httpd on the node based on
the `httpd.conf.erb` template located in the cookbook:

``` ruby
template "/etc/httpd/conf/httpd-#{new_resource.instance_name}.conf" do
  source 'httpd.conf.erb'
  variables(
    instance_name: new_resource.instance_name,
    port: new_resource.port
  )
  action :create
end
```

where

-   `source` gets the `httpd.conf.erb` template from this cookbook
-   `variables` assigns the `instance_name` and `port` properties to
    variables in the template

{{< note >}}

When writing a shared custom resource, you may need to use templates
that ship with the custom resource. However, you will need to specify
the cookbook containing the template by using the cookbook property in
the template resource. If this is not set, then Chef Infra Client will
look for templates in the location of the cookbook that is using the
resource and won't be able to find them. Example: `cookbook 'website'`

{{< /note >}}

#### directory

Use the **directory** resource to create the `/var/www/vhosts` directory
on the node:

``` ruby
directory "/var/www/vhosts/#{new_resource.instance_name}" do
  recursive true
  action :create
end
```

#### service

Use the **service** resource to enable, and then start the service:

``` ruby
service "httpd-#{new_resource.instance_name}" do
  action [:enable, :start]
end
```

### Create Templates

The `/templates` directory must contain two templates:

-   `httpd.conf.erb` to configure Apache httpd
-   `httpd.service.erb` to tell systemd how to start and stop the
    website

#### httpd.conf.erb

`httpd.conf.erb` stores information about the website and is typically
located under the `/etc/httpd`:

``` ruby
ServerRoot "/etc/httpd"
Listen <%= @port %>
Include conf.modules.d/*.conf
User apache
Group apache
<Directory />
  AllowOverride none
  Require all denied
</Directory>
DocumentRoot "/var/www/vhosts/<%= @instance_name %>"
<IfModule mime_module>
  TypesConfig /etc/mime.types
</IfModule>
```

Copy it as shown, add it under `/templates`, and then name the file
`httpd.conf.erb`.

**Template Variables**

The `httpd.conf.erb` template has two variables:

-   `<%= @instance_name %>`
-   `<%= @port %>`

They are:

-   Declared as properties of the custom resource
-   Defined as variables in a **template** resource block within the
    custom resource
-   Tunable from a recipe when using `port` and `instance_name` as
    properties in that recipe
-   `instance_name` defaults to the `'name'` of the custom resource if
    not specified as a property

#### httpd.service.erb

`httpd.service.erb` tells systemd how to start and stop the website:

``` none
[Unit]
Description=The Apache HTTP Server - instance <%= @instance_name %>
After=network.target remote-fs.target nss-lookup.target

[Service]
Type=notify

ExecStart=/usr/sbin/httpd -f /etc/httpd/conf/httpd-<%= @instance_name %>.conf -DFOREGROUND
ExecReload=/usr/sbin/httpd -f /etc/httpd/conf/httpd-<%= @instance_name %>.conf -k graceful
ExecStop=/bin/kill -WINCH ${MAINPID}

KillSignal=SIGCONT
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

Copy it as shown, add it under `/templates`, and then name it
`httpd.service.erb`.

### Final Resource

``` ruby
property :instance_name, String, name_property: true
property :port, Integer, required: true

action :create do
  package 'httpd' do
    action :install
  end

  template "/lib/systemd/system/httpd-#{new_resource.instance_name}.service" do
    source 'httpd.service.erb'
    variables(
      instance_name: new_resource.instance_name
    )
    action :create
  end

  template "/etc/httpd/conf/httpd-#{new_resource.instance_name}.conf" do
    source 'httpd.conf.erb'
    variables(
      instance_name: new_resource.instance_name,
      port: new_resource.port
    )
    action :create
  end

  directory "/var/www/vhosts/#{new_resource.instance_name}" do
    recursive true
    action :create
  end

  service "httpd-#{new_resource.instance_name}" do
    action [:enable, :start]
  end

end
```

### Final Cookbook Directory

When finished adding the templates and building the custom resource, the
cookbook directory structure should look like this:

``` text
/website
  metadata.rb
  /recipes
    default.rb
  README.md
  /resources
    httpd.rb
  /templates
    httpd.conf.erb
    httpd.service.erb
```

### Recipe

The custom resource name is inferred from the name of the cookbook
(`website`), the name of the resource file (`httpd`), and is separated
by an underscore(`_`): `website_httpd`. The custom resource may be used
in a recipe.

``` ruby
website_httpd 'httpd_site' do
  port 81
  action :create
end
```

which does the following:

-   Installs Apache httpd
-   Assigns an instance name of `httpd_site` that uses port 81
-   Configures httpd and systemd from a template
-   Creates the virtual host for the website
-   Starts the website using systemd

## Custom Resource DSL

The following sections describe additional Custom Resource DSL methods
that were not used in the preceding scenario:

### action_class

Use the `action_class` block to make methods available to the actions in
the custom resource. Modules with helper methods created as files in the
cookbook library directory may be included. New action methods may also
be defined directly in the `action_class` block. Code in the
`action_class` block has access to the new_resource properties.

Assume a helper module has been created in the cookbook
`libraries/helper.rb` file.

``` ruby
module Sample
  module Helper
    def helper_method
      # code
    end
  end
end
```

Methods may be made available to the custom resource actions by using an
`action_class` block.

``` ruby
property file, String

action :delete do
  helper_method
  FileUtils.rm(new_resource.file) if file_exist
end

action_class do

  def file_exist
    ::File.exist?(new_resource.file)
  end

  require 'fileutils'

  include Sample::Helper

end
```

### converge_if_changed

{{% dsl_custom_resource_method_converge_if_changed %}}

#### Multiple Properties

{{% dsl_custom_resource_method_converge_if_changed_multiple %}}

### default_action

{{% dsl_custom_resource_method_default_action %}}

### load_current_value

{{% dsl_custom_resource_method_load_current_value %}}

### new_resource.property

{{< readFile_shortcode file="dsl_custom_resource_method_new_resource.md" >}}

### property

{{% dsl_custom_resource_method_property %}}

#### ruby_type

{{% dsl_custom_resource_method_property_ruby_type %}}

#### sensitive

A property can be marked sensitive by specifying `sensitive: true` on
the property. This prevents the contents of the property from being
exported to data collection and sent to an Automate server.

Note: This feature was introduced in Chef Client 12.14.

#### validators

{{% dsl_custom_resource_method_property_validation_parameter %}}

#### desired_state

{{% dsl_custom_resource_method_property_desired_state %}}

#### identity

{{% dsl_custom_resource_method_property_identity %}}

### Block Arguments

{{% dsl_custom_resource_method_property_block_argument %}}

### property_is_set?

{{% dsl_custom_resource_method_property_is_set %}}

### provides

{{% dsl_custom_resource_method_provides %}}

### reset_property

{{% dsl_custom_resource_method_reset_property %}}

### coerce

`coerce` is used to transform user input into a canonical form. The
value is passed in, and the transformed value returned as output. Lazy
values will **not** be passed to this method until after they are
evaluated.

`coerce` is run in the context of the instance, which gives it access to
other properties.

``` ruby
property :mode, coerce: proc { |m| m.is_a?(String) ? m.to_s(8) : m }
```
