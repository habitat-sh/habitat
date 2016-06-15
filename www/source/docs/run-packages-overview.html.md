---
title: How to run packages
---

# Run packages

Habitat packages are run under the Habitat supervisor. At runtime, you can join supervisors together in a service group running the same topology, send configuration updates to that group, and more. You can also export the supervisor together with the package to an external immutable format, such as a Docker container.

## Running packages for testing
Packages can be tested in the interactive studio environment or natively on a host machine running Linux. To run packages directly:

1. [Build your package](/docs/create-packages-build) inside an interactive studio. Do not exit the studio after it is built.
2. To start your service, type `hab start yourorigin/yourname`, substituting the name and origin of the package you built in step 1. Your service should now be running.

If your host machine is running Linux, do the following to run your packages:

* Add the `hab` user and group.

      sudo useradd hab -u 42 -g 42 -d / -s /bin/sh -r
      sudo groupadd -og 42 hab

* Run the `hab` CLI as root.

      sudo hab start yourorigin/yourname


## Running packages in any environment

The `hab` program can also be installed on servers. It will retrieve the necessary components (like the current release of the supervisor) in order to run packages. Thus, you can type `hab start yourorigin/yourname` on any compatible system.

You can also write an init script or a systemd unit file to start the supervisor. For example, for systemd:

       [Unit]
       Description=My Habitat Application

       [Service]
       ExecStart=/usr/bin/hab start yourorigin/yourapp

       [Install]
       WantedBy=default.target

## Section details
This section is divided into the following areas:

- [Service groups](/docs/run-packages-service-groups): Defines service groups and how to join them.
- [Topologies](/docs/run-packages-topologies): Explains topologies in Habitat and how to join them.
- [Configuration updates](/docs/run-packages-apply-config-updates): Learn how to apply configuration updates to multiple services in a service group.
- [Upload files](/docs/run-packages-upload-files): Learn how to upload files to multiple services in a service group.
- [Security](/docs/run-packages-security): Describes how to encrypt communication between supervisors, and between users and service groups.
- [Binding](/docs/run-packages-binding): Learn how to bind to unknown service group names at runtime.
- [Update strategy](/docs/run-packages-update-strategy): Describes how the supervisor and its peers within a service group should respond when a new version of a package is available.
- [Run multiple packages](/docs/run-packages-director): Learn about the director and how to use it to start up multiple services on the same machine.
- [Export packages](/docs/run-packages-export): Learn how to export packages into multiple external, immutable runtime formats and work with container cluster managers like Kubernetes and Mesos.
- [Monitor services through the HTTP API](/docs/run-packages-monitoring): Discover how to use the HTTP API to retrieve census, status, and health information from your running services.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-service-groups">Service groups</a></li>
</ul>
