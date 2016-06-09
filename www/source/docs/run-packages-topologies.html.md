---
title: Running packages in topologies
---

# Topologies

A topology describes the intended relationship between peers within a service group. Three topologies ship with Habitat by default: standalone, leader-follower and initializer. Both of these topologies employ [leader election](/docs/internals-leader-election) to define a leader.

## Standalone

The standalone topology is what a supervisor starts with by default if no topology is specified, or if the topology is explicitly specified with `--topology standalone` when starting the supervisor. The standalone topology means that the service group members do not have any defined relationship with one another, other than sharing the same configuration.

## Leader-Follower Topology

In a leader-follower topology, one of the members of the service group is elected the leader, and the other members of that service group become the followers of that leader. This topology is common for database systems like MySQL or PostgreSQL, where applications send writes to one member, and those writes are replicated to one or more read replicas.

As with any topology using leader election, you must start at least three peers using the `--topology leader` flag to the supervisor.

       hab start yourname/yourdb --topology leader --group yourdb.production

The first supervisor will block until it has quorum. You would start additional members by pointing them at the ring, using the `--peer` argument:

       hab start yourname/yourdb --topology leader --group yourdb.production --peer 192.168.5.4

Note that the `--peer` need not be a peer that is in the service group `yourdb.production`; it merely needs to be one in the ring that the other member(s) are in.

Once you have quorum, one member is elected a leader, the supervisors in the service group update the service's configuration in concordance with the policy defned at package build time, and the service group starts up.

### Defining Leader and Follower Behavior in Plans

Because Habitat provides for automation that is built into the application package, this includes letting the application developer define the application's behavior when run under different topologies, even from the same immutable package. Here is an example of a configuration template marked up with conditional logic that will cause the running application to behave differently based on whether it is a leader or a follower:

       {{#if svc.me.follower}}
       {{#with svc.leader}}
       slaveof {{ip}} {{port}}
       {{/with}}
       {{/if}}

This logic says that if this peer is a follower, it will become a read replica of the IP and port of service leader (`svc.leader`), which is has found by service discovery through the ring. However, if this peer is the leader, the entire list of statements here evaluate to empty text -- meaning that the peer starts up as the leader.

## Initializer Topology

The initializer topology is very similar to leader-follower, except that the elected leader will block the startup of the peers until it has come up fully. This topology is suitable for systems where, on first bootup, a long-running initialization process must occur before any other operations can proceed.

The initializer topology can be started with the `--topology initializer` argument to the supervisor.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-director">Run multiple packages</a></li>
</ul>
