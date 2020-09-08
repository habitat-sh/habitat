The following is an example of using the `platform_family?` method in
the Recipe DSL to create a variable that can be used with other
resources in the same recipe. In this example, `platform_family?` is
being used to ensure that a specific binary is used for a specific
platform before using the **remote_file** resource to download a file
from a remote location, and then using the **execute** resource to
install that file by running a command.

``` ruby
if platform_family?('rhel')
  pip_binary = '/usr/bin/pip'
else
  pip_binary = '/usr/local/bin/pip'
end

remote_file "#{Chef::Config[:file_cache_path]}/distribute_setup.py" do
  source 'http://python-distribute.org/distribute_setup.py'
  mode '0755'
  not_if { File.exist?(pip_binary) }
end

execute 'install-pip' do
  cwd Chef::Config[:file_cache_path]
  command <<-EOF
    # command for installing Python goes here
    EOF
  not_if { File.exist?(pip_binary) }
end
```

where a command for installing Python might look something like:

``` ruby
#{node['python']['binary']} distribute_setup.py
#{::File.dirname(pip_binary)}/easy_install pip
```