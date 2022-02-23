+++
title = "Running Chef Habitat Supervisors"
description = "Running Chef Habitat Packages"
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Running Supervisors"
    identifier = "habitat/supervisors/sup-run Run Packages on the Supervisor"
    parent = "habitat/supervisors"
    weight = 20

+++

Use Chef Habitat packages to start services under the Chef Habitat Supervisor. At runtime, you can join services together in a service group running the same topology, send configuration updates to that group, and more. You can also export the Supervisor together with the package to an external immutable format, such as a Docker container or a virtual machine.

> Note Linux-based packages can run on Linux distributions running kernel 2.6.32 or later. Windows-based packages can run on Windows Server 2012 or later and Windows 8 64-bit or later.

Information about [installing Chef Habitat]({{< relref "install_habitat" >}}) and configuring your workstation can be found in the previous section.

## Starting the Supervisor

In order to run a Chef Habitat-packaged service, you must first run a Chef Habitat Supervisor. There are two ways to start up a Supervisor, and it is important to know the implications of each, and which method is appropriate for your circumstances. These instructions describe the behavior of the 0.56.0 Supervisor and later, which dramatically simplified how Supervisors start up. These instructions also deal with the Supervisor "by itself"; later on, we'll see how to integrate it into different operational scenarios (e.g., systemd, Windows Services, etc.). It is useful to understand the underlying concepts first.

For further details about these commands, including all the arguments and options they take, please consult [the hab documentation]({{< relref "habitat_cli" >}}).

hab sup run
: Executing `hab sup run` will start a Supervisor process in the foreground. If this is the first time a Supervisor has been run on the system, nothing else will happen; the process will be waiting for the user to "load" services into it. However, if this is _not_ the first time the Supervisor has been run, any previously loaded services that were not "stopped" (i.e., told not to run) will be started up as well.

When executing `hab sup run`, additional options can be passed that allow the Supervisor to communicate with other Supervisors (such as `--peer`, `--permanent-peer`, etc.), forming a connected network of Supervisors; this is the communication backbone that any services running on the Supervisors use to communicate with each other.

hab sup run \<PACKAGE_IDENTIFIER\>
: When you pass a package identifier (e.g., `core/redis`) as an argument to `hab sup run`, it will start up a Supervisor process, and then load and start the given package in what is effectively a single operation. This is a convenience that is intended primarily for container entry-point workflows, where a single defined service is the only thing ever intended to run on the Supervisor, but it can also be used for local testing or experimentation.

This version of `hab sup run` can also accept options that affect how the _package_ should run, such as `--strategy` and `--topology`, in addition to the aforementioned Supervisor-specific options. It can be thought of as running the identifier-less `hab sup run`, along with a `hab sup load`, all as a single command.

### Selecting a startup method

In most cases, you should always start up a Supervisor explicitly, using `hab sup run` _without_ a package identifier argument, _especially_ in production environments.

The `<PACKAGE_IDENTIFIER>` invocation mixes Supervisor-specific and service-specific options, it can sometimes be difficult to reason about, depending on how complex your usecase is. As a result, that form should be limited to constrained container-based usecases, and perhaps local simple testing or evaluation scenarios. Additionally, since a Supervisor has state -- the services it is manages -- you may end up starting additional services, depending on what you've done in the past on the same machine. That is, `hab sup run <PACKAGE_IDENTIFIER>` is not, in general, synonymous with "only run this single service".

For all other uses, it is far preferable to start a Supervisor using the identifier-less `hab sup run` invocation, and manage the population of services it runs using `hab svc load` and other commands.

Throughout this documentation, unless otherwise explicitly stated, assume that a Supervisor has been started with `hab sup run`, and that any services were loaded using a separate `hab svc load` invocation.

## Testing a package locally

Packages can be tested in the interactive studio environment or natively on a workstation running Linux or Windows.

When entering an interactive studio, a Supervisor is started for you in the background by default. To run packages inside of this Supervisor:

1. [Build a package]({{< relref "pkg_build" >}}) inside an interactive studio. Do not exit the studio after it is built.
2. To start your service in the running Supervisor, type `hab svc load yourorigin/yourname`, substituting the name and origin of the package you built in Step 1. Your service should now be running.

Because the Supervisor is running in the background, you will not see the Supervisor output as you start your service. However you can use the `sup-log` (or `Get-SupervisorLog` on Windows) command that will stream the tail of the Supervisor output (you can also look at the contents of `/hab/sup/default/sup.log`, which is where the Studio directs its Supervisor output).

If your host machine is running Linux, do the following to run your packages for one-off evaluations (not production uses!):

* Add the `hab` user and group.

    ```bash
    $ sudo adduser --group hab
    $ sudo useradd -g hab hab
    ```

* Run the `hab` Supervisor as root.

    ```bash
    $ sudo hab sup run yourorigin/yourname
    ```

You may use the same `hab run` command on Windows but omit the `sudo` command. However, you should be inside of an elevated shell. Also, note that the `hab` user is not necessary on Windows. If it is absent, services will run under the identity of the current user. If a `hab` user is present, you will need to provide its password via the`--password` argument:

```bash
PS C:\> $cred = Get-Credential hab
PS C:\> hab sup run yourorigin/yourname --password $cred.GetNetworkCredential().Password
```

In all cases, you may wish to run `hab svc unload <yourorigin>/<yourname>` when you are done working with your package, to remove it from the Supervisor. Otherwise, your Supervisor will try to start your service each time it start up.

For more structured ways of running the Chef Habitat Supervisor on servers, please see [Running Chef Habitat on Servers]({{< relref "running_habitat_servers" >}}).

## Loading a Service

To load a service into a Supervisor, you use the `hab svc load` subcommand. As an example, to load `yourorigin/yourname` in a Leader topology, with a Rolling update strategy, and a Group of "acme", run the following:

```bash
$ hab svc load yourorigin/yourname --topology leader --strategy rolling --group acme
```

Running the `hab svc load` subcommand multiple times with different package identifiers will result in multiple services running on the same Supervisor. Let's add `core/redis` to the Supervisor for some fun:

```bash
$ hab svc load core/redis
```

## Unloading a Service

To remove a service from a Supervisor, you use the `hab svc unload` subcommand. If the service is was running, then it will be stopped first, then removed. This means that the next time the Supervisor is started (or restarted), it will not run this unloaded service. For example, to remove the `yourorigin/yourname` service:

```bash
$ hab svc unload yourorigin/yourname
```

## Stopping a Running Service

Sometimes you need to stop a running service for a period of time, for example during a maintenance outage. Rather than completely removing a service from supervision, you can use the `hab svc stop` subcommand which will shut down the running service and leave it in this state until you start it again with the `hab svc start` subcommand, explained next. This means that all service-related options such as service topology, update strategy, etc. are preserved until the service is started again. For example, to stop the running `core/redis` service:

```bash
$ hab svc stop core/redis
```

## Restarting a Stopped Service

To resume running a service which has been loaded but stopped (via the `hab svc stop` subcommand explained above), you use the `hab svc start` subcommand. Let's resume our `core/redis` service with:

```bash
$ hab svc start core/redis
```

> Note: in Chef Habitat versions prior to 0.56.0, `hab svc start` could also be used to load up a service if it wasn't already loaded. In 0.56.0 and later, however, this has changed; `hab svc start` can only operate on services that have previously been loaded.

## Querying the Supervisor for Service Status

You can query all services currently loaded or running under the local Supervisor using the `hab svc status` command. This command will list all services loaded by the Supervisor along with their current state. The `status` command includes the version and release of the service and for services that are running, it will include the `PID` of the running service.

To retrieve status for an individual service, you can pass the service identifier:

```bash
$ hab svc status core/mysql
```

The following exit codes are emitted by the `status` command:

* `0` - The status command successfully reports status on loaded services
* `1` - A generic error has occurred calling the `hab` cli
* `2` - A service identifier was passed to `hab svc status` and that service is not loaded by the Supervisor
* `3` - There is no local running Supervisor

