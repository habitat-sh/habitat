To send notifications to multiple resources, just use multiple
attributes. Multiple attributes will get sent to the notified resources
in the order specified.

``` ruby
template '/etc/netatalk/netatalk.conf' do
  notifies :restart, 'service[afpd]', :immediately
  notifies :restart, 'service[cnid]', :immediately
end

service 'afpd'
service 'cnid'
```