+++
title = "Service Restarts"
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Service Restarts"
    identifier = "habitat/reference/service-restarts Service Restarts"
    parent = "habitat/reference"
+++

Starting with Chef Habitat 1.6.491, you can configure a service restart backoff period to control how quickly a service is restarted following the failure of an init or run hook. Before this release, if there was a failure of the init or run hook, the supervisor would attempt to restart the service immediately, potentially leading to excessive load on external APIs similar to a denial-of-service attack.

## Overview

The restart backoff behavior is set by three parameters:

- The minimum backoff period sets the minimum duration in seconds to wait before restarting a service.
- The maximum backoff period sets the maximum duration in seconds to wait before restarting a service.
- The restart cooldown period sets the time in seconds to wait before resetting the current backoff duration to the minimum backoff period. This is important because it ensures that the supervisor will not mistake a slow start with a service failure.

Enable these values using the [`sup run`]({{< relref "habitat_cli#hab-sup" >}}) command by passing in the number of seconds to the following parameters:

- `service-min-backoff-period`
- `service-max-backoff-period`
- `service-restart-cooldown-period`

For example:

```bash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 60  core/redis
```

You can also set this behavior using these parameters in the [supervisor configuration file]({{< relref "sup_config" >}}):

- `service_min_backoff_period`
- `service_max_backoff_period`
- `service_restart_cooldown_period`.

Chef Habitat uses a decorrelated jitter algorithm to determine the backoff period. See [this blog post](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/) for a more in-depth comparison of various backoff algorithms and their efficiency.

{{< note >}}
There is no way to change the backoff algorithm. However, if you wish to have a simple fixed backoff, set the `service-min-backoff-period` and `service-max-backoff-period` to the same time in seconds.
{{< /note >}}

## Service Failure Detection

Adding restart backoff behavior requires the ability to detect when a service has successfully started to reset the backoff period.
Unfortunately, there is no clean way to differentiate between a service failure and a service simply taking too long to startup. A health-check hook would enable the detection of successful service startups; however, if a health check is absent, there is no way to know if the service started up. There may also be cases where the initial health check succeeds, but the service goes down shortly afterward.

We attempt to solve this problem by using a restart cooldown period. The cooldown period is a continuous duration of time without a restart, after which we assume a service has started up successfully. It is important to configure this correctly to ensure the backoff period does not get reset prematurely.
We recommend setting the `service-restart-cooldown-period` to be at least double your expected startup time to be safe. In general, a longer cooldown will not have an adverse effect; however, a shorter one may prevent the backoff behavior completely.

See the examples below to more properly understand this.

## Examples

This example shows a slow service with an **incorrect** configuration.

The service will continue to restart after 5 seconds because the service crashes again after a short restart cooldown period has passed, potentially leading to excessive load on external APIs.

```bash
# Lets say the service takes 30 seconds after starting up to crash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 10  ORG_NAME/SERVICE_NAME

# T = 0, service starts up
# T = 30, service crashes, will be restarted after 5 secs
# T = 35, service is restarted
# T = 45, service backoff period is reset to 5 secs because 10 secs has elapsed since the last restart
# T = 65, service crashes again, it will now be restarted after 5 secs due to the backoff period resetting at T=45
# T = 70, service is restarted again
```

This example shows a slow service with a **correct** configuration.

The service will restarts at a random time, which would reduce the load on external APIs.

```bash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 60  ORG_NAME/SERVICE_NAME

# T = 0, service starts up
# T = 30, service crashes, will be restarted after 5 secs
# T = 35, service is restarted
# T = 65, service crashes again, it will restart after a random duration between 5 and 20 secs, let's assume 15.
# T = 80, service is restarted again, notice that the backoff period has not been reset.
# T = 140, service backoff period is reset to 5 secs because 60 secs has elapsed since the last restart
```
