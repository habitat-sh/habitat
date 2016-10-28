---
title: Container orchestration with Habitat
---

# Container orchestration with Habitat

Habitat packages may be exported with the supervisor directly into a [Docker or ACI-formatted container](/docs/run-packages-export/), but frequently the container itself will run within a container orchestrator such as Kubernetes or Mesos. Container orchestrators provide scheduling and resource allocation, ensuring workloads are running and available. Containerized Habitat packages may run within these runtimes, managing the applications while the runtimes handle the environment surrounding the application (ie. compute, networking, security).

## Amazon EC2 Container Service

[EC2 Container Service (ECS)](https://aws.amazon.com/ecs/) is a container management service provided by Amazon Web Services. [Habitat-built Docker images can be deployed to ECS](/docs/container-orchestration-ecs/) while ECS manages the infrastructure needed for running the Docker containers.

## Apache Mesos and DC/OS

[Apache Mesos](https://mesos.apache.org/) is an open source distributed systems kernel and the distributed systems kernel for [Mesosphere's DC/OS](https://dcos.io) distributed platform. The `hab pkg export mesos` command can create native [Mesos containers from Habitat packages](/docs/container-orchestration-mesos/) and launch them as applications within the Marathon .

## Kubernetes

[Kubernetes](http://kubernetes.io/) is an open source container orchestator embedded in several distributed platforms including [Google's Container Engine](https://cloud.google.com/container-engine/) and [CoreOS' Tectonic](https://tectonic.com/). Habitat packages are supported in both Docker and ACI container formats and can be [deployed within Kubernetes](/docs/container-orchestration-kubernetes/).

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/internals-overview">Habitat internals</a></li>
</ul>
