The `/etc/opscode/chef-server.rb` file contains all of the non-default
configuration settings used by the Chef Infra Server. The default
settings are built into the Chef Infra Server configuration and should
only be added to the `chef-server.rb` file to apply non-default values.
These configuration settings are processed when the
`chef-server-ctl reconfigure` command is run. The `chef-server.rb` file
is a Ruby file, which means that conditional statements can be used
within it.