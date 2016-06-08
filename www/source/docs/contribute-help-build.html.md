---
title: Help build Habitat
---

# Help Build Habitat

We have been hard at work on Habitat since fall 2015 and are now excited to share it with you, as an open-source project [licensed](/legal/licensing) under the Apache 2.0 License.

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
* maintenance window: don't restart anything until supervisor a maintenance window is reached

We would love contributions to the code base to add these and any other update strategies that represent real-life deployment patterns.

## Support for Microsoft Windows

We believe that the application-centric approach that Habitat takes can and should be extended to Microsoft Windows. If you are interested in getting either the Habitat build system or supervisor system working on Windows, please get in touch, either on the [forum](https://forum.habitat.sh/) or in [Slack](http://slack.habitat.sh/).

## Scalability and Load Testing

The Habitat supervisor and its [communication protocols](/docs/internals-supervisor) have been designed to be very scalable. Our belief is that supervisor rings of thousands or even tens-of-thousands of nodes are possible, but we have not yet engaged in formal verification. If you are a distributed systems expert interested in testing scalability, we'd for you to try it and to share your results with us.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/contribute-write-plans">Contributing Build Plans</a></li>
</ul>
