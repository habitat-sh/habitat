---
title: Monitor Habitat services
description: Monitor services through the HTTP API
---

# Monitoring services using the `hab sup status` command
You can query all services currently loaded or running under the local supervisor using the `hab sup status` command. This command will list all persistent services loaded by the supervisor along with their current state. It will also list transient services that are currently running or in a `starting` or `restarting` state. The `status` command includes the version and release of the servicwe and for services that are running, it will include the `PID` of the running service.

To retrieve status for an individual service, you can pass the service identifier:

```
$ hab sup status core/mysql
```

The following exit codes are emitted by the `status` command:

* `0` - The status command successfully reports status on loaded services
* `1` - A generic error has occured calling the `hab` cli
* `2` - A service identifier was passed to `hab sup status` and that service is not loaded by the supervisor
* `3` - There is no local running supervisor

# Monitor services through the HTTP API
When a service starts, the supervisor exposes the status of its services' health and other information through an HTTP API endpoint. This information can be useful in monitoring service health, results of leader elections, and so on.

The HTTP API provides information on the following endpoints:

* `/census` - Returns the current Census of Services on the Ring (roughly what you see as a service in config.toml).
* `/services` - Returns an array of all the services running under this supervisor.
* `/services/{name}/{group}/config` - Returns this service groups current configuration.
* `/services/{name}/{group}/{organization}/config` - Same as above, but includes the organization.
* `/services/{name}/{group}/health` - Returns the current health check for this service.
* `/services/{name}/{group}/{organization}/health` - Same as above, but includes the organization.
* `/butterfly` - Debug information about the rumors stored via Butterfly.

## Usage
Connect to the supervisor of the running service using the following syntax. This example uses `curl` to do the GET request.

```
$ curl http://172.17.0.2:9631/services
```

> Note: The default listening port on the supervisor is 9631; however, that can be changed by using the `--listen-http` option when starting a service.

Depending on the endpoint you hit, the data may be formatted in JSON, TOML, or plain text.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/share-packages-overview">Share packages</a></li>
</ul>
