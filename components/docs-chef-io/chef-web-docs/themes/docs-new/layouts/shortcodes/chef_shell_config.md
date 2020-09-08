chef-shell determines which configuration file to load based on the
following:

1.  If a configuration file is specified using the `-c` option,
    chef-shell will use the specified configuration file
2.  If a NAMED_CONF is given, chef-shell will load
    \~/.chef/NAMED_CONF/chef_shell.rb
3.  If no NAMED_CONF is given chef-shell will load
    \~/.chef/chef_shell.rb if it exists
4.  If no chef_shell.rb can be found, chef-shell falls back to load:
    -   /etc/chef/client.rb if -z option is given.
    -   /etc/chef/solo.rb if --solo-legacy-mode option is given.
    -   .chef/config.rb if -s option is given.
    -   .chef/knife.rb if -s option is given.