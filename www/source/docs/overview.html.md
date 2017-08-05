---
title: Habitat Docs - Overview
description: Documentation and tutorials for all aspects of Habitat including getting started; creating, building, and running Habitat packages; and implementation details.
---

# Overview

Habitat centers application configuration, management, and behavior around the application itself, not the infrastructure that the app runs on. This allows Habitat to be deployed and run on various infrastructure environments, such as bare metal, VM, containers, and PaaS.

The Habitat documentation is broken out into the following sections:

**Getting Started**

- [Download and install Habitat](/tutorials/download): Download the `hab` command-line interface tool (CLI) for Mac, Linux, and Windows to get started using Habitat.
- [Tutorials](/tutorials): Location for our getting started and advanced tutorials. You should start here if you are new to Habitat.
- [Concepts](/docs/concepts-overview): Describes the major components of Habitat and how they work.

**Using Habitat**

- [Create packages](/docs/create-packages-overview): Learn how to create a plan, what all of the plan settings are, how to configure a package, and how to build packages.
- [Run packages](/docs/run-packages-overview): Learn how to run a package natively as well as through an external runtime format, such as a Docker or rkt container.
- [Share packages](/docs/share-packages-overview): Describes how to upload, share, and run Habitat packages from the public depot.
- [Continuous deployment](/docs/continuous-deployment-overview): Explains how Habitat supports continuous deployment and the implementation details of deploying Habitat packages through Chef Automate.
- [Container orchestration](/docs/container-orchestration): Describes how to use Habitat with container orchestration technologies such as EC2 Container Service, Mesos, and Kubernetes.
- [Habitat internals](/docs/internals-overview): Provides deeper explanations on topics such as how the supervisor works, how leader election happens, etc.

**Reference**

- [CLI reference](/docs/reference/habitat-cli): Usage and basic help documentation for all `hab` CLI commands and subcommands.
- [Plan syntax](/docs/reference/plan-syntax): All settings, variables, callbacks, functions, and other configuration options that can be used when creating your plan.
- [Environment variables](/docs/reference/environment-vars): All environment variables that you can modify when using the `hab` CLI.
- [Package contents](/docs/reference/package-contents): Dependency, build, and configuration files that are included in a Habitat package.

**Contribute**

- [Help build Habitat](/docs/contribute-help-build): Additional functionality that we would love the community to help us define and implement.

## Where to begin

Whether you're new to Habitat or have a little bit of experience under your
belt, you've come to the right place.

### Are you new to Habitat?

If you're just getting started and want a quick introduction, then we recommend
you review the following items in the order listed.

<div class="callout-box--container row">
  <div class="callout-box columns medium-4">
    <a href="/tutorials/get-started/demo/" class="button secondary">Try Habitat demo</a>
    <p>Complete a short<br>10-minute interactive demo</p>
  </div>
  <div class="callout-box columns medium-4">
    <a href="/tutorials" class="button secondary">Go to tutorials</a>
    <p>Get your first<br>hands-on experience</p>
  </div>
  <div class="callout-box columns medium-4">
    <a href="/docs/concepts-overview" class="button secondary">Review concepts</a>
    <p>Start learning the<br>fundamentals of Habitat</p>
  </div>
</div>

### Already know your way around?

If you already have experience with Habitat and simply want to download the source
code and get cookin', then these links should provide everything you need.

<div class="callout-box--container row">
  <div class="callout-box columns medium-4">
    <a href="/tutorials/download" class="button secondary">Download and Habitat</a>
    <p>Get Habitat and install it on your workstation</p>
  </div>
  <div class="callout-box columns medium-4">
    <a href="/docs/contribute-help-build/" class="button secondary">Contribute to Habitat</a>
    <p>Habitat is open source;<br>let's build together</p>
  </div>
  <div class="callout-box columns medium-4">
    <a href="https://bldr.habitat.sh/#/sign-in" class="button secondary">Sign in to Web App</a>
    <p>Manage your information and<br>browse official packages</p>
  </div>
</div>
