The following is an example of how to install the `foo123` module for
Nginx. This module adds shell-style functionality to an Nginx
configuration file and does the following:

-   Declares three variables
-   Gets the Nginx file from a remote location
-   Installs the file using Bash to the path specified by the
    `src_filepath` variable

<!-- -->

``` ruby
# the following code sample is similar to the ``upload_progress_module``
# recipe in the ``nginx`` cookbook:
# https://github.com/chef-cookbooks/nginx

src_filename = "foo123-nginx-module-v#{
  node['nginx']['foo123']['version']
}.tar.gz"
src_filepath = "#{Chef::Config['file_cache_path']}/#{src_filename}"
extract_path = "#{
  Chef::Config['file_cache_path']
  }/nginx_foo123_module/#{
  node['nginx']['foo123']['checksum']
}"

remote_file 'src_filepath' do
  source node['nginx']['foo123']['url']
  checksum node['nginx']['foo123']['checksum']
  owner 'root'
  group 'root'
  mode '0755'
end

bash 'extract_module' do
  cwd ::File.dirname(src_filepath)
  code <<-EOH
    mkdir -p #{extract_path}
    tar xzf #{src_filename} -C #{extract_path}
    mv #{extract_path}/*/* #{extract_path}/
    EOH
  not_if { ::File.exist?(extract_path) }
end
```