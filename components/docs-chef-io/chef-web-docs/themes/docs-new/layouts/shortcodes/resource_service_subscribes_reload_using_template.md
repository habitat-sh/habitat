To reload a service that is based on a template, use the **template**
and **service** resources together in the same recipe, similar to the
following:

``` ruby
template '/tmp/somefile' do
  mode '0755'
  source 'somefile.erb'
end

service 'apache' do
  action :enable
  subscribes :reload, 'template[/tmp/somefile]', :immediately
end
```

where the `subscribes` notification is used to reload the service
whenever the template is modified.