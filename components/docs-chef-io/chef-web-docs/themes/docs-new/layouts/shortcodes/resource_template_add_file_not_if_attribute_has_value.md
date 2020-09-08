The following example shows how to use the `not_if` condition to create
a file based on a template and using the presence of an attribute value
on the node to specify the condition:

``` ruby
template '/tmp/somefile' do
  mode '0755'
  source 'somefile.erb'
  not_if { node['some_value'] }
end
```