---
title: Getting started with Habitat
---

# Getting started with Habitat
Traditionally, the focus for managing applications and application behavior has been at the infrastructure level. However, tying an application to a specific infrastructure makes it difficult to deploy and manage that application on a different infrastructure. For example, moving an application from on-prem to a cloud PaaS environment can be a big job.

Habitat shifts the focus of application management and configuration from the infrastructure to the application itself. It allows you to create packages, which encapsulate the application, runtime dependencies, metadata, and any configuration information. A package contains everything an application needs to run on its target platform. When these packages are installed and run as services, applications become dynamically configurable, topology aware, and have monitoring capabilities built in.

As a first step in understanding Habitat, this tutorial will show you how to create a package, and then build and run it to demonstrate the capabilities of Habitat.

**Prerequisites**

This tutorial currently supports Linux and Mac OS X as host operating systems. Before starting this tutorial, you need to have the following:

*   The `hab` command-line interface tool. See [Get Habitat](/docs/get-habitat) if you don't already have this installed on your machine.
{::comment}*    An active GitHub account is recommended. If you don't already have an account, [sign up](https://github.com/) for one now. Note: This is only required if you want to use the build service to build your packages. {:/comment}
*   Your favorite text editor.
*   Open egress on TCP port 9636. This is needed to upload and download packages from the public depot.
*   If you are running Mac OS X on your host machine, then you need [Docker Toolbox](https://www.docker.com/products/docker-toolbox) installed. The toolbox also installs Oracle VM VirtualBox, which will run and manage the Linux VM where the Docker daemon resides. Make sure you have a Docker VM running before proceeding through the tutorial.
*   If you are running a Linux distribution such as Ubuntu on your host machine, then you need to install the [Docker Engine](https://docs.docker.com/linux/) CLI tool.

  > Note: The minimum Docker version for Habitat is greater than or equal to the version specified in the core/docker plan, which currently is 1.11.1.

Now that you have the prerequisites out of the way, let's start off by learning some key Habitat concepts in the next step.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started-basic-concepts" class="button cta">Next - Concepts</a></li>
</ul>
