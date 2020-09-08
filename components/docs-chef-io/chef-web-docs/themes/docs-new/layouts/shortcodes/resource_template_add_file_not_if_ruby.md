The following example shows how to use the `not_if` condition to create
a file based on a template and then Ruby code to specify the condition:

``` ruby
template '/tmp/somefile' do
  mode '0755'
  source 'somefile.erb'
  not_if do
    File.exist?('/etc/passwd')
  end
end
```