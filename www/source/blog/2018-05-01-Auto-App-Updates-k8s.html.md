---
title: Automate Application Updates with Habitat and Kubernetes
date: 2018-05-01
author: Tasha Drew
tags: kubernetes, channels, builder
category: update
classes: body-article
---

The Habitat team dreams of a world in which applications are built with the intelligence they need to run well, handle a variety of failure scenarios, update themselves when new software and dependencies are available, and connect seamlessly to the services they depend upon or support. In addition, these applications should be able to run wherever you need them to, without any tweaks to the application definition or build process - because Habitat allows you to export your services to various formats post build. 

Once we’ve exported your application to the format you want, the application should be able to perform within that ecosystem in a highly performant and native way. This requires additional automation and integration in order to ensure applications  know how to behave natively and intelligently. For the Cloud Native world, specifically Kubernetes, the Habitat team has been leveraging a bunch of exciting technology to ensure this experience, with exporters to Helm and Kubernetes manifests, a Habitat Operator to ensure operational behavior and allow users to leverage powerful native Kubernetes capabilities while keeping their applications in sync, and an Open Service Broker.  

Last week, Habitat core team member [Elliott Davis](https://twitter.com/libsysguy) demonstrated the next step in the Habitat to Kubernetes workflow: Habitat application rebuilds (which can be triggered by source code and upstream dependency updates) can now be automatically deployed to applications running on Kubernetes. (He will also be demonstrating [this live at Kubecon EU, May 2nd at 2pm](http://sched.co/Dqua) with the one and only [Fletcher Nichol](https://twitter.com/fnichol) for those who are in town to check it out and ask questions). 

Every Habitat application and package has channels that running services can subscribe to for updates. The defaults are `unstable` and `stable`, but a user can define as many as they would like to, for example `development`, `qa`, `staging` and `production`. You can now subscribe your Habitat-built applications running on Kubernetes to a Habitat Builder channel of your choice, and they will be able to follow standard Habitat automation and update themselves when an update is made available on their channel. 

Check out Elliott’s demo of the Habitat Updater Service on YouTube, and let us know what you think! 

<iframe width="560" height="315" src="https://www.youtube.com/embed/z9MP52kwBgc?rel=0" frameborder="0" allow="autoplay; encrypted-media" allowfullscreen></iframe>

![Image of Kubernetes and Hab](https://www.habitat.sh/images/infographics/deploy-services-to-kubernetes-with-habitat-flow-6ddca9cd.png)

### Got questions? 
* [Ask and answer questions on the Habitat forums](https://forums.habitat.sh/) 
* [Chat with the Habitat Community on Slack](http://slack.habitat.sh/) 
* [Learn more about Habitat](https://www.habitat.sh/) 

### Read more: 
* [Habitat Operator for Kubernetes on GitHub](https://github.com/habitat-sh/habitat-operator) 
* [Habitat + Open Service Broker](https://www.habitat.sh/blog/2018/05/Hab-OSB/) 
* [Helm and Habitat](https://www.habitat.sh/blog/2018/02/Habitat-Helm/) 

