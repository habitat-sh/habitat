---
title: Services
---

# Services

A service in Habitat is defined as a Habitat package running under a Habitat supervisor. Services are always run within named service groups, even if there is only one.

## Service Group

A set of one or more running services with a shared configuration and topology. If a service is started without explicitly naming the group, it's assigned to the `default` group for the name of that package. For example:

- `redis.default`
- `postgres.financialdb` (possibly running in a cluster)
- `postgres.userdb` (possibly running in a cluster)

## Topology

A service group topology is the inter-relationship of the services running within that service group. 

### Standalone

This is the default topology, useful for services inside a group that are completely independent from one another. Note that this still means they can share the same configuration.

### Initializer

The initializer topology means that one member of the service group must start up fully before the others. An example is where a database leader must start and initialize a database before any other followers can start up and attempt to connect. In this situation, the supervisors running the follower services will all block and not complete start-up until the elected leader's health check returns success.

### Leader

This topology allows a distributed application running on at least three Habitat nodes to use a leader/follower configuration. Leaders are elected with Habitat's out-of-the-box leader election algorithm, and followers are restarted to reflect a configuration that follows the new leader. Subsequent elections due to leader failure will update both leader and follower configuration data, as well as restart followers.

## Leader election

Habitat provides leader election capabilities out of the box for any topology that needs it. Leader election is implemented in the supervisor, using the [Bully](https://en.wikipedia.org/wiki/Bully_algorithm) algorithm. Habitat is a weakly consistent system, so it does not use Paxos or Raft, etc. for consensus.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/concepts-depot">Depot</a></li>
</ul>
