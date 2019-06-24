---
title: Kubecon, Chef Habitat, and Kubernetes!
date: 2017-12-06
author: Tasha Drew
tags: kubernetes, exporter, operator, k8s
category: product
classes: body-article
---
With Kubecon North America kicking off today in Austin, Texas, we're happy and excited to share a bunch of work that extends and enhances the Chef Habitat and Kubernetes workflow to make it even more easy and powerful to build and run your applications with Chef Habitat while you deploy and manage your infrastructure using Kubernetes.

For those new to our project, Chef Habitat is Chef.io’s next generation application automation framework focused on continuously building, deploying, and running your application to scale as it grows across a distributed infrastructure. We are an Apache 2.0 open source project and CNCF member.

In partnership with our friends and Kubernetes afficianados at [Kinvolk.io](https://kinvolk.io/), we are announcing the following updates:

## Chef Habitat Operator for Kubernetes

First released in October, the Chef Habitat Operator for Kubernetes has a bunch of updates and is the recommended way to integrate Chef Habitat and Kubernetes into a unified whole. It leverages the Kubernetes API to unify communciation between your Kubernetes clusters and Chef Habitat Builder as it builds and manages your applications, and enables you to use both `kubectl` and the `hab` cli and keep all of your components in sync.

- [Chef Habitat Operator for Kubernetes on Github](https://github.com/habitat-sh/habitat-operator)
- [Announcement blog](https://github.com/habitat-sh/habitat-operator)

## Chef Habitat Kubernetes Exporter

When you are building your applications using Chef Habitat, you can export them to the correct format for the environment you want to deploy them into. To extend the Chef Habitat and Kubernetes workflow, we are introducing the [Chef Habitat Kubernetes Exporter](https://kinvolk.io/blog/2017/12/introducing-the-habitat-kubernetes-exporter/). It exports your Chef Habitat artifacts into a docker container with a Kubernetes manifest that can then be deployed to a Kubernetes cluster running the Chef Habitat Kubernetes Operator.

## Chef Habitat and Google Kubernetes Engine (GKE)

Follow simple step-by-step instructions for how to deploy an application to GKE using the Chef Habitat Operator for Kubernetes in [Getting started with Chef Habitat on Kubernetes](https://kinvolk.io/blog/2017/12/get-started-with-habitat-on-kubernetes/).

## Kubernetes the Hab way

You can use Chef Habitat to help manage the components that set up your Kubernetes clusters! Check out [this blog](https://kinvolk.io/blog/2017/12/kubernetes-the-hab-way/) about "Kubernetes the Hab way" to see how.

## Chef Habitat Builder: Automated Kubernetes Deploys

Chef Habitat Builder was [announced in early access in October](https://www.habitat.sh/blog/2017/10/Habitat-Builder-for-the-People/) and allows you to programmatically and automatically build all of your applications and services as you update your application code on Github, and as your application's depenencies have upstream changes. Using Chef Habitat's [channel promotion](https://www.habitat.sh/docs/using-habitat/#continuous-deployment), you can automatically deploy updates to development environments and then promote to production environments when ready. 

Today, learn how to leverage Chef Habitat Builder for [Automated Kubernetes Deploys](https://kinvolk.io/blog/2017/12/automated-build-to-kubernetes-with-habitat-builder/).

## More information

- [Chef Habitat Operator for Kubernetes on Github](https://github.com/habitat-sh/habitat-operator)
- [Chef Habitat on Github](https://github.com/habitat-sh/)
- We're at Kubecon! Swing by our booth to chat.
- [Talk to us on Slack](http://slack.habitat.sh)
