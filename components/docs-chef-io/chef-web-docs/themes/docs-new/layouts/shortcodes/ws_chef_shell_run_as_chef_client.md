By default, chef-shell loads in standalone mode and does not connect to
the Chef Infra Server. The chef-shell can be run as a Chef Infra Client
to verify functionality that is only available when Chef Infra Client
connects to the Chef Infra Server, such as search functionality or
accessing data stored in data bags.

chef-shell can use the same credentials as knife when connecting to a
Chef Infra Server. Make sure that the settings in chef-shell.rb are the
same as those in config.rb, and then use the `-z` option as part of the
command. For example:

``` bash
chef-shell -z
```