---
title: Run packages in a service group
---

# Run packages in a service group
A service group is a logical grouping of services with the same package and topology type connected together in a ring. They are created to share configuration and file updates among the services within those groups and can be segmented based on workflow or deployment needs (QA, Production, and so on). Updates can also be [encrypted](/docs/run-packages-security#service-group-encryption) so that only members of a specific service group can decrypt the contents.

By default, every service joins the `default` service group unless otherwise specified at runtime.

In addition, multiple service groups can reside in the same ring. This allows data exposed by supervisors to be shared with other members of the ring regardless of which group they are in.

## Joining a service group
To join a service group, run your service either natively with the `hab start` command, or through an external runtime format such as a Docker container. A best practice is to name the group _servicename_._environment_ to support continuous deployment workflows.

Here's how to start up the service using the supervisor (`hab-sup`) directly:

    hab start myorigin/myapp --group myapp.prod

Here's how to run the same command from a Docker container:

    docker run -it myorigin/myapp --group myapp.prod

You will see a similar output below when your service starts. The census entry shows which service group your service belongs to using the format _servicename_._groupname_. Keep this service instance running.

    hab-sup(MN): Starting myorigin/myapp
    hab-sup(GS): Supervisor 172.17.0.2: 38a2ac4f-5348-48df-86f0-b293284740ce
    hab-sup(GS): Census myapp.myapp.prod: c60bfa12-c913-4f89-b4f3-d082d1310c3d
    ...

Services only join together to form a ring when a peer IP address is specified by other similar services at runtime.  In a new window or external runtime format, run the same command you ran above, but this time, reference the peer IP address of the previous service specified in the supervisor output above.

    hab start myorigin/myapp --group myapp.prod --peer 172.17.0.2

 The output for this new service shows that it has either formed a new ring with the service above, or joined an existing ring where the other service was a member. Specifying the group name and peer values together ensures that the new service has joined the `myapp.prod` service group.

    hab-sup(MN): Starting myorigin/myapp
    hab-sup(GS): Supervisor 172.17.0.3: 426f2b49-fb04-41fa-b656-f43260ab122e
    hab-sup(GS): Census myapp.myapp.prod: 1f9412eb-c1df-4df8-a07a-65540333826a
    hab-sup(GS): Starting inbound gossip listener
    hab-sup(GS): Joining gossip peer at 172.17.0.2:9634
    ...

> Note: It is important that you specified the group value above. If not, then your new service would have joined the `default` service group, but remained a gossip peer of the previous service.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-topologies">Topologies</a></li>
</ul>
