The following example shows how to use the `only_if` condition to create
a file based on a template and using the presence of an attribute on the
node to specify the condition:

``` ruby
template '/tmp/somefile' do
  mode '0755'
  source 'somefile.erb'
  only_if { node['some_value'] }
end
```