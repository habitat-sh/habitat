+++
title = "Service Restarts"
description = "When does the supervisor restart a service"
gh_repo = "habitat"

[menu]
  [menu.habitat]
    title = "Service Restarts"
    identifier = "habitat/reference/service-restarts Service Restarts"
    parent = "habitat/reference"
+++

Since the 1.6.477 Supervisor release, it is possible to configure a service restart backoff period to control how quickly a service is restarted following the
failure of an init or run hook. Before this, if there was a failure of the init / run hook the supervisor would attempt to restart the service immediately,
potentially leading to excessive load on external APIs similar to a Denial of Service attack.

## Overview

The new restart backoff behavior can be enabled by explicitly passing the `service-min-backoff-period`, `service-max-backoff-period` and 
`service-restart-cooldown-period` parameters to the `sup run` command. These can also be configured in the [supervisor configuration file]({{< relref "sup_config" >}}) as `service_min_backoff_period`, `service_max_backoff_period` and `service_restart_cooldown_period`.

```bash
$ hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 60  core/redis
```

The algorithm we use to determine the backoff period is called *Decorrelated Jitter*. It is based on research done by the folks at AWS. You can find a more indepth
comparison of various backoff algorithms and their efficiency on their [blog](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/).

{{< note >}}
There is no way to change the backoff algorithm, however if you wish to have a simple fixed backoff, you can simply set the `service-min-backoff-period` and 
`service-max-backoff-period` to the same fixed duration.
{{< /note >}}

## Service Failure Detection

The introduction of the restart backoff behavior requires the capability to detect when a service has successfully started in order to reset the backoff period.
Unfortunately there is no clean way to differentiate between a service failure and a service simply taking too long to startup. A health-check hook
would have enabled detection of successful service startup, however if a health-check is absent there is no way to know if the service started up. There may
also be cases where the initial health-check succeeds but the service goes down shortly afterwards. 

We attempt to solve this problem through the use of a restart cooldown period. The cooldown period is the continuous duration of time without a restart after which 
we assume a service has been started up successfully. It is important to configure this correctly to ensure the backoff period does not get reset prematurely.
Our recommendation is to configure the `service-restart-cooldown-period` to be at least double your expected startup time to be safe. In general a longer cooldown
will not have an adverse effect, however a shorter one may prevent the backoff behavior completely.

You can take a look at the following examples to more properly understand this

#### An slow service with incorrect configuration

```bash
# Lets say the service takes 30 seconds after starting up to crash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 10  my-org/my-slow-service 
 
# T = 0, service starts up 
# T = 30, service crashes, will be restarted after 5 secs 
# T = 35, service is restarted
# T = 45, service backoff period is reset to 5 secs because 10 secs has elapsed since the last restart
# T = 65, service crashes again, it will now be restarted after 5 secs due to the backoff period resetting at T=45
# T = 70, service is restarted again
```

#### A slow service with correct configuration

```bash
# Lets say the service takes 30 seconds after starting up to crash
hab sup run --service-min-backoff-period 5 --service-max-backoff-period 20 --service-restart-cooldown-period 60  my-org/my-slow-service 
 
# T = 0, service starts up 
# T = 30, service crashes, will be restarted after 5 secs 
# T = 35, service is restarted
# T = 65, service crashes again, it will now be restarted after a random duration between 5 and 20 secs, let's assume 15.
# T = 80, service is restarted again, notice that the backoff period has not been reset.
# T = 140, service backoff period is reset to 5 secs because 60 secs has elapsed since the last restart
```

