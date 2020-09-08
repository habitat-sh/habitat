The following example shows how to use the `only_if` condition to create
a file based on a template, and then use Ruby to specify a condition:

``` ruby
template '/tmp/somefile' do
  mode '0755'
  source 'somefile.erb'
  only_if { ! ::File.exist?('/etc/passwd') }
end
```