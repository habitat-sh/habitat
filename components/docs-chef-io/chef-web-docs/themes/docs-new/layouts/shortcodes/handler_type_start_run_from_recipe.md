The **chef-client** cookbook can be configured to automatically install
and configure gems that are required by a start handler. For example:

``` ruby
node.override['chef_client']['load_gems']['chef-reporting'] = {
  :require_name => 'chef_reporting',
  :action => :install
}

node.override['chef_client']['config']['start_handlers'] = [
  {
    :class => 'Chef::Reporting::StartHandler',
    :arguments => []
  }
]

include_recipe 'chef-client::config'
```