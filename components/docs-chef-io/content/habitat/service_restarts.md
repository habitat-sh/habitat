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
- The restart cooldown period sets the time in seconds to wait before resetting the current backoff duration to the minimum backoff period.

{{< note >}}

The restart cooldown period is important because it ensures that the supervisor handles potential failures during restart correctly.
If the cooldown period is not long enough, a slow service may still be restarting after the cooldown period has passed.
If a service fails during a restart in that scenario, the service will not backoff correctly before the following restart.
We recommend setting the restart cooldown period to be at least double your expected startup time to be safe.

{{< /note >}}

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

**Slow service with an _incorrect_ configuration**

```bash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 10  ORG_NAME/SERVICE_NAME
```

In the event of a failure during startup with the above configuration, the service will continue to restart after 5 seconds because the service crashes again after the short restart cooldown period has passed, potentially leading to excessive load on external APIs:

1. T = 0, service starts up
1. T = 30, service crashes, will be restarted after 5 secs
1. T = 35, service is restarted
1. T = 45, service backoff period is reset to 5 secs because 10 secs has elapsed since the last restart
1. T = 65, service crashes again, it will now be restarted after 5 secs due to the backoff period resetting at T=45
1. T = 70, service is restarted again

**Slow service with a _correct_ configuration**

```bash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 60  ORG_NAME/SERVICE_NAME
```

In the event of a failure during startup with the above configuration, the service will restart at a random time (15 seconds in this example) which would reduce the load on external APIs:

1. T = 0, service starts up
1. T = 30, service crashes, will be restarted after 5 secs
1. T = 35, service is restarted
1. T = 65, service crashes again, it will restart after a random duration between 5 and 20 secs, let's assume 15.
1. T = 80, service is restarted again, notice that the backoff period has not been reset.
1. T = 140, service backoff period is reset to 5 secs because 60 secs has elapsed since the last restart
