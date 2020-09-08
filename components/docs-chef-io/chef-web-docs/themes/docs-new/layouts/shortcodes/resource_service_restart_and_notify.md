The following example shows how start a service named `example_service`
and immediately notify the Nginx service to restart.

``` ruby
service 'example_service' do
  action :start
  notifies :restart, 'service[nginx]', :immediately
end
```