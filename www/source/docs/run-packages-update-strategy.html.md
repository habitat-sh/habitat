---
title: Update Strategy
---

# Update Strategy

The Habitat supervisor can be configured to leverage an optional _update strategy_, which describes how the supervisor and its peers within a service group should respond when a new version of a package is available.

To use an update strategy, the supervisor is configured to watch a [depot](/docs/concepts-depot) for new versions.

## Configuring an Update Strategy

Habitat supports two update strategies: `rolling` and `at-once`.

To start a supervisor with the auto-update strategy, pass the `--strategy` argument to a supervisor start command, and optionally specify the depot URL:

       hab start yourorigin/yourapp --strategy rolling --url https://willem.habitat.sh/v1/depot

### Rolling Strategy

This strategy ensures that all supervisors running a service within a service group are running the same version of software. An update leader is elected which all supervisors within a service group will update around. All update followers will first ensure they are running the same version of a service that their leader is running, and then, the leader will poll a depot for a newer version of the service's package.

Once the update leader finds a new version it will update and wait until all other alive members in the service group have also been updated before once again attempting to find a newer version of software to update to. Updates will happen more or less one at a time until completion with the exception of a new node being introduced into the service group during the middle of an update.

If your service group is also running with the `--topology leader` flag, the leader of that election will never become the update leader, so all followers within a leader topology will update first.

It's important to note that because we must perform a leader election to determine an update leader, *you need to have at least 3 supervisors running a service group to take advantage of the rolling update strategy*.

### At-Once Strategy

This strategy does no peer coordination with other supervisors in the service group; it merely updates the underlying Habitat package whenever it detects that a new version has been published to a depot. No coordination between supervisors is done, each supervisor will poll a remote depot on their own.

## Configuring an Update Strategy with a Depot Channel

A _channel_ in a depot is a point-in-time snapshot of the state of the depot. In point of fact, it is a [materialized view](https://en.wikipedia.org/wiki/Materialized_view) of the depot, starting with the specific `origin/package/version/release` quad, and encapsulating all of the transitive dependencies of that quad. This is very useful for continuous deployment purposes:

* By convention, you name the channel in the depot after the name of your service group (e.g. `myapp.production`).
* You deliver new versions of `myapp` as Habitat packages to the depot.
* When you are ready to roll out a new version of the application, you update the channel corresponding to the intended environment.
* The supervisors in that service group, configured with an appropriate update strategy, update their underlying Habitat package, optionally coordinating with one another, and restart the service.

Configuring the supervisors'  update strategy URL to point to a channel ensures that new versions of the application do not get deployed until the channel is updated, thereby preventing unstable versions from reaching environments for which they are not intended.

To start a supervisor with a strategy and pointing to a channel, modify slightly the URL to the depot:

       hab start yourorigin/yourapp --strategy rolling --url https://yourdepot.example.com/v1/depot/channels/yourchannel

`yourchannel` represents the channel you have created in the depot.

_At the moment, the `hab` command-line tool lacks the ability to create and manage channels. To use channels, you must run your own depot server and use the internal depot maintenance tool to manage channels_.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-multiple-services">Multiple services</a></li>
</ul>
