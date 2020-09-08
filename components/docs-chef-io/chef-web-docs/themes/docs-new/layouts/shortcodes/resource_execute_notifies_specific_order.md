To notify multiple resources, and then have these resources run in a
certain order, do something like the following:

``` ruby
execute 'foo' do
  command '...'
  notifies :create, 'template[baz]', :immediately
  notifies :install, 'package[bar]', :immediately
  notifies :run, 'execute[final]', :immediately
end

template 'baz' do
  ...
  notifies :run, 'execute[restart_baz]', :immediately
end

package 'bar'

execute 'restart_baz'

execute 'final' do
  command '...'
end
```

where the sequencing will be in the same order as the resources are
listed in the recipe: `execute 'foo'`, `template 'baz'`,
`execute [restart_baz]`, `package 'bar'`, and `execute 'final'`.