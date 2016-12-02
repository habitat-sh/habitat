---
title: Kubernetes with Habitat
---

[Kubernetes](http://kubernetes.io/) is an open source container cluster manager that is available as a stand-alone platform or embedded in several distributed platforms including [Google's Container Engine](https://cloud.google.com/container-engine/) and [Tectonic](https://tectonic.com/) by [CoreOS](https://coreos.com/). Habitat and Kubernetes are complementary, Kubernetes focuses on providing a platform for deployment, scaling, and operations of application containers across clusters of hosts while Habitat manages the creation and configuration of those application containers.

## Docker and ACI

Habitat packages are supported in both Docker and ACI formats. Kubernetes currently supports the Docker runtime and integration of the rkt container runtime (an implementation of the App Container spec) is under active development.

## kubectl

Habitat packages exported as containers may be deployed into Kubernetes through the [`kubectl` command](http://kubernetes.io/docs/user-guide/pods/single-container/). Using the [Docker exporter](/docs/run-packages-export) to create a containerized application, the container may be launched like this example:

       kubectl run mytutorial --image=myorigin/mytutorial --port=8080

Assuming the Docker image is pulled from `myorigin/mytutorial` we are exposing port 8080 on the container for access. Networking ports exposed by Habitat need to be passed to `kubectl run` as `--port` options. We can see our deployment with the `kubectl get` command:

       kubectl get pods -l run=mytutorial

## Environment variables and Networking

Kubernetes supports passing [environment variables](http://kubernetes.io/docs/user-guide/environment-guide/) into containers, this needs to be documented for use with Habitat.

## Multi-container Pods

Multi-container pod support through Habitat is still under active development. There may need to be a generated configuration file output from a CLI. Habitat's gossip protocol is on port 9638 and transmitted via UDP. It needs to be available by default to all of the containers in the pod.
