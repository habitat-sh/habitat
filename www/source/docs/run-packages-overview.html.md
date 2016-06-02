---
title: How to run packages
---

# Run packages

Habitat packages are run under the Habitat supervisor. You can also export the supervisor together with the package to an external immutable format, such as a Docker container.

## Running packages in the studio

Packages can be tested in the interactive studio environment. To run packages directly:

1. [Build your artifact](/docs/build-artifacts-overview) inside an interactive studio. Do not exit the studio after it is built.
2. To start your service, type `hab start yourorigin/yourname`, substituting the name and origin of the artifact you built in step 1. Your service should now be running.

## Exporting to Docker

This topic describes how to create a Docker container for any artifact by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter`.
2. Install the Docker exporter:

      hab pkg install core/hab-pkg-dockerize

3. Install the Habitat package you want to create a Docker container from, for example:

      hab pkg install yourorigin/yourpackage

4. Ensure that all the utilities needed to create a Docker container are on the path:

      export PATH=$PATH:`hab pkg path core/hab-studio`/bin:`hab pkg path core/hab-pkg-dockerize`/bin:`hab pkg path core/docker`/bin

5. Run the Docker exporter on the package:

      hab-pkg-dockerize yourorigin/yourpackage

6. You can now exit the studio. The new Docker image exists on your computer and can be examined with `docker images` or run with `docker run`.

For an example of running a Habitat service in a Docker container, see the [Run your service](/tutorials/getting-started-process-build) step in the Getting Started tutorial.
