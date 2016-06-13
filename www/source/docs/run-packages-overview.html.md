---
title: How to run packages
---

# Run packages

Habitat packages are run under the Habitat supervisor. You can also export the supervisor together with the package to an external immutable format, such as a Docker container.

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

## Exporting to Docker

You can create a Docker container image for any package by performing the following steps:

1. Ensure you have a Docker daemon running on your host system. The exporter shares the Docker socket (`unix:///var/run/docker.sock`) into the studio.
2. Create an interactive studio with the `hab studio enter` command.
3. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Docker container image, for example:

       hab pkg install yourorigin/yourpackage

4. Run the Docker exporter on the package.

       hab pkg export docker yourorigin/yourpackage

5. You can now exit the studio. The new Docker container image exists on your computer and can be examined with `docker images` or run with `docker run`.

## Exporting to an Application Container Image (ACI)

You can create an Application Container Image (ACI) for any package by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create an ACI, for example:

       hab pkg install yourorigin/yourpackage

3. Run the ACI exporter on the package.

       hab pkg export aci yourorigin/yourpackage

4. Note that this will create unsigned ACI images. If you wish to sign your ACI with default options, pass `SIGN=true`:

       SIGN=true hab pkg export aci yourorigin/yourpackage

5. The `.aci` can now be moved to any runtime capable of running ACIs (e.g. [rkt](https://coreos.com/rkt/) on CoreOS) for execution.

## Export to an Apache Mesos (Marathon) application

Mesos can run Docker containers but the native format is a filesystem sandbox with cgroups and
namespaces. You can create these from Habitat packages by following these steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create an ACI, for example:

       hab pkg install yourorigin/yourpackage

3. Run the Mesos exporter on the package.

       hab pkg export mesos yourorigin/yourpackage

4. This will create a Marathon-format tarball in the results directory, and also print the JSON needed to load the application into Marathon.

5. Note that the default resource allocation for the application is very small: 0.5 units of CPU, no disk, one instance, and 256MB of memory. To change these resource allocations, pass different values to the Mesos exporter as environment variables:

       CPU=1.0 DISK=5 INSTANCES=2 MEM=512 hab pkg export mesos yourorigin/yourpackage

## Examples

For an example of running a Habitat service in a Docker container, see the [Run your service](/tutorials/getting-started-process-build) step in the Getting Started tutorial.
