---
title: Kubernetes with Habitat
---

[Kubernetes](http://kubernetes.io/) is an open source container cluster manager that is available as a stand-alone platform or embedded in several distributed platforms including [Google's Container Engine](https://cloud.google.com/container-engine/) and [Tectonic](https://tectonic.com/) by [CoreOS](https://coreos.com/). Habitat and Kubernetes are complementary: Kubernetes focuses on providing a platform for deployment, scaling, and operations of application containers across clusters of hosts while Habitat manages the creation and configuration of those application containers.

## Habitat Kubernetes Operator 

The [Habitat Kubernetes Operator](https://github.com/kinvolk/habitat-operator) is on-going work to create an operator that leverages Kubernetes API services to create a native and robust integration between the two technologies. Follow along on [github](https://github.com/kinvolk/habitat-operator), and join us in our #kubernetes channel in the [Habitat Slack Channel](https://slack.habitat.sh) for more information.

## Docker and ACI

Habitat packages can be exported in both Docker and ACI formats (as well as others). Kubernetes currently supports the Docker runtime and integration of the rkt container runtime (an implementation of the App Container spec) is under active development.

## kubectl

Habitat packages exported as containers may be deployed into Kubernetes through the [`kubectl` command](http://kubernetes.io/docs/user-guide/pods/single-container/). Using the [Docker exporter](/docs/run-packages-export) to create a containerized application, the container may be launched like this example:

```
$ kubectl run mytutorial --image=myorigin/mytutorial --port=8080
```

Assuming the Docker image is pulled from `myorigin/mytutorial` we are exposing port 8080 on the container for access. Networking ports exposed by Habitat need to be passed to `kubectl run` as `--port` options. We can see our deployment with the `kubectl get` command:

```
$ kubectl get pods -l run=mytutorial
```

## Environment variables and Networking

Kubernetes supports passing [environment variables](http://kubernetes.io/docs/user-guide/environment-guide/) into containers, this needs to be documented for use with Habitat.

## Multi-container Pods

Multi-container pod support through Habitat is still under active development. There may need to be a generated configuration file output from a CLI. Habitat's gossip protocol is on port 9638 and transmitted via UDP. It needs to be available by default to all of the containers in the pod.

## Related Reading 

* [In flight: Running a service group in kubernetes](https://github.com/habitat-sh/habitat/issues/2804)
* [Habitat Kubernetes Operator](https://github.com/kinvolk/habitat-operator)
* [Export a habitat package](/docs/run-packages-export)
* [Habitat CLI](/docs/habitat-cli)
