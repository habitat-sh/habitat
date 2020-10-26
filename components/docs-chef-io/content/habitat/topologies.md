+++
title = "Service Group Topologies"
description = "Service Group Topologies"

[menu]
  [menu.habitat]
    title = "Service Group Topologies"
    identifier = "habitat/services/topologies"
    parent = "habitat/services"

+++

A topology describes the intended relationship between peers within a service group. Two topologies ship with Chef Habitat by default: standalone, and leader-follower. The leader-follower topology employs [leader election](/internals/#election-internals) to define a leader.

## Standalone

The standalone topology is what a Supervisor starts with by default if no topology is specified, or if the topology is explicitly specified with `--topology standalone` when starting the Supervisor. The standalone topology means that the service group members do not have any defined relationship with one another, other than sharing the same configuration.

## Leader-follower Topology

In a leader-follower topology, one of the members of the service group is elected the leader, and the other members of that service group become the followers of that leader. This topology is common for database systems like MySQL or PostgreSQL, where applications send writes to one member, and those writes are replicated to one or more read replicas.

As with any topology using leader election, you must start at least three peers using the `--topology leader` flag to the Supervisor.

```bash
$ hab sup run --topology leader --group production
$ hab svc load <ORIGIN>/<NAME>
```

The first Supervisor will block until it has quorum. You would start additional members by pointing them at the ring, using the `--peer` argument:

```bash
$ hab sup run --topology leader --group production --peer 192.168.5.4
$ hab svc load <ORIGIN>/<NAME>
```

> Note: The `--peer` service does not need to be a peer that is in the same service group; it merely needs to be in the same ring that the other member(s) are in.

Once you have quorum, one member is elected a leader, the Supervisors in the service group update the service's configuration in concordance with the policy defined at package build time, and the service group starts up.

### Defining Leader and Follower Behavior in Plans

Chef Habitat allows you to use the same immutable package in different deployment scenarios. Here is an example of a configuration template with conditional logic that will cause the running application to behave differently based on whether it is a leader or a follower:

```handlebars
{{#if svc.me.follower}}
   {{#with svc.leader as |leader|}}
     replicaof {{leader.sys.ip}} {{leader.cfg.port}}
   {{/with}}
{{/if}}
```

This logic says that if this peer is a follower, it will become a read replica of the IP and port of service leader (`svc.leader`), which is has found by service discovery through the ring. However, if this peer is the leader, the entire list of statements here evaluate to empty text -- meaning that the peer starts up as the leader.

## Robustness, Network Boundaries and Recovering from Partitions

Within a leader-follower topology it is possible to get into a partitioned state where nodes are unable to achieve quorum. To solve this a permanent peer can be used to heal the netsplit. To set this pass the `--permanent-peer` option, or it's short form `-I`, to the Supervisor.

```bash
$ hab sup run --topology leader --group production --permanent-peer
$ hab svc load <ORIGIN>/<NAME>
```

When this is used we will attempt to ping the permanent peer and achieve quorum, even if they are confirmed dead.

The notion of a permanent peer is an extension to the original [SWIM](http://www.cs.cornell.edu/projects/Quicksilver/public_pdfs/SWIM.pdf) gossip protocol. It can add robustness provided everyone has a permanent member on both sides of the split.
