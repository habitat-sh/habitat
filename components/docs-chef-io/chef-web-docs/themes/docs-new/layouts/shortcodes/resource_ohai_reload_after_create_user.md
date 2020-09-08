``` ruby
ohai 'reload_passwd' do
  action :nothing
  plugin 'etc'
end

user 'daemonuser' do
  home '/dev/null'
  shell '/sbin/nologin'
  system true
  notifies :reload, 'ohai[reload_passwd]', :immediately
end

ruby_block 'just an example' do
  block do
    # These variables will now have the new values
    puts node['etc']['passwd']['daemonuser']['uid']
    puts node['etc']['passwd']['daemonuser']['gid']
  end
end
```