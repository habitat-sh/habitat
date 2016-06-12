---
title: Building packages concepts
---

# Concepts

Here are the concepts that you will learn about in this tutorial:

-   **Package**: A portable, cryptographically-signed tarball that contains an application, runtime, dependencies, and any configuration data.
-   **Supervisor**: A process manager that supports the application or service in the package. It handles any real-time configuration updates to the application, participates in communication with other supervisors running in deployed Habitat services, and provides health monitoring and other status messages about the service, which can be queried by another application.
-   **Habitat service**: A running supervisor with the application or service of the package running inside it.
-   **Plan**: Shell scripts that defines what your package contains, how it is built, and how it can be configured.
-   **Depot**: A repository that contains application and dependency packages. You can connect to the public Habitat depot as well as create a local depot.

In the next step, you will set up your Habitat development environment.

<hr>
<ul class="main-content--button-nav">
  <li><a href="/tutorials/getting-started-setup-environment" class="button cta">Next - Set up your environment</a></li>
  <li><a href="/tutorials/getting-started-overview/">Back to previous step</a></li>
</ul>
