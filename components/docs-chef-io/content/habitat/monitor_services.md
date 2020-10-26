+++
title = "Monitoring Services"
description = "Monitoring Services"

[menu]
  [menu.habitat]
    title = "Monitoring Services"
    identifier = "habitat/services/monitor-services"
    parent = "habitat/services"

+++

Use the HTTP API to monitor services. When a service starts, the Supervisor exposes the status of its services' health and other information through an HTTP API endpoint. This information can be useful in monitoring service health, results of leader elections, and so on.

## Authentication

The Supervisor currently supports simple HTTP authentication using Bearer tokens. By default, no authentication is used. If you would like to require authentication, export the `HAB_SUP_GATEWAY_AUTH_TOKEN` environment variable before starting the Supervisor. All HTTP requests will then require that same token to be present in an Authorization header, or they will receive a 401 Unauthorized response.

## Endpoints

The HTTP API provides information on the following endpoints:

* `/butterfly` - Debug information about the rumors stored via Butterfly.
* `/census` - Returns the current Census of Services on the Ring (roughly what you see as a service in config.toml).
* `/services` - Returns an array of all the services running under this Supervisor.
* `/services/{name}/{group}` - Returns the information of a single loaded service.
* `/services/{name}/{group}/config` - Returns this service group's current configuration.
* `/services/{name}/{group}/health` - Returns the current health check for this service.
* `/services/{name}/{group}/{organization}` - Returns information of a single loaded service scoped to an organization
* `/services/{name}/{group}/{organization}/config` - Returns the service group's current configuration, but includes the organization.
* `/services/{name}/{group}/{organization}/health` - Same as above, but includes the organization.

### Errors

Most of the HTTP API endpoint return these errors:

| Error | Description |
| +++-- | +++++++++-- |
| 404 | Service not loaded |
| 503 | Supervisor hasn't fully started. Try again later. |

The `/health` endpoints return:

| Error | Description |
| +++-- | +++++++++-- |
| 404 | Service not loaded |
| 500 | Health Check - Unknown |
| 503 | Health Check - Critical |

## Usage

Connect to the Supervisor of the running service using the following syntax. This example uses `curl` to do the GET request.

```bash
$ curl http://172.17.0.2:9631/services
```

> Note: The default listening port on the Supervisor is 9631; however, that can be changed by using the `--listen-http` option when starting a service.

Depending on the endpoint you hit, the data may be formatted in JSON, TOML, or plain text.

### Example

```bash
$ HAB_SUP_GATEWAY_AUTH_TOKEN="sekret" hab sup run
hab-sup(MR): Supervisor Member-ID e89b6616d2c040c8a82f475b00ba8c69
hab-sup(MR): Starting gossip-listener on 0.0.0.0:9638
hab-sup(MR): Starting ctl-gateway on 0.0.0.0:9632
hab-sup(MR): Starting http-gateway on 0.0.0.0:9631
```

```bash
$ curl -v http://172.17.0.2:9631/services
*   Trying 172.17.0.2...
* TCP_NODELAY set
* Connected to 172.17.0.2 (172.17.0.2) port 9631 (#0)
> GET /services HTTP/1.1
> Host: 172.17.0.2:9631
> User-Agent: curl/7.54.0
> Accept: */*
>
< HTTP/1.1 401 Unauthorized
< content-length: 0
< date: Thu, 15 Nov 2018 22:39:41 GMT
<
* Connection #0 to host 172.17.0.2 left intact
```

```bash
$ curl -v -H "Authorization: Bearer sekret" http://172.17.0.2:9631/services
*   Trying 172.17.0.2...
* TCP_NODELAY set
* Connected to 172.17.0.2 (172.17.0.2) port 9631 (#0)
> GET /services HTTP/1.1
> Host: 172.17.0.2:9631
> User-Agent: curl/7.54.0
> Accept: */*
> Authorization: Bearer sekret
>
< HTTP/1.1 200 OK
< content-length: 2
< content-type: application/json
< date: Thu, 15 Nov 2018 22:41:42 GMT
<
* Connection #0 to host 172.17.0.2 left intact
[]
```
