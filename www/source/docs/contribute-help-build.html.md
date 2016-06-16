---
title: Help build Habitat
---

# Help Build Habitat

We have been hard at work on Habitat since fall 2015 and are now excited to share it with you, as an open-source project [licensed](/legal/licensing) under the Apache 2.0 License. You can find the source on the [Habitat GitHub repository](https://github.com/habitat-sh/habitat).

While we believe the foundational elements of Habitat are in place, we lack the wide variety of scenarios that come from building, deploying and managing applications in the real world. This is where we hope you'll come in and contribute code upon the strong foundation we've laid.

The remainder of this document describes some of the areas of the product that can be extended.

## More Topologies

Habitat ships on day one with [three topologies](/docs/run-packages-topologies): standalone, leader-follower, and initializer. There are undoubtedly more that are needed in order to represent the semantics of real systems.

If you have a topology to contribute, we would love to hear from you with a pull request. Topologies live inside the [supervisor code base](https://github.com/habitat-sh/habitat/tree/master/components/sup/src/topology).

## More Update Strategies

Habitat includes a single update strategy, [automatic](https://github.com/habitat-sh/habitat/blob/master/components/sup/src/package/updater.rs), which automatically restarts the underlying process if it detects a newer version of the package in a [depot](/docs/concepts-depot). This update strategy does no peer coordination with other members in a service group.

There are many more update strategies used in continuous deployment that we are aware of. Some examples:

* canary: restart one peer in a service group, then restart the others in any order
* by percentage: restart only some subset of a service group
* maintenance window: don't restart anything until a maintenance window is reached

We would love contributions to the code base to add these and any other update strategies that represent real-life deployment patterns.

## Support for Microsoft Windows

We believe that the application-centric approach that Habitat takes can and should be extended to Microsoft Windows. If you are interested in getting either the Habitat build system or supervisor system working on Windows, please get in touch, either on the [forum](https://forums.habitat.sh/) or in [Slack](http://slack.habitat.sh/) in the #windows channel.

## Support for Container Cluster Managers

While Habitat has initial support for Mesos and Kubernetes, we're working on supporting more frameworks that manage containers in production. Much of this work is documentation. Habitat packages exported into Docker and ACI-formatted containers should work with many runtimes and frameworks. We need to document how applications are managed within those systems. If you would like to help add Habitat support please reach out to us.

## Scalability and Load Testing

The Habitat supervisor and its [communication protocols](/docs/internals-supervisor) have been designed to be very scalable. Our belief is that supervisor rings of thousands or even tens of thousands of nodes are possible, but we have not yet engaged in formal verification. If you are a distributed systems expert interested in testing scalability, we'd for you to try it and to share your results with us.
