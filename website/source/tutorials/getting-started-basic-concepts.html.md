---
title: Getting started concepts
---

# Basic concepts

Here are the basic concepts that you will learn about in this tutorial:

-   **Artifact**: A portable, signed tarball that contains an application, runtime, dependencies, and any configuration data. When an artifact is downloaded, unpacked, and installed, it is known as a package.
-   **Supervisor**: A process manager that supports the application in the package payload. It handles any real-time configuration updates to the application, participates in communication with other supervisors running in deployed Habitat services, and provides health monitoring and other status messages about the service, which can be queried by another application.
-   **Habitat service**: A running supervisor with the package payload running inside it.
-   **Plan**: A shell script that defines what your package contains and how it is built.
-   **Depot**: A repository of packages where you can upload and download application and dependency packages. You can connect to the public Habitat depot as well as create a local depot.

In the [next step](/tutorials/getting-started-setup-environment), you will set up your Habitat development environment.
