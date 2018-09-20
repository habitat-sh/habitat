# {{ pkg_name }}

Brief description of {{ pkg_name }}

## Maintainers

Names of core plans maintainers (The Habitat Maintainers humans@habitat.sh is usually sufficient)

## Type of Package

This should state whether the package is a service package or a binary package.

A service package is something that will be run by the Habitat supervisor (i.e. core/postgresql).  A service package must always have a run file or define pkg_svc_run in the plan.sh file.

A binary package is something that packages up a standalone binary, something that does not need to run under the Habitat supervisor (i.e. core/dep). They are often used as dependencies of other packages. Binary packages do not have a run file and do not need to define pkg_svc_run in the plan.sh file.

## Usage

How would a user use this package?  i.e. can a user simply call the package as a dependency of their application?  Or is there more they need to do?

## Bindings

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

How do other services that want to consume this service bind to it?

Checkout [the core/postgresql](https://github.com/habitat-sh/core-plans/tree/master/postgresql) README for a good example of this.

## Topologies

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

What topologies does this plan support?

### Standalone

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

Check out [the Habitat docs on standalone](https://www.habitat.sh/docs/using-habitat/#standalone) for more details on what the standalone topology is and what it does.

If this plan can be used with the standalone topology, how do you do it?

Checkout [the core/postgresql](https://github.com/habitat-sh/core-plans/tree/master/postgresql) README for a good example of this.

### Leader-Follower

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

If this plan can be used with the leader/follower topology, how do you do it?

Check out [the Habitat docs on Leader-Follower](https://www.habitat.sh/docs/using-habitat/#leader-follower-topology) for more details on what the leader-follower topology is and what it does.

Checkout [the core/postgresql](https://github.com/habitat-sh/core-plans/tree/master/postgresql) README for a good example of this (look under the Clustering heading)

## Update Strategies

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

What update strategies would work best for this plan?

Checkout [the update strategy documentation](https://www.habitat.sh/docs/using-habitat/#update-strategy) for information on the strategies Habitat supports.

### Configuration Updates

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

Checkout the [configuration update](https://www.habitat.sh/docs/using-habitat/#configuration-updates) documentation for more information on what configuration updates are and how they are executed.

Link to the plan's default.toml file to list all the configurable values of the plan.

If your plan has configuration values that require a complete rebuild when updated, note those here.

## Scaling
*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

(Optional, but recommended)

How would a user scale this service?

Can this service be run in a cluster and/or as high availability?

## Monitoring

*(This is only required for service packages, not [binary wrapper packages](https://www.habitat.sh/docs/best-practices/#binary-wrapper-packages))*

(Optional, but recommended)

How would a user monitor the health of this surface at the application layer?

This is separate from information about Habitat's HTTP API monitoring service.  This section should include some suggestions about how someone could monitor the application or service outside of Habitat - i.e. using something like sumologic, logstash, etc.  It does not need to be prescriptive, but should include some suggestions of where someone might start.

## Notes

(Optional)

Anything that does not fit in the above sections should go here - i.e. how does this fit into a user's development workflow?
