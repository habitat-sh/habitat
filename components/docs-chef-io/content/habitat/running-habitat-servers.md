+++
title = "Running Chef Habitat on Servers (Linux and Windows)"
description = "Running Chef Habitat on Servers (Linux and Windows)"

[menu]
  [menu.habitat]
    title = "Running Chef Habitat on Servers (Linux and Windows)"
    identifier = "habitat/reference/running-habitat-servers"
    parent = "habitat/reference"

+++

Chef Habitat can be run on bare metal servers, as well as virtual machines. Currently, Chef Habitat can run on Linux and Windows platforms, and in all cases, running a Supervisor boils down to running `hab sup run`. How that happens depends on which platform you choose to use.

## Running Chef Habitat on Linux

First, you must [install Chef Habitat](docs/install-habitat) itself on the machine.

Second, many packages default to running as the `hab` user, so you should ensure that both a `hab` user and group exist:

```bash
sudo groupadd hab
sudo useradd -g hab hab
```

Finally, you will need to wire Chef Habitat up to your systems init system. This may be SysVinit, SystemD, runit, etc. The details will be different for each system, but in the end, you must call `hab sup run`.

### Running under SystemD

A basic SystemD unit file for Chef Habitat might look like this. This assumes that you have already created the `hab` user and group, as instructed above, and that your `hab` binary is linked to `/bin/hab`.

    [Unit]
    Description=The Chef Habitat Supervisor

    [Service]
    ExecStart=/bin/hab sup run

    [Install]
    WantedBy=default.target

Depending on your needs and deployment, you will want to modify the options passed to `hab sup run`. In particular, if you wish to participate in larger Supervisor networks, you will need to pass at least one `--peer` option.

## Running Chef Habitat on Windows

As with Linux, you must first [install Chef Habitat](docs/install-habitat) on the machine. Unlike Linux, however, the Windows Supervisor has no requirements for any `hab` user.

On Windows, you can run the Supervisor as a Windows Service. You can use the `windows-service` Chef Habitat package to host the Supervisor inside the Windows Service Control Manager:

```bash
PS C:\\> hab pkg install core/windows-service
```
