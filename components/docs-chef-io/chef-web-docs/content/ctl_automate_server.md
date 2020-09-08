+++
title = "automate-ctl (executable)"
draft = false
robots = "noindex"


aliases = ["/ctl_automate_server.html", "/release/automate/ctl_delivery_server.html"]

[menu]
  [menu.legacy]
    title = "Chef Automate CTL (Deprecated)"
    identifier = "legacy/workflow/ctl_automate_server.md Chef Automate CTL (Deprecated)"
    parent = "legacy/workflow"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/ctl_automate_server.md)



{{% chef_automate_mark %}}

{{% EOL_a1 %}}

The Chef Automate server includes a command-line utility named
`automate-ctl`. Use this command-line tool to:

-   Manage enterprises, users, and projects
-   Reconfigure the Chef Automate server
-   Start and stop individual services
-   Tail Chef Automate server log files

{{< note >}}

All commands must be run as `sudo`.

{{< /note >}}

{{% delivery_ctl_note %}}

## cleanse

The `cleanse` subcommand is used to re-set the Chef Automate server to
the state it was in prior to the first time the `reconfigure` subcommand
is run. This command will:

-   Destroy all data and logs
-   Create a backup of the system config files and place them in a
    directory in root, such as `/root/delivery-cleanse-2015-12-15T15:51`

The software that was put on-disk by the package installation will
remain. Re-run `automate-ctl reconfigure` to recreate the default data
and configuration files.

This subcommand has the following syntax:

``` none
automate-ctl cleanse
```

## create-backup

{{% automate_ctl_create_backup %}}

## create-enterprise

The `create-enterprise` subcommand is used to create a Chef Automate
enterprise. A public key is required.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl create-enterprise ENT_NAME --ssh-pub-key-file=FILE_NAME
```

{{< note >}}

The `ENT_NAME` value must be alphanumeric.

{{< /note >}}

## create-user

The `create-user` subcommand is used to create a user. (The validation
key for the organization may be returned to `STDOUT` when creating a
user with this command.)

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl create-user ENT-NAME USER-NAME --password [PASSWORD] --password-file [FILE] --roles ["COMMA-SEPARATED-LIST"]--ssh-pub-key-file=[PATH-TO-PUBLIC-KEY-FILE]
```

**Example**

``` bash
automate-ctl create-user enterprise john_smith --password my_password --roles reviewer,committer
```

## create-users

The `create-users` subcommand is used to create one or more users from a
TSV file.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl create-user TSV-FILE-PATH
```

**Example**

``` bash
automate-ctl create-user MyUserList.TSV
```

## data-summary

New in Chef Automate 1.6.192.

The `data-summary` subcommand is used to get the summary of Chef
Automate's data store. The default setting for `data-summary` is to
display the complete data summary of the Chef Automate Elasticsearch
cluster which includes the cluster, converge, compliance and node state
information. You may optionally pass one or many flags to limit the
output to specific data groupings.

**Syntax**

``` bash
automate-ctl data-summary [options]
    -c, --compliance                 Display compliance and inspec data
    -f, --format string              The output format ([text], json)
    -h, --help                       Show the help message
    -i, --insights                   Display insights and converge data
    -n, --node                       Display the node-state data
    -s, --cluster                    Display the Elasticsearch cluster data
    -u, --unit string                Select the unit of measurement ([b], kb, mb, gb)
```

**Examples**

Summarize Chef Automate's data usage using the `data-summary` command's
default behavior.

``` bash
automate-ctl data-summary
CLUSTER NAME   DISK FREE  MEM FREE  AVG ES CPU %  AVG OS CPU %  AVG ES HEAP  AVG ES NON HEAP
chef-insights  23.58 GB   0.45 GB   0             2             0.23 GB      0.08 GB

NODE NAME               DISK FREE  MEM FREE  AVG ES CPU %  AVG OS CPU %  AVG ES HEAP  AVG ES NON HEAP
t3HQTkyNQ-aSt8h2KK3TXQ  23.58 GB   0.45 GB   0             2             0.23 GB      0.08 GB

INDEX NAME  DELETED NODES  TOTAL NODES  TOTAL SIZE
node-state  0              1            0.0 GB

INDICES GROUP  INDICES TOTAL  TOTAL CONVERGES  AVG DAILY CONVERGE  TOTAL SIZE  AVG DAILY SIZE
insights       1              2                2                   0.0 GB      0.0 GB

INDEX NAME           TOTAL CONVERGES  TOTAL SIZE
insights-2017.10.16  2                0.0 GB

INDICES GROUP  INDICES TOTAL  TOTAL INSPEC RUNS  AVG DAILY INSPEC RUNS  TOTAL SIZE  AVG DAILY SIZE
compliance     1              1                  1                      0.0 GB      0.0 GB

INDEX NAME             TOTAL INSPEC RUNS  TOTAL SIZE
compliance-2017.10.16  1                  0.0 GB
```

Symmarize Chef Automate's compliance data in kilobytes.

``` bash
automate-ctl data-summary -c -u kb
INDICES GROUP  INDICES TOTAL  TOTAL INSPEC RUNS  AVG DAILY INSPEC RUNS  TOTAL SIZE  AVG DAILY SIZE
compliance     1              1                  1                      22.79 KB    22.79 KB

INDEX NAME             TOTAL INSPEC RUNS  TOTAL SIZE
compliance-2017.10.16  1                  22.79 KB
```

Summarize Chef Automate's data usage with JSON formatting.

``` bash
automate-ctl data-summary -f json
{"cluster":{"name":"chef-insights","nodes":[{"es_cpu_percent":0,"es_max_file_descriptors":50000,"es_open_file_descriptors":219,"os_cpu_percent":3,"es_mem_total_virtual_in_b":4892397568,"fs_free_in_b":38063587328,"fs_total_in_b":63381999616,"jvm_heap_max_in_b":1064042496,"jvm_heap_used_in_b":250139784,"jvm_non_heap_used_in_b":89278448,"os_mem_total_in_b":4397072384,"os_mem_used_in_b":3916091392}],"averages":{"es_cpu_percent":0,"es_max_file_descriptors":50000,"es_open_file_descriptors":219,"os_cpu_percent":3,"es_mem_total_virtual_in_b":4892397568,"fs_free_in_b":38063587328,"fs_total_in_b":63381999616,"jvm_heap_max_in_b":1064042496,"jvm_heap_used_in_b":250139784,"jvm_non_heap_used_in_b":89278448,"os_mem_total_in_b":4397072384,"os_mem_used_in_b":3916091392}},"indices":{"totals":{"converges":2,"deleted_nodes":0,"docs":22,"indices":5,"inspec_summaries":1,"nodes":1,"size_in_bytes":502067},"insights":{"totals":{"converges":2,"docs":2,"indices":1,"size_in_b":229142},"averages":{"converges":2,"docs":2,"size_in_b":229142},"indices":[{"converges":2,"docs":2,"size_in_b":229142}]},"compliance":{"totals":{"docs":19,"indices":1,"inspec_summaries":1,"size_in_b":23333},"averages":{"docs":19,"inspec_summaries":1,"size_in_b":23333},"indices":[{"docs":19,"inspec_summaries":1,"size_in_b":23333}]},"node_state":{"totals":{"deleted_nodes":0,"docs":1,"nodes":1,"size_in_b":249592}}}}
```

### Explanation of fields

`cluster`

:   Elasticsearch cluster statistics for each node in the cluster.

`es_cpu_percent`

:   Elasticsearch processes CPU usage in percent.

`es_max_file_descriptors`

:   Maximum number of files that Elasticsearch can concurrently open.

`es_open_file_descriptors`

:   Current number of files that Elasticsearch has open.

`os_cpu_percent`

:   Operating system reported CPU usage in percent.

`es_mem_total_virtual_in_b`

:   Maximum amount of virtual memory that Elasticsearch is allowed to
    allocate in bytes.

`fs_free_in_b`

:   Unallocated filesystem space in the Elasticsearch repository path in
    bytes.

`fs_total_in_b`

:   Total filesystem space in the Elasticsearch repository path in bytes

`jvm_heap_max_in_b`

:   Maximum amount of heap memory that the Elasticsearch Java Virtual
    Machine is allowed to allocate in bytes.

`jvm_heap_used_in_b`

:   The Elasticsearch Java Virtual Machine's currently allocated amount
    of heap memory in bytes.

`jvm_non_heap_used_in_b`

:   The Elasticsearch Java Virtual Machine's currently allocated amount
    of non-heap memory in bytes.

`os_mem_total_in_b`

:   The operating system's total memory amount in bytes.

`os_mem_used_in_b`

:   The operating system's total memory used in bytes.

`converges`

:   The count of Chef Infra Client converges have started.

`deleted_nodes`

:   Count of nodes that have been deleted but not purged from Chef
    Automate.

`docs`

:   Total Elasticsearch document count.

`indices`

:   The indices that are available in the indices group.

`inspec_summaries`

:   Count of inspec runs that have completed.

`nodes`

:   Total node count.

`size_in_bytes`

:   The total size of the index or indices in bytes.

## delete-backups

The `delete-backups` subcommand is used to delete Chef Automate backup
archives and Elasticsearch snapshots. The command matches a given
regular expression and prompts the user to confirm deletion of each
matched backup or snapshot.

**Syntax**

``` bash
automate-ctl delete-backups REGEX [options]
     --force                      Agree to all warnings and prompts
     --max-archives [integer]     Maximum number of backup archives to keep
     --max-snapshots [integer]    Maximum number of Elasticsearch snapshots to keep
     --pattern [string]           Delete backups matching the Ruby RegExp pattern
     --no-wait-for-lock           Do not wait for Elasticsearch lock<Paste>
 -h, --help                       Show the usage message
```

**Examples**

Deleting a single Automate backup archive:

:   `automate-ctl delete-backups 2016-10-14-08-38-55-chef-automate-backup.zst`

Deleting a single Elasticsearch snapshot:

:   `automate-ctl delete-backups 2016-10-14-08-38-55-chef-automate-backup$`

Deleting all backup archives and snapshots from October, 2016:

:   `automate-ctl delete-backups 2016-10-.+-chef-automate-backup --force`

## delete-elasticsearch-lock

The `delete-elasticsearch-lock` subcommand is used to delete the
exclusive Elasticsearch lock document that is used by several of Chef
Automate's services to coordinate major operations. Each service should
create and remove this lock automatically, but in the event of an issue
an operator can use this command to manually free the lock. The
`--stale-lock-only` option (added in Chef Automate 1.8.3) ensures that a
lock is only deleted if it is older than the currently running
Elasticsearch process.

Added in Chef Automate version 1.6.87.

**Syntax**

``` bash
automate-ctl delete-elasticsearch-lock [options]
     --force                      Agree to all warnings and prompts
     --stale-lock-only            Only delete the lock if it is older than the Elasticsearch process
 -h, --help                       Show the usage message
     --stale-lock-only            Cleans stale lock files
```

**Examples**

``` bash
automate-ctl delete-elasticsearch-lock

HOSTNAME            PROCESS  PID    TIME
automate.myorg.com  reaper   12345  2017-08-11T16:46:33Z

Removing the Elasticsearch lock before the process completes can cause race conditions. Are you sure you wish to proceed? (yes/no):
yes
```

## delete-enterprise

The `delete-enterprise` subcommand is used to delete a Chef Automate
enterprise.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl delete-enterprise ENT_NAME
```

**Example**

``` bash
automate-ctl delete-enterprise pedant-testing-org
```

## delete-project

The `delete-project` subcommand is used to delete a Chef Automate
project.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl delete-project ENT_NAME ORG_NAME PROJECT_NAME
```

## delete-user

The `delete-user` subcommand is used to delete a user.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl delete-user ENT_NAME USER_NAME
```

**Example**

``` bash
automate-ctl delete-user ENT_NAME john_smith
```

## delete-runner

The `delete-runner` subcommand deletes a remote node configured as a job
runner, which was used by Chef Automate to run phase jobs. For more
information on runners, please see the [Runners
documentation](/runners/).

Added in Chef Automate version 1.7.114.

**Syntax**

``` bash
automate-ctl delete-runner FQDN [options]

  Arguments:
    FQDN       Fully qualified domain name of the remote host that will be deleted as a runner

  Options:
   -h, --help                            Show the usage message
   -e, --enterprise                      Legacy option, only required if you have more than one enterprise configured. Workflow enterprise to delete the runner from
   -y, --yes                             Skip configuration confirmation and overwrite any existing Chef Infra Server nodes of the same name as FQDN
```

**Example**

``` bash
automate-ctl delete-runner
```

Delete the runner runner-hostname.mydomain.co when there is only one
enterprise configured.

``` bash
automate-ctl delete-runner runner-hostname.mydomain.co
```

Delete the runner runner-hostname.mydomain.co when multiple enterprises
are configured.

``` bash
automate-ctl install-runner runner-hostname.mydomain.co -e myenterprise
```

## delete-node

The `delete-node` subcommand is used to delete a node and it's
corresponding history from Chef Automate. The user must provide some
combination of the node's UUID, name, organization name, and chef server
FQDN to determine which node to delete. In the event that multiple nodes
are found, a list of matching nodes will displayed. Narrow the search by
providing more search parameters or use the UUID to delete the node.

New in Chef Automate 1.6.87.

**Hint:** You can also determine the UUID of a node via the web browser
address bar:

![image](/images/chef_automate_node_uuid.png)

{{< note >}}

Compliance data is **not** deleted by default. You must pass `-c` to
delete these records.

{{< /note >}}

**Syntax**

``` none
automate-ctl delete-node OPTIONS
   -u, --uuid UUID                  The UUID of the node you wish to delete
   -n, --name NODE_NAME             The name of the node you wish to delete
   -o, --org ORG_NAME               The organization name of the node you wish to delete
   -s, --chef-server-fqdn FQDN      The fully qualified domain name of the node's Chef server
   -b, --batch-size string          Maximum number of documents to modify in each Elasticsearch bulk request
   -d, --[no-]node-data             Delete the node run and converge data
   -c, --[no-]compliance-data       Delete the node compliance data
       --force                      Agree to all warnings and prompts
       --purge                      Purge all node data (not recommended)
   -r, --request-timeout SECONDS    The Elasticsearch client request timeout in seconds
```

**Examples**

``` bash
automate-ctl delete-node -n chef-test
Multiple nodes were found matching your request. Please specify the UUID and try again: automate-ctl delete-node --uuid <UUID>

NAME       ORG        CHEF SERVER FQDN  UUID
chef-test  chef_solo  localhost         f44c40a4-a0bb-4120-bd75-079972d98072
chef-test  chef_dev   chef-server.dev   8703593e-723a-4394-a36d-34da11a2f668

ERROR: Too many nodes found, please delete by node UUID
```

``` bash
automate-ctl delete-node -u f44c40a4-a0bb-4120-bd75-079972d98072
Delete 2 records associated with node 'chef-test f44c40a4-a0bb-4120-bd75-079972d98072'.
Do you wish to proceed? (yes/no):
yes
```

## doctor

The `doctor` command validates the configuration files.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl doctor
```

## gather-logs

The `gather-logs` command is used to collect the logs from Chef Automate
into a compressed file archive. It will create a tbz2 file in the
current working directory, with the timestamp as the file name.

By default, it collects the most current log file as well as any others
that have been modified in the last 180 minutes. If the `--all-logs`
option is given, all log files are collected.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl gather-logs
      --all-logs          Gather all of the logs, regardless of size or age.
```

{{< warning >}}

The `--all-logs` option can potentially take up a large amount of disk
space.

{{< /warning >}}

## generate-password-reset-token

The `generate-password-reset-token` command is used to unset the
password for an existing Chef Automate user, and generate a token that
allows them to set a new password. The command returns a URL pointing to
the Chef Automate UI, allowing the user to enter a new password.

The token is embedded in that URL and has an expiry of two hours. This
command may be issued again to get a new token. After the command has
been run, the previously stored password will no longer work. Issued API
tokens (e.g. in existing UI sessions or for use with the [Delivery
CLI](/delivery_cli/)) will not be revoked.

When a token is consumed (through the web UI), all issued password reset
tokens for this user will be revoked.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl generate-password-reset-token ENTERPRISE_NAME USER_NAME
```

**Example**

``` bash
automate-ctl generate-password-reset-token Chef admin
Password reset with token successful. Go to this URL to set a new password:
URL: https://automate.fqdn/e/Chef/#/reset-password/admin/nzfcEPQULoY0NR-xg7OxxBl5Q3htausWXY92GskR3ZE
```

## help

The `help` subcommand is used to print a list of all available
`automate-ctl` commands.

This subcommand has the following syntax:

``` bash
automate-ctl help
```

## install-build-node

THe `install-build-node` subcommand configures a named node too act as a
build node in a delivery cluster. For more information on delivery,
please see the [Workflow Overview](/workflow/). For more information
on delivery commands, please see [Delivery CLI](/delivery_cli/).

**Syntax** This subcommand has the following syntax:

``` bash
automate-ctl install-build-node [options]
     -h, --help                       Prints this help
     -I PATH_TO_INSTALLER,            The location of the ChefDK package for the build node (Required)
     --installer
     -f, --fqdn FQDN                  FQDN of the remote host that will be configured into a build node
     -u, --username USERNAME          Username to use for authentication to the remote host
     -P, --password PASSWORD          Password to use for authentication to the remote host
     -p, --port PORT                  Port to connect to on the remote host
     -i [IDENTITY_FILE],              The SSH identity file used for authentication -
     --ssh-identity-file          will prompt if flag is specified but no filename is given
     -o                               overwrite this node's entryin chef server if it's already registered
     --[no-]overwrite-registration
     -V VERSION,                      Job dispatch version to use(v1 [default] or v2)
        --job-dispatch-version
     -a, --admin-user NAME            Admin user name (necessary for job dispatch version or v2)
     -t, --admin-token TOKEN          Admin token (necessary for job dispatch version or v2)
     -e, --enterprise ENTERPRISE      Enterprise to use (necessary for job dispatch version or v2)
```

## install-runner

The `install-runner` subcommand configures a remote node as a job
runner, which are used by Chef Automate to run phase jobs. For more
information on runners, please see the [Runners
documentation](/runners/).

**Syntax**

``` bash
automate-ctl install-runner FQDN USERNAME [options]

  Arguments:
    FQDN       Fully qualified domain name of the remote host that will be configured into a runner
    USERNAME   The username used for authentication to the remote host that will be configured into a runner

  Options:
   -h, --help                            Show the usage message
   -i, --ssh-identity-file FILE          SSH identity file used for authentication to the remote host
   -I, --installer FILE                  The location of the ChefDK package for the runner.
                                         This option cannot be passed with --chefdk-version as that option specifies remote download.
                                         If neither are passed, the latest ChefDK will be downloaded remotely

   -p, --port PORT                       SSH port to connect to on the remote host (Default: 22)
   -P, --password [PASSWORD]             Pass if you need to set a password for ssh and / or sudo access.
                                         You can pass the password in directly or you will be prompted if you simply pass --password.
                                         If --ssh-identify-file is also passed, will only be used for sudo access

   -v, --chefdk-version VERSION          Custom version of ChefDK you wish to download and install.
                                         This option cannot be passed with --installer as that option specifies using a package local to this server.
                                         If neither are passed, the latest ChefDK will be downloaded remotely

   -y, --yes                             Skip configuration confirmation and overwrite any existing Chef Infra Server nodes of the same name as FQDN
   -e, --enterprise                      Legacy option, only required if you have more than one enterprise configured. Workflow enterprise to add the runner into
   --fips-custom-cert-filename FILENAME  If you have a self-signed or self-owned Certificate Authority (CA) and wish to operate in FIPS mode, pass this flag the path to a file containing your custom certificate chain on your Automate server. This file will be copied to the runner and used when running jobs in FIPS mode. If you have purchased a certificate from a known CA for Automate server, you can ignore this flag. Please see the Automate FIPS docs for details.
   --full-ohai                           If `--full-ohai` flag set, Chef will run with full Ohai plugins.
```

{{< note >}}

The username provided must be a user who has sudo access on the remote
node. If the user is a member of a domain, then the username value
should be entered as `user@domain`.

{{< /note >}}

{{< note >}}

At least one of `--password [PASSWORD]` or `--ssh-identity-file FILE`
are necessary for ssh access.

{{< /note >}}

{{< note >}}

`install-runner` calls the `knife bootstrap` subcommand to configure the
runner, so custom configurations can be installed on the runner by using
the [client.d copying feature](/workstation/knife_bootstrap/). All config files
inside `~/.chef/client.d` directory on the Chef Automate server get
copied into the `/etc/chef/client.d` directory on the runner.

{{< /note >}}

**Example**

``` bash
automate-ctl install-runner
```

Installing the latest ChefDK via download and CLI prompt for SSH / Sudo
password.

``` bash
automate-ctl install-runner runner-hostname.mydomain.co ubuntu --password
```

Installing with a ChefDK file local to your Workflow server, an SSH Key,
and passwordless sudo.

``` bash
automate-ctl install-runner runner-hostname.mydomain.co ubuntu -i ~/.ssh/id_rsa -I ./chefdk.deb
```

Installing a custom version of ChefDK via download, a identity file for
ssh access, and a Sudo password.

``` bash
automate-ctl install-runner runner-hostname.mydomain.co ubuntu -v 0.18.30 -p my_password -i ~/.ssh/id_rsa
```

## list-backups

The `list-backups` subcommand is used to list Chef Automate backup
archives and Elasticsearch snapshots.

**Syntax**

``` bash
automate-ctl list-backups [options]
     --all                        List all backups and snapshots (default)
     --automate                   Only list Chef Automate's backup archives
     --elasticsearch              Only list Chef Automate's Elasticsearch snapshots
     --format [string]            The output format. 'text' or 'json'
 -h, --help                       Show the usage message
```

**Examples**

Return a list all backups as JSON:

:   `automate-ctl list-backups --format json`

## list-enterprises

The `list-enterprises` subcommand is used to list all of the enterprises
currently present on the Chef Automate server.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl list-enterprises
```

## list-users

The `list-users` subcommand is used to view a list of users.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl list-users ENT_NAME
```

## migrate-change-description

The `migrate-change-description` subcommand is used to migrate the
change description live run.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl migrate-change-description ENT_NAME ORG_NAME PROJECT_NAME CHANGE
```

## migrate-change-description-dry-run

The `migrate-change-description-dry-run` subcommand is used to execute a
dry run migration of the change description.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl migrate-change-description-dry-run ENT_NAME ORG_NAME PROJECT_NAME CHANGE
```

## migrate-compliance

The `migrate-compliance` subcommand is used to execute the migration of
compliance data for the purpose of synchronising the `compliance-latest`
elasticsearch index with reporting times-series data, if needed.

New in Automate 1.7.114

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl migrate-compliance [options]
   -debug          Turn on debug logging
```

## migrate-github-project

The `migrate-github-project` subcommand is used to execute migration of
a project to a new GitHub integration.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl migrate-github-project (ENTERPRISE | ENTERPRISE ORG | ENTERPRISE ORG PROJECT)
```

## migrate-patchset-diffs

The `migrate-patchset-diffs` subcommand is used to update patchset diffs
to include numstat.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl migrate-patchset-diffs ENT_NAME ORG_NAME PROJECT_NAME PATCHSET_DIFF
```

## migrate-patchset-diffs-dry-run

The `migrate-patchset-diffs-dry-run` subcommand is used to execute a dry
run update of patchset diffs to include numstat.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl migrate-patchset-diffs-dry-run ENT_NAME ORG_NAME PROJECT_NAME PATCHSET_DIFF
```

## node-summary

The `node-summary` subcommand produces a summary of the nodes that are
known to Chef Automate.

New in Chef Automate 0.5.328.

The default setting for `node-summary` is to display the name, UUID,
status, and the last time the nodes checked in via Chef Infra Client,
Chef InSpec, or the liveness agent.

**Syntax**

``` bash
automate-ctl node-summary [options]
    -f, --format string              The output format. 'text' or 'json'
    -r, --request-timeout int        The Elasticsearch client request timeout in seconds
    -h, --help                       Show this message
```

**Examples**

Produce a summary of nodes known to Automate using the `node-summary`
default behavior.

``` bash
automate-ctl node-summary
NAME                              UUID                                  STATUS            LAST CHECKIN
chef-test-1                       f44c40a4-a0bb-4120-bd75-079972d98072  success           2017-02-22T19:41:14.000Z
chef-test-2                       8703593e-723a-4394-a36d-34da11a2f668  missing           2017-02-25T19:54:08.000Z
agentless-scan-node1.example.com  63d49e04-f1f2-4d80-61a0-4f332d58b492  scan-unreachable  2017-12-05T20:29:39Z
agentless-scan-node2.example.com  825e90c1-cb23-4f6a-6c0e-35e5b2d12ea4  scan-passed       2017-12-07T18:50:57Z
```

Produce a summary of nodes known to Automate in JSON.

``` bash
automate-ctl node-summary --format json
[
  {
    "chef_version": "12.21.3",
    "checkin": "2017-02-22T19:41:14.000Z",
    "@timestamp": "2017-02-22T19:41:14.000Z",
    "platform_version": "10.12.3",
    "fqdn": "chef-test-1",
    "name": "chef-test-1",
    "organization_name": "chef",
    "platform_family": "mac_os_x",
    "platform": "mac_os_x",
    "status": "success",
    "uuid": "f44c40a4-a0bb-4120-bd75-079972d98072",
    "chef_server_status": "present"
  },
  ...
]
```

### Explanation of fields

`chef_version`

:   The Chef Infra Client version of that ran on the node.

`checkin`

:   The last time Chef Infra Client ran on the node.

`@timestamp`

:   The time when the node's information was received by Chef Automate.

`platform_version`

:   Platform version information discovered by ohai on the node.

`fqdn`

:   Fully qualified domain name of the node.

`name`

:   Name of the node in Chef Infra Server.

`organization_name`

:   The name of the Chef Infra Server organization the node belongs to.

`platform_family`

:   Platform family information discovered by ohai on the node.

`platform`

:   Platform information discovered by ohai on the node.

`status`

:   `success` if the last Chef Infra Client run succeeded on the node.

    `failure` if the last Chef Infra Client run failed on the node.

    `live` if the liveness agent has successfully updated Chef Automate,
    but Chef Infra Client has not run within the expected check-in
    duration configured in Chef Automate (default is 12 hours).

    `missing` if Chef Infra Client did not run within the expected
    check-in duration configured in Chef Automate (default is 12 hours).

    `scan-failed` if a node set up for ad-hoc scanning failed its latest
    compliance scan.

    `scan-passed` if a node set up for ad-hoc scanning passed its latest
    compliance scan.

    `scan-skipped` if a node set up for ad-hoc scanning skipped its latest
    compliance scan.

    `scan-unreachable` if a node set up for ad-hoc scanning either could not be
    reached for scanning or has not been scanned within the past hour.

`uuid`

:   The universally unique identifier of the node in Chef Automate.

`chef_server_status`

:   This field is only populated in Opsworks for Chef Automate
    instances.

    `present`: Node is still present on the Chef Infra Server.

    `missing`: Node is still present on the Chef Infra Server.

`ec2`

:   EC2 information discovered by ohai on the node. This field is only
    populated in Chef Automate instances that are running on EC2

## preflight-check

The `preflight-check` subcommand is used to check for common problems in
your infrastructure environment before setup and configuration of Chef
Automate begins.

New in Chef Automate 0.6.64.

This subcommand has the following syntax:

``` bash
automate-ctl preflight-check
```

## reconfigure

The `reconfigure` subcommand is used to reconfigure the Chef Automate
server after changes are made to the delivery configuration file,
located at `/etc/delivery/delivery.rb`. When changes are made to the
delivery configuration file, they are not applied to the Chef Automate
configuration until after this command is run. This subcommand also
restarts any services for which the `service_name['enabled']` setting is
set to `true`. This subcommand also reconfigures rubygems installed into
Automate with an overly restrictive `umask`. The default timeout is 60
seconds.

This subcommand has the following syntax:

``` bash
automate-ctl reconfigure
```

## rename-enterprise

The `rename-enterprise` subcommand is used to rename an existing Chef
Automate enterprise.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl rename-enterprise CURRENT_ENT_NAME NEW_ENT_NAME
```

## reset-password

The `reset-password` command is used to reset the password for an
existing Chef Automate user.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl reset-password ENTERPRISE_NAME USER_NAME NEW_PASSWORD
```

## restore-backup

The `restore-backup` subcommand is used to restore Chef Automate backup
archives and Elasticsearch snapshots.

The command is intended to restore an Automate instance completely from
backup, however, it does support restoring only specific data types when
given compatible backup archives and snapshots.

{{< note >}}

Backups created with the older `automate-ctl backup-data` command are
not supported with this command. If you wish to restore an older backup
please install the version of Chef Automate that took the backup and use
`automate-ctl restore-data`

{{< /note >}}

**Syntax**

``` console
automate-ctl restore-backup /path/to/chef-automate-backup.zst [ELASTICSEARCH_SNAPSHOT] [options]
automate-ctl restore-backup us-east-1:s3_bucket:chef-automate-backup.zst [ELASTICSEARCH_SNAPSHOT] [options]
automate-ctl restore-backup ELASTICSEARCH_SNAPSHOT [options]
     --digest [int]               The SHA digest of the backup archive
     --force                      Agree to all warnings and prompts
     --no-chef-server-config      Do not restore the Chef Infra Server config if present
     --no-census                  Do not restore Chef Automate's census data
     --no-compliance-profiles     Do not restore Chef Automate's compliance profiles
     --no-config                  Do not restore Chef Automate's configuration directory
     --no-db                      Do not restore Chef Automate's database
     --no-git                     Do not restore Chef Automate's git repositories
     --no-license                 Do not restore Chef Automate's license file
     --no-notifications           Do not restore Chef Automate's notifications rulestore
     --no-rabbit                  Do not restore Chef Automate's RabbitMQ data
     --no-wait                    Do not wait for non-blocking restore operations
     --no-wait-for-lock           Do not wait for Elasticsearch lock
     --quiet                      Do not output non-error information
     --retry-limit                Maximum number of times to retry archive downloads from S3
     --snapshot-timeout [int]     Maximum number of seconds to wait when restoring an Elasticsearch snapshot
     --staging-dir [string]       The path to use for temporary files during restore
 -h, --help                       Show the usage message
```

{{< note >}}

The `ELASTICSEARCH_SNAPSHOT` value is optional when given a backup
archive path.

{{< /note >}}

**Examples**

:   `automate-ctl restore-backup us-east-1:your-s3-bucket:2016-10-14-08-38-55-chef-automate-backup.zst 2016-10-14-08-38-55-chef-automate-backup`
    `automate-ctl restore-backup 2016-10-14-08-38-55-chef-automate-backup`
    `automate-ctl restore-backup us-east-1:your-s3-bucket:2016-10-14-08-38-55-chef-automate-backup.zst --no-census --no-license --no-config`

## revoke-token

The `revoke-token` subcommand is used to revoke a user's token.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl revoke-token ENT_NAME USER_NAME
```

## show-config

The `show-config` subcommand is used to view the configuration that will
be generated by the `reconfigure` subcommand. This command is most
useful in the early stages of a deployment to ensure that everything is
built properly prior to installation.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl show-config
```

## setup

The `setup` subcommand is used to configure the Chef Automate Server.

**Syntax** This subcommand has the following syntax:

``` bash
automate-ctl setup [options]
     -h, --help                       Prints this help
     --minimal                    [Pre-Release] Set up Chef Automate with a minimal default configuration.
     -l, --license LICENSE            Location of Chef Automate license file.
     -f, --fqdn FQDN                  The external fully qualified domain name of this node (Already set in delivery.rb.  Do not set via flag.)
     -k, --key CHEF_AUTOMATE_USER_KEY Location of Chef Automate user key (Already set in delivery.rb.  Do not set via flag.)
     --server-url CHEF_SERVER_URL Chef Infra Server URL (Already set in delivery.rb.  Do not set via flag.)
     --supermarket-fqdn SUPERMARKET_FQDN
                                  Internal Supermarket FQDN
     -e CHEF_AUTOMATE_ENTERPRISE_NAME,
     --enterprise                 Name of the Chef Automate Enterprise to create.
     --[no-]build-node            Install a build node after Chef Automate Server setup completes.
     --[no-]configure             Apply configuration changes automatically after Chef Automate Server setup completes.
```

## telemetry

The `telemetry` subcommand is used in conjunction with additional
subcommands to enable, disable, or show the status of telemetry on the
server.

**Syntax** This subcommand has the following syntax:

``` bash
automate-ctl telemetry status
```

**Examples**

Query current status: `automate-ctl telemetry status`

Enable telemetry: `automate-ctl telemetry enable`

Disable telemetry: `automate-ctl telemetry disable`

## uninstall

The `uninstall` subcommand is used to remove the Chef Automate
application, but without removing any of the data. This subcommand will
shut down all services (including the `runit` process supervisor).

This subcommand has the following syntax:

``` bash
automate-ctl uninstall
```

{{< note >}}

To revert the `uninstall` subcommand, run the `reconfigure` subcommand
(because the `start` subcommand is disabled by the `uninstall` command).

{{< /note >}}

## update-project-hooks

The `update-project-hooks` subcommand is used to update git hooks for
all projects.

**Syntax**

This subcommand has the following syntax:

``` bash
automate-ctl update-project-hooks ENT_NAME ORG_NAME PROJECT_NAME
```

## Service Subcommands

{{% ctl_common_service_subcommands %}}

### graceful-kill

The `kill` subcommand is used to send a `SIGKILL` to all services. This
command can also be run for an individual service by specifying the name
of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl kill name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### hup

The `hup` subcommand is used to send a `SIGHUP` to all services. This
command can also be run for an individual service by specifying the name
of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl hup name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### int

The `int` subcommand is used to send a `SIGINT` to all services. This
command can also be run for an individual service by specifying the name
of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl int name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### kill

The `kill` subcommand is used to send a `SIGKILL` to all services. This
command can also be run for an individual service by specifying the name
of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl kill name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### once

The supervisor for the Chef Automate server is configured to restart any
service that fails, unless that service has been asked to change its
state. The `once` subcommand is used to tell the supervisor to not
attempt to restart any service that fails.

This command is useful when troubleshooting configuration errors that
prevent a service from starting. Run the `once` subcommand followed by
the `status` subcommand to look for services in a down state and/or to
identify which services are in trouble. This command can also be run for
an individual service by specifying the name of the service in the
command.

This subcommand has the following syntax:

``` bash
automate-ctl once name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### restart

The `restart` subcommand is used to restart all services enabled on the
Chef Automate server, or to restart an individual service by specifying
the name of that service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl restart name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand. When a service is
successfully restarted the output should be similar to:

``` bash
ok: run: service_name: (pid 12345) 1s
```

### service-list

The `service-list` subcommand is used to display a list of all available
services. A service that is enabled is labeled with an asterisk (\*).

This subcommand has the following syntax:

``` bash
automate-ctl service-list
```

### start

The `start` subcommand is used to start all services that are enabled in
the Chef Automate server. This command can also be run for an individual
service by specifying the name of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl start name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand. When a service is
successfully started the output should be similar to:

``` bash
ok: run: service_name: (pid 12345) 1s
```

The supervisor for the Chef Automate server is configured to wait seven
seconds for a service to respond to a command from the supervisor. If
you see output that references a timeout, it means that a signal has
been sent to the process, but that the process has yet to actually
comply. In general, processes that have timed out are not a big concern,
unless they are failing to respond to the signals at all. If a process
is not responding, use a command like the `kill` subcommand to stop the
process, investigate the cause (if required), and then use the `start`
subcommand to re-enable it.

### status

The `status` subcommand is used to show the status of all services
available to the Chef Automate server. The results will vary based on
the configuration of a given server. This subcommand has the following
syntax:

``` bash
automate-ctl status
```

and will return the status for all services. Status can be returned for
individual services by specifying the name of the service as part of the
command:

``` bash
automate-ctl status name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

When service status is requested, the output should be similar to:

``` bash
run: service_name: (pid 12345) 12345s; run: log: (pid 1234) 67890s
```

where

-   `run:` is the state of the service (`run:` or `down:`)
-   `service_name:` is the name of the service for which status is
    returned
-   `(pid 12345)` is the process identifier
-   `12345s` is the uptime of the service, in seconds

For example:

``` bash
down: opscode-erchef: (pid 35546) 10s
```

By default, runit will restart services automatically when the services
fail. Therefore, runit may report the status of a service as `run:` even
when there is an issue with that service. When investigating why a
particular service is not running as it should be, look for the services
with the shortest uptimes. For example, the list below indicates that
the **opscode-erchef** should be investigated further:

``` bash
run: oc-id
run: opscode-chef: (pid 4327) 13671s; run: log: (pid 4326) 13671s
run: opscode-erchef: (pid 5383) 5s; run: log: (pid 4382) 13669s
run: opscode-expander: (pid 4078) 13694s; run: log: (pid 4077) 13694s
run: opscode-expander-reindexer: (pid 4130) 13692s; run: log: (pid 4114) 13692s
```

#### Log Files

A typical status line for a service that is running any of the Chef
Automate server front-end services is similar to the following:

``` bash
run: name_of_service: (pid 1486) 7819s; run: log: (pid 1485) 7819s
```

where:

-   `run` describes the state in which the supervisor attempts to keep
    processes. This state is either `run` or `down`. If a service is in
    a `down` state, it should be stopped
-   `name_of_service` is the service name, for example: `opscode-solr4`
-   `(pid 1486) 7819s;` is the process identifier followed by the amount
    of time (in seconds) the service has been running
-   `run: log: (pid 1485) 7819s` is the log process. It is typical for a
    log process to have a longer run time than a service; this is
    because the supervisor does not need to restart the log process in
    order to connect the supervised process

If the service is down, the status line will appear similar to the
following:

``` bash
down: opscode-solr4: 3s, normally up; run: log: (pid 1485) 8526s
```

where

-   `down` indicates that the service is in a down state
-   `3s, normally up;` indicates that the service is normally in a run
    state and that the supervisor would attempt to restart this service
    after a reboot

### stop

The `stop` subcommand is used to stop all services enabled on the Chef
Automate server. This command can also be run for an individual service
by specifying the name of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl stop name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand. When a service is
successfully stopped the output should be similar to:

``` bash
ok: diwb: service_name: 0s, normally up
```

For example:

``` bash
automate-ctl stop
```

will return something similar to:

``` bash
ok: down: nginx: 393s, normally up
ok: down: opscode-chef: 391s, normally up
ok: down: opscode-erchef: 391s, normally up
ok: down: opscode-expander: 390s, normally up
ok: down: opscode-expander-reindexer: 389s, normally up
ok: down: opscode-solr4: 389s, normally up
ok: down: postgresql: 388s, normally up
ok: down: rabbitmq: 388s, normally up
ok: down: redis_lb: 387s, normally up
```

### tail

The `tail` subcommand is used to follow all of the Chef Automate server
logs for all services. This command can also be run for an individual
service by specifying the name of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl tail name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### term

The `term` subcommand is used to send a `SIGTERM` to all services. This
command can also be run for an individual service by specifying the name
of the service in the command.

This subcommand has the following syntax:

``` bash
automate-ctl term name_of_service
```

where `name_of_service` represents the name of any service that is
listed after running the `service-list` subcommand.

### usr1

The `usr1` subcommand is used to send the services a USR1.

### usr2

The `usr2` subcommand is used to send the services a USR2.
