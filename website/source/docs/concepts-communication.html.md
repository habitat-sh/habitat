---
title: Service communication
---

# Service Communication


## Service Group

A set of running services with a shared configuration and topology. If a service is started without a group, it's assigned to the `default` group. For example:

- `redis.default`
- `postgres.financialdb` (possibly running in a cluster)
- `postgres.userdb` (possibly running in a cluster)

## Topology

A service topology is a state machine that wraps the lifecycle events of a service around the process supervisor and package manager. It is responsible for:

- Processing the main event loop
- Registering callbacks with the discovery system

### Standalone

This is the default topology, useful for applications that don't share state and/or aren't dependent upon start order.

### Initializer

This is the building block of complicated topologies which require a leader. It is used when a single member of your cluster should perform additional applications level initialization and/or if the other members of your cluster need to perform additional initialization steps. We guarantee that the leader will perform it's initialization sequence before the followers attempt to run thier initialization sequences.

### Leader

This topology allows a distributed application running on at least two Habitat nodes to use a leader/follower configuration. Leaders are elected with Habitat's out-of-the-box leader election algorithm, and followers are restarted to reflect a configuration that follows the new leader. Subsequent elections due to leader failure will update both leader and follower configuration data, as well as restart followers.

## Leader election

Given that Habitat is a weakly consistent system, we have opted not to use Paxos or Raft for consensus. Instead, Habitat uses [Bully](https://en.wikipedia.org/wiki/Bully_algorithm) under the hood for simple and efficient leader election.

## Census

The census is the core of our service discovery mechanism. It keeps track of every supervisor in our group, and handles reading, writing, and serializing it with the discovery backend.

Think of each supervisor in the system as a *census entry*; taken together, they form a *census*. Operations to discover or mutate the state of the census happen through algorithms that arrive at the same conclusion given the same inputs.

An example is leader election; it's handled here by having a consistent (and simple) algorithm for selecting a leader deterministically for the group. We rely on the eventual consistency of every supervisors census entry to elect a new leader in a reasonable amount of time.

## Sidecar

The Habitat sidecar is an http service that exposes cluster metadata, statistics, and general diagnostic information useful for monitoring and support in the form of a JSON document. The sidecar also keeps the latest result from any `health_check` hooks that are run as part of a service group.
