The following recipe shows how an attributes file can be used to store
certain settings. An attributes file is located in the `attributes/`
directory in the same cookbook as the recipe which calls the attributes
file. In this example, the attributes file specifies certain settings
for Python that are then used across all nodes against which this recipe
will run.

Python packages have versions, installation directories, URLs, and
checksum files. An attributes file that exists to support this type of
recipe would include settings like the following:

``` ruby
default['python']['version'] = '2.7.1'

if python['install_method'] == 'package'
  default['python']['prefix_dir'] = '/usr'
else
  default['python']['prefix_dir'] = '/usr/local'
end

default['python']['url'] = 'http://www.python.org/ftp/python'
default['python']['checksum'] = '80e387...85fd61'
```

and then the methods in the recipe may refer to these values. A recipe
that is used to install Python will need to do the following:

-   Identify each package to be installed (implied in this example, not
    shown)
-   Define variables for the package `version` and the `install_path`
-   Get the package from a remote location, but only if the package does
    not already exist on the target system
-   Use the **bash** resource to install the package on the node, but
    only when the package is not already installed

<!-- -->

``` ruby
#  the following code sample comes from the ``oc-nginx`` cookbook on |github|: https://github.com/cookbooks/oc-nginx

version = node['python']['version']
install_path = "#{node['python']['prefix_dir']}/lib/python#{version.split(/(^\d+\.\d+)/)[1]}"

remote_file "#{Chef::Config[:file_cache_path]}/Python-#{version}.tar.bz2" do
  source "#{node['python']['url']}/#{version}/Python-#{version}.tar.bz2"
  checksum node['python']['checksum']
  mode '0755'
  not_if { ::File.exist?(install_path) }
end

bash 'build-and-install-python' do
  cwd Chef::Config[:file_cache_path]
  code <<-EOF
    tar -jxvf Python-#{version}.tar.bz2
    (cd Python-#{version} && ./configure #{configure_options})
    (cd Python-#{version} && make && make install)
  EOF
  not_if { ::File.exist?(install_path) }
end
```