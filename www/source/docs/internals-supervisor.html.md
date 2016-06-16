---
title: Supervisor Internals
---

# Supervisor Internals

The Habitat supervisor is similar in some ways to well-known process supervisors like [systemd](https://www.freedesktop.org/wiki/Software/systemd/), [runit](http://smarden.org/runit/) or [smf](https://en.wikipedia.org/wiki/Service_Management_Facility). For example, it accepts and passes POSIX signals to its child processes; restarts child processes if and when they fail; ensures that children processes terminate cleanly, and so on.

Because the basic functionality of process supervision is well-known, this document does not discuss those details. Instead, this document focuses strictly on the internals of the feature that makes the Habitat supervisor special: the fact that each supervisor is connected to others in a peer-to-peer, masterless network which we refer to as a _ring_. This allows supervisors to share configuration data with one another and adapt to changing conditions in the ring by modifying their own configuration.

## Architecture

Supervisors are configured to form a ring by starting new supervisors using the `--peer` argument and pointing them at peers that already exist. In a real-life deployment scenario, supervisors in a ring would also have a shared encryption key, so that inter-supervisor traffic is encrypted. (See the [security](/docs/run-packages-security) documentation for more details.)

Supervisor rings can be very large, comprising potentially thousands of supervisors. The [inter-supervisor communication protocol](#protocols) is low-bandwidth and designed to not interfere with your application's actual production traffic.

Rings are further divided into _service groups_, each of which has a name. All supervisors within a service group share the same configuration and topology. It is typical to name each service group using the pattern `service_name`.`environment` and have these service group names correspond to a _view_ within the depot. In this way, the supervisors can self-update whenever the view is updated. For more information on this, please read the [continuous deployment](/docs/continuous-deployment-overview) topic.

## Protocols

### Transport Protocols

Supervisors communicate with each other using the [Micro Transport Protocol](https://en.wikipedia.org/wiki/Micro_Transport_Protocol), or µTP. µTP is a UDP-based variant of the BitTorrent peer-to-peer file sharing protocol that provides reliable, ordered delivery.

By using a protocol that treats occasional poor network performance and member loss -- either temporary or permanent -- as a fact of life, the Habitat supervisor accounts for real-world operational characteristics and builds reliable communication semantics on top of unreliable systems. What we give up, though, is strong, immediate consistency: the Habitat ring is an eventually consistent system, as we will see shortly.

### Application Protocols

There are two main application protocols that run on top of the transport protocol previously described: a _membership and failure detection_ protocol and a _gossip_ protocol. Habitat implements [SWIM](http://prakhar.me/articles/swim/) for membership & failure detection, and piggybacks [Newscast](http://www.cs.unibo.it/bison/publications/ap2pc03.pdf) on top of it for disseminating information ("rumors").

#### Membership and Failure Detection

When a supervisor joins a ring that already has peers, it announces its arrival by gossiping a rumor about its membership to the peers it was pointed to. This rumor -- in addition to any other rumors -- is spread around the ring during the failure detection phase of the protocol.

In addition, each service group maintains a _census_ of all the members of that particular service group. Each census entry contains more specific information pertinent to the configuration of that service group: what package the supervisor runs, what is the IP address of the supervisor, and so on.

Both the membership and census protocols in action can be seen when a supervisor starts up as it prints the GUIDs of both the membership entry and the census entry:

       hab-sup(MN): Starting yourorg/yourapp
       hab-sup(GS): Supervisor 192.168.0.9: f0cc478e-6347-4372-807d-6a55373a7fc6
       hab-sup(GS): Census yourapp.default: 3087de37-21b6-4e6e-a265-61785228772b

The failure detection protocol in Habitat serves both as a way to maintain a correct membership list in an architecture that does not have a master, as well as a rumor distribution mechanism. It works as follows.

1. Each peer in the ring maintains its own list of other peers, and its own understanding about the state (dead/alive/suspect) of those peers.
2. Every interval _i_ (currently, 200ms) a thread wakes up, selects a random set of peers to ping, and piggybacks a list of rumors on top. (This list of rumors is the full list from all time; thus, one can regard it as a [CmRDT](https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type).)
3. One of two things happens:
   1. The pinged peer responds with an ACK and sends back a list of their rumors. Each peer processes the rumors they haven't seen.
   2. The pinged peer is unreachable. Either it is hard-unreachable (fails immediately), or soft-unreachable (connection hangs until some timeout, at which point it is a hard failure). A failure causes a move into the `ping-req`, or failure confirmation, phase.
4. Another random set of peers are selected on the next interval _i_. The process repeats until all peers in the membership list have been touched, and then the process restarts.

##### Failure Confirmation

During the _ping-req_, or failure confirmation, phase of the failure detector, some or all members will attempt to determine if the peer is truly unreachable. The originating peer picks three random peers other than the unreachable one and sends the list of rumors plus the membership list to them. Those members in turn attempt to communicate with the unreachable peer. If the peer is reachable by any of these three, then the rumor and membership list is transmitted to that peer, and the original sender is notified that all is well.

However, if the peer turns out to not be reachable from any of the randomly selected three peers, the peer that times out first generates a "suspect" rumor to the whole ring, at which the entire ring attempts to send that rumor to the peer marked suspect. If the suspect peer receives this rumor and is capable of responding, it will do so by gossipping an "alive" rumor and incrementing its incarnation version in the membership list. This incarnation trumps any "suspect" rumors.

If the "suspect" rumor also times out, then the peer is marked "confirmed" to indicate that it is truly dead, the confirmation rumor is gossipped around the membership list, and all members remove the confirmed-dead member from their list. They will never communicate with the confirmed-dead member again -- unless that member recovers, and communicates with them.

#### Network Partitions and Permanent Peers

It is possible, in a long-running network partition scenario, for members to completely disappear from the network and never recover. For example, take a single peer out of a ring size _N_ that gets partitioned off: all the other _N-1_ peers in the ring will mark that peer as suspect, and eventually confirm it as dead. The peer itself will also mark all the other _N-1_ members as dead. Even if the partition heals, the peer will never rejoin the ring, since it will believe all the other peers are dead and not communicate with them, and vice-versa.

As a countermeasure, Habitat has the concept of being able to start a supervisor as a permanent peer using the `--permanent-peer` flag. We recommend that you run a permanent peer in each possible failure domain and make it part of the ring. Permanent peers will never be marked as suspect or dead, thus providing a communications avenue of last-resort when recovering from network partitions.

## Papers

* Many more details about the operation of SWIM can be found in its [paper](https://www.cs.cornell.edu/~asdas/research/dsn02-swim.pdf).
* For information about the newscast approach to rumor dissemination, please refer to the [paper](http://www.cs.unibo.it/bison/publications/ap2pc03.pdf).

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/internals-leader-election">Leader Election</a></li>
</ul>
