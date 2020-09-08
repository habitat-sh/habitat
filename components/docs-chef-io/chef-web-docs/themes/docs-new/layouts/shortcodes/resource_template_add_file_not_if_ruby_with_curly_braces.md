The following example shows how to use the `not_if` condition to create
a file based on a template and using a Ruby block (with curly braces) to
specify the condition:

``` ruby
template '/tmp/somefile' do
  mode '0755'
  source 'somefile.erb'
  not_if { File.exist?('/etc/passwd') }
end
```