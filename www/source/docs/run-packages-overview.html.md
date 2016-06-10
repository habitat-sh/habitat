---
title: How to run packages
---

# Run packages

Habitat packages are run under the Habitat supervisor. You can also export the supervisor together with the package to an external immutable format, such as a Docker container.

## Running packages in the studio for testing

Packages can be tested in the interactive studio environment. To run packages directly:

1. [Build your package](/docs/build-packages-overview) inside an interactive studio. Do not exit the studio after it is built.
2. To start your service, type `hab start yourorigin/yourname`, substituting the name and origin of the package you built in step 1. Your service should now be running.

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
3. Install or [build](/docs/build-packages-overview) the Habitat package from which you want to create a Docker container image, for example:

       hab pkg install yourorigin/yourpackage

4. Run the Docker exporter on the package.

       hab pkg export docker yourorigin/yourpackage

5. You can now exit the studio. The new Docker container image exists on your computer and can be examined with `docker images` or run with `docker run`.

## Exporting to An Application Container Image (ACI)

You can create an Application Container Image (ACI) for any package by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install or [build](/docs/build-packages-overview) the Habitat package from which you want to create an ACI, for example:

       hab pkg install yourorigin/yourpackage

3. Run the ACI exporter on the package.

       hab pkg export aci yourorigin/yourpackage

4. Note that this will create unsigned ACI images. If you wish to sign your ACI with default options, pass `SIGN=true`:

       SIGN=true hab pkg export aci yourorigin/yourpackage

5. The `.aci` can now be moved to any runtime capable of running ACIs (e.g. [rkt](https://coreos.com/rkt/) on CoreOS) for execution.

## Examples

For an example of running a Habitat service in a Docker container, see the [Run your service](/tutorials/getting-started-process-build) step in the Getting Started tutorial.
