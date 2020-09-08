The following example shows how the **template** resource can be used in
a recipe to combine settings stored in an attributes file, variables
within a recipe, and a template to set the IP addresses that are used by
the Nginx service. The attributes file contains the following:

``` ruby
default['nginx']['dir'] = '/etc/nginx'
```

The recipe then does the following to:

-   Declare two variables at the beginning of the recipe, one for the
    remote IP address and the other for the authorized IP address
-   Use the **service** resource to restart and reload the Nginx service
-   Load a template named `authorized_ip.erb` from the `/templates`
    directory that is used to set the IP address values based on the
    variables specified in the recipe

<!-- -->

``` ruby
node.default['nginx']['remote_ip_var'] = 'remote_addr'
node.default['nginx']['authorized_ips'] = ['127.0.0.1/32']

service 'nginx' do
  supports :status => true, :restart => true, :reload => true
end

template 'authorized_ip' do
  path "#{node['nginx']['dir']}/authorized_ip"
  source 'modules/authorized_ip.erb'
  owner 'root'
  group 'root'
  mode '0755'
  variables(
    :remote_ip_var => node['nginx']['remote_ip_var'],
    :authorized_ips => node['nginx']['authorized_ips']
  )

  notifies :reload, 'service[nginx]', :immediately
end
```

where the `variables` property tells the template to use the variables
set at the beginning of the recipe and the `source` property is used to
call a template file located in the cookbook's `/templates` directory.
The template file looks similar to:

``` ruby
geo $<%= @remote_ip_var %> $authorized_ip {
  default no;
  <% @authorized_ips.each do |ip| %>
  <%= "#{ip} yes;" %>
  <% end %>
}
```