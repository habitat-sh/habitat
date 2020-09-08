The config.rb configuration can include arbitrary Ruby code to extend
configuration beyond static values. This can be used to load
environmental variables from the workstation. This makes it possible to
write a single config.rb file that can be used by all users within your
organization. This single file can also be checked into your chef-repo,
allowing users to load different config.rb files based on which
chef-repo they execute the commands from. This can be especially useful
when each chef-repo points to a different chef server or organization.

Example config.rb:

``` none
current_dir = File.dirname(__FILE__)
  user = ENV['CHEF_USER'] || ENV['USER']
  node_name                user
  client_key               "#{ENV['HOME']}/chef-repo/.chef/#{user}.pem"
  validation_client_name   "#{ENV['ORGNAME']}-validator"
  validation_key           "#{ENV['HOME']}/chef-repo/.chef/#{ENV['ORGNAME']}-validator.pem"
  chef_server_url          "https://api.opscode.com/organizations/#{ENV['ORGNAME']}"
  syntax_check_cache_path  "#{ENV['HOME']}/chef-repo/.chef/syntax_check_cache"
  cookbook_path            ["#{current_dir}/../cookbooks"]
  cookbook_copyright       "Your Company, Inc."
  cookbook_license         "Apache-2.0"
  cookbook_email           "cookbooks@yourcompany.com"

  # Amazon AWS
  knife[:aws_access_key_id] = ENV['AWS_ACCESS_KEY_ID']
  knife[:aws_secret_access_key] = ENV['AWS_SECRET_ACCESS_KEY']
```