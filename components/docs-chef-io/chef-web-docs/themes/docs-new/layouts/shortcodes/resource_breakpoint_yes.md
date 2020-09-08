``` ruby
breakpoint "before yum_key node['yum']['repo_name']['key']" do
  action :break
end

yum_key node['yum']['repo_name']['key'] do
  url  node['yum']['repo_name']['key_url']
  action :add
end

breakpoint "after yum_key node['yum']['repo_name']['key']" do
  action :break
end

breakpoint "before yum_repository 'repo_name'" do
  action :break
end

yum_repository 'repo_name' do
  description 'description'
  key node['yum']['repo_name']['key']
  mirrorlist node['yum']['repo_name']['url']
  includepkgs node['yum']['repo_name']['includepkgs']
  exclude node['yum']['repo_name']['exclude']
  action :create
end

breakpoint "after yum_repository 'repo_name'" do
  action :break
end
```

where the name of each breakpoint is an arbitrary string. In the
previous examples, the names are used to indicate if the breakpoint is
before or after a resource, and then also to specify which resource.