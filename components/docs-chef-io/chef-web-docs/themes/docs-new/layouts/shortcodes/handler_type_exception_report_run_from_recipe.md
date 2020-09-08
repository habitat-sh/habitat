The **chef_handler** resource allows exception and report handlers to
be enabled from within recipes, which can then added to the run-list for
any node on which the exception or report handler should run. The
**chef_handler** resource is available from the **chef_handler**
cookbook.

To use the **chef_handler** resource in a recipe, add code similar to
the following:

``` ruby
chef_handler 'name_of_handler' do
  source '/path/to/handler/handler_name'
  action :enable
end
```

For example, a handler for Growl needs to be enabled at the beginning of
a Chef Infra Client run:

``` ruby
chef_gem 'chef-handler-growl'
```

and then is activated in a recipe by using the **chef_handler**
resource:

``` ruby
chef_handler 'Chef::Handler::Growl' do
  source 'chef/handler/growl'
  action :enable
end
```