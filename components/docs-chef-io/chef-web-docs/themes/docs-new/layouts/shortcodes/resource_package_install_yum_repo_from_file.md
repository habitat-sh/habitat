The following example shows how to install new Yum repositories from a
file, where the installation of the repository triggers a creation of
the Yum cache that forces the internal cache for Chef Infra Client to
reload:

``` ruby
execute 'create-yum-cache' do
 command 'yum -q makecache'
 action :nothing
end

ruby_block 'reload-internal-yum-cache' do
  block do
    Chef::Provider::Package::Yum::YumCache.instance.reload
  end
  action :nothing
end

cookbook_file '/etc/yum.repos.d/custom.repo' do
  source 'custom'
  mode '0755'
  notifies :run, 'execute[create-yum-cache]', :immediately
  notifies :create, 'ruby_block[reload-internal-yum-cache]', :immediately
end
```