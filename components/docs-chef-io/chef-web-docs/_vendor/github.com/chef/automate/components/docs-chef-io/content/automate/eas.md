+++
title = "Chef EAS"

date = 2019-10-18T18:54:09+00:00
draft = false
[menu]
  [menu.automate]
    title = "Chef EAS"
    parent = "automate/applications"
    identifier = "automate/applications/eas.md Chef EAS"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/eas.md)

The Chef Enterprise Application Stack (EAS) allows organizations to automate infrastructure, security, and application delivery together, helping them deploy software quickly while maintaining compliance with industry regulations. Chef EAS helps teams drive efficiency and consistency for any application across multi-cloud and heterogeneous infrastructure.

Chef EAS includes the following Chef solutions:

* **Chef Habitat** builds and deploys your applications because delivering applications is where you derive your business value
* **Chef Infra** configures your infrastructure, because your applications also needs to run on a stable and scalable system
* **Chef Inspec** validates and secures both your applications and your infrastructure
* **Chef Automate** visualizes your applications, their infrastructure, and the compliance status for both.
* **Chef Workstation** provides a unified experience for all of Chef EAS

## Chef EAS Application Management with Chef Habitat

Chef Habitat automates building and deploying applications, turning a web of delicate procedures into a resilient and repeatable process.
A Chef Habitat package contains the compiled binary and a manifest of all the dependencies required for building and running your application.
This results in a lightweight and portable artifact that you can deploy to bare metal or virtual machines, as well as export to immutable formats, such as Docker containers or Kubernetes pods.
Bundling applications with a complete dependency manifest ensures that your application behaves as consistently on a developer's laptop as it does in its on-prem VM farm staging environment, and it also behaves as consistently there as it does in its cloud-based deployment environment.

## The Chef Automate EAS Application Tab

With the new application operations dashboard in Chef Automate, operations teams gain comprehensive and customizable visibility into the health of the services that make up the application. This makes it easier to understand what is degrading the health of an applications and to keep it running smoothly.

The Chef Automate EAS Application dashboard allows you to organize and display your applications data from Chef Habitat in an intuitive way. The Chef Automate EAS Application Dashboard provides visibility into your application artifacts and channels by letting you see which versions of your packages are running on your system.
Chef Automate organizes data from the application and environment tags provided by the Chef Habitat supervisor.

## Prerequisites

The Chef Automate EAS application feature introduces several concepts from Chef Habitat, which are introduced in the following [Glossary]({{< relref "applications_dashboard.md#Glossary" >}}).

Find Chef Automate EAS system requirements, installation, and configuration instructions in the [Setting up the Applications Dashboard]({{< relref "applications_setup.md" >}}).

### Glossary

_Application_ -
An application is a program that is made up of multiple underlying services.
Chef Habitat provides automation capabilities for building and delivering your applications service-by-service to any environment regardless of the infrastructure on which it is deployed.
In Chef Habitat, you define your application and its dependencies, configuration, management, and behavior in a `plan.sh` or `plan.ps1` file.

_Application Lifecycle_ -
In a typical enterprise software an application goes through stages of development, testing, acceptance, production, which together are called the _application lifecycle_. The application lifecycle is supported by codified infrastructure and configuration that correlates with each stage, which is called an _application delivery pipeline_.

_Artifact_ -
A Chef Habitat artifact (also known as a "hart file") is a software package produced by the Chef Habitat build tools. It is comparable to a deb file on Debian-based Linux systems, or a rpm file on RedHat-based Linux systems. While very simple application have two or three underlying services, most modern applications follow architecture patterns with many services that result in multiple artifacts.
Artifacts are built according to the instructions in the `plan.sh` or `plan.ps1` file.
Each artifact contains a software library or application, configuration settings for the application, and lifecycle hooks.

_Channel_ -
Chef Habitat supports continuous deployments through the use of channels, which are tags used to describe the status of your artifact. Channel are conceptual spaces expressed in code by adding a tag to an artifact, which is called a "promotion". Once promoted, an artifact is considered to "be" in that channel.
In most cases, artifacts have three possible channels: "unstable", "testing", and "stable". The "unstable" tag denotes an artifact that is still in active development, "testing" means that the artifact is a candidate for release, and "stable" means that it is ready for consumption.
When you upload your artifact to Builder, Chef Habitat labels your application artifact with the `unstable` tag, which means that it is in the "unstable" channel.
When you promote your application artifact to another channel, such as "test" or "stable", Chef Habitat applies the new tag and makes it available to the respective channel.
You can apply more than one tag to a single artifact, for example, artifacts are often tagged for both the "unstable" and "test" channels.
For more information, see the Chef Habitat [Continuous Deployment Using Channels](https://www.habitat.sh/docs/using-habitat/#continuous-deployment-using-channels) documentation.

_Deployment_ -
Each instance of an artifact downloaded from a channel into an environment is called a deployment.

_Environment_ -
An environment is the coded expression of the combined infrastructure and configuration that your application requires.
Chef Habitat supports continuous delivery by letting you define how the environments in your application's delivery pipeline consume each of your artifacts. Examples of environments include "development," "acceptance," and "production".
Every environment in your application lifecycle has its own Supervisors, service groups, and services.

_Supervisor_ -
The Supervisor is Chef Habitat's process manager, which means it controls the processes related to a package, including starting, monitoring, and updating services. Each Supervisor runs a single instance of a service.
Each Supervisor in an environment subscribes to a channel, which means they watch for new versions of services promoted in that channel.
Once a supervisor detects a new artifact in a channel, it deploys the new package into its own environment and updates all of the services for that service group.
For more information about automated deployments, see the Chef Habitat [update-strategy](https://www.habitat.sh/docs/using-habitat/#update-strategy) documentation.

_Service_ -
A service is any single running instance of a Chef Habitat package running in an environment and managed by a Supervisor.

_Service Group_ -
A service group is a label that you apply to multiple instances of a single service running in an environment. Service groups let you manage all of the services with this label as a single entity with a single operation.
For example, if you have five Redis replicas running in your test environment, then these as a service group collects them as a single thing.
This lets you stop, restart, or reconfigure all five examples of this Redis service in a single operation.
Service groups also let you gather information for configuration templating by extracting information from each of them--such as their ip addresses--in a single operation.
