``` ruby
template '/etc/chef/server.rb' do
  source 'server.rb.erb'
  owner 'root'
  group 'root'
  mode '0755'
  notifies :restart, 'service[chef-solr]', :delayed
  notifies :restart, 'service[chef-solr-indexer]', :delayed
  notifies :restart, 'service[chef-server]', :delayed
end
```