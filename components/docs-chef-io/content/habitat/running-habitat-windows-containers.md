+++
title = "Running Chef Habitat Windows Containers"
description = "Running Chef Habitat Windows Containers"

[menu]
  [menu.habitat]
    title = "Running Chef Habitat Windows Containers"
    identifier = "habitat/containers/running-habitat-windows-containers Windows Containers"
    parent = "habitat/containers"

+++

## Container Base Image

Exported Windows images use `microsoft/windowsservercore` as their base. This is the equivalent of a minimal Windows Server 2016 Core install. So you should not expect non default features and roles to be enabled such as IIS or Active Directory. Consider using an `init` hook to install any features needed by your Chef Habitat service.

## Container Pull and Startup Time

The `microsoft/windowsservercore` image is approximately 5GB. Due to this large size, you can expect that the first time you run an exported Chef Habitat service, pulling down the image may take several minutes. This wait should only occur on the very first `docker run` of any Chef Habitat Windows service. Additionally, depending on the Windows host operating system, running the container may also take considerably longer than what one is accustomed to with Linux based containers. This startup time will be highly influenced by the container isolation mode described below.

## Windows Containers and Host Kernel Isolation

There are two types of Windows containers and each runs under a different level of kernel isolation.

### Windows Server Containers

These containers, like their Linux counterparts, share the host's kernel. You can expect these containers to start pretty quickly and this is the default container type on Windows Server 2016 hosts.

### Hyper-V Containers

Windows Hyper-V containers run inside of a very minimal Hyper-V VM. As a result, they do not share the host's kernel and offer a higher level of security and isolation. The cost of this isolation is that it will take longer for the container to start - perhaps a noticeable delay. Also be aware that the VM is provisioned with a default 1 GB limit of memory. If your service requires more than a gigabyte of memory, you can use the `--memory` argument with `docker run` and pass a larger limit.

```bash
docker run --memory 2GB -it core/mysql
```

On a Windows 10 host, Windows containers will always run inside of Hyper-V isolation. Kernel sharing Windows Server containers are only available on Windows Server 2016 hosts. On Windows Server 2016, Windows Server containers are the default container type but you can force `docker run` to use Hyper-V containers by setting the `--isolation` argument to `hyperv`.

```bash
docker run --isolation hyperv -it core/mysql
```

## Host Loopback Network

A common container pattern is to forward the container port to a local port and then access the container application by accessing `localhost` on the forwarded port. With Windows containers, published ports cannot be accessed using `localhost`. You will instead need to use the IP address of the host or the IP of the individual container to access the application's endpoint.
