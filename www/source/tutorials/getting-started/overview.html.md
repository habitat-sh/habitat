---
title: Getting started with Habitat
---

# Getting started with Habitat
Traditionally, the focus for managing applications and application behavior has been at the infrastructure level. However, tying an application to a specific infrastructure makes it difficult to deploy and manage that application on a different infrastructure. For example, moving an application from on-prem to a cloud PaaS environment can be a big job.

Habitat shifts the focus of application management and configuration from the infrastructure to the application itself. It allows you to create packages, which encapsulate the application, runtime dependencies, metadata, and any configuration information. A package contains everything an application needs to run on its target platform. When these packages are installed and run as services, applications become dynamically configurable, topology aware, and have monitoring capabilities built in.

As a first step in understanding Habitat, this tutorial will show you how to create a package, and then build and run it to demonstrate the capabilities of Habitat.

**Prerequisites**

Before starting this tutorial, you need to have the following:

*   The `hab` command-line interface tool. See [Get Habitat](/docs/get-habitat) if you don't already have this installed on your machine.
*    An active GitHub account is recommended. If you don't already have an account, [sign up](https://github.com/) for one now. Note: This is required to upload and share your packages with others in the Habitat community.
*   Your favorite text editor.
*   If you are running Mac OS X on your host machine, then you need [Docker for Mac](https://www.docker.com/products/docker) installed. Make sure you have a Docker machine running before proceeding through the tutorial.

    > Note: The minimum Docker version for Habitat is greater than or equal to the version specified in the core/docker plan, which currently is 1.11.2.


The remainder of the steps in this tutorial will be tailored by platform, so select the OS platform where you wish to build and run Habitat packages. If you want to change the platform at any step in the tutorial, use the selector at the top of the page.

<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started/mac/basic-concepts" class="button cta">Mac</a></li>
  <li><a href="/tutorials/getting-started/linux/basic-concepts" class="button cta">Linux</a></li>
</ul>
