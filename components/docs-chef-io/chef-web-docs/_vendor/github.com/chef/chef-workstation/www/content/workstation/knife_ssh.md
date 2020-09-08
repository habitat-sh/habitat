+++
title = "knife ssh"
draft = false

aliases = ["/knife_ssh.html", "/knife_ssh/"]

[menu]
  [menu.workstation]
    title = "knife ssh"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_ssh.md knife ssh"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_ssh.md)

{{% knife_ssh_summary %}}

## Syntax

This subcommand has the following syntax:

``` bash
knife ssh SEARCH_QUERY SSH_COMMAND (options)
```

## Options

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

This subcommand has the following options:

`-a SSH_ATTR`, `--attribute SSH_ATTR`

:   The attribute used when opening an SSH connection. The default
    attribute is the FQDN of the host. Other possible values include a
    public IP address, a private IP address, or a hostname.

`-A`, `--forward-agent`

:   Enable SSH agent forwarding.

`-C NUM`, `--concurrency NUM`

:   The number of allowed concurrent connections.

`-e`, `--exit-on-error`

:   Use to exit immediately upon error.

`-G GATEWAY`, `--ssh-gateway GATEWAY`

:   The SSH tunnel or gateway that is used to run a bootstrap action on
    a machine that is not accessible from the workstation.

`--ssh-gateway-identity SSH_GATEWAY_IDENTITY`

:   The SSH identity file used to connect to the SSH gateway.

    *New in Chef Client 13.0.*

`-i IDENTITY_FILE`, `--ssh-identity-file IDENTIFY_FILE`

:   The SSH identity file used for authentication. Key-based
    authentication is recommended.

`-m`, `--manual-list`

:   Define a search query as a space-separated list of servers. If there
    is more than one item in the list, put quotes around the entire
    list. For example: `--manual-list "server01 server02 server03"`

`--[no-]host-key-verify`

:   Use `--no-host-key-verify` to disable host key verification. Default
    setting: `--host-key-verify`.

`OTHER`

:   The shell type. Possible values: `interactive`, `screen`, `tmux`,
    `macterm`, or `cssh`. (`csshx` is deprecated in favor of `cssh`.)

`-p PORT`, `--ssh-port PORT`

:   The SSH port.

`-P PASSWORD`, `--ssh-password PASSWORD`

:   The SSH password. This can be used to pass the password directly on
    the command line. If this option is not specified (and a password is
    required) knife prompts for the password.

`SEARCH_QUERY`

:   The search query used to return a list of servers to be accessed
    using SSH and the specified `SSH_COMMAND`. This option uses the same
    syntax as the search subcommand. If the `SEARCH_QUERY` does not
    contain a colon character (`:`), then the default query pattern is
    `tags:*#{@query}* OR roles:*#{@query}* OR fqdn:*#{@query}* OR addresses:*#{@query}*`,
    which means the following two search queries are effectively the
    same:

<!-- -->

``` bash
knife search ubuntu
```

or:

``` bash
knife search node "tags:*ubuntu* OR roles:*ubuntu* OR fqdn:*ubuntu* (etc.)"
```

`SSH_COMMAND`

:   The command to be run against the results of a search query.

`-t SECONDS`, `--ssh-timeout SECONDS`

:   The amount of time (in seconds) to wait for an SSH connection time
    out.

`--tmux-split`

:   Split the Tmux window. Default value: `false`.

`-x USER_NAME`, `--ssh-user USER_NAME`

:   The SSH user name.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

## Examples

The following examples show how to use this knife subcommand:

**Find server uptime**

To find the uptime of all of web servers running Ubuntu on the Amazon
EC2 platform, enter:

``` bash
knife ssh "role:web" "uptime" -x ubuntu -a ec2.public_hostname
```

to return something like:

``` bash
ec2-174-129-127-206.compute-1.amazonaws.com  13:50:47 up 1 day, 23:26,  1 user,  load average: 0.25, 0.18, 0.11
ec2-67-202-63-102.compute-1.amazonaws.com    13:50:47 up 1 day, 23:33,  1 user,  load average: 0.12, 0.13, 0.10
ec2-184-73-9-250.compute-1.amazonaws.com     13:50:48 up 16:45,  1 user,  load average: 0.30, 0.22, 0.13
ec2-75-101-240-230.compute-1.amazonaws.com   13:50:48 up 1 day, 22:59,  1 user,  load average: 0.24, 0.17, 0.11
ec2-184-73-60-141.compute-1.amazonaws.com    13:50:48 up 1 day, 23:30,  1 user,  load average: 0.32, 0.17, 0.15
```

**Run Chef Infra Client on all nodes**

``` bash
knife ssh 'name:*' 'sudo chef-client'
```

**Force a Chef Infra Client run**

To force a Chef Infra Client run on all of the web servers running
Ubuntu on the Amazon EC2 platform, enter:

``` bash
knife ssh "role:web" "sudo chef-client" -x ubuntu -a ec2.public_hostname
```

to return something like:

``` bash
ec2-67-202-63-102.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:37 +0000] INFO: Starting Chef Run (Version 0.9.10)
ec2-174-129-127-206.compute-1.amazonaws.com [Fri, 22 Oct 2010 14:18:37 +0000] INFO: Starting Chef Run (Version 0.9.10)
ec2-184-73-9-250.compute-1.amazonaws.com    [Fri, 22 Oct 2010 14:18:38 +0000] INFO: Starting Chef Run (Version 0.9.10)
ec2-75-101-240-230.compute-1.amazonaws.com  [Fri, 22 Oct 2010 14:18:38 +0000] INFO: Starting Chef Run (Version 0.9.10)
ec2-184-73-60-141.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:38 +0000] INFO: Starting Chef Run (Version 0.9.10)
ec2-174-129-127-206.compute-1.amazonaws.com [Fri, 22 Oct 2010 14:18:39 +0000] INFO: Chef Run complete in 1.419243 seconds
ec2-174-129-127-206.compute-1.amazonaws.com [Fri, 22 Oct 2010 14:18:39 +0000] INFO: cleaning the checksum cache
ec2-174-129-127-206.compute-1.amazonaws.com [Fri, 22 Oct 2010 14:18:39 +0000] INFO: Running report handlers
ec2-174-129-127-206.compute-1.amazonaws.com [Fri, 22 Oct 2010 14:18:39 +0000] INFO: Report handlers complete
ec2-67-202-63-102.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:39 +0000] INFO: Chef Run complete in 1.578265 seconds
ec2-67-202-63-102.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:39 +0000] INFO: cleaning the checksum cache
ec2-67-202-63-102.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:39 +0000] INFO: Running report handlers
ec2-67-202-63-102.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:39 +0000] INFO: Report handlers complete
ec2-184-73-9-250.compute-1.amazonaws.com    [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Chef Run complete in 1.638884 seconds
ec2-184-73-9-250.compute-1.amazonaws.com    [Fri, 22 Oct 2010 14:18:40 +0000] INFO: cleaning the checksum cache
ec2-184-73-9-250.compute-1.amazonaws.com    [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Running report handlers
ec2-184-73-9-250.compute-1.amazonaws.com    [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Report handlers complete
ec2-75-101-240-230.compute-1.amazonaws.com  [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Chef Run complete in 1.540257 seconds
ec2-75-101-240-230.compute-1.amazonaws.com  [Fri, 22 Oct 2010 14:18:40 +0000] INFO: cleaning the checksum cache
ec2-75-101-240-230.compute-1.amazonaws.com  [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Running report handlers
ec2-75-101-240-230.compute-1.amazonaws.com  [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Report handlers complete
ec2-184-73-60-141.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Chef Run complete in 1.502489 seconds
ec2-184-73-60-141.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:40 +0000] INFO: cleaning the checksum cache
ec2-184-73-60-141.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Running report handlers
ec2-184-73-60-141.compute-1.amazonaws.com   [Fri, 22 Oct 2010 14:18:40 +0000] INFO: Report handlers complete
```

**Run a command based on search query**

To query for all nodes that have the `webserver` role and then use SSH
to run the command `sudo chef-client`, enter:

``` bash
knife ssh "role:webserver" "sudo chef-client"
```

**Upgrade all nodes**

``` bash
knife ssh name:* "sudo aptitude upgrade -y"
```

**Specify the shell type**

To specify the shell type used on the nodes returned by a search query:

``` bash
knife ssh roles:opscode-omnitruck macterm
```

where `screen` is one of the following values: `cssh`, `interactive`,
`macterm`, `screen`, or `tmux`. If the node does not have the shell type
installed, knife will return an error similar to the following:

``` bash
you need the rb-appscript gem to use knife ssh macterm.
`(sudo) gem    install rb-appscript` to install
ERROR: LoadError: cannot load such file -- appscript
```
