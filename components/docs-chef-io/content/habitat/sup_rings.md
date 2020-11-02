+++
title = "Setting Up a Ring"
description = "Setting Up a Ring"

[menu]
  [menu.habitat]
    title = "Setting Up a Ring"
    identifier = "habitat/supervisors/sup-rings"
    parent = "habitat/supervisors"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/habitat-sh/habitat/blob/master/components/docs-chef-io/content/habitat/sup_rings.md)

## Bastion Ring / Permanent Peers

A "Bastion Ring" is a pattern for preventing rumor loss and a split brain in a network of Chef Habitat Supervisors - it is highly recommended for any real environment use case. Create a minimum of 3 Supervisors and join them together. They should not run any services and they should be marked as permanent peers - their only job is to spread rumors to each other. Then, when you provision additional Supervisors pass the network address of each Supervisor running in the Bastion Ring to the `--peer` argument of `hab sup run`. It's recommended to create a Bastion Ring in any network zones which may become partitioned due to a hardware failure. For example, if you have a Chef Habitat ring spanning multiple data centers and clouds, each should have a bastion ring of a minimum of 3 Supervisors in addition to the Supervisors running your services.

## Using a Scheduler

**Note:** If you are using a container scheduler such as the Kubernetes `kube-scheduler`, Docker Swarm mode, Mesos DC/OS's Marathon or Chronos, or a PaaS such as Cloud Foundry, you should not follow the bastion ring pattern, because the scheduler handles persistence and orchestration on your behalf.

More resources on schedulers:

- [Chef Habitat Container Orchestration]({{< relref "container_orchestration" >}})
- Kubernetes `kube-scheduler`: https://kubernetes.io/docs/concepts/scheduling-eviction/kube-scheduler
- Docker Swarm mode: https://docs.docker.com/engine/swarm
- Mesos DC/OS Marathon or Chronos: https://mesosphere.github.io/marathon/
- Cloud Foundry: https://www.habitat.sh/get-started/cloudfoundry

## Initial Peer(s)

The initial peer(s) is a requirement of any distributed system. In Chef Habitat, a new Supervisor that is starting up looks for an initial peer(s) to join to begin sharing information about the health and status of peers and other services, to increase the health of the overall Ring.
