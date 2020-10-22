+++
title = "About Chef Habitat"

[menu]
  [menu.habitat]
    title = "About Chef Habitat"
    identifier = "habitat/About Chef Habitat"
    parent = "habitat"
    weight = 5
+++

Chef Habitat centers application configuration, management, and behavior around
the application itself, not the infrastructure that the app runs on.
It provides automation that can programmatically and declaratively build,
deploy, and manage your application and services, both stateful and stateless.
And it can be deployed and run on various infrastructure environments
including bare metal, VM, containers, and PaaS.

## Chef Habitat Builder

[Chef Habitat Builder]({{< relref "builder-overview" >}}) acts as the core of
Chef’s Application Delivery Enterprise hub. You can run Chef Habitat Builder as
a cloud-based service or on premises.

Habitat Builder provides package storage, search, and an API for clients.

Plan files are stored in the Chef Habitat Builder SaaS where they can be viewed
and accessed by the Chef Habitat community. The plan files can also be shared with
the on-premises version of Bilder and then be copied and maintained locally.

## Plans

A [plan]({{< relref "plan-overview" >}}) is a directory comprised of shell scripts
and optional configuration files that define how you download, configure, make,
install, and manage the lifecycle of the software in the artifact.

## Supervisor

The Supervisor is a process manager that has two primary responsibilities. First,
it starts and monitors child services defined in the plan it is running. Second,
it receives and acts upon information from the other Supervisors to which it is
connected.

## Services

A [service]({{< relref "about_services" >}}) is a Chef Habitat package running under a Chef
Habitat Supervisor. Services can be joined together in a [service group]({{< relref "service_groups" >}}),
which is a group of services with the same package and topology type connected together
across a Supervisor network.

## Installing Chef Habitat

The Chef Habitat CLI can be [installed]({{< relref "install-habitat" >}}) on
Linux, Mac, and Windows.
