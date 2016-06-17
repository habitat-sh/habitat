---
title: Apache Mesos and DC/OS with Habitat
---

# Apache Mesos and DC/OS

[Apache Mesos](https://mesos.apache.org/) is an open source distributed systems kernel and the distributed systems kernel for [Mesosphere's DC/OS](https://dcos.io) distributed platform.

## Mesos Containerizers

Mesos has support for [containerizers](http://mesos.apache.org/documentation/latest/containerizer/) for running commands and applications within isolated containers. Mesos supports Docker and its own [Mesos containerizer](http://mesos.apache.org/documentation/latest/mesos-containerizer/) format. The Mesos containerizer provides lightweight containerization with cgroups/namespaces isolation without actual isolation. The `hab pkg export mesos` command creates a mostly empty base filesystem with the application and the Habitat supervisor and packages it into a compressed tarball.

## Marathon Applications

[Marathon](https://mesosphere.github.io/marathon/) is a container orchestration platform for Mesos and DC/OS, handling the scheduling and deployment of applications. [Marathon applications](https://mesosphere.github.io/marathon/docs/application-basics.html) support Docker and the Mesos container formats, wrapping them in JSON metadata describing the resources needed to deploy the application. Once the application has been deployed to Marathon, it schedules it across the Mesos cluster and ensures the application is running optimally.

# Export to a Mesos container and Marathon application

You can create native Mesos containers from Habitat packages by following these steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.

2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Marathon application, for example:

       hab pkg install yourorigin/yourpackage

3. Run the Mesos exporter on the package.

       hab pkg export mesos yourorigin/yourpackage

4. This will create a Mesos container-format tarball in the results directory, and also print the JSON needed to load the application into Marathon. Note that the tarball needs to be uploaded to a download location and the `"uris"` in the JSON need to be updated manually. This is an example of the output:

       {
       "id": "yourorigin/yourpackage",
       "cmd": "/bin/id -u hab &>/dev/null || /sbin/useradd hab; /bin/chown -R hab:hab *; mount -t proc proc proc/; mount -t sysfs sys sys/;mount -o bind /dev dev/; /usr/sbin/chroot . ./init.sh start yourorigin/yourpackage",
       "cpus": 0.5,
       "disk": 0,
       "mem": 256,
       "instances": 1,
       "uris": [ "https://storage.googleapis.com/mesos-habitat/yourorigin/yourpackage-0.0.1-20160611121519.tgz" ]
       }

5. Note that the default resource allocation for the application is very small: 0.5 units of CPU, no disk, one instance, and 256MB of memory. To change these resource allocations, pass different values to the Mesos exporter as command line options (defaults are documented with `--help`).

6. From the DC/OS web interface, launch the Marathon Service.

    ![Screen shot of DC/OS Services](/images/mesos1-services-marathon.png)

7. Select "Create Application".

    ![Screen shot of Marathon Applications List](/images/mesos2-new-application.png)

8. Click on the "JSON Mode" selector and enter the JSON output of the Mesos exporter and click "Create Application".

    ![Screen shot of Marathon New Application JSON Mode](/images/mesos3-new-application-json.png)

9. Marathon will then deploy the application and enter the "Running" status.

    ![Screen shot of Marathon Application Running](/images/mesos4-application.png)

## Debugging

You can get to the output from the running application by clicking on the "Marathon" service from the DC/OS "Services" tab. Select the application and the "Log Viewer" and choose either the "Error" or "Output" to see `stderr` and `stdout` respectively. If you have SSH access into the nodes, the Mesos container directories are beneath `/var/lib/mesos/slave/slaves`.
    ![Screen shot of Debugging a Running Application](/images/mesos5-debugging.png)

## Future Enhancements

This is a basic integration, there are many improvements yet to be made. Here are a few examples:

 * Marathon environment variables are not passed into the Habitat package "cmd" yet.
 * Networking ports exposed by Habitat need to be added to the JSON.
 * The Habitat gossip protocol needs to be included as a default exposed port.
 * If Marathon is running the [artifact store](https://mesosphere.github.io/marathon/docs/artifact-store.html), support uploading the tarball directly into it.
 * Upload applications directly to the [Marathon application API](https://mesosphere.github.io/marathon/docs/application-basics.html).
 * Marathon supports unpacking several archive formats. Native `.hart` support could be added directly to Marathon.
