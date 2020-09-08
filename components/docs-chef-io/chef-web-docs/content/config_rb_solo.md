+++
title = "solo.rb"
draft = false

aliases = ["/config_rb_solo.html"]

[menu]
  [menu.infra]
    title = "solo.rb"
    identifier = "chef_infra/features/chef_solo/config_rb_solo.md solo.rb"
    parent = "chef_infra/features/chef_solo"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/config_rb_solo.md)

A solo.rb file is used to specify the configuration details for
chef-solo.

-   This file is loaded every time this executable is run
-   The default location in which chef-solo expects to find this file is
    `/etc/chef/solo.rb`; use the `--config` option from the command line
    to change this location
-   This file is not created by default
-   When a `solo.rb` file is present in this directory, the settings
    contained within that file will override the default configuration
    settings

## Settings

This configuration file has the following settings:

`add_formatter`

:   A 3rd-party formatter. (See
    [nyan-cat](https://github.com/andreacampi/nyan-cat-chef-formatter)
    for an example of a 3rd-party formatter.) Each formatter requires
    its own entry.

`checksum_path`

:   The location in which checksum files are stored. These are used to
    validate individual cookbook files, such as recipes. The checksum
    itself is stored in the Chef Infra Server database and is then
    compared to a file in the checksum path that has a filename
    identical to the checksum.

`cookbook_path`

:   The Chef Infra Client sub-directory for cookbooks. This value can be
    a string or an array of file system locations, processed in the
    specified order. The last cookbook is considered to override local
    modifications.

`data_bag_path`

:   The location from which a data bag is loaded. Default value:
    `/var/chef/data_bags`.

`environment`

:   The name of the environment.

`environment_path`

:   The path to the environment. Default value:
    `/var/chef/environments`.

`file_backup_path`

:   The location in which backup files are stored. If this value is
    empty, backup files are stored in the directory of the target file.
    Default value: `/var/chef/backup`.

`file_cache_path`

:   The location in which cookbooks (and other transient data) files are
    stored when they are synchronized. This value can also be used in
    recipes to download files with the **remote_file** resource.

`json_attribs`

:   The path to a file that contains JSON data.

`lockfile`

:   The location of the Chef Infra Client lock file. This value is
    typically platform-dependent, so should be a location defined by
    `file_cache_path`. The default location of a lock file should not on
    an NF mount. Default value: a location defined by `file_cache_path`.

`log_level`

:   The level of logging to be stored in a log file. Possible levels:
    `:auto` (default), `debug`, `info`, `warn`, `error`, or `fatal`.

`log_location`

:   The location of the log file. Default value: `STDOUT`.

`minimal_ohai`

:   Run the Ohai plugins for name detection and resource/provider
    selection and no other Ohai plugins. Set to `true` during
    integration testing to speed up test cycles.

`node_name`

:   The name of the node.

`recipe_url`

:   The URL location from which a remote cookbook tar.gz is to be
    downloaded.

`rest_timeout`

:   The time (in seconds) after which an HTTP REST request is to time
    out. Default value: `300`.

`role_path`

:   The location in which role files are located. Default value:
    `/var/chef/roles`.

`run_lock_timeout`

:   The amount of time (in seconds) to wait for a Chef Infra Client lock
    file to be deleted. A Chef Infra Client run will not start when a
    lock file is present. If a lock file is not deleted before this time
    expires, the pending Chef Infra Client run will exit. Default value:
    not set (indefinite). Set to `0` to cause a second Chef Infra Client
    to exit immediately.

`sandbox_path`

:   The location in which cookbook files are stored (temporarily) during
    upload.

`solo`

:   Run Chef Infra Client in chef-solo mode. This setting determines if
    Chef Infra Client is to attempt to communicate with the Chef Infra
    Server. Default value: `false`.

`syntax_check_cache_path`

:   All files in a cookbook must contain valid Ruby syntax. Use this
    setting to specify the location in which knife caches information
    about files that have been checked for valid Ruby syntax.

`umask`

:   The file mode creation mask, or umask. Default value: `0022`.

`verbose_logging`

:   Set the log level. Options: `true`, `nil`, and `false`. When this is
    set to `false`, notifications about individual resources being
    processed are suppressed (and are output at the `:info` logging
    level). Setting this to `false` can be useful when a Chef Infra
    Client is run as a daemon. Default value: `nil`.

## Examples

**Using Chef Automate Data Collector**

This example solo.rb file uses the `data_collector` settings to send
data to an available Chef Automate system. Since Chef Automate generates
a self-signed SSL certificate by default, you will need to add the
certificate (located under `/var/opt/delivery/nginx/` on the Chef
Automate server) to your `trusted_certs_dir` directory, as seen in this
example:

``` ruby
chef_server_url  "https://localhost:8989"
log_location   STDOUT
node_name "YOUR_NODES_FQDN"
trusted_certs_dir "/etc/chef/trusted_certs"

data_collector.server_url "https://YOUR_AUTOMATE_FQDN/data-collector/v0"
data_collector.mode       :both
data_collector.token = "YOURTOKEN"
```

You can run it like this

``` ruby
chef-solo -c solo.rb
```

**All Options**

A sample solo.rb file that contains all possible settings (listed
alphabetically):

``` ruby
add_formatter :nyan
add_formatter :foo
add_formatter :bar
checksum_path '/var/chef/checksums'
cookbook_path [
               '/var/chef/cookbooks',
               '/var/chef/site-cookbooks'
              ]
data_bag_path '/var/chef/data_bags'
environment 'production'
environment_path '/var/chef/environments'
file_backup_path '/var/chef/backup'
file_cache_path '/var/chef/cache'
json_attribs nil
lockfile nil
log_level :info
log_location STDOUT
node_name 'mynode.example.com'
recipe_url 'http://path/to/remote/cookbook'
rest_timeout 300
role_path '/var/chef/roles'
sandbox_path 'path_to_folder'
solo false
syntax_check_cache_path
umask 0022
verbose_logging nil
```
