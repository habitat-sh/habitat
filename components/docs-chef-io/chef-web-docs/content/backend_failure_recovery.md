+++
title = "Chef Backend Failure Recovery"
draft = false

aliases = ["/backend_failure_recovery.html"]

[menu]
  [menu.infra]
    title = "Backend Failure Recovery"
    identifier = "chef_infra/managing_chef_infra_server/backend_failure_recovery.md Backend Failure Recovery"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/backend_failure_recovery.md)

This document contains the recommended actions for responding to
failures in your Chef Backend cluster.

{{< note >}}

If you have concerns about applying the Chef Backend recovery process to
your cluster, please consult with Support before taking the steps
outlined in this guide.

{{< /note >}}

## Assumptions

All instructions currently assume a 3-node backend cluster running Chef
Backend 2.1.0 or greater. The user should have SSH access with root
privileges to all nodes in the cluster.

## Node Failures

This section covers how to respond to failures that have brought the
entire node down or off the network.

### Single-node Failure

Temporary single-node failures require little administrator intervention
to resolve. Once the administrator has addressed the failure and
restarted the node, it will reconnect to the cluster and sync from the
current leader.

However if the failing node cannot be brought back online, it must be
replaced:

1. Run `chef-backend-ctl remove-node NODE_NAME` from any working cluster member to remove the offending node (it doesn't have to be the leader).
1. Run `chef-backend-ctl cleanse` on the offending node.  This will save configuration files under the root directory by default.
1. Check to make sure `/var/opt/chef-backend` was deleted by `chef-backend-ctl cleanse`.
1. Make a directory for the configuration files: `mkdir /etc/chef-backend`.
1. Copy `/root/chef-backend-cleanse*` to `/etc/chef-backend/`.
1. Run `chef-backend-ctl join-cluster LEADER_IP --recovery`

See the [installation
instructions](/install_server_ha/#step-3-install-and-configure-remaining-backend-nodes)
for more details on joining nodes to the cluster.

### Two-node Failure

In the case of a two-node failure in a standard three-node
configuration, the cluster is no longer able to operate, as leader
election requires a quorum of two nodes.

This procedure assumes that the remaining node has the most up-to-date
copy of the data. If that is not the case it is advised that you restore
the existing node from a backup before proceeding. See the [Backup and
Restore](/server_backup_restore/#backup-and-restore-a-chef-backend-install)
documentation for details.

1.  On the surviving node, run the following command:

    ``` none
    chef-backend-ctl create-cluster --quorum-loss-recovery
    ```

2.  On each of the two new nodes, install `chef-backend-ctl` and join to
    the cluster using:

    ``` none
    chef-backend-ctl join-cluster IP_OF_LEADER -s PATH_TO_SHARED_SECRETS
    ```

    See the [installation
    instructions](/install_server_ha/#step-3-install-and-configure-remaining-backend-nodes)
    for more details on joining nodes to the cluster.

## Partitions

For the purpose of this section, a **partition** refers to the loss of
network connectivity between two nodes. From the perspective of other
nodes in the cluster, it is impossible to tell whether a node is down or
has been partitioned. However, because a partition is often
characterized by the node and the software on the node still being up,
this section covers some special care to take when recovering a cluster
that has been partitioned at the network level.

### No Loss of Quorum

If the network partition did not result in a loss of quorum, then the
failed nodes in the cluster should recover on their own once
connectivity is restored.

### Loss of Quorum

This section covers two potential remediation options for instances
where a lack of network connectivity has resulted in loss of quorum
between nodes.

#### Promoting a Specific Node

This procedure only works currently if the administrator can take action
before the network split resolves itself.

1.  Resolve the network partition. As the nodes come back online, they
    will all move into a `waiting_for_leader` state.
2.  To promote a node, run `chef-backend-ctl promote NODE_NAME_OR_IP`

#### Promoting a Previous Leader

If a recently deposed leader is likely the node with the most up-to-date
data, you may want to reinstate its leadership.

1.  To ensure that the deposed leader can come up correctly, you will
    need to override the safety check that prevents deposed leaders from
    starting PostgreSQL. On the deposed leader node that is being
    promoted, run the following command:

    ``` bash
    rm /var/opt/chef-backend/leaderl/data/no-start-pgsql
    ```

2.  Then restart PostgresSQL:

    ``` none
    chef-backend-ctl restart postgresql
    ```

3.  Finally, promote the deposed leader node:

    ``` none
    chef-backend-ctl promote NODE_NAME_OR_IP
    ```

## Service Level Failures

This section documents the expected behavior that occurs when a single
service fails. This currently extends to the service's process on the
machine dying, not more complicated service failures where the process
is up and taking requests but returning incorrect results.

Note that the number of service-level failures that a service can
sustain depends upon whether or not the failure is happening in
isolation. In general an operator should assume that the cluster can
sustain a failure on a single node, but a second failure is likely to
cause a loss of availability if the first failure is not resolved.

### PostgreSQL

The leader/follower state of PostgresSQL is managed by Leaderl. Leaderl
performs health checks on PostgreSQL and fails over to a follower if the
health check fails.

Assuming that `etcd` and `leaderl` are running properly, two of the
three nodes can have service-level PostgreSQL failures. Once the
service-level problems have been resolved, the two failed nodes can be
resynced from the leader node.

### Elasticsearch

-   Elasticsearch manages its own availability. 1 of the 3 nodes can
    have a service-level Elasticsearch failure without affecting the
    availability of the cluster.
-   Elasticsearch failovers are independent of PostgreSQL failovers;
    however, since the Chef Infra Server can only talk to a single
    Elasticsearch instance, if Elasticsearch fails on the leader node,
    Leaderl will failover (including a PostgreSQL failover) to another
    node.
-   Once the root cause of the service-level problems has been
    identified and solved, the failed node should be able to rejoin the
    cluster.

### Etcd

Etcd is used by Leaderl to elect a PostgreSQL leader and store status
and cluster state information. Its availability is required for Leaderl
to continue functioning properly. 1 of the 3 nodes can have
service-level etcd failures and the cluster should remain available. If
the Etcd failure is on the current leader, a PostgreSQL failover will
occur.

### Leaderl

Leaderl is responsible for ensuring that leadership is assigned to a
node that can resolve all requests. If Leaderl fails on the leader node,
it will be unable to respond to failures in the PostgreSQL service. The
other nodes in the cluster will detect Leaderl's failure and attempt to
take over as leader. However, since Leaderl on the failing node is down,
PostgreSQL may still be up and accepting connections.

## Other Failures

### Handling nodes reporting `partially_synced: true`

When a node starts to sync from a leader, Leaderl will write the
following file to disk:

``` none
/var/opt/chef-backend/leaderl/data/unsynced
```

When the sync completes successfully the file is removed. If the sync
fails, the file will remain in place, the node will be placed in a
`waiting_for_leader` state, and will report as leader ineligible. The
cluster will report an unhealthy status until the issue is resolved.

Resolving the issue requires an understanding of what caused the sync
failure. One way to determine the cause is by manually running a sync
and inspecting the output:

``` none
chef-backend-ctl stop leaderl
PSQL_INTERNAL_OK-true chef-backend-ctl pgsql-follow LEADER_IP --verbose
```

Once you've resolved the issue and can run the `pgsql-follow` command
successfully, you can manually remove the sentinel file and restart
Leaderl:

``` none
rm /var/opt/chef-backend/leaderl/data/unsynced
chef-backend-ctl start leaderl
```

## General Follower Recovery Process

Initial attempts to recover should follow this general pattern and use
the scenarios and tools shown above to assist in the recovery steps:

1.  With the cluster down, take a filesystem level backup of all backend
    nodes.

2.  Check the health of the leader and repair if necessary. If the <span
    class="title-ref">/var/log/chef-backend/leaderl/current</span> logs
    do not show leadership changes and the
    `chef-backend-ctl cluster-status` shows a solid etcd/pgsql leader,
    then you are good to continue.

    **Note**: Any leadership repair process is a very dangerous step
    that can result in data loss. We recommend [opening a ticket with
    Support](https://www.chef.io/support/) to go over any cluster
    leadership issue.

    Any leadership repair process often involves removing an internal
    pgsql lock file that prevents promotion of what is thought as the
    last leader. This file is placed automatically by leaderl when it
    demotes a leader
    `/var/opt/chef-backend/leaderl/data/no-start-pgsql`. Refer to the
    [Promoting a Previous Leader
    section](#promoting-a-previous-leader)
    for more details.

3.  If necessary, promote what is thought as the most recent leader.
    Refer to the [Promoting a Previous Leader
    section](/backend_failure_recovery/#promoting-a-previous-leader)
    for more detail.

4.  Sync the followers from the leader using a full basebackup because
    the WAL entries have likely already rotated. When the WAL entries
    have already roated away, the followers will complain in the
    `/var/log/chef-backend/postgresql/X.Y/current` logfile about being
    unable to sync. Using just the <span
    class="title-ref">--recovery</span> flag will result in timeouts of
    the `chef-backend-ctl join-cluster` command. It's impossible for a
    follower to sync and rejoin while in this state because it doesn't
    have current enough info. Below is an example error message
    highlighting followers being unable to rejoin:

<!-- -->

``` none
2018-04-25_16:36:29.42242 FATAL:  the database system is starting up
2018-04-25_16:36:30.90058 LOG:  started streaming WAL from primary at 16F3/2D000000 on timeline 88
2018-04-25_16:36:30.90124 FATAL:  could not receive data from WAL stream: ERROR:  requested WAL segment      00000058000016F30000002D has already been removed
2018-04-25_16:36:30.90125
```

In a Chef Backend High Availability deployment, the etcd service is
extremely sensitive and can get into a bad state across backend nodes
due to disk and/or network latency. When this happens, it is common for
the cluster to be unable to automatically failover/recover.

To attempt manual recovery on a follower that exhibits the symptoms
previously shown, try issuing the following commands on problematic
followers that will not sync. **Do this on one follower at a time.** You
can check output from the `chef-backend cluster-status` command
periodically to watch the state of the cluster change:

``` bash
chef-backend-ctl stop leaderl
chef-backend-ctl cluster-status
PSQL_INTERNAL_OK=true chef-backend-ctl pgsql-follow --force-basebackup --verbose LAST_LEADER_IP
chef-backend-ctl start
```
