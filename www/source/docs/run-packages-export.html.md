---
title: Export a package
---

# Export a package
Packages can be exported into multiple external, immutable runtime formats. This topic will be updated as more formats are supported in the future. Currently there are exports for: docker, ACI, mesos, tar. 

The command to export a package is `hab pkg export <FORMAT> <PKG_IDENT>`. See the [Habitat CLI Reference Guide](/docs/reference/habitat-cli/#hab-pkg-export/) for more CLI information.

Read on for more detailed instructions.

## Exporting to Docker

You can create a Docker container image for any package by performing the following steps:

1. Ensure you have a Docker daemon running on your host system. The exporter shares the Docker socket (`unix:///var/run/docker.sock`) into the studio.

2. Create an interactive studio with the `hab studio enter` command.

3. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Docker container image, for example:

    ```
    $ hab pkg install yourorigin/yourpackage
    ```

4. Run the Docker exporter on the package.
   
    ```
    $ hab pkg export docker yourorigin/yourpackage
    ```

5. You can now exit the studio. The new Docker container image exists on your computer and can be examined with `docker images` or run with `docker run`.

For an example of using Docker Compose to run multiple Habitat containers together, see the [Run the Sample App](/tutorials/sample-app/mac/run-app/) step in the Build a Sample App tutorial.

## Exporting to a tarball 

1. Enter the habitat studio by using `hab studio enter`. 

2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a tarball, for example: 

    ```
    $ hab pkg install yourorigin/yourpackage 
    ```

3. Run the tar exporter on the package. 

    ```
    $ hab pkg export tar yourorigin/yourpackage 
    ```

4. Your package in a tar file exists locally on your computer in the format `origin-package-version-timestamp.tar.gz` and can be deployed and run on a target machine. 

## Exporting to an Application Container Image (ACI)

You can create an Application Container Image (ACI) for any package by performing the following steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create an ACI, for example:

    ```
    $ hab pkg install yourorigin/yourpackage
    ```

3. Run the ACI exporter on the package.

    ```
    $ hab pkg export aci yourorigin/yourpackage
    ```

4. Note that this will create unsigned ACI images. If you wish to sign your ACI with default options, pass `SIGN=true`:

    ```
    $ SIGN=true hab pkg export aci yourorigin/yourpackage
    ```

5. The `.aci` can now be moved to any runtime capable of running ACIs (e.g. [rkt](https://coreos.com/rkt/) on CoreOS) for execution.

##Exporting to Apache Mesos and DC/OS 

1. Create an interactive studio in any directory with the `hab studio enter` command.

2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Marathon application, for example: 

    ```
    $ hab pkg install yourorigin/yourpackage 
    ```

3. Run the Mesos exporter on the package. 

    ```
    $ hab pkg export mesos yourorigin/yourpackage 
    ```

4. This will create a Mesos container-format tarball in the results directory, and also print the JSON needed to load the application into Marathon. Note that the tarball needs to be uploaded to a download location and the "uris" in the JSON need to be updated manually. This is an example of the output:

    ```
    { "id": "yourorigin/yourpackage", "cmd": "/bin/id -u hab &>/dev/null || /sbin/useradd hab; /bin/chown -R hab:hab *;
    mount -t proc proc proc/; mount -t sysfs sys sys/;mount -o bind /dev dev/; /usr/sbin/chroot . ./init.sh start
    yourorigin/yourpackage", "cpus": 0.5, "disk": 0, "mem": 256, "instances": 1, "uris":
    ["https://storage.googleapis.com/mesos-habitat/yourorigin/yourpackage-0.0.1-20160611121519.tgz" ] }
    ```

5. Note that the default resource allocation for the application is very small: 0.5 units of CPU, no disk, one instance, and 256MB of memory. To change these resource allocations, pass different values to the Mesos exporter as command line options (defaults are documented with `--help`).

6. See the article [Apaches Mesos and DC/OS](/docs/container-orchestration-mesos) for more information on getting your application running on Mesos. 

<hr>
<ul class="main-content--link-nav">
  <li>Continue to the next topic</li>
  <li><a href="/docs/run-packages-monitoring">Monitoring</a></li>
</ul>
