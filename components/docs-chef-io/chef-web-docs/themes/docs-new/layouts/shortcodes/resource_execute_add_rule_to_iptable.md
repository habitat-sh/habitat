The following example shows how to add a rule named `test_rule` to an IP
table using the **execute** resource to run a command using a template
that is defined by the **template** resource:

``` ruby
execute 'test_rule' do
  command 'command_to_run
    --option value
    ...
    --option value
    --source #{node[:name_of_node][:ipsec][:local][:subnet]}
    -j test_rule'
  action :nothing
end

template '/etc/file_name.local' do
  source 'routing/file_name.local.erb'
  notifies :run, 'execute[test_rule]', :delayed
end
```

where the `command` property for the **execute** resource contains the
command that is to be run and the `source` property for the **template**
resource specifies which template to use. The `notifies` property for
the **template** specifies that the `execute[test_rule]` (which is
defined by the **execute** resource) should be queued up and run at the
end of a Chef Infra Client run.