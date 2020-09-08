+++
title = "High Availability: Chef Backend"
draft = false

aliases = ["/install_server_ha.html"]

[menu]
  [menu.infra]
    title = "Install High Availability"
    identifier = "chef_infra/setup/chef_infra_server/install_server_ha.md Install High Availability"
    parent = "chef_infra/setup/chef_infra_server"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_server_ha.md)

This topic introduces the underlying concepts behind the architecture of
the high availability Chef Infra Server cluster. The topic then
describes the setup and installation process for a high availability
Chef Infra Server cluster comprised of five total nodes (two frontend
and three backend).

## Overview

The Chef Infra Server can operate in a high availability configuration
that provides automated load balancing and failover for stateful
components in the system architecture. This type of configuration
typically splits the servers into two segments: The backend cluster, and
the frontend group.

-   The frontend group, comprised of one (or more) nodes running the
    Chef Infra Server. Nodes in the frontend group handle requests to
    the Chef Infra Server API and access to the Chef management console.
    Frontend group nodes should be load balanced, and may be scaled
    horizontally by increasing the number of nodes available to handle
    requests.

-   The backend cluster, comprised of three nodes working together,
    provides high availability data persistence for the frontend group.

    {{< note spaces=4 >}}

    At this time, backend clusters can only have three nodes.

    {{< /note >}}

![image](/images/chef_server_ha_cluster.svg)

{{< important >}}

When doing cloud deployments, Chef HA clusters are not meant to be
geographically dispersed across multiple regions or datacenters;
however, in cloud providers such as AWS, you can deploy HA clusters
across multiple Availability Zones within the same region.

{{< /important >}}

### Key Differences From Standalone Chef server

There are several key differences between the high availability Chef
Infra Server cluster and a standalone Chef Infra Server instance.

-   While Apache Solr is used in standalone Chef Infra Server instances,
    in the high availability Chef Infra Server cluster it is replaced
    with Elasticsearch. Elasticsearch provides more flexible clustering
    options while maintaining search API compatibility with Apache Solr.
-   Writes to the search engine and the database are handled
    asynchronously via RabbitMQ and chef-expander in standalone Chef
    Infra Server instances. However, the high availability Chef server
    cluster writes to the search engine and the database simultaneously.
    As such the RabbitMQ and chef-expander services are no longer
    present in the high availability Chef Infra Server cluster.
-   Standalone Chef Infra Server instances write Bookshelf data to the
    filesystem. In a high availability Chef Infra Server cluster,
    Bookshelf data is written to the database.

## Recommended Cluster Topology

### Nodes

-   The HA backend installation requires three cluster nodes. Chef has
    not tested and does not support installations with other numbers of
    backend cluster nodes.
-   One or more frontend group nodes

#### Hardware Requirements

The following are a list of general hardware requirements for both
frontend and backend servers. The important guideline you should follow
are that frontend servers tend to be more CPU bound and backend servers
are more disk and memory bound. Also, disk space for backend servers
should scale up with the number of nodes that the servers are managing.
A good rule to follow is to allocate 2 MB per node. The disk values
listed below should be a good default value that you will want to modify
later if/when your node count grows.

-   64-bit architecture

{{% system_requirements_ha %}}

### Network Services

-   A load balancer between the rest of the network, and the frontend
    group (Not provided). Because management console session data is
    stored on each node in the frontend group individually, the load
    balancer should be configured with sticky sessions.

### Network Port Requirements

#### Inbound from load balancer to frontend group

-   TCP 80 (HTTP)
-   TCP 443 (HTTPS)

#### Inbound from frontend group to backend cluster

-   TCP 2379 (etcd)
-   TCP 5432 (PostgreSQL)
-   TCP 7331 (leaderl)
-   TCP 9200-9300 (Elasticsearch)

#### Peer communication, backend cluster

-   2379 (etcd)
-   2380 (etcd)
-   5432 (PostgreSQL)
-   9200-9400 (Elasticsearch)

## Installation

These instructions assume you are using the minimum versions:

-   Chef Server : 12.5.0
-   Chef Backend : 0.8.0

Download [Chef Infra Server](https://downloads.chef.io/chef-server/) and
[Chef Backend (chef-backend)](https://downloads.chef.io/chef-backend/)
if you do not have them already.

Before creating the backend HA cluster and building at least one Chef
Infra Server to be part of the frontend group, verify:

-   The user who will install and build the backend HA cluster and
    frontend group has root access to all nodes.
-   The number of backend and frontend nodes that are desired. It is
    required to have three backend nodes, but the number of frontend
    nodes may vary from a single node to a load-balanced tiered
    configuration.
-   SSH access to all boxes that will belong to the backend HA cluster
    from the node that will be the initial bootstrap.
-   A time synchronization policy is in place, such as Network Time
    Protocol (NTP). Drift of less than 1.5 seconds must exist across all
    nodes in the backend HA cluster.

### Step 1: Create Cluster

The first node must be bootstrapped to initialize the cluster. The node
used to bootstrap the cluster will be the cluster leader when the
cluster comes online. After bootstrap completes this node is no
different from any other back-end node.

1.  Install the Chef Backend package on the first backend node as root.

    -   Download [Chef Backend
        (chef-backend)](https://downloads.chef.io/chef-backend/)
    -   In Red Hat/CentOS: `yum install PATH_TO_RPM`
    -   In Debian/Ubuntu: `dpkg -i PATH_TO_DEB`

2.  Update `/etc/chef-backend/chef-backend.rb` with the following
    content:

    ``` ruby
    publish_address 'external_IP_address_of_this_box' # External ip address of this backend box
    ```

3.  If any of the backends or frontends are in different networks from
    each other then add a `postgresql.md5_auth_cidr_addresses` line to
    `/etc/chef-backend/chef-backend.rb` with the following content where
    `, "<NET-1_IN_CIDR>", ..., "<NET-N_IN_CIDR>"` is the list of all of
    the networks that your backends and frontends are in. See the
    [Configuring Frontend and Backend Members on Different
    Networks](/install_server_ha/#configuring-frontend-and-backend-members-on-different-networks)
    section for more information:

    ``` ruby
    publish_address 'external_IP_address_of_this_box' # External ip address of this backend box
    postgresql.md5_auth_cidr_addresses = ["samehost", "samenet", "<NET-1_IN_CIDR>", ..., "<NET-N_IN_CIDR>"]
    ```

4.  Run `chef-backend-ctl create-cluster`.

### Step 2: Shared Credentials

The credentials file `/etc/chef-backend/chef-backend-secrets.json`
generated by bootstrapping must be shared with the other nodes. You may
copy them directly, or expose them via a common mounted location.

For example, to copy using ssh:

``` bash
scp /etc/chef-backend/chef-backend-secrets.json <USER>@<IP_BE2>:/home/<USER>
scp /etc/chef-backend/chef-backend-secrets.json <USER>@<IP_BE3>:/home/<USER>
```

Delete this file from the destination after Step 4 has been completed
for each backend being joined to the cluster.

### Step 3: Install and Configure Remaining Backend Nodes

For each additional node do the following in sequence (if you attempt to
join nodes in parallel the cluster may fail to become available):

1.  Install the Chef Backend package on the node.

    -   Download [Chef Backend
        (chef-backend)](https://downloads.chef.io/chef-backend/)
    -   In Red Hat/CentOS: `yum install PATH_TO_RPM`
    -   In Debian/Ubuntu: `dpkg -i PATH_TO_DEB`

2.  If you added a `postgresql.md5_auth_cidr_addresses` line to the
    leader's `/etc/chef-backend/chef-backend.rb` in [Step 1: Create
    Cluster](/install_server_ha/#step-1-create-cluster) then update
    this node's `/etc/chef-backend/chef-backend.rb` with the following
    content where `postgresql.md5_auth_cidr_addresses` is set to the
    same value used in the leader's `chef-backend.rb`. If all of the
    backend and frontend clusters are in the same network then you don't
    need to modify this node's `/etc/chef-backend/chef-backend.rb` at
    all.

    ``` ruby
    publish_address 'external_IP_address_of_this_box' # External ip address of this backend box
    postgresql.md5_auth_cidr_addresses = ["samehost", "samenet", "<NET-1_IN_CIDR>", ..., "<NET-N_IN_CIDR>"]
    ```

3.  As root or with sudo:

    ``` bash
    chef-backend-ctl join-cluster <IP_BE1> -s /home/<USER>/chef-backend-secrets.json
    ```

4.  Answer the prompts regarding which public IP to use. As an
    alternative, you may specify them on the `chef-backend join-cluster`
    command line. See `chef-backend-ctl join-cluster --help` for more
    information. If you manually added the `publish_address` line to
    `/etc/chef-backend/chef-backend.rb` then you will not be prompted
    for the public IP and you should not use the `--publish-address`
    option to specify the the public IP on the
    `chef-backend join-cluster` command line.

5.  If you copied the shared `chef-backend-secrets.json` file to a user
    HOME directory on this host, remove it now.

6.  Repeat these steps for each follower node, after which the cluster
    is online and available. From any node in the backend HA cluster,
    run the following command:

    ``` bash
    chef-backend-ctl status
    ```

    should return something like:

    ``` bash
    Service        Local Status        Time in State  Distributed Node Status
    elasticsearch  running (pid 6661)  1d 5h 59m 41s  state: green; nodes online: 3/3
    etcd           running (pid 6742)  1d 5h 59m 39s  health: green; healthy nodes: 3/3
    leaderl        running (pid 6788)  1d 5h 59m 35s  leader: 1; waiting: 0; follower: 2; total: 3
    postgresql     running (pid 6640)  1d 5h 59m 43s  leader: 1; offline: 0; syncing: 0; synced: 2
    ```

### Step 4: Generate Chef Infra Server Configuration

Log into the node from Step 1, and we will generate our chef-server
frontend node configuration:

``` bash
chef-backend-ctl gen-server-config <FE1-FQDN> -f chef-server.rb.FE1
scp chef-server.rb.FE1 USER@<IP_FE1>:/home/<USER>
```

{{< note >}}

`/etc/chef-backend/chef-backend-secrets.json` is *not* made available to
Chef Infra Server frontend nodes.

{{< /note >}}

### Step 5: Install and Configure First Frontend

On the first frontend node, assuming that the generated configuration
was copied as detailed in Step 4:

1.  Install the current chef-server-core package
2.  Run
    `cp /home/<USER>/chef-server.rb.<FE1> /etc/opscode/chef-server.rb`
3.  As the root user, run `chef-server-ctl reconfigure`

### Step 6: Adding More Frontend Nodes

For each additional frontend node you wish to add to your cluster:

1.  Install the current chef-server-core package.

2.  Generate a new `/etc/opscode/chef-server.rb` from any of the backend
    nodes via

    ``` bash
    chef-backend-ctl gen-server-config <FE_NAME-FQDN> > chef-server.rb.<FE_NAME>
    ```

3.  Copy it to `/etc/opscode` on the new frontend node.

4.  From the first frontend node configured in Step 5, copy the
    following files from the first frontend to `/etc/opscode` on the new
    frontend node:

    -   /etc/opscode/private-chef-secrets.json

    {{< note spaces=4 >}}

    For Chef Server versions prior to 12.14, you will also need to copy
    the key files:

    -   /etc/opscode/webui_priv.pem
    -   /etc/opscode/webui_pub.pem
    -   /etc/opscode/pivotal.pem

    {{< /note >}}

5.  On the new frontend node run `mkdir -p /var/opt/opscode/upgrades/`.

6.  From the first frontend node, copy
    `/var/opt/opscode/upgrades/migration-level` to the same location on
    the new node.

7.  On the new frontend run `touch /var/opt/opscode/bootstrapped`.

8.  On the new frontend run `chef-server-ctl reconfigure` as root.

### Upgrading Chef Infra Server on the Frontend Machines

1.  On one frontend server, follow the [standalone upgrade
    process](/upgrade_server/#standalone).
2.  Copy `/var/opt/opscode/upgrades/migration-level` from the first
    upgraded frontend to `/var/opt/opscode/upgrades/migration-level` on
    each of the remaining frontends.
3.  Once the updated file has been copied to each of the remaining
    frontends, perform the [standalone upgrade
    process](/upgrade_server/#standalone) on each of the frontend
    servers.

### Configuring Frontend and Backend Members on Different Networks

By default, PostgreSQL only allows systems on its local network to
connect to the database server that runs it and the `pg_hba.conf` used
by PostgreSQL controls network access to the server. The default
`pg_hba.conf` has the following four entries:

``` none
host    all         all         samehost               md5
hostssl replication replicator  samehost               md5
host    all         all         samenet                md5
hostssl replication replicator  samenet                md5
```

To allow other systems to connect, such as members of a frontend group
that might exist on a different network, you will need to authorize that
usage by adding the following line to the
`/etc/chef-backend/chef-backend.rb` file on all of the backend members.

``` none
postgresql.md5_auth_cidr_addresses = ["samehost", "samenet", "<YOURNET IN CIDR>"]
```

After setting the `md5_auth_cidr_addresses` value and reconfiguring the
server, two entries will be created in `pg_hba.conf` for each value in
the `md5_auth_cidr_addresses` array. Existing values in `pg_hba.conf`
will be overwritten by the values in the array, so we must also specify
"samehost" and "samenet", which will continue to allow systems on a
local network to connect to PostgreSQL.

For example, if a frontend host at 192.168.1.3 can reach a backend
member over the network, but the backend's local network is 192.168.2.x,
you would add the following line to `/etc/chef-backend/chef-backend.rb`

``` none
postgresql.md5_auth_cidr_addresses = ["samehost", "samenet", "192.168.1.3/24"]
```

which would result in the following two entries being added to the
`pg_hba.conf` file.

``` none
host    all         all         samehost               md5
hostssl replication replicator  samehost               md5
host    all         all         samenet                md5
hostssl replication replicator  samenet                md5
host    all         all         192.168.1.3/24         md5
hostssl replication replicator  192.168.1.3/24         md5
```

Running `chef-backend-ctl reconfigure` on all the backends will allow
that frontend to complete its connection.

{{< important >}}

The `postgresql.md5_auth_cidr_addresses` subnet settings must be
identical for all members of the backend cluster. In the case where the
subnet settings of the frontend cluster are different from the subnet
settings of the backend cluster, the values set on the members of the
backend cluster should contain the subnet of the frontend cluster. This
guarantees that all members of a cluster can still communicate with each
other after a cluster change of state occurs. For example, if the
frontend subnet setting is "192.168.1.0/24" and the backend subnet
setting is "192.168.2.0/24", then the
`postgresql.md5_auth_cidr_addresses` subnet settings must be
`postgresql.md5_auth_cidr_addresses = ["samehost", "samenet", "192.168.1.0/24", 192.168.2.0/24]`

{{< /important >}}

## Cluster Security Considerations

A backend cluster is expected to run in a trusted environment. This
means that untrusted users that communicate with and/or eavesdrop on
services provided by the backend cluster can potentially view sensitive
data.

### Communication Between Nodes

PostgreSQL communication between nodes in the backend cluster is
encrypted, and uses password authentication. All other communication in
the backend cluster is unauthenticated and happens in the clear (without
encryption).

### Communication Between Frontend Group & Backend Cluster

PostgreSQL communication from nodes in the frontend group to the leader
of the backend cluster uses password authentication, but communication
happens in the clear (without encryption).

Elasticsearch communication is unauthenticated and happens in the clear
(without encryption).

### Securing Communication

Because most of the peer communication between nodes in the backend
cluster happens in the clear, the backend cluster is vulnerable to
passive monitoring of network traffic between nodes. To help prevent an
active attacker from intercepting or changing cluster data, Chef
recommends using iptables or an equivalent network ACL tool to restrict
access to PostgreSQL, Elasticsearch and etcd to only hosts that need
access.

By service role, access requirements are as follows:

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Service</th>
<th>Access Requirements</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>PostgreSQL</td>
<td>All backend cluster members and all Chef Infra Server frontend group nodes.</td>
</tr>
<tr class="even">
<td>Elasticsearch</td>
<td>All backend cluster members and all Chef Infra Server frontend group nodes.</td>
</tr>
<tr class="odd">
<td>etcd</td>
<td>All backend cluster members and all Chef Infra Server frontend group nodes.</td>
</tr>
</tbody>
</table>

### Services and Secrets

Communication with PostgreSQL requires password authentication. The
backend cluster generates PostgreSQL users and passwords during the
initial cluster-create. These passwords are present in the following
files on disk:

<table style="width:100%;">
<colgroup>
<col style="width: 61%" />
<col style="width: 14%" />
<col style="width: 14%" />
<col style="width: 9%" />
</colgroup>
<thead>
<tr class="header">
<th>Secret</th>
<th>Owner</th>
<th>Group</th>
<th>Mode</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>/etc/chef-backend/secrets.json</code></td>
<td><code>root</code></td>
<td><code>chef_pgsql</code></td>
<td><code>0640</code></td>
</tr>
<tr class="even">
<td><code>/var/opt/chef-backend/leaderl/data/sys.config</code></td>
<td><code>chef_pgsql</code></td>
<td><code>chef_pgsql</code></td>
<td><code>0600</code></td>
</tr>
<tr class="odd">
<td><code>/var/opt/chef-backend/PostgreSQL/9.5/recovery.conf</code></td>
<td><code>chef_pgsql</code></td>
<td><code>chef_pgsql</code></td>
<td><code>0600</code></td>
</tr>
</tbody>
</table>

The following services run on each node in the backend cluster. The user
account under which the service runs as listed the second column:



The following services run on each node in the backend cluster. The user
account under which the service runs as listed the second column:

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Service</th>
<th>Process Owner</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>postgresql</code></td>
<td><code>chef_pgsql</code></td>
</tr>
<tr class="even">
<td><code>elasticsearch</code></td>
<td><code>chef-backend</code></td>
</tr>
<tr class="odd">
<td><code>etcd</code></td>
<td><code>chef-backend</code></td>
</tr>
<tr class="even">
<td><code>leaderl</code></td>
<td><code>chef_pgsql</code></td>
</tr>
<tr class="odd">
<td><code>epmd</code></td>
<td><code>chef_pgsql</code> (or first user launching an erlang process)</td>
</tr>
</tbody>
</table>

#### Chef Infra Server frontend

The `chef-backend-ctl gen-server-config` command, which can be run as
root from any node in the backend cluster, will automatically generate a
configuration file containing the superuser database access credentials
for the backend cluster PostgreSQL instance.

### Software Versions

The backend HA cluster uses the Chef installer to package all of the
software necessary to run the services included in the backend cluster.
For a full list of the software packages included (and their versions),
see the file located at `/opt/chef-backend/version-manifest.json`.

Do not attempt to upgrade individual components of the Chef package. Due
to the way Chef packages are built, modifying any of the individual
components in the package will lead to cluster instability. If the
latest version of the backend cluster is providing an out-of-date
package, please bring it to the attention of Chef by filling out a
ticket with <support@chef.io>.

## chef-backend.rb Options

The `chef-backend.rb` file is generated using
`chef-backend-ctl gen-sample-backend-config` and controls most of the
various feature and configuration flags going into a Chef HA backend
node. A number of these options control the reliability, stability and
uptime of the backend PostgreSQL databases, the elastic search index,
and the leader election system. Please refrain from changing them unless
you have been advised to do so.

-   `fqdn` Host name of this node.
-   `hide_sensitive` Set to `false` if you wish to print deltas of
    sensitive files and templates during `chef-backend-ctl reconfigure`
    runs. `true` by default.
-   `ip_version` Set to either `'ipv4'` or `'ipv6'`. `'ipv4'` by
    default.
-   `publish_address` Externally resolvable IP address of this back-end
    node.

### Common 'Runit' flags for any backend service

See <https://github.com/chef-cookbooks/runit> for details. Many of the
flags are repeated across the various backend services - they are only
documented once at the top here. The same defaults are used unless
specified below.

-   `postgresql.enable` Sets up and runs this service. `true` by
    default.
-   `postgresql.environment` A hash of environment variables with their
    values as content used in the service's env directory.
-   `postgresql.log_directory` The directory where the svlogd log
    service will run. `'/var/log/chef-backend/postgresql/<version>'` by
    default.
-   `postgresql.log_rotation.file_maxbytes` The maximum size a log file
    can grow to before it is automatically rotated. `104857600` by
    default (100MB).
-   `postgresql.log_rotation.num_to_keep` The maximum number of log
    files that will be retained after rotation. `10` by default.
-   `etcd.enable`
-   `etcd.log_directory` `'/var/log/chef-backend/etcd'` by default
-   `etcd.log_rotation.file_maxbytes`
-   `etcd.log_rotation.num_to_keep`
-   `elasticsearch.enable`
-   `elasticsearch.log_directory`
    `'/var/log/chef-backend/elasticsearch'` by default. Also affects
    `path.logs` in the elastic search configuration yml.
-   `elasticsearch.log_rotation.file_maxbytes`
-   `elasticsearch.log_rotation.num_to_keep`
-   `leaderl.enable`
-   `leaderl.log_directory` `'/var/log/chef-backend/leaderl'` by
    default.
-   `leaderl.start_down` Set the default state of the runit service to
    'down' by creating \<sv_dir\>/down file. `true` by default.
-   `leaderl.log_rotation.file_maxbytes`
-   `leaderl.log_rotation.num_to_keep`

### PostgreSQL settings

-   `postgresql.db_superuser` Super user account to create. Password is
    in chef-backend-secrets.json. `'chef_pgsql'` by default.
-   `postgresql.md5_auth_cidr_addresses` A list of authorized addresses
    from which other backend nodes can connect to perform streaming
    replication. `samehost` and `samenet` are special symbols to allow
    connections from the this node's IP address and its subnet. You may
    also use `all` to match any IP address. You may specify a hostname
    or IP address in CIDR format (`172.20.143.89/32` for a single host,
    or `172.20.143.0/24` for a small network. See
    <https://www.postgresql.org/docs/9.5/static/auth-pg-hba-conf.html>
    for alternative formats. `["samehost", "samenet"]` by default.
-   `postgresql.replication_user` Username used by postgres streaming
    replicator when accessing this node. `'replicator'` by default.
-   `postgresql.username` `'chef_pgsql'` by default.

### PostgreSQL settings given to `postgresql.conf`

See <https://www.postgresql.org/docs/9.5/static/runtime-config.html> for
details. Some defaults are provided:

-   `postgresql.archive_command ''`
-   `postgresql.archive_mode 'off'`
-   `postgresql.archive_timeout 0`
-   `postgresql.checkpoint_completion_target 0.5`
-   `postgresql.checkpoint_timeout '5min'`
-   `postgresql.checkpoint_warning '30s'`
-   `postgresql.effective_cache_size` Automatically calculated based on
    available memory.
-   `postgresql.hot_standby 'on'`
-   `postgresql.keepalives_count 2` Sets `tcp_keepalives_count`
-   `postgresql.keepalives_idle 60` Sets `tcp_keepalives_idle`
-   `postgresql.keepalives_interval 15` Sets `tcp_keepalives_interval`
-   `postgresql.log_checkpoints true`
-   `postgresql.log_min_duration_statement -1`
-   `postgresql.max_connections 350`
-   `postgresql.max_replication_slots 12`
-   `postgresql.max_wal_senders 12`
-   `postgresql.max_wal_size 64`
-   `postgresql.min_wal_size 5`
-   `postgresql.port 5432`
-   `postgresql.shared_buffers` Automatically calculated based on
    available memory.
-   `postgresql.wal_keep_segments 32`
-   `postgresql.wal_level 'hot_standby'`
-   `postgresql.wal_log_hints on`
-   `postgresql.work_mem '8MB'`

### etcd settings

-   `etcd.client_port 2379` Port to use for ETCD_LISTEN_CLIENT_URLS
    and ETCD_ADVERTISE_CLIENT_URLS.
-   `etcd.peer_port 2380` Port to use for ETCD_LISTEN_PEER_URLS and
    ETCD_ADVERTISE_PEER_URLS.

The following settings relate to etcd's consensus protocol. Chef Backend
builds its own leader election on top of etcd's consensus protocol.
Updating these settings may be advisable if you are seeing frequent
failover events as a result of spurious etcd connection timeouts. The
current defaults assume a high-latency environment, such those you might
find if deploying Chef Backend to various cloud providers.

-   `etcd.heartbeat_interval 500` ETCD_HEARTBEAT_INTERVAL in
    milliseconds. This is the frequency at which the leader will send
    heartbeats to followers. Etcd's documentation recommends that this
    is set roughly to the round-trip times between members. (The default
    before 1.2 was 100)
-   `etcd.election_timeout 5000` ETCD_ELECTION_TIMEOUT in
    milliseconds. This controls how long an etcd node will wait for
    heartbeat before triggering an election. Per Etcd's documentation,
    this should be 5 to 10 times larger than the
    `etcd.heartbeat_interval`. Increasing `etcd.election_timeout`
    increases the time it will take for `etcd` to detect a failure. (The
    default value before 1.2 was 1000)
-   `etcd.snapshot_count 5000` ETCD_SNAPSHOT_COUNT which is the number
    of committed transactions to trigger a snapshot to disk.

{{< note >}}

Even though the defaults assume a high-latency environment, cloud
deployments should be restricted to the same datacenter, or in AWS, in
the same region. This means that geographically-dispersed cluster
deployments are not supported. Multiple Availability Zones *are*
supported as long as they are in the same region.

{{< /note >}}

For additional information on the etcd tunables, see
<https://coreos.com/etcd/docs/latest/tuning.html>.

### Elastic Search JVM settings

-   `elasticsearch.heap_size` Automatically computed by elastic search
    based on available memory. Specify in MB if you wish to override.
-   `elasticsearch.java_opts` Flags to directly pass to the JVM when
    launching elastic search. If you override a heap flag here, the
    setting here takes precedence.
-   `elasticsearch.new_size` Java heap's new generation size.

### Elastic Search configuration

See
<https://www.elastic.co/guide/en/elasticsearch/reference/current/settings.html>
for details.

-   `elasticsearch.plugins_directory '/var/opt/chef-backend/elasticsearch/plugins'`
    Sets `path.plugins`.
-   `elasticsearch.port 9200` Sets `http.port`.
-   `elasticsearch.scripts_directory '/var/opt/chef-backend/elasticsearch/scripts'`
    Sets `path.scripts`.

### Chef HA backend leader management service settings

-   `leaderl.db_timeout` Socket timeout when connecting to PostgreSQL in
    milliseconds. `2000` by default.
-   `leaderl.http_acceptors` Http threads that responds to monitoring
    and leadership status requests from HAProxy. `10` by default.
-   `leaderl.http_address` The address that leaderl listens on. This
    address should not be `127.0.0.1`. It should be reachable from any
    front-end node. `'0.0.0.0'` by default.
-   `leaderl.http_port` `7331` by default.
-   `leaderl.leader_ttl_seconds` The number of seconds it takes the
    leader key to expire. Increasing this value will increase the amount
    of time the cluster will take to recognize a failed leader. Lowering
    this value may lead to frequent leadership changes and thrashing.
    `30` by default (`10` by default before 1.2).
-   `leaderl.required_active_followers` The number of followers that
    must be syncing via a PostgreSQL replication slot before a new
    leader will return 200 to /leader HTTP requests. If an existing
    leader fails to maintain this quorum of followers, the /leader
    endpoint will return 503 but active connections will still be able
    to complete their writes to the database. 0 by default.
-   `leaderl.runsv_group` The group that sensitive password files will
    belong to. This is used internally for test purposes and should
    never be modified otherwise. `'chef_pgsql'` by default.
-   `leaderl.status_internal_update_interval_seconds` How often we check
    for a change in the leader service's status. 5 seconds by default.
-   `leaderl.status_post_update_interval_seconds` How often etcd is
    updated with the leader service's current status. 10 seconds by
    default.
-   `leaderl.username 'chef_pgsql'`
-   `leaderl.log_rotation.max_messages_per_second` Rate limit for the
    number of messages that the Erlang error_logger will output. `1000`
    by default.
-   `leaderl.etcd_pool.ibrowse_options` Internal options to affect how
    requests to etcd are made (see
    <https://github.com/cmullaparthi/ibrowse/blob/master/doc/ibrowse.html>).
-   `leaderl.epmd_monitor.check_interval` How often (in milliseconds) to
    check that leaderl is registered with the Erlang Port Mapping Daemon
    (epmd). `60000` by default.

### Chef HA backend leader health status settings

-   `leaderl.health_check.interval_seconds` How frequently, in seconds,
    to poll the service for health status. We recommend setting this to
    at least 5 times the value of `leaderl.leader_ttl_seconds`. 5 by
    default (2 by default before version 1.2)
-   `leaderl.health_check.max_bytes_behind_leader` Limit on maximum
    different between elected leader and current node in bytes.
    `52428800` (50MB) by default.
-   `leaderl.health_check.max_elasticsearch_failures` Number of Elastic
    Search API failures allowed before health check fails. 5 by default.
-   `leaderl.health_check.max_etcd_failures` Number of etcd failures
    allowed before health check fails. 5 by default.
-   `leaderl.health_check.max_pgsql_failures` Number of PostgreSQL
    connection failures allowed before health check fails. 5 by default.
-   `leaderl.health_check.fatal_system_checks` Whether or not system
    check failures (such as disk space failures) will result in the node
    being marked ineligible for leadership. `false` by default. **Added
    in Chef Backend 1.4.**
-   `leaderl.health_check.disk_paths` An array containing the paths to
    check for sufficient disk space.
    `[/var/log/chef-backend, /var/opt/chef-backend]` by default. **Added
    in Chef Backend 1.4.**
-   `leaderl.health_check.disk_min_space_mb` The minimum amount of disk
    space (in megabytes) required for a disk health check to pass. `250`
    by default. **Added in Chef Backend 1.4.**

### Chef HA backend leader connection pool settings

See <https://github.com/seth/pooler/blob/master/README.org> for details.
These are internal settings that affect the responsiveness, uptime and
reliability of the backend cluster. They should not be modified unless
you are advised to do so by Support.

-   `leaderl.etcd_pool.cull_interval_seconds 60`
-   `leaderl.etcd_pool.http_timeout_ms 5000`
-   `leaderl.etcd_pool.init_count 10`
-   `leaderl.etcd_pool.max_age_seconds 60`
-   `leaderl.etcd_pool.max_connection_duration_seconds 300`
-   `leaderl.etcd_pool.max_count 10`

### SSL settings

If `certificate` and `certificate_key` are nil, the SSL Certificate will
be auto-generated using the other parameters provided. Otherwise, they
are on-disk locations to user-provided certificate.

-   `ssl.certificate` Provide this path if you have a pre-generated SSL
    cert.
-   `ssl.certificate_key` Provide this path if you have a pre-generated
    SSL cert.
-   `ssl.ciphers` Ordered list of allowed SSL ciphers. This will be
    updated based on security considerations and the version of OpenSSL
    being shipped.
-   `ssl.company_name`
-   `ssl.country_name`
-   `ssl.data_dir` Where certificates will be stored.
    `'/var/opt/chef-backend/ssl/'` by default
-   `ssl.duration` 3650 days by default (10 years).
-   `ssl.key_length` 2048 by default.
-   `ssl.organizational_unit_name`

## chef-backend-ctl

The Chef Infra Server backend HA cluster includes a command-line utility
named chef-backend-ctl. This command-line tool is used to manage the
Chef Infra Server backend HA cluster, start and stop individual
services, and tail Chef Infra Server log files. For more information,
see the [chef-backend-ctl documentation](/ctl_chef_backend/).
