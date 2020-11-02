+++
title = "About Services"
aliases = ["/habitat/using-habitat/"]

date = 2020-10-26T18:37:38-07:00
draft = false

[menu]
  [menu.habitat]
    title = "About Services"
    identifier = "habitat/services/About Services"
    parent = "habitat/services"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/about_services.md)

A service is a Chef Habitat package running under a Chef Habitat Supervisor.

## Service Group

A service group is a set of one or more running services with a shared configuration
and topology. If a service is started without explicitly naming the
group, it is assigned to the default group for the name of that package. For example:

- `redis.default`
- `postgres.financialdb` (possibly running in a cluster)
- `postgres.userdb` (possibly running in a cluster)

## Topology

Chef Habitat allows you to define the topology of your service groups, which bakes
in certain behaviors.

### Standalone

This is the default topology, useful for services inside a group that are completely
independent from one another. Note that this still means they can share the same
configuration.

### Leader / Follower

This topology allows a distributed application running on at least three Chef Habitat
nodes to use a leader/follower configuration. Leaders are elected with Chef Habitat's
leader election algorithm, and followers are restarted to reflect a configuration
that follows the new leader. Subsequent elections due to leader failure will update
both leader and follower configuration data.

You can read more about the internals behind the elections in our [advanced developer
documentation]({{< relref "sup_crypto#leader-election" >}}).