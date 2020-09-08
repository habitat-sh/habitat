When `%w` syntax uses a variable, such as `|foo|`, double quoted strings
should be used.

Right:

``` ruby
%w(openssl.cnf pkitool vars Rakefile).each do |foo|
  template "/etc/openvpn/easy-rsa/#{foo}" do
    source "#{foo}.erb"
    ...
  end
end
```

Wrong:

``` ruby
%w(openssl.cnf pkitool vars Rakefile).each do |foo|
  template '/etc/openvpn/easy-rsa/#{foo}' do
    source '#{foo}.erb'
    ...
  end
end
```