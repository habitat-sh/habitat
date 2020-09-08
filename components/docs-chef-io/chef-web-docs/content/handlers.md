+++
title = "About Handlers"
draft = false

aliases = ["/handlers.html"]

[menu]
  [menu.infra]
    title = "Handlers"
    identifier = "chef_infra/features/handlers.md Handlers"
    parent = "chef_infra/features"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/handlers.md)

{{% handler %}}

{{% handler_types %}}

## Exception/Report Handlers

{{% handler_type_exception_report %}}

### Run from Recipes

{{% handler_type_exception_report_run_from_recipe %}}

### Run from client.rb

A simple exception or report handler may be installed and configured at
run-time. This requires editing of a node's client.rb file to add the
appropriate setting and information about that handler to the client.rb
or solo.rb files. Depending on the handler type, one (or more) of the
following settings must be added:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>exception_handlers</code></td>
<td>A list of exception handlers that are available to Chef Infra Client during a Chef Infra Client run.</td>
</tr>
<tr class="even">
<td><code>report_handlers</code></td>
<td>A list of report handlers that are available to Chef Infra Client during a Chef Infra Client run.</td>
</tr>
</tbody>
</table>

When this approach is used, the client.rb file must also tell Chef Infra
Client how to install and run the handler. There is no default install
location for handlers. The simplest way to distribute and install them
is via RubyGems, though other methods such as GitHub or HTTP will also
work. Once the handler is installed on the system, enable it in the
client.rb file by requiring it. After the handler is installed, it may
require additional configuration. This will vary from handler to
handler. If a handler is a very simple handler, it may only require the
creation of a new instance. For example, if a handler named
`MyOrg::EmailMe` is hardcoded for all of the values required to send
email, a new instance is required. And then the custom handler must be
associated with each of the handler types for which it will run.

For example:

``` ruby
require '/var/chef/handlers/email_me'         # the installation path

email_handler = MyOrg::EmailMe.new            # a simple handler

start_handlers << email_handler               # run at the start of the run
report_handlers << email_handler              # run at the end of a successful run
exception_handlers << email_handler           # run at the end of a failed run
```

## Start Handlers

{{% handler_type_start %}}

### Run from Recipes

{{% handler_type_start_run_from_recipe %}}

### Run from client.rb

A start handler can be configured in the client.rb file by adding the
following setting:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>start_handlers</code></td>
<td>A list of start handlers that are available to Chef Infra Client at the start of a Chef Infra Client run.</td>
</tr>
</tbody>
</table>

For example, the Reporting start handler adds the following code to the
top of the client.rb file:

``` ruby
begin
  require 'chef_reporting'
  start_handlers << Chef::Reporting::StartHandler.new()
rescue LoadError
  Chef::Log.warn 'Failed to load #{lib}. This should be resolved after a chef run.'
end
```

This ensures that when a Chef Infra Client run begins the
`chef_reporting` event handler is enabled. The `chef_reporting` event
handler is part of a gem named `chef-reporting`. The **chef_gem**
resource is used to install this gem:

``` ruby
chef_gem 'chef-reporting' do
  action :install
end
```

## Event Handlers

{{% dsl_handler_summary %}}

### on Method

{{% dsl_handler_method_on %}}

### Event Types

{{% dsl_handler_event_types %}}

### Examples

The following examples show ways to use the Handler DSL.

#### Send Email

{{% dsl_handler_slide_send_email %}}

**Define How Email is Sent**

{{< readFile_shortcode file="dsl_handler_slide_send_email_library.md" >}}

**Add the Handler**

{{% dsl_handler_slide_send_email_handler %}}

**Test the Handler**

{{% dsl_handler_slide_send_email_test %}}

#### etcd Locks

{{% dsl_handler_example_etcd_lock %}}

#### HipChat Notifications

{{% dsl_handler_example_hipchat %}}

## Handlers and Cookbooks

The following cookbooks can be used to load handlers during a Chef Infra
Client run.

### chef_handler

Exception and report handlers can be distributed using the
**chef_handler** cookbook. This cookbook is authored and maintained by
Chef and exposes a custom resource that can be used to enable custom
handlers from within recipes and to include product-specific handlers
from cookbooks. The **chef_handler** cookbook can be accessed here:
<https://github.com/chef-cookbooks/chef_handler>. See the `README.md`
for additional information.

### Chef Infra Client

Start handlers can be distributed using the **chef-client** cookbook,
which will install the handler on the target node during the initial
configuration of the node. This ensures that the start handler is always
present on the node so that it is available to Chef Infra Client at the
start of every run.

## Custom Handlers

{{% handler_custom %}}

### Syntax

{{< readFile_shortcode file="handler_custom_syntax.md" >}}

### report Interface

{{< readFile_shortcode file="handler_custom_interface_report.md" >}}

### Optional Interfaces

{{% handler_custom_optional_interfaces %}}

#### data

{{% handler_custom_interface_data %}}

#### run_report_safely

{{% handler_custom_interface_run_report_safely %}}

#### run_report_unsafe

{{% handler_custom_interface_run_report_unsafe %}}

### run_status Object

{{% handler_custom_object_run_status %}}

## Examples

The following sections show examples of handlers.

### Cookbook Versions

{{% handler_custom_example_cookbook_versions %}}

#### cookbook_versions.rb

{{< readFile_shortcode file="handler_custom_example_cookbook_versions_handler.md" >}}

#### default.rb

{{% handler_custom_example_cookbook_versions_recipe %}}

### Reporting

Start handler functionality was added when Chef started building add-ons
for the Chef Infra Server. The Reporting add-on is designed to create
reporting data based on a Chef Infra Client run. And since Reporting
needs to be able to collect data for the entire Chef Infra Client run,
Reporting needs to be enabled before anything else happens at the start
of a Chef Infra Client run.

{{< note >}}

The start handler used by the Reporting add-on for the Chef Infra Server
is always installed using the **chef-client** cookbook.

{{< /note >}}

#### start_handler.rb

The following code shows the start handler used by the Reporting add-in
for the Chef Infra Server:

``` ruby
require 'chef/handler'
require 'chef/rest'
require 'chef/version_constraint'

class Chef
  class Reporting
    class StartHandler < ::Chef::Handler

      attr_reader :config

      def initialize(config={})
        @config = config
      end

      def report
        version_checker = Chef::VersionConstraint.new('< 11.6.0')
        if version_checker.include?(Chef::VERSION)
          Chef::Log.info('Enabling backported resource reporting Handler')
          rest = Chef::REST.new(Chef::Config[:chef_server_url], @run_status.node.name, Chef::Config[:client_key])
          resource_reporter = Chef::Reporting::ResourceReporter.new(rest)
          @run_status.events.register(resource_reporter)

          resource_reporter.run_started(@run_status)
        else
         Chef::Log.debug('Chef Version already has new Resource Reporter - skipping startup of backport version')
        end
      end
    end
  end
end
```

### json_file Handler

{{< readFile_shortcode file="handler_custom_example_json_file.md" >}}

### error_report Handler

{{< readFile_shortcode file="handler_custom_example_error_report.md" >}}

### Community Handlers

{{% handler_community_handlers %}}
