The following example shows how to set up IPv4 packet forwarding using
the **execute** resource to run a command named `forward_ipv4` that uses
a template defined by the **template** resource:

``` ruby
execute 'forward_ipv4' do
  command 'echo > /proc/.../ipv4/ip_forward'
  action :nothing
end

template '/etc/file_name.conf' do
  source 'routing/file_name.conf.erb'
  notifies :run, 'execute[forward_ipv4]', :delayed
end
```

where the `command` property for the **execute** resource contains the
command that is to be run and the `source` property for the **template**
resource specifies which template to use. The `notifies` property for
the **template** specifies that the `execute[forward_ipv4]` (which is
defined by the **execute** resource) should be queued up and run at the
end of a Chef Infra Client run.