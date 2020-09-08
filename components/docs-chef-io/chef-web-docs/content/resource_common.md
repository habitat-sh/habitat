+++
title = "Common Resource Functionality"
draft = false

aliases = ["/resource_common.html"]

[menu]
  [menu.infra]
    title = "Common Resource Functionality"
    identifier = "chef_infra/cookbook_reference/resources/resource_common.md Common Resource Functionality"
    parent = "chef_infra/cookbook_reference/resources"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/resource_common.md)

All resources (including custom resources) share a set of common
actions, properties, conditional executions, notifications, and relative
path options.

## Actions

{{% resources_common_actions %}}

### Examples

The following examples show how to use common actions in a recipe.

**Use the :nothing action**

{{% resource_service_use_nothing_action %}}

## Properties

{{% resources_common_properties %}}

### Examples

The following examples show how to use common properties in a recipe.

**Use the ignore_failure common property**

{{% resource_package_use_ignore_failure_attribute %}}

**Use the retries common property**

{{% resource_service_use_supports_attribute %}}

## Guards

{{% resources_common_guards %}}

{{< note >}}

When using the `not_if` and `only_if` guards with the **execute**
resource, the guard's environment is inherited from the resource's
environment. For example:

``` ruby
execute 'bundle install' do
  cwd '/myapp'
  not_if 'bundle check' # This is run from /myapp
end
```

{{< /note >}}

### Properties

{{% resources_common_guards_properties %}}

### Arguments

{{% resources_common_guards_arguments %}}

### not_if Examples

**Update if not already updated**

The following example shows how to use `not_if` to guard against running
the `apt-get-update` command when a file already exists that is the same
as the updated file:

``` ruby
execute "apt-get-update" do
  command "apt-get update"
  ignore_failure true
  not_if { ::File.exist?('/var/lib/apt/periodic/update-success-stamp') }
end
```

**Ensure a node can resolve a host**

The following example shows how to use a custom block of Ruby code to
ensure that a node can resolve the host. If the node can resolve the
host, Chef Infra Client will do nothing. If the node cannot resolve the
host, Chef Infra Client will configure the host:

``` ruby
ruby_block "ensure node can resolve API FQDN" do
  block do
    fe = Chef::Util::FileEdit.new("/etc/hosts")
    fe.insert_line_if_no_match(/#{node['chef-server']['api_fqdn']}/,
                               "127.0.0.1 #{node['chef-server']['api_fqdn']}")
    fe.write_file
  end
  not_if { Resolv.getaddress(node['chef-server']['api_fqdn']) rescue false }
end
```

**Prevent installs on older versions**

The following example shows how to use `not_if` to prevent ZeroMQ from
being installed when the node on which the install is to occur has a
version of Red Hat Enterprise Linux that is older than version 6.0:

``` ruby
ark "test_autogen" do
  url 'https://github.com/zeromq/libzmq/tarball/master'
  extension "tar.gz"
  action :configure
  not_if { platform_family?('rhel') && node['platform_version'].to_f < 6.0 }
end
```

**Set the administrator if not already set**

The following example shows how to set the administrator for Nagios on
multiple nodes, except when the package already exists on a node:

``` ruby
%w{adminpassword adminpassword-repeat}.each do |setting|
  execute "debconf-set-selections::#{node['nagios']['server']['vname']}-cgi::#{node['nagios']['server']['vname']}/#{setting}" do
    command "echo #{node['nagios']['server']['vname']}-cgi #{node['nagios']['server']['vname']}/#{setting} password #{random_initial_password} | debconf-set-selections"
    not_if "dpkg -l #{node['nagios']['server']['vname']}"
  end
end
```

### only_if Examples

**Install packages only when necessary**

The following example shows how to use `only_if` with one (or more)
cookbook attributes to ensure that packages are only installed when
necessary. In this case, three attributes exist in the
`/attributes/default.rb` file: `use_openssl`, `use_pcre`, and
`use_zlib`. Each of these attributes are defined as `false` by default.
The `only_if` attributes are used to test for the presence of these
packages on the target node before then asking Chef Infra Client to
complete the process of installing these packages. If the packages are
already present, Chef Infra Client will do nothing.

``` ruby
package 'libpcre3-dev' do
  only_if { node['haproxy']['source']['use_pcre'] }
end

package 'libssl-dev' do
  only_if { node['haproxy']['source']['use_openssl'] }
end

package 'zlib1g-dev' do
  only_if { node['haproxy']['source']['use_zlib'] }
end
```

**Remove a recipe if it belongs to a specific run-list**

The following example shows how to use `only_if` to only remove a recipe
named `recipe[ntp::undo]`, but only when that recipe is part of the
`recipe[ntp::default]` run-list:

``` ruby
ruby_block 'remove ntp::undo from run list' do
  block do
    node.run_list.remove('recipe[ntp::undo]')
  end
  only_if { node.run_list.include?('recipe[ntp::default]') }
end
```

**Re-register ASP.Net if it's already installed**

The following example shows how to use `only_if` to ensure that Chef
Infra Client will attempt to register ASP.NET only if the executable is
installed on the system, on both 32- and 64-bit systems:

``` ruby
aspnet_regiis = "#{ENV['WinDir']}\\Microsoft.NET\\Framework\\v4.0.30319\\aspnet_regiis.exe"
execute 'Register ASP.NET v4' do
  command "#{aspnet_regiis} -i"
  only_if { ::File.exist?(aspnet_regiis) }
  action :nothing
end

aspnet_regiis64 = "#{ENV['WinDir']}\\Microsoft.NET\\Framework64\\v4.0.30319\\aspnet_regiis.exe"
execute 'Register ASP.NET v4 (x64)' do
  command "#{aspnet_regiis64} -i"
  only_if { ::File.exist?(aspnet_regiis64) }
  action :nothing
end
```

## Guard Interpreters

{{% resources_common_guard_interpreter %}}

### Attributes

{{% resources_common_guard_interpreter_attributes %}}

### Inheritance

{{% resources_common_guard_interpreter_attributes_inherit %}}

### Examples

{{% resources_common_guard_interpreter_example_default %}}

## Lazy Evaluation

{{% resources_common_lazy_evaluation %}}

## Notifications

{{% resources_common_notification %}}

### Timers

{{% resources_common_notification_timers %}}

### Notifies

{{% resources_common_notification_notifies %}}

{{% resources_common_notification_notifies_syntax %}}

Changed in Chef Client 12.6 to use `:before` timer with the `notifies`
and `subscribes` properties to specify that the action on a notified
resource should be run before processing the resource block in which the
notification is located.

#### Examples

The following examples show how to use the `notifies` notification in a
recipe.

**Delay notifications**

{{% resource_template_notifies_delay %}}

**Notify immediately**

{{% resource_template_notifies_run_immediately %}}

**Notify multiple resources**

{{% resource_template_notifies_multiple_resources %}}

**Notify in a specific order**

{{% resource_execute_notifies_specific_order %}}

**Reload a service**

{{% resource_template_notifies_reload_service %}}

**Restart a service when a template is modified**

{{% resource_template_notifies_restart_service_when_template_modified %}}

**Send notifications to multiple resources**

{{% resource_template_notifies_send_notifications_to_multiple_resources %}}

**Execute a command using a template**

{{% resource_execute_command_from_template %}}

**Restart a service, and then notify a different service**

{{% resource_service_restart_and_notify %}}

**Restart one service before restarting another**

{{% resource_before_notification_restart %}}

**Notify when a remote source changes**

{{% resource_remote_file_transfer_remote_source_changes %}}

### Subscribes

{{% resources_common_notification_subscribes %}}

{{% resources_common_notification_subscribes_syntax %}}

#### Examples

The following examples show how to use the `subscribes` notification in
a recipe.

**Verify a configuration update**

{{% resource_execute_subscribes_prevent_restart_and_reconfigure %}}

**Reload a service when a template is updated**

{{% resource_service_subscribes_reload_using_template %}}

## Relative Paths

{{% resources_common_relative_paths %}}

### Examples

{{% resource_template_use_relative_paths %}}

## Run in Compile Phase

{{% resources_common_compile %}}

### Using the compile_time property

{{< readFile_shortcode file="resources_common_compile_begin.md" >}}

## Windows File Security

{{% resources_common_windows_security %}}

### Access Control Lists (ACLs)

{{% resources_common_windows_security_acl %}}

### Inheritance

{{% resources_common_windows_security_inherits %}}
