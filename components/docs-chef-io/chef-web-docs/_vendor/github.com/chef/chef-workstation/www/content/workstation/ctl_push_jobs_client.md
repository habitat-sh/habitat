+++
title = "push-jobs-client (executable)"
draft = false

aliases = ["/ctl_push_jobs_client.html", "/ctl_push_jobs_client/"]

[menu]
  [menu.workstation]
    title = "push-jobs-client (executable)"
    identifier = "chef_workstation/chef_workstation_tools/ctl_push_jobs_client.md push-jobs-client (executable)"
    parent = "chef_workstation/chef_workstation_tools"
    weight = 140
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/ctl_push_jobs_client.md)

{{% push_jobs_summary %}}

{{% ctl_push_jobs_client_summary %}}

## Options

This command has the following syntax:

    push-jobs-client OPTION VALUE OPTION VALUE ...

This command has the following options:

`--allow_unencrypted`

:   Allow unencrypted connections to 1.x servers

`-c CONFIG`, `--config CONFIG`

:   The configuration file to use. The Chef Infra Client and Chef Push
    Jobs client use the same configuration file: client.rb. Default
    value: `Chef::Config.platform_specific_path("/etc/chef/client.rb")`.

`-d DIR`, `-file_dir DIR`

:   Set the directory for temporary files. Default value:
    `/tmp/chef-push`.

`-h`, `--help`

:   Show help for the command.

`-k KEY_FILE`, `--client-key KEY_FILE`

:   The location of the file that contains the client key.

`-l LEVEL`, `--log_level LEVEL`

:   The level of logging to be stored in a log file.

`-L LOCATION`, `--logfile LOCATION`

:   The location of the log file. This is recommended when starting any
    executable as a daemon.

`-N NODE_NAME`, `--node-name NODE_NAME`

:   The name of the node.

`-S URL`, `--server URL`

:   The URL for the Chef Infra Server.

`-v`, `--version`

:   The version of Chef Push Jobs.

## Examples

None.
