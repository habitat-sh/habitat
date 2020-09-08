A template named `gemrc.erb` is located in a cookbook's `/templates`
directory:

``` ruby
:sources:
- http://<%= node['gem_file']['host'] %>:<%= node['gem_file']['port'] %>/
```

A recipe can be built that does the following:

-   Builds a `.gemrc` file based on a `gemrc.erb` template
-   Runs a `Gem.configuration` command
-   Installs a package using the `.gemrc` file

<!-- -->

``` ruby
template '/root/.gemrc' do
  source 'gemrc.erb'
  action :create
  notifies :run, 'ruby_block[refresh_gemrc]', :immediately
end

ruby_block 'refresh_gemrc' do
  action :nothing
  block do
    Gem.configuration = Gem::ConfigFile.new []
  end
end

gem_package 'di-ruby-lvm' do
  gem_binary '/opt/chef/embedded/bin/gem'
  action :install
end
```