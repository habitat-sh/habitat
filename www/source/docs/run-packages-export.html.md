---
title: Export a package
---

# Export a package
Packages can be exported into multiple external, immutable runtime formats. This topic will be updated as more formats are supported in the future. Currently there are exports for: docker, ACI, mesos, tar. 

The command to export a package is `hab pkg export <FORMAT> <PKG_IDENT>`. [Habitat CLI Reference Guide](/docs/reference/habitat-cli/#hab-pkg-export/)

Read on for more detailed instructions.

## Exporting to Docker

You can create a Docker container image for any package by performing the following steps:

1. Ensure you have a Docker daemon running on your host system. The exporter shares the Docker socket (`unix:///var/run/docker.sock`) into the studio.
2. Create an interactive studio with the `hab studio enter` command.
3. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Docker container image, for example:

       hab pkg install yourorigin/yourpackage

4. Run the Docker exporter on the package.

       hab pkg export docker yourorigin/yourpackage

5. You can now exit the studio. The new Docker container image exists on your computer and can be examined with `docker images` or run with `docker run`.

For an example of using Docker Compose to run multiple Habitat containers together, see the [Run the Sample App](/tutorials/sample-app/mac/run-app/) step in the Build a Sample App tutorial.

## Exporting to a tarball 

1. Enter the habitat studio by using `hab studio enter`. 
2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a tarball, for example: 

        hab pkg install yourorigin/yourpackage 

3. Run the tar exporter on the package. 

        hab pkg export tar yourorigin/yourpackage 

4. Your package in a tar file exists locally on your computer in the format `origin-package-version-timestamp.tar.gz` and can be deployed and run on a target machine. 

## Exporting to an Application Container Image (ACI)

You can create an Application Container Image (ACI) for any package by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create an ACI, for example:

       hab pkg install yourorigin/yourpackage

3. Run the ACI exporter on the package.

       hab pkg export aci yourorigin/yourpackage

4. Note that this will create unsigned ACI images. If you wish to sign your ACI with default options, pass `SIGN=true`:

       SIGN=true hab pkg export aci yourorigin/yourpackage

5. The `.aci` can now be moved to any runtime capable of running ACIs (e.g. [rkt](https://coreos.com/rkt/) on CoreOS) for execution.

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-monitoring">Monitoring</a></li>
</ul>
