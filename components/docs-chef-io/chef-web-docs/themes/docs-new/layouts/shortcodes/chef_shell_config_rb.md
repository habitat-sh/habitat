The chef-shell.rb file can be used to configure chef-shell in the same
way as the client.rb file is used to configure Chef Infra Client. For
example, to configure chef-shell to authenticate to the Chef Infra
Server, copy the `node_name`, `client_key`, and `chef_server_url`
settings from the config.rb file:

``` ruby
node_name                'your-knife-clientname'
client_key               File.expand_path('~/.chef/my-client.pem')
chef_server_url          'https://api.opscode.com/organizations/myorg'
```

and then add them to the chef-shell.rb file. Other configuration
possibilities include disabling Ohai plugins (which will speed up the
chef-shell boot process) or including arbitrary Ruby code in the
chef-shell.rb file.