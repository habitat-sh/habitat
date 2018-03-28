---
title: Habitat Builder for the People!
date: 2017-10-09
author: reset
tags: builder, launch
category: builder
classes: body-article
published: true
---

Today, the Habitat team is thrilled to announce the availability of [Habitat Builder](/blog/2017/05/Builder) for all projects and origins.

![](media/2017-10-09-Habitat-Builder-for-the-People/banner.png)

Habitat enables you to build, deploy, and manage your application, and automate the relationship between those lifecycle events to keep your application healthy, secure, and up to date.

To get started with Habitat, you [write a plan](/docs/developing-packages/). If you have a web application that is using a language stack that we have a Scaffolding for (as of publication, Ruby, Node, or Gradle), you can use that [Scaffolding](/docs/glossary/#glossary-scaffolding) to write a plan for you. A plan defines your application’s build and runtime dependencies, and tells Habitat where your application code lives.

With today’s launch you can connect your application’s GitHub repository to Builder and connect one or more Habitat plans in your repository to a package in your Builder origin. You then get several cool benefits:

* Every time you push new code to your application’s GitHub repository, Builder will automatically build a new package for your application.
* Every time a package is built and uploaded to Builder that your package depends upon, Builder will automatically rebuild your application’s package. This keeps your application up to date when packages are patched against security vulnerabilities without you needing to take any specific action.
* You can set up your Habitat Supervisors in a development environment to automatically subscribe to a Habitat Builder [release channel](/docs/reference/#sts=Habitat%20Promote%20Packages%20Through%20Channels), and they will:
  * Follow the [update strategy](/docs/using-habitat/#sts=Update%20Strategy) you have specified for that service group (none, rolling, all at once)
  * Detect a new package on the `unstable` release channel
  * Download and install the new package
  * Run any health checks, smoke tests, compliance tests, that you have specified in your [application lifecycle hooks](/docs/reference/#reference-hooks)
* Once your development environment has automatically updated to your latest build, you can check it out, see that everything is the way you want it, and then promote that new build to the `stable` channel. You can even create custom release channel such as `pre-production`, `union`, or `staging` to best match your organization’s continuous deployment strategy.
* Habitat Builder allows you to do all of this for both public and private GitHub repos, and also allows you to make your origin’s packages public or private

Habitat Builder provides one unified build pipeline to power your builds for stateful and stateless applications and services. Habitat build artifacts are isolated, atomic, and immutable. This means you can export these artifacts to the right format for your application, service, and infrastructure, and the artifact remains exactly the same. Today, we are:

* Adding a Cloud Foundry export type, in addition to the docker, aci, tar, and mesos types we already support. [Read more about exports](/docs/developing-packages/#pkg-exports)
* Adding a “Publishing” phase to Habitat Builder, so you can publish your Habitat builds as docker containers to your Docker Hub every time Builder creates a new build for you.
* We plan to add more formats and publishing locations over time! Tell us if there are some you are especially excited for.
* Announcing the alpha release of the [Habitat Operator for Kubernetes](https://github.com/habitat-sh/habitat-operator) developed by our amazing partners at [Kinvolk](https://kinvolk.io/blog/2017/10/habitat-operator---running-habitat-services-with-kubernetes/)!

We are constantly shipping new features and enhancements to Habitat Builder, Supervisor, and Studio, and we love interacting with our community. Ways to keep in touch:

* Read [our code on GitHub](https://github.com/habitat-sh)
* Talk to our core maintainers [on our Slack channel](http://slack.habitat.sh)
* Track [our work on GitHub](https://github.com/habitat-sh/habitat/projects/1)
* Study [our roadmap on our webpage](/community)

Want to get started? We have a bunch of resources for new and returning users!

* [Kick the tires in 10 minutes with a sample node application](/demo)
* Take a series of Habitat [tutorials](/tutorials/)
* Read our [docs](/docs)
