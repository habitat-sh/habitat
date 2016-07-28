---
title: Update Strategy
---

# Update Strategy

The Habitat supervisor can be configured to implement an optional _update strategy_, which describes how the supervisor and its peers within a service group should respond when a new version of a package is available.

To use an update strategy, the supervisor be configured to watch a [depot](/docs/concepts-depot) for new versions. Habitat provides a public depot, but you can also run one inside your own datacenter using the `core/hab-depot` package.

## Configuring an Update Strategy

The current version of Habitat supports only one update strategy: `at-once`. This strategy does no peer coordination with other supervisors in the service group; it merely updates the underlying Habitat package whenever it detects that a new version has been published to a depot.

To start a supervisor with the auto-update strategy, pass the `--strategy` argument to a supervisor start command, and optionally specify the depot URL:

       hab start yourorigin/yourapp --strategy at-once --url https://willem.habitat.sh/v1/depot

## Configuring an Update Strategy with a Depot Channel

A _channel_ in a depot is a point-in-time snapshot of the state of the depot. In point of fact, it is a [materialized channel](https://en.wikipedia.org/wiki/Materialized_channel) of the depot, starting with the specific `origin/package/version/release` quad, and encapsulating all of the transitive dependencies of that quad. This is very useful for continuous deployment purposes:

* By convention, you name the channel in the depot after the name of your service group (e.g. `myapp.production`)
* You deliver new versions of `myapp` as Habitat packages to the depot
* When you are ready to roll out a new version of the application, you update the channel corresponding to the intended environment
* The supervisors in that service group, configured with an appropriate update strategy, update their underlying Habitat package, optionally coordinating with one another, and restart the service.

Configuring the supervisors'  update strategy URL to point to a channel ensures that new versions of the application do not get deployed until the channel is updated, thereby preventing unstable versions from reaching environments for which they are not intended.

To start a supervisor with a strategy and pointing to a channel, modify slightly the URL to the depot:

       hab start yourorigin/yourapp --strategy at-once --url https://yourdepot.example.com/v1/depot/channels/yourchannel

`yourchannel` represents the channel you have created in the depot.

_At the moment, the `hab` command-line tool lacks the ability to create and manage channels. To use channels, you must run your own depot server and use the internal depot maintenance tool to manage channels_.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-director">Run multiple packages</a></li>
</ul>
