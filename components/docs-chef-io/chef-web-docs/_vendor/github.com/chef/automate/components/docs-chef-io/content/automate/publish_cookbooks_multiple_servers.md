+++
title = "Publish Cookbooks"

draft = false
[menu]
  [menu.automate]
    title = "Publish Cookbooks"
    parent = "automate/workflow"
    identifier = "automate/workflow/publish_cookbooks_multiple_servers.md Publish Cookbooks"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/publish_cookbooks_multiple_servers.md)

Workflow is a legacy feature for Chef Automate, which was designed for managing changes to both infrastructure and application code, giving your operations and development teams a common platform for developing, testing, and deploying cookbooks, applications, and more.

{{< warning >}}
Workflow is available in Chef Automate for existing users. If you are not already using Workflow, but are interested in the solution it offers, please contact your sales or success representative for support with continuous integration pipelines.
{{< /warning >}}

The `delivery-sugar` cookbook exposes some libraries and a resource that
you can use to publish a cookbook (or multiple cookbooks) to multiple
Chef Infra Servers or organizations. The following examples show how to
publish to both a single Chef Infra Server and how to extend that methodology
to publish to multiple Chef Infra Servers.

## Prerequisites

Before you begin, you must copy the `config.rb` file and the client key
.pem file (referenced as `client_key` in `config.rb`) to the build
nodes/runners that you will use in the build-cookbook. This can be done
manually by logging in to the build nodes/runners and copying the files
or automated through the use of a secure copy tool like `scp`.

The following is an example of a `config.rb` file for a `test` user that
points to the Chef Infra Server `chef-test-server.example.com` and the
organization `your_org`. The `test.pem` file is the value specified in
the `client_key` setting.

```ruby
current_dir = File.dirname(__FILE__)
log_location      STDOUT
node_name         'test'
client_key        "#{current_dir}/test.pem"
trusted_certs_dir '/etc/chef/trusted_certs'
chef_server_url   'https://chef-test-server.example.com/organizations/your_org'
```

## Publish a cookbook to a single Chef Infra Server

To publish a cookbook to a Chef Infra Server, use the `delivery_chef_cookbook`
resource and reference the `config.rb` file that you copied to your
build node/runner.

This example shows how to publish a cookbook called `rally` to a single
Chef Infra Server.

```ruby
knife_rb = '/path/to/the/knife_rb/file/in/the/build-node/config.rb'

delivery_chef_cookbook 'rally' do
  path '/path/to/the/cookbook/in/the/build-node/rally'
  chef_server DeliverySugar::ChefServer.new(knife_rb)
end
```

{{< note >}}
The default action for `delivery_chef_cookbook` is `:upload`, so you do
not need to explicitly include that in your `delivery_chef_cookbook`
implementation.
{{< /note >}}

## Publish Cookbook to Multiple Chef Infra Servers

Publishing to multiple servers uses the `delivery_chef_cookbook` in much
the same way as publishing to a single Chef Infra Server except you reference
multiple Chef Infra Server objects through an array.

In the following example, imagine you have two Chef Infra Servers, one in San
Francisco and another one in New York. Also, assume you have copied the
correct `config.rb` and `client_key` files to the build nodes/runners
for each Chef Infra Server.

For this particular example, you want the cookbook uploaded at the very
end of the workflow pipeline, in the **Functional** phase of the
**Delivered** Stage. This requires that you modify the **Functional**
recipe (`recipes/functional.rb`) of the **build-cookbook** within your
project, as shown below.

```ruby
# Run it only in Delivered::Functional
#
# This helper is coming from delivery-sugar
# => https://github.com/chef-cookbooks/delivery-sugar/blob/master/libraries/delivery_dsl.rb#L105,L113
if delivery_environment.eql?('delivered')

  # Previously generated config.rb files
  ny_knife_rb = '/var/opt/delivery/workspace/chef_servers/ny/config.rb'
  sf_knife_rb = '/var/opt/delivery/workspace/chef_servers/sf/config.rb'

  # ChefServer Objects
  chef_server_ny = DeliverySugar::ChefServer.new(ny_knife_rb)
  chef_server_sf = DeliverySugar::ChefServer.new(sf_knife_rb)

  delivery_chef_cookbook delivery_project do
    path delivery_workspace_repo
    chef_server [chef_server_ny, chef_server_sf]
  end
end
```
