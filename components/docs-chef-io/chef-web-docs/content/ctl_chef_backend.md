+++
title = "chef-backend-ctl"
draft = false

aliases = ["/ctl_chef_backend.html"]

[menu]
  [menu.infra]
    title = "chef-backend-ctl"
    identifier = "chef_infra/managing_chef_infra_server/ctl_chef_backend.md chef-backend-ctl"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 160
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/ctl_chef_backend.md)

The Chef Infra Server backend HA cluster includes a command-line utility
named chef-backend-ctl. This command-line tool is used to manage the
Chef Infra Server backend HA cluster, start and stop individual
services, and tail Chef Infra Server log files.

## backup

Use the `backup` subcommand is to backup the data for a node in the
backend HA cluster. This command is typically run against a follower
node. Use the `--force` option to run this command against all nodes in
the backend HA cluster. The backup is created as a tar.gz file and is
located in `/var/opt/chef-backup/`.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl backup (options)
```

### Options

{{% ctl_chef_backend_cleanse_options %}}

### Examples

**Backup a node in the backend HA cluster**

From a follower node, run the following command:

``` bash
chef-backend-ctl backup
```

## create-cluster

Use the `create-cluster` subcommand to initialize the cluster state,
including the PostgreSQL data store, and then bootstrap the first node
in a backend HA cluster or assist in the recovery of the entire backend
HA cluster.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl create-cluster (options)
```

### Options

This subcommand has the following options:

`--elasticsearch-wait-time`

:   The maximum amount of time (in seconds) to wait for Elasticsearch to
    start. Default value: `30`.

`--etcd-wait-time`

:   The maximum amount of time (in seconds) to wait for etcd to start.
    Default value: `30`.

`--quorum-loss-recovery`

:   Resets the cluster identifier in etcd to this node.

    If nodes in a backend HA cluster are not available, etcd may not be
    able to form a cluster. If etcd cannot form a cluster, rebuild the
    cluster. First reset the cluster identifier on an active node,
    rebuild the nodes that will be part of the cluster, and then rejoin
    the rebuilt nodes to the cluster by using the
    `chef-backend-ctl join-cluster` subcommand.

`-y`, `--yes`

:   Do not prompt for confirmation.

### Examples

None.

## cleanse

The `cleanse` subcommand is used to re-set a machine in the Chef Infra
Server backend HA cluster to the state it was in prior to the first time
the `reconfigure` subcommand is run. This command will destroy all data,
configuration files, and logs. The software that was put on-disk by the
package installation will remain; re-run `chef-backend-ctl reconfigure`
to recreate the default data and configuration files.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl cleanse
```

### Options

{{% ctl_chef_backend_cleanse_options %}}

### Examples

None.

## cluster-status

Use the `cluster-status` subcommand to return a list of all accessible
nodes, their role (leader, follower), and the status for PostgreSQL and
Elasticsearch.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl cluster-status (options)
```

### Options

This subcommand has the following options:

`--json`

:   Return cluster health information as JSON.

### Examples

**Return cluster health data as JSON**

``` bash
chef-backend-ctl cluster-status --json
```

## demote

Use the `demote` subcommand to demote the current leader in the backend
HA cluster, after which a new leader is elected from the group of
available followers in the backend HA cluster. This command will:

-   Complete with an exit code of `0` if the original leader was demoted
    and a new leader was elected successfully.
-   Return an error message and a non-zero exit code if leader election
    is prevented because failover has been disabled (for either the
    cluster or the node) or if a new leader could not be elected within
    the allowed time.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl demote
```

### Examples

None.

## force-leader

Use the `force-leader` subcommand to force the node from which the
command is run to become the leader in the backend HA cluster if there
is not already an elected leader.

This command should only be run if:

-   The backend cluster appears to not have an elected and available
    leader
-   All of the nodes in the backend HA cluster are believed to be in a
    healthy state; if one (or more) nodes are not healthy, first remove
    the unhealthy nodes, rebuild, and then rejoin them to the cluster

This command will:

-   Run the `chef-backend-ctl cluster-status` subcommand to determine if
    a leader exists.

    {{< warning spaces =4 >}}

    Nodes in the backend HA cluster may not be visible to each other
    when they are located in network partitions. This may prevent a
    cluster status check from discovering that a leader exists. If nodes
    exist on network partitions, to prevent this scenario, first run
    `chef-backend-ctl cluster-status`, and then verify the expected
    number of nodes in the backend HA cluster as healthy and
    `waiting_for_leader` before running this command.

    {{< /warning >}}

-   Complete with an exit code of `0` if the node from which the command
    is run becomes the leader.

-   Return an error message and a non-zero exit code if a leader already
    exists.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl force-leader
```

### Examples

None.

## gather-logs

Use the `gather-logs` subcommand to gather the log files for a machine
in the Chef Infra Server backend HA cluster into a tarball that contains
all of the important log files and system information.

This subcommand has the following syntax:

``` bash
chef-backend-ctl gather-logs
```

## gen-sample-backend-config

Use the `gen-sample-backend-config` subcommand to generate output that
contains all of the backend HA cluster settings along with their default
values. Use this subcommand to get the values for `publish_address` and
`vip_interface` prior to bootstrapping a new node for the backend HA
cluster.

{{< warning >}}

Service-specific configuration settings---`etcd`, `elasticsearch`,
`leaderl`, and `postgresl`---are generated automatically by the backend
and should only be tuned under guidance from Chef. Service-specific
configuration settings must be identical on all nodes in the backend HA
cluster unless directed otherwise.

{{< /warning >}}

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl gen-sample-backend-config
```

### Example Output

The following example shows the results of running the
`chef-backend-ctl gen-sample-backend-config` subcommand. The settings
and output will vary, depending on the configuration. The
`elasticsearch`, `etcd`, `leaderl`, and `postgresql` settings are
generated automatically and should not be modified:

``` ruby
fqdn = 'be1'
hide_sensitive = true
ip_version = 'ipv4'
publish_address = '10.0.2.15'
vip = '10.0.2.15'
vip_interface = 'eth0'
etcd.client_port = 2379
etcd.enable = true
etcd.log_directory = '/var/log/chef-backend/etcd'
etcd.peer_port = 2380
etcd.log_rotation.file_maxbytes = 104857600
etcd.log_rotation.num_to_keep = 10
postgresql.archive_command = ''
postgresql.archive_mode = 'off'
postgresql.archive_timeout = 0
postgresql.checkpoint_completion_target = 0.5
postgresql.checkpoint_timeout = '5min'
postgresql.checkpoint_warning = '30s'
postgresql.data_dir = '/var/opt/chef-backend/postgresql/9.5/data'
postgresql.db_superuser = 'chef_pgsql'
postgresql.effective_cache_size = '496MB'
postgresql.enable = true
postgresql.hot_standby = 'on'
postgresql.keepalives_count = 2
postgresql.keepalives_idle = 60
postgresql.keepalives_interval = 15
postgresql.log_directory = '/var/log/chef-backend/postgresql/9.5'
postgresql.log_min_duration_statement = -1
postgresql.max_connections = 350
postgresql.max_replication_slots = 12
postgresql.max_wal_senders = 12
postgresql.max_wal_size = 64
postgresql.md5_auth_cidr_addresses = '["samehost", "samenet"]'
postgresql.min_wal_size = 5
postgresql.port = 5432
postgresql.replication_user = 'replicator'
postgresql.shared_buffers = '248MB'
postgresql.shmall = 4194304
postgresql.shmmax = 17179869184
postgresql.username = 'chef_pgsql'
postgresql.wal_keep_segments = 32
postgresql.wal_level = 'hot_standby'
postgresql.wal_log_hints = 'on'
postgresql.work_mem = '8MB'
postgresql.log_rotation.file_maxbytes = 104857600
postgresql.log_rotation.num_to_keep = 10
elasticsearch.data_dir = '/var/opt/chef-backend/elasticsearch/data'
elasticsearch.enable = true
elasticsearch.heap_size = 248
elasticsearch.java_opts = ''
elasticsearch.log_directory = '/var/log/chef-backend/elasticsearch'
elasticsearch.new_size = 32
elasticsearch.plugins_directory = '/var/opt/chef-backend/elasticsearch/plugins'
elasticsearch.port = 9200
elasticsearch.scripts_directory = '/var/opt/chef-backend/elasticsearch/scripts'
elasticsearch.temp_directory = '/var/opt/chef-backend/elasticsearch/'
elasticsearch.log_rotation.file_maxbytes = 104857600
elasticsearch.log_rotation.num_to_keep = 10
leaderl.control_worker_timeout_seconds = 30
leaderl.db_timeout = 2000
leaderl.enable = true
leaderl.health_check_interval_seconds = 2
leaderl.leader_ttl_seconds = 10
leaderl.log_directory = '/var/log/chef-backend/leaderl'
leaderl.status_internal_update_interval_seconds = 5
leaderl.status_post_update_interval_seconds = 10
leaderl.log_rotation.file_maxbytes = 104857600
leaderl.log_rotation.max_messages_per_second = 1000
leaderl.log_rotation.num_to_keep = 10
leaderl.etcd_pool.cull_interval_seconds = 60
leaderl.etcd_pool.http_timeout_ms = 5000
leaderl.etcd_pool.ibrowse_options = '{inactivity_timeout, infinity}'
leaderl.etcd_pool.init_count = 10
leaderl.etcd_pool.max_age_seconds = 60
leaderl.etcd_pool.max_connection_duration_seconds = 300
leaderl.etcd_pool.max_count = 10
ssl.certificate = nil
ssl.certificate_key = nil
ssl.ciphers = (a list of cipers, not shown)
ssl.company_name = 'YouCorp'
ssl.country_name = 'US'
ssl.data_dir = '/var/opt/chef-backend/ssl/'
ssl.duration = 3650
ssl.key_length = 2048
ssl.organizational_unit_name = 'Operations'
```

## gen-server-config

Use the `gen-server-config` subcommand to generate output for the
`/etc/opscode/chef-server.rb` configuration file. This command may be
run from any machine in the backend HA cluster, but must be run
separately for each node that is part of the frontend group. This
command will:

-   Complete with an exit code of `0` if the `chef-server.rb` file is
    created successfully.
-   Return an error message and a non-zero exit code if a node has not
    been bootstrapped or joined or if a FQDN is not provided.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl gen-server-config FQDN
```

### Configure the Front End

1.  On any node in the backend HA cluster, run the following command for
    each node in the frontend group:

    ``` bash
    chef-backend-ctl gen-server-config FQDN -f chef-server.rb.fqdn
    ```

    where `FQDN` is the FQDN for the frontend machine. The generated
    `chef-server.rb` file will contain all of the values necessary for
    any frontend Chef Infra Server to connect to and bootstrap against
    the backend HA cluster.

2.  On each frontend machine, install the `chef-server-core` package
    (version 12.4.0 or higher).

3.  On each frontend machine, copy the generated `chef-server.rb`.fqdn
    to `/etc/opscode/chef-server.rb`.

4.  On each frontend machine, with root permission, run the following
    command:

    ``` bash
    chef-server-ctl reconfigure
    ```

### Example Output

The following example shows the results of running the
`chef-backend-ctl gen-server-config` subcommand. The settings and output
will vary, depending on the configuration. These settings should be
modified carefully:

``` ruby
fqdn "frontend1.chef-demo.com"
postgresql['external'] = true
postgresql['vip'] = '192.0.2.0'
postgresql['db_superuser'] = 'chef_pgsql'
postgresql['db_superuser_password'] = '...6810e52a01e562'
opscode_solr4['external'] = true
opscode_solr4['external_url'] = 'http://192.0.2.0:9200'
opscode_erchef['search_provider'] = 'elasticsearch'
opscode_erchef['search_queue_mode'] = 'batch'
bookshelf['storage_type'] = :sql
rabbitmq['enable'] = false
rabbitmq['management_enabled'] = false
rabbitmq['queue_length_monitor_enabled'] = false
opscode_expander['enable'] = false
dark_launch['actions'] = false
opscode_erchef['nginx_bookshelf_caching'] = :on
opscode_erchef['s3_url_expiry_window_size'] = '50%'
```

{{< note >}}

The `opscode_solr4`, `postgresql`, and `rabbitmq` services are disabled
in this configuration file for the frontend machines when running the
Chef Infra Server with a backend HA cluster.

{{< /note >}}

## help

Use the `help` subcommand to print a list of all available
chef-backend-ctl commands.

This subcommand has the following syntax:

``` bash
chef-backend-ctl help
```

## join-cluster

Use the `join-cluster` subcommand to configure a node to be a member of
the backend HA cluster as a peer of the node at the specified
`PEER_NODE_IP` IP address. This command will query the existing cluster
to identify any necessary configuration information. If the
`--publish-address` and `--vip-interface` options are not specified, and
are specified in `chef-backend.rb`, the command will prompt with a list
of items from which to choose.

This command will return an error message and a non-zero exit code when:

-   The `--secrets-file-path` option is specified, a non-empty secrets
    file already exists at `/etc/chef-backend/secrets.json`, and the
    user declines to overwrite it.
-   The `--secrets-file-path` option is specified, but does not specify
    a valid file and/or the file at `/etc/chef-backend/secrets.json` is
    empty or does not exist.
-   The IP address specified by the `--publish-address` option does not
    exist on the node.
-   The interface specified by the `--vip-interface` option does not
    exist on the node.
-   Any IP address on the node is already registered in the backend HA
    cluster.
-   The node is already configured for the backend HA cluster and the
    `--recovery` option is not specified.
-   The `--recovery` option is specified, but no existing installation
    is found.
-   `--publish-address` and/or `--vip-interface` are specified, but a
    non-empty `chef-backend.rb` file already exists. (This command will
    not overwrite a `chef-backend.rb` file.)

If successful, this command will generate a `chef-backend.rb` file at
`/etc/chef-backend/chef-backend.rb` with the values for the
`publish_address`, `vip_interface`, and `vip` added automatically.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl join-cluster PEER_NODE_IP (options)
```

where `PEER_NODE_IP` is the IP address of a peer in the cluster to be
joined.

### Options

This subcommand has the following options:

`-i INTERFACE`, `--vip-interface INTERFACE`

:   The network interface to which the backend VIP will bind in the
    event that this node becomes leader. If not provided, and if not
    specified in `chef-backend.rb`, this command will prompt to choose
    from a list of interfaces that are currently available on the node.

    {{< note spaces=4 >}}

    This option should only be used the first time a node joins the
    backend HA cluster.

    {{< /note >}}

`-p IP_ADDRESS`, `--publish-address IP_ADDRESS`

:   The IP address that is published within the backend HA cluster. This
    IP address must be accessible to all nodes in the backend HA
    cluster. If not provided, and if not specified in `chef-backend.rb`,
    this command will prompt to choose from a list of IP addresses that
    are currently bound on the node.

    {{< note spaces=4 >}}

    This option should only be used the first time a node joins the
    backend HA cluster.

    {{< /note >}}

`--recovery`

:   Force this node to rejoin the backend HA cluster if it has been
    removed via the `chef-backend-ctl remove-node` or
    `chef-backend-ctl bootstrap --with-quorum-recovery` commands.

    {{< note spaces=4 >}}

    This option will run against the existing `chef-backend.rb` file,
    which means the `--vip-interface` and `--publish-address` options
    should not be specified. (They are already defined in the
    `chef-backend.rb` file.)

    {{< /note >}}

`-s PATH`, `--secrets-file-path PATH`

:   The path to the location of the `secrets.json` file on the
    bootstrapping node. Default value: `/etc/chef-backend/secrets.json`.

`-y`, `--yes`

:   Do not prompt for confirmation.

### Examples

None.

## promote

Use the `promote` subcommand to promote the named node to be leader of
the backend HA cluster. This command will:

-   Complete with an exit code of `0` when the leader of the backend HA
    cluster is replaced as leader by the named node.
-   Return an error message and a non-zero exit code if the named node
    is already leader because failover has been disabled (for either the
    cluster or the node) or if the new leader could not be promoted
    within the allowed time.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl promote NODE
```

### Examples

None.

## reconfigure

Use the `reconfigure` subcommand to reconfigure a machine in the Chef
Infra Server backend HA cluster. This subcommand will also restart any
services for which the `service_name['enabled']` setting is set to
`true`.

This subcommand has the following syntax:

``` bash
chef-backend-ctl reconfigure
```

## remove-node

Use the `remove-node` subcommand to remove the named node from the
backend HA cluster by removing the node's status from etcd and deleting
it from the etcd cluster. This command is useful when a node is going to
be replaced or if the `join-cluster` command was unsuccessful.

This command may not be run from the node that is to be removed; the
node itself must be shut down physically or have all services stopped
(via the the `chef-backend-ctl stop` command). If the node is still
running or otherwise available to the backend HA cluster, this command
will return an error message and a non-zero exist code.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl remove-node NODE_NAME
```

### Options

This subcommand has the following options:

`-y`, `--yes`

:   Do not prompt for confirmation.

### Examples

None.

## restore

Use the `restore` subcommand to restore a backup created by the
`chef-backend-ctl backup` subcommand. This command should be executed on
the leader node in the backend HA cluster. This command will delete all
existing data and replace it with the data in the backup archive.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl restore PATH (options)
```

where `PATH` is the path to a tar.gz file that was created by the
`chef-backend-ctl backup` subcommand.

### Options

This subcommand has the following options:

`-d DIRECTORY`, `--staging-dir DIRECTORY`

:   The path to an empty directory to be used during the restore
    operation. This directory must have enough available space to expand
    all data in the backup archive.

`-y`, `--yes`

:   Do not prompt for confirmation.

### Examples

**Restore data to the backend leader**

From the leader node, run the following command:

``` bash
chef-backend-ctl restore /var/opt/chef-backup/backup_file.tgz
```

## set-cluster-failover

Use the `set-cluster-failover` subcommand to enable or disable failover
across the backend HA cluster.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl set-cluster-failover STATE
```

where `STATE` may be one of `on`, `off`, `true`, `false`, `enabled`, or
`disabled`.

## set-node-failover

Use the `set-node-failover` subcommand to enable or disable failover for
a node in the backend HA cluster.

### Syntax

This subcommand has the following syntax:

``` bash
chef-backend-ctl set-node-failover STATE
```

where `STATE` may be one of `on`, `off`, `true`, `false`, `enabled`, or
`disabled`.

## show-config

The `show-config` subcommand is used to view the configuration that will
be generated by the `reconfigure` subcommand. This command is most
useful in the early stages of a deployment to ensure that everything is
built properly prior to installation.

This subcommand has the following syntax:

``` bash
chef-backend-ctl show-config
```

## status

Use the `status` subcommand to show the status of all services available
to a node in the backend HA cluster. This subcommand has the following
syntax:

``` bash
chef-backend-ctl status
```

and will return the status for all services. Status can be returned for
individual services by specifying the name of the service as part of the
command:

``` bash
chef-backend-ctl status SERVICE_NAME
```

For example, full output:

``` bash
chef-backend-ctl status
```

is similar to:

``` bash
Service        Local Status        Time in State  Distributed Node Status
elasticsearch  running (pid 6661)  1d 5h 59m 41s  state: green; nodes online: 3/3
etcd           running (pid 6742)  1d 5h 59m 39s  health: green; healthy nodes: 3/3
leaderl        running (pid 6788)  1d 5h 59m 35s  leader: 1; waiting: 0; follower: 2; total: 3
postgresql     running (pid 6640)  1d 5h 59m 43s  leader: 1; offline: 0; syncing: 0; synced: 2
```

which shows status for a healthy backend HA cluster. The first three
columns--`Service`, `Local Status`, and `Time in State` summarize the
local state. The `Distributed Node Status` column shows:

-   A three node cluster
-   All nodes healthy and online
-   A leader selected with two followers (for both leader/follower
    status for the `leaderl` service and a leader/synced state for the
    `postgresql` service)

Simple output:

``` bash
chef-backend-ctl status --simple
```

is similar to:

``` bash
run: elasticsearch: (pid 6661) 106983s; run: log: (pid 6667) 106983s
run: etcd: (pid 6742) 106981s; run: log: (pid 6630) 106984s
run: leaderl: (pid 6788) 106976s; run: log: (pid 6739) 106982s
run: postgresql: (pid 6640) 106984s; run: log: (pid 6653) 106983s
```

which shows the state of the services, process identifiers, and uptime
(in seconds). Simple output is useful if one (or more) nodes in the
backend HA cluster are down or in a degraded state.

## uninstall

The `uninstall` subcommand is used to remove the Chef Infra Server
application from a machine in the backend HA cluster, but without
removing any of the data. This subcommand will shut down all services
(including the `runit` process supervisor).

This subcommand has the following syntax:

``` bash
chef-backend-ctl uninstall
```

{{< note >}}

To revert the `uninstall` subcommand, run the `reconfigure` subcommand
(because the `start` subcommand is disabled by the `uninstall` command).

{{< /note >}}

## Service Subcommands

{{% ctl_common_service_subcommands %}}

{{< warning >}}

The following commands are disabled when an external PostgreSQL database
is configured for the Chef Infra Server: `hup`, `int`, `kill`, `once`,
`restart`, `service-list`, `start`, `stop`, `tail`, and `term`.

{{< /warning >}}

### hup

Use the `hup` subcommand to send a `SIGHUP` to all services on a machine
in the Chef Infra Server backend HA cluster. This command can also be
run for an individual service by specifying the name of the service in
the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl hup SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.

### int

Use the `int` subcommand to send a `SIGINT` to all services on a machine
in the Chef Infra Server backend HA cluster. This command can also be
run for an individual service by specifying the name of the service in
the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl int SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.

### kill

Use the `kill` subcommand to send a `SIGKILL` to all services on a
machine in the Chef Infra Server backend HA cluster. This command can
also be run for an individual service by specifying the name of the
service in the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl kill SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.

### once

The supervisor for a machine in the Chef Infra Server backend HA cluster
is configured to restart any service that fails, unless that service has
been asked to change its state. The `once` subcommand is used to tell
the supervisor to not attempt to restart any service that fails.

This command is useful when troubleshooting configuration errors that
prevent a service from starting. Run the `once` subcommand followed by
the `status` subcommand to look for services in a down state and/or to
identify which services are in trouble. This command can also be run for
an individual service by specifying the name of the service in the
command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl once SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.

### restart

Use the `restart` subcommand to restart all services enabled on a
machine in the Chef Infra Server backend HA cluster, or to restart an
individual service by specifying the name of that service in the
command.

{{< warning >}}

When running the Chef Infra Server in a high availability configuration,
restarting all services may trigger failover.

{{< /warning >}}

This subcommand has the following syntax:

``` bash
chef-backend-ctl restart SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand. When a service is
successfully restarted the output should be similar to:

``` bash
ok: run: service_name: (pid 12345) 1s
```

### service-list

Use the `service-list` subcommand to display a list of all available
services on a machine in the Chef Infra Server backend HA cluster. A
service that is enabled is labeled with an asterisk (\*).

This subcommand has the following syntax:

``` bash
chef-backend-ctl service-list
```

### start

Use the `start` subcommand to start all services that are enabled on a
machine in the Chef Infra Server backend HA cluster. This command can
also be run for an individual service by specifying the name of the
service in the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl start SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand. When a service is
successfully started the output should be similar to:

``` bash
ok: run: service_name: (pid 12345) 1s
```

The supervisor for a machine in the Chef Infra Server backend HA cluster
is configured to wait seven seconds for a service to respond to a
command from the supervisor. If you see output that references a
timeout, it means that a signal has been sent to the process, but that
the process has yet to actually comply. In general, processes that have
timed out are not a big concern, unless they are failing to respond to
the signals at all. If a process is not responding, use a command like
the `kill` subcommand to stop the process, investigate the cause (if
required), and then use the `start` subcommand to re-enable it.

### stop

Use the `stop` subcommand to stop all services enabled on the Chef Infra
Server backend HA cluster. This command can also be run for an
individual service by specifying the name of the service in the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl stop SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand. When a service is
successfully stopped the output should be similar to:

``` bash
ok: diwb: service_name: 0s, normally up
```

For example:

``` bash
chef-backend-ctl stop
```

will return something similar to:

``` bash
ok: down: etcd: 393s, normally up
ok: down: postgresql: 388s, normally up
```

### tail

Use the `tail` subcommand to follow all of the logs for all services on
a machine in the Chef Infra Server backend HA cluster. This command can
also be run for an individual service by specifying the name of the
service in the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl tail SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.

### term

Use the `term` subcommand to send a `SIGTERM` to all services on a
machine in the Chef Infra Server backend HA cluster. This command can
also be run for an individual service by specifying the name of the
service in the command.

This subcommand has the following syntax:

``` bash
chef-backend-ctl term SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.
