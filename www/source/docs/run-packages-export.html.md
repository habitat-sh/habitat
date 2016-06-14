---
title: Export a package
---

# Export a package
Packages can be exported into multiple external, immutable runtime formats. This topic will be updated as more formats are supported in the future.

## Exporting to Docker

You can create a Docker container image for any package by performing the following steps:

1. Ensure you have a Docker daemon running on your host system. The exporter shares the Docker socket (`unix:///var/run/docker.sock`) into the studio.
2. Create an interactive studio with the `hab studio enter` command.
3. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Docker container image, for example:

       hab pkg install yourorigin/yourpackage

4. Run the Docker exporter on the package.

       hab pkg export docker yourorigin/yourpackage

5. You can now exit the studio. The new Docker container image exists on your computer and can be examined with `docker images` or run with `docker run`.

For an example of running a Habitat service in a Docker container, see the [Run your service](/tutorials/getting-started-process-build) step in the Getting Started tutorial.

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

## Exporting and Running on Container Cluster Managers

Habitat packages may be exported with the supervisor directly into a Docker or ACI-formatted container, but frequently the container itself will run within a container cluster manager such as Kubernetes or Mesos. Container cluster managers provide scheduling and resource allocation, ensuring workloads are running and available. Containerized Habitat packages may run within these runtimes, managing the applications while the runtimes handle the environment surrounding the application (ie. compute, networking, security).

### Apache Mesos and DC/OS

[Apache Mesos](https://mesos.apache.org/) is an open source container cluster manager and the container cluster manager for the [DC/OS](https://dcos.io) distributed platform. The `pkg-mesosize` command can create native [Mesos containers from Habitat packages](/docs/marathon/).

<!-- ### Kubernetes -->

<!-- [Kubernetes](http://kubernetes.io/) is an open source container cluster manager embedded in several distributed platforms including [Google's Container Engine](https://cloud.google.com/container-engine/) and [Tectonic](https://tectonic.com/) by CoreOS. Habitat packages are supported in both Docker and ACI container formats and can be [deployed within Kubernetes](/docs/kubernetes/). -->
