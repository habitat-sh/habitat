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

You can create a Docker container for any package by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install the Docker exporter:

      hab pkg install core/hab-pkg-dockerize

3. Install the Habitat package you want to create a Docker container from, for example:

      hab pkg install yourorigin/yourpackage

4. Run the Docker exporter on the package. (`hab pkg exec` sets the correct paths needed to find all the exporter's dependencies.)

      hab pkg exec core/hab-pkg-dockerize hab-pkg-dockerize yourorigin/yourpackage

5. You can now exit the studio. The new Docker image exists on your computer and can be examined with `docker images` or run with `docker run`.

## Exporting to An Application Container Image (ACI)

You can create an Application Container Image (ACI) for any package by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install the ACI exporter:

      hab pkg install core/hab-pkg-aci

3. Install the Habitat package you want to create a Docker container from, for example:

      hab pkg install yourorigin/yourpackage

4. Run the ACI exporter on the package.

      hab pkg exec core/hab-pkg-aci hab-pkg-aci yourorigin/yourpackage

5. Note that this will create unsigned ACI images. If you wish to sign your ACI with default options, pass `SIGN=true`:

      SIGN=true hab pkg exec core/hab-pkg-aci hab-pkg-aci yourorigin/yourpackage

6. The `.aci` can now be moved to any runtime capable of running ACIs (e.g. rkt on CoreOS) for execution.

## Examples

For an example of running a Habitat service in a Docker container, see the [Run your service](/tutorials/getting-started-process-build) step in the Getting Started tutorial.
