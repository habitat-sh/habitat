---
title: Kubernetes with Habitat
---

I think the launch content is habitat into kubectl run, how to use the env and ports
# DC/OS and Mesos

Habitat blah blah
Marathon

## Export to an Apache Mesos (Marathon) application

may be exported to Mesos directly with the    supports packaging applications  Mesos with native Marathon containers

 deploying applications
Mesos can run Docker containers but the native format is a filesystem sandbox with cgroups and
namespaces. You can create these from Habitat packages by following these steps:

1. Create an interactive studio in any directory with the `hab studio enter` command.
2. Install or [build](/docs/create-packages-build) the Habitat package from which you want to create a Marathon application, for example:

       hab pkg install yourorigin/yourpackage

3. Run the Mesos exporter on the package.

       hab pkg export mesos yourorigin/yourpackage

4. This will create a Marathon-format tarball in the results directory, and also print the JSON needed to load the application into Marathon.

5. Note that the default resource allocation for the application is very small: 0.5 units of CPU, no disk, one instance, and 256MB of memory. To change these resource allocations, pass different values to the Mesos exporter as environment variables:

       CPU=1.0 DISK=5 INSTANCES=2 MEM=512 hab pkg export mesos yourorigin/yourpackage


##

hab-pkg-mesosize

  {
   "id": "mattray/mytutorialapp",
   "cmd": "/bin/id -u hab &>/dev/null || /sbin/useradd hab; /bin/chown -R hab:hab *; mount -t proc proc proc/; mount -t sysfs sys sys/;mount -o bind /dev dev/; /usr/sbin/chroot . ./init.sh start mattray/mytutorialapp",
   "cpus": .5,
   "disk": 0,
   "mem": 256,
   "instances": 1,
   "uris": [ "https://storage.googleapis.com/mesos-habitat/mattray-mytutorialapp-0.0.1-20160611121519.tgz" ]
  }

### Discuss code

### Uploading the artifact


### Navigating the UI

### Debugging

On the node


[root@tk-gs1-centos-72-372400 slaves]# pwd
/var/lib/mesos/slave/slaves


## Future enhancements

This is early stages, there are lots of improvements that could be made.
Environment variables
Networking ports auto-exposed
Habitat gossip protocol
Mesos can use docker but the native format is a filesystem sandbox with cgroups and namespaces. hab-pkg-mesosize creates a mostly empty hab-studio and packages it into a tarball. hab-pkg-mesosize provides the JSON syntax for loading the application into Marathon via the UI, documentation to be provided in a separate PR. The tarball still needs an accessible URL, that may be done manually or investigated in a later PR for pushing it into Mesos' artifact store. Networking is not addressed in the current format either.

Native .hart support

Upcoming

https://mesosphere.github.io/marathon/docs/application-basics.html

https://mesosphere.github.io/marathon/docs/artifact-store.html

http://mesos.apache.org/documentation/latest/containerizer/
https://mesos.apache.org/documentation/latest/mesos-containerizer/
