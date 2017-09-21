---
title: Supervisor Internals
---

# Supervisor Internals

The Habitat Supervisor is similar in some ways to well-known process supervisors like [systemd](https://www.freedesktop.org/wiki/Software/systemd/), [runit](http://smarden.org/runit/) or [smf](https://en.wikipedia.org/wiki/Service_Management_Facility). For example, it accepts and passes POSIX signals to its child processes; restarts child processes if and when they fail; ensures that children processes terminate cleanly, and so on.

Because the basic functionality of process supervision is well-known, this document does not discuss those details. Instead, this document focuses strictly on the internals of the feature that makes the Habitat Supervisor special: the fact that each Supervisor is connected to others in a peer-to-peer, masterless network which we refer to as a _ring_. This allows Supervisors to share configuration data with one another and adapt to changing conditions in the ring by modifying their own configuration.

## Architecture

Supervisors are configured to form a ring by starting new Supervisors using the `--peer` argument and pointing them at peers that already exist. In a real-life deployment scenario, Supervisors in a ring would also have a shared encryption key, so that inter-supervisor traffic is encrypted. (See the [security](/docs/run-packages-security) documentation for more details.)

Supervisor rings can be very large, comprising potentially thousands of Supervisors. The [inter-supervisor communication protocol](#protocols) is low-bandwidth and designed to not interfere with your application's actual production traffic.

Rings are further divided into _service groups_, each of which has a name. All Supervisors within a service group share the same configuration and topology. It is typical to name each service group using the pattern `service_name`.`environment` and have these service group names correspond to a _channel_ within the depot. In this way, the Supervisors can self-update whenever the channel is updated. For more information on this, please read the [continuous deployment](/docs/continuous-deployment-overview) topic.

## Butterfly

Habitat uses a gossip protocol named "Butterfly". It is a variant of
[SWIM](http://prakhar.me/articles/swim) for membership and failure detection
(over UDP), and a ZeroMQ based variant of
[Newscast](http://www.cs.unibo.it/bison/publications/ap2pc03.pdf) for
gossip. This protocol provides failure detection, service
discovery, and leader election to the Habitat Supervisor.

Butterfly is an eventually consistent system - it says, with a very high degree of probability, that a given piece of information will be received by every member of the network. It makes no guarantees as to when that state will arrive; in practice, the answer is usually "quite quickly". :)

### Vocabulary

* _Members_: Butterfly keeps track of "members"; each habitat Supervisor is a single member.
* _Peer_: All the members a given member is connected to are its "peers". A member is seeded with a list of "initial peers".
* _Health_: The status of a given member, from the perspective of its peers.
* _Rumor_: A piece of data shared with all the members of a ring; examples are election, membership, services, or configuration.
* _Heat_: How many times a given rumor has been shared with a given member.
* _Ring_: All the members connected to one another form a Ring.
* _Incarnation_: A counter used to determine which message is "newer".

### Transport Protocols

Supervisors communicate with each other using UDP and ZeroMQ, over port 9638.

### Information Security

Butterfly encrypts traffic on the wire using Curve25519 and a symmetric key. If a ring is configured to use transport level encryption, only members with a matching key are allowed to communicate.

Service Configuration and Files can both be encrypted with public keys.

### Membership and Failure Detection

Butterfly servers keep track of what members are present in a ring, and are constantly checking each other for failure. Any given member is in one of three health states:

* Alive: this member is responding to health checks.
* Suspect: this member has stopped responding to our health check, and will be marked confirmed if we do not receive proof it is still alive soon.
* Confirmed: this member has been un-responsive long enough that we can cease attempting to check its health.

The essential flow is:

* Randomize the list of all known members who are not Confirmed dead.
* Every 3.1 seconds, pop a member off the list, and send it a "PING" message.
* If we receive an "ACK" message before 1 second elapses, the member remains Alive.
* If we do not receive an "ACK" in 1 second, choose 5 peers (the "PINGREQ targets"), and send them a "PINGREQ(member)" message for the member who failed the PING.
* If any of our PINGREQ targets receive an ACK, they forward it to us, and the member remains Alive.
* If we do not receive an ACK via PINGREQ with 2.1 seconds, we mark the member as Suspect, and set an expiration timer of 9.3 seconds.
* If we do not receive an Alive status for the member within the 9.3 second suspicion expiration window, the member is marked as Confirmed.
* Move on to the next member, until the list is exhausted; start the process again.

When a Supervisor sends the PING, ACK and PINGREQ messages, it includes information about the 5 most recent members. This enables membership to be gossiped through the failure protocol itself.

This process provides several nice attributes:

* It is resilient to partial network partitions.
* Due to the expiration of suspected members, confirmation of death spreads quickly.
* The amount of network traffic generated by a given node is constant, regardless of network size.
* The protocol uses single UDP packets which fit within 512 bytes.

Butterfly differs from SWIM in the following ways:

* Rather than sending messages to update member state, we send the entire member.
* We support encryption on the wire.
* Payloads are protocol buffers.
* We support "persistent" members - these are members who will continue to have the failure detection protocol run against them, even if they are confirmed dead. This enables the system to heal from long-lived total partitions.
* Members who are confirmed dead, but who later receive a membership rumor about themselves being suspected or confirmed, respond by spreading an Alive rumor with a higher incarnation. This allows members who return from a partition to re-join the ring gracefully.

### Gossip

Butterfly uses ZeroMQ to disseminate rumors throughout the network. Its flow:

* Randomize the list of all known members who are not Confirmed dead.
* Every second, take 5 members from the list.
* Send each member every rumor that has a Heat lower than 3; update the heat for each rumor sent.
* When the list is exhausted, start the loop again.

Whats good about this system:

* ZeroMQ provides a scalable PULL socket, that processes incoming messages from multiple peers as a single fair-queue.
* It has no back-chatter - messages are PUSH-ed to members, but require no receipt acknowledgement.
* Messages are sent over TCP, giving them some durability guarantees.
* In common use, the gossip protocol becomes inactive; if there are no rumors to send to a given member, nothing is sent.

## Papers

* Many more details about the operation of SWIM can be found in its [paper](https://www.cs.cornell.edu/~asdas/research/dsn02-swim.pdf).
* For information about the newscast approach to rumor dissemination, please refer to the [paper](http://www.cs.unibo.it/bison/publications/ap2pc03.pdf).

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/internals-leader-election">Leader Election</a></li>
</ul>
