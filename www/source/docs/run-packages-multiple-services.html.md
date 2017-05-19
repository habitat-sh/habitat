---
title: Running Multiple Services with one Supervisor
---

# Run Multiple Services with one Supervisor

The Habitat supervisor is designed to supervise one or more services concurrently so if you are running Habitat on bare metal or a virtual machine there is only need for one supervisor. This is also useful in a container environment if you require a secondary so-called "sidecar" service running alongside your primary service.

## Starting only the Supervisor

Starting the supervisor is as simple as running:

		hab sup run

This command also lets you override the default gossip and http gateway binding ports, just like when using `hab start`.

## Running the Supervisor with a Host's init System

As only one supervisor is required on a host, this means that only one system service needs to be added. Your choice of Linux distribution may dictate which init system is in use (i.e. SysVinit, Systemd, runit, etc), but all options boil down to simply running `hab sup run` as the runnable program. The following example assumes that the Habitat program `hab` is installed, binlinked as `/bin/hab`, and a `hab` user and group are present.

For example, a suitable Systemd unit would simply be:

		[Unit]
		Description=The Habitat Supervisor

		[Service]
		ExecStart=/bin/hab sup run

		[Install]
		WantedBy=default.target

It is important to start the supervisor via the `hab` program as upgrades to the `core/hab` Habitat package will also upgrade the version of the supervisor on next start.

## Loading a Service for Supervision

To add a service to a Supervisor, you use the `hab svc load` subcommand. It has many of the same service-related flags and options as `hab start`, so there's nothing extra to learn here (for more details, read through the [Run packages sections](/docs/run-packages-overview)). For example, to load `yourorigin/yourname` in a Leader topology, a Rolling update strategy and a Group of "acme", run the following:

		hab svc load yourorigin/yourname --topology leader --strategy rolling --group acme

Running the `hab svc load` subcommand multiple times with different package identifiers will result in multiple services running on the same supervisor. Let's add `core/redis` to the supervisor for some fun:

		hab svc load core/redis

## Unloading a Service from Supervision

To unload and consequently remove a service from supervision, you use the `hab svc unload` subcommand. If the service is was running, then it will be stopped first, then removed last. This means that the next time the Supervisor is started (or restarted), it will not run this unloaded service. For example, to remove the `yourorigin/yourname` service:

		hab svc unload yourorigin/yourname

## Stopping a Loaded Running Service

Sometimes you need to stop a running service for a period of time, for example during a maintenance outage. Rather than completely removing a service from supervision, you can use the `hab svc stop` subcommand which will shut down the running service and leave it in this state until you start it again with the `hab svc start` subcommand, explained next. This means that all service-related options such as service topology, update strategy, etc. are preserved until the service is started again. For example, to stop the running `core/redis` service:

		hab svc stop core/redis

## Starting a Loaded Stopped Service

To resume running a service which has been loaded but stopped (via the `hab svc stop` subcommand explained above), you use the `hab svc start` subcommand. Let's resume our `core/redis` service with:

		hab svc start core/redis

## Querying the supervisor for service status
You can query all services currently loaded or running under the local supervisor using the `hab sup status` command. This command will list all persistent services loaded by the supervisor along with their current state. It will also list transient services that are currently running or in a `starting` or `restarting` state. The `status` command includes the version and release of the service and for services that are running, it will include the `PID` of the running service.

To retrieve status for an individual service, you can pass the service identifier:

      hab sup status core/mysql

The following exit codes are emitted by the `status` command:

* `0` - The status command successfully reports status on loaded services
* `1` - A generic error has occured calling the `hab` cli
* `2` - A service identifier was passed to `hab sup status` and that service is not loaded by the supervisor
* `3` - There is no local running supervisor

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-export">Export packages</a></li>
</ul>
