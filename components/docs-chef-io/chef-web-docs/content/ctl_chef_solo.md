+++
title = "chef-solo (executable)"
draft = false

aliases = ["/ctl_chef_solo.html"]

[menu]
  [menu.infra]
    title = "chef-solo (executable)"
    identifier = "chef_infra/features/chef_solo/ctl_chef_solo.md chef-solo (executable)"
    parent = "chef_infra/features/chef_solo"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/ctl_chef_solo.md)

{{% chef_solo_summary %}}

## Options

This command has the following syntax:

``` bash
chef-solo OPTION VALUE OPTION VALUE ...
```

This command has the following options:

`-c CONFIG`, `--config CONFIG`

:   The configuration file to use.

`-d`, `--daemonize`

:   Run the executable as a daemon. This option may not be used in the
    same command with the `--[no-]fork` option.

    This option is only available on machines that run in UNIX or Linux
    environments. For machines that are running Microsoft Windows that
    require similar functionality, use the `chef-client::service` recipe
    in the `chef-client` cookbook:
    <https://supermarket.chef.io/cookbooks/chef-client>. This will
    install a Chef Infra Client service under Microsoft Windows using
    the Windows Service Wrapper.

`-E ENVIRONMENT_NAME`, `--environment ENVIRONMENT_NAME`

:   The name of the environment.

`-f`, `--[no-]fork`

:   Contains Chef Infra Client runs in a secondary process with
    dedicated RAM. When a Chef Infra Client run is complete, the RAM is
    returned to the master process. This option helps ensure that a Chef
    Infra Client uses a steady amount of RAM over time because the
    master process does not run recipes. This option also helps prevent
    memory leaks such as those that can be introduced by the code
    contained within a poorly designed cookbook. Use `--no-fork` to
    disable running Chef Infra Client in fork node. Default value:
    `--fork`. This option may not be used in the same command with the
    `--daemonize` and `--interval` options.

`-F FORMAT`, `--format FORMAT`

:   {{% ctl_chef_client_options_format %}}

`--force-formatter`

:   Show formatter output instead of logger output.

`--force-logger`

:   Show logger output instead of formatter output.

`-g GROUP`, `--group GROUP`

:   The name of the group that owns a process. This is required when
    starting any executable as a daemon.

`-h`, `--help`

:   Show help for the command.

`-i SECONDS`, `--interval SECONDS`

:   The frequency (in seconds) at which Chef Infra Client runs. When
    running Chef Infra Client at intervals, apply `--splay` and
    `--interval` values before a Chef Infra Client run. This option may
    not be used in the same command with the `--[no-]fork` option.

`-j PATH`, `--json-attributes PATH`

:   The path to a file that contains JSON data.

    {{< readFile_shortcode file="node_ctl_run_list.md" >}}

    {{< warning >}}

    {{< readFile_shortcode file="node_ctl_attribute.md" >}}

    {{< /warning >}}

`-l LEVEL`, `--log_level LEVEL`

:   The level of logging to be stored in a log file. Possible levels:
    `auto` (default), `debug`, `error`, `fatal`, `info`, `trace`, or `warn`.
    Default value: `warn` (when a terminal is available) or `info` (when
    a terminal is not available).

`-L LOGLOCATION`, `--logfile c`

:   The location of the log file. This is recommended when starting any
    executable as a daemon.

`--legacy-mode`

:   Cause Chef Infra Client to use the original chef-solo mode instead
    of chef local mode. This is not recommended.

    Removed in Chef Client 14.

`--minimal-ohai`

:   Run the Ohai plugins for name detection and resource/provider
    selection and no other Ohai plugins. Set to `true` during
    integration testing to speed up test cycles.

`--[no-]color`

:   View colored output. Default setting: `--color`.

`-N NODE_NAME`, `--node-name NODE_NAME`

:   The name of the node.

`-o RUN_LIST_ITEM`, `--override-runlist RUN_LIST_ITEM`

:   Replace the current run-list with the specified items.

`-r RECIPE_URL`, `--recipe-url RECIPE_URL`

:   The URL of the remote cookbook `tar.gz` file that you want to
    download.

    In Chef Client 14, the short `-r` form will be removed, as it
    conflicts with the ability to specify a run list.

`--run-lock-timeout SECONDS`

:   The amount of time (in seconds) to wait for a Chef Infra Client lock
    file to be deleted. Default value: not set (indefinite). Set to `0`
    to cause a second Chef Infra Client to exit immediately.

`-s SECONDS`, `--splay SECONDS`

:   A random number between zero and `splay` that is added to
    `interval`. Use splay to help balance the load on the Chef Infra
    Server by ensuring that many Chef Infra Client runs are not
    occurring at the same interval. When running Chef Infra Client at
    intervals, apply `--splay` and `--interval` values before a Chef
    Infra Client run.

`-u USER`, `--user USER`

:   The user that owns a process. This is required when starting any
    executable as a daemon.

`-v`, `--version`

:   The Chef Infra Client version.

`-W`, `--why-run`

:   Run the executable in why-run mode, which is a type of Chef Infra
    Client run that does everything except modify the system. Use
    why-run mode to understand the decisions that Chef Infra Client
    makes during a run and to learn more about the current and proposed
    state of the system.

## Run as Non-root User

chef-solo may be run as a non-root user. For example, the `sudoers` file
can be updated similar to:

``` ruby
# chef-solo privilege specification
chef ALL=(ALL) NOPASSWD: /usr/bin/chef-solo
```

where `chef` is the name of the non-root user. This would allow
chef-solo to run any command on the node without requiring a password.

## Examples

**Run chef-solo using solo.rb settings**

``` bash
chef-solo -c ~/chef/solo.rb
```

**Use a URL**

``` bash
chef-solo -c ~/solo.rb -j ~/node.json -r http://www.example.com/chef-solo.tar.gz
```

The tar.gz is archived into the `file_cache_path`, and then extracted to
`cookbooks_path`.

**Use a directory**

``` bash
chef-solo -c ~/solo.rb -j ~/node.json
```

chef-solo will look in the solo.rb file to determine the directory in
which cookbooks are located.

**Use a URL for cookbook and JSON data**

``` bash
chef-solo -c ~/solo.rb -j http://www.example.com/node.json --recipe-url http://www.example.com/chef-solo.tar.gz
```

where `--recipe-url` corresponds to `recipe_url` and `-j` corresponds to
`json_attribs`, both of which are [configuration
options](/config_rb_solo/) in `solo.rb`.
