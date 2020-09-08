``` ruby
# the following code sample thanks to gaffneyc @ https://gist.github.com/918711

execute 'clean-yum-cache' do
  command 'yum clean all'
  action :nothing
end

file '/etc/yum.repos.d/bad.repo' do
  action :delete
  notifies :run, 'execute[clean-yum-cache]', :immediately
  notifies :create, 'ruby_block[reload-internal-yum-cache]', :immediately
end
```