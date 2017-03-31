---
title: Getting started with Habitat
---

# Getting started with Habitat
Traditionally, the focus for managing applications and application behavior has been at the infrastructure level. However, tying an application to a specific infrastructure makes it difficult to deploy and manage that application on a different infrastructure. For example, moving an application from on-prem to a cloud PaaS environment can be a big job.

Habitat shifts the focus of application management and configuration from the infrastructure to the application itself. It allows you to create packages, which encapsulate the application, runtime dependencies, metadata, and any configuration information. A package contains everything an application needs to run on its target platform. When these packages are installed and run as services, applications become dynamically configurable, topology aware, and have monitoring capabilities built in.

As a first step in understanding Habitat, this tutorial will show you how to create a package, and then build and run it to demonstrate the capabilities of Habitat.

**Prerequisites**

Before starting this tutorial, you need to have the following:

*   An active GitHub account is recommended. If you don't already have an account, [sign up](https://github.com/) for one now. Note: This is required to upload and share your packages with others in the Habitat community.
*   Your favorite text editor.
*   If you are running Mac OS X on your host machine, then you need [Docker for Mac](https://www.docker.com/products/docker#/mac) installed.
*   If you are running Microsoft Windows, you will need [Docker for Windows](https://www.docker.com/products/docker#/windows) installed. Also, if you are running an older Windows 10 version such as 1511, ANSI escape sequences are not supported. This means the color output and other formatting used by the `hab` CLI will not render properly in your PowerShell window. You can use console emulators like [ConEmu](https://conemu.github.io/) to run PowerShell with ANSI color support.

For both OS X and Windows, make sure you have a Docker machine running before proceeding through the tutorial. Docker is not required for the Linux version of the tutorial unless you choose to create a Docker container from your package. 

> Note: The minimum Docker version required for Habitat is greater than or equal to the version specified in the core/docker plan, which currently is 1.11.2.

The remainder of the steps in this tutorial will be tailored by platform, so select the OS platform where you wish to build and run Habitat packages. If you want to change the platform at any step in the tutorial, use the selector at the top of the page.

<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started/mac/basic-concepts" class="button cta">Mac version</a></li>
  <li><a href="/tutorials/getting-started/linux/basic-concepts" class="button cta">Linux version</a></li>
  <li><a href="/tutorials/getting-started/windows/basic-concepts" class="button cta">Windows version</a></li>
</ul>
