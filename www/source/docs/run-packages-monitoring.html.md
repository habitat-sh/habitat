---
title: Monitor Habitat services
description: Monitor services through the HTTP API
---

# Monitor services through the HTTP API
When a service starts, the supervisor exposes the status of leader elections, gossip rumors, health, and other information through an HTTP API endpoint. This information can be useful in monitoring service health, results of leader elections, and so on.

The HTTP API provides information on the following endpoints:

* `/census` - Returns information about the census.
* `/config` - Returns the current running configuration.
* `/election` - Returns the status of either an ongoing or finished election when a supervisor runs in a topology where leader election occurs, such as leader-follower or initializer.
* `/gossip` - Returns information about the gossip ring.
* `/health` - Runs the package's [health_check](/docs/reference/plan-syntax#hooks), if one is defined. Returns the status, and outputs both the status and config.
* `/status` - Returns the current status from the supervisor's perspective.

## Usage
Connect to the supervisor of the running service using the following syntax. This example uses `curl` to do the GET request.

      curl http://172.17.0.2:9631/status

> Note: The default listening port on the supervisor is 9631; however, that can be changed by using the `--listen-http` option when starting a service.

If you were retrieving the `status` on a running `core/redis` package, the response back would look similar to the following:

      core/redis/3.0.7/20160529151526: up for PT361.542547264S

Depending on the endpoint you hit, the data may be formatted in JSON, TOML, or plain text.

For a complete list of values on each endpoint and the output formats, see the [HTTP API Reference](/docs/reference/http-api).
