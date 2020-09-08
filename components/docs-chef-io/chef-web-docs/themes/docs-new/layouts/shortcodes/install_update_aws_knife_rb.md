Open `.chef/config.rb` in a text editor and modify the `chef_server_url`
with your new public DNS. For example:

``` bash
vim ~/chef-repo/.chef/config.rb
```

will open a `config.rb` file similar to:

``` ruby
current_dir = ::File.dirname(__FILE__)
log_level                :info
log_location             $stdout
node_name                'your_username'
client_key               "#{current_dir}/your_username.pem"
validation_client_name   'your_orgname-validator'
validation_key           "#{current_dir}/your_orgname-validator.pem"
chef_server_url          'https://<YOUR NEW PUBLIC DNS>/organizations/your_org'
cookbook_path            ["#{current_dir}/../cookbooks"]
```