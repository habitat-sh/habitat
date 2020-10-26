+++
title = "Service Groups"
description = "Service Groups"

[menu]
  [menu.habitat]
    title = "Service Groups"
    identifier = "habitat/reference/services/service-groups"
    parent = "habitat/services"

+++

A service group is a logical grouping of services with the same package and topology type connected together across a Supervisor network.
They are created to share configuration and file updates among the services within those groups and can be segmented based on workflow or deployment needs (QA, Production, and so on).
Updates can also be [encrypted](/using-habitat/using-encryption) so that only members of a specific service group can decrypt the contents.

By default, every service joins the _service-name_.**default** service group unless otherwise specified at runtime.

In addition, multiple service groups can reside in the same Supervisor network. This allows data exposed by Supervisors to
be shared with other members of the ring, regardless of which group they are in.

## Joining a Service Group

To join services together in a group, they must be running on Supervisors that are participating in the same Supervisor gossip network (i.e., they are ultimately peered together), and they must be using the same group name. To illustrate, we'll show two `core/redis` services joining into the same group.

First, we'll start up two Supervisors on different hosts, peering the second one back to the first.

```bash
$ hab sup run # on 172.18.0.2 (Supervisor A)
hab-sup(MR): Supervisor Member-ID e89b6616d2c040c8a82f475b00ba8c69
hab-sup(MR): Starting gossip-listener on 0.0.0.0:9638
hab-sup(MR): Starting ctl-gateway on 0.0.0.0:9632
hab-sup(MR): Starting http-gateway on 0.0.0.0:9631
```

```bash
$ hab sup run --peer=172.18.0.2:9638 # on 172.18.0.3 (Supervisor B)
hab-sup(MR): Supervisor Member-ID bc8dc23243e44fee8ea7b9023073c28a
hab-sup(MR): Starting gossip-listener on 0.0.0.0:9638
hab-sup(MR): Starting ctl-gateway on 0.0.0.0:9632
hab-sup(MR): Starting http-gateway on 0.0.0.0:9631
```

Now, run the following on each Supervisor to load `core/redis` in the "prod" group:

```
hab svc load core/redis --group=prod
```

Each will start up, and will be joined into the same group; here is Supervisor A's output:
![Supervisor A running Redis](/images/habitat/supervisor_a_before.png)

And here is Supervisor B's output:
![Supervisor B running Redis](/images/habitat/supervisor_b_before.png)

Note that they are both listening on the same port.

To prove they are in the same group, we can apply a configuration change; if they are in the same group, they should both receive the change.

Let's change the port they're running on, using the `hab config apply` command, run from Supervisor A.

```bash
echo 'port = 2112' | hab config apply redis.prod 1
```

Both service instances restart with the new configuration. Supervisor A's output is:

![Supervisor A running Redis on a new port](/images/habitat/supervisor_a_after.png)

and Supervisor B's output is:

![Supervisor B running Redis on a new port](/images/habitat/supervisor_b_after.png)

Note that they have both restarted (as evidenced by the new PID values), and that both are now running on port 2112, as we instructed.

Had the services been in different groups, the configuration change would not have applied to both of them (it was targeted at `redis.prod`). If the Supervisors has not been in gossip communication (achieved here through the use of the `--peer` option when Supervisor B was started), the configuration rumor (injected into Supervisor A's gossip network) would not have made it to `core/redis` service running on Supervisor B.
