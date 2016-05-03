---
title: Getting stated with Habitat
---

# Getting Started with Habitat
Traditionally, the focus for managing applications and application behavior has been at the infrastructure level. However, tying an application to a specific infrastructure makes it difficult to deploy and manage that application on a different infrastructure. For example, moving an application from on-prem to a cloud PaaS environment can be a big job.

Habitat shifts the focus of application management and configuration from the infrastructure to the application itself. It allows you to create artifacts, which encapsulate the application, runtime dependencies, metadata, and any configuration information. When installed and run as services, they are dynamically configurable, topology aware, and have monitoring capabilities built in. The artifact contains everything the client application needs to run on its target platform.

As a first step in understanding Habitat, this tutorial will show you how to create an artifact, and then build and run it to demonstrate the capabilities of Habitat.

**Prerequisites**

Before starting this tutorial, you need to have the following:

*   An active GitHub account. If you don't already have an account, [sign up](https://github.com/) for one now.
*   Your favorite text editor.
*   Docker Toolbox installed and Docker running on your local VM. See the [Get Started documentation](https://docs.docker.com/mac/) for instructions on how to install them. The toolbox also installs Oracle VM VirtualBox, which will run and manage the Linux VM where the Docker daemon resides. Make sure you have a Docker VM running before proceeding through the tutorial.
*   TCP port 9636 open. This is needed to upload and download artifacts from the public depot.

Now that you have the prerequisites out of the way, let's start off by learning some key Habitat concepts in the [next step](/tutorials/getting-started-basic-concepts).
