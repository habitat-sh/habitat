``` ruby
template '/etc/www/configures-apache.conf' do
  notifies :restart, 'service[apache]', :immediately
end
```