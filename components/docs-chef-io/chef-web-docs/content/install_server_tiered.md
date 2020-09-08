+++
title = "Tiered Installation"
draft = false

aliases = ["/install_server_tiered.html"]

[menu]
  [menu.infra]
    title = "Tiered Installation"
    identifier = "chef_infra/setup/chef_infra_server/install_server_tiered.md Tiered Installation"
    parent = "chef_infra/setup/chef_infra_server"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_server_tiered.md)

This topic describes how to set up the Chef Infra Server with a single
back end and multiple load-balanced frontend servers.

![image](/images/chef_server_tiered.png)

## Prerequisites

Before installing the Chef Infra Server software, perform the following
steps:

-   The backend server must be accessible from each frontend server. A
    virtual IP address is created and managed by the Chef Infra Server,
    but will also need to be added to the DNS so that all machines in
    the tiered configuration may access it.
-   Persistent data on the backend Chef Infra Server is primarily
    composed of cookbook files and directories. Separate disks should be
    dedicated entirely to storing this data prior to installing the Chef
    Infra Server.
-   Load-balancing should be used with frontend servers, along with a
    DNS entry for the virtual IP address used for load balancing. This
    virtual IP address is added to the chef-server.rb file as the
    `api_fqdn`.
-   All required ports must be open. See the Firewalls section (below)
    for the list of ports. All connections to and from the Chef Infra
    Server are accomplished via TCP. Refer to the operating system's
    manual or your systems administrators for instructions on how to
    configure to ports, if necessary.
-   The hostname for the Chef Infra Server must be an FQDN, including
    the domain suffix, and must be resolvable by the backend and
    frontend servers. See [Hostnames,
    FQDNs](/install_server_pre/#hostnames) for more information.
-   `chef-server-ctl reconfigure` will not bind the `backend_vip` to the
    backend server. The easiest thing to do is just define `backend_vip`
    as the already configured main IP address of the backend system. If
    you need to use an additional address, it will need to be configured
    and bound on the system before `chef-server-ctl reconfigure` is run.

## Basic Hardware Requirements

For a tiered deployment, your backend server should support the
following hardware requirements:

-   64-bit architecture
-   8 total cores (physical or virtual)
-   16GB RAM
-   Fast, redundant storage (SSD/RAID-based solution)
    -   50 GB/backend server (SSD if on premises, Premium Storage in
        Microsoft Azure, EBS-Optimized GP2 in AWS)
-   1 GigE NIC interface
-   A back-end server; all other systems will be front-end servers.

## Disk Configuration

Persistent data on the backend server of the Chef Infra Server is
primarily composed of cookbook files and directories. Separate disks
should be dedicated entirely to storing this data prior to installing
the Chef Infra Server. These disks should be part of a SSD or hardware
RAID-based solution that ensure redundancy and high IOPS. This
configuration guide assumes that:

-   \~50GB of raw, unpartitioned disk space is available. Disk space
    should scale up with the number of nodes that the backend server is
    managing. A good rule to follow is to allocate 2 MB per node.
-   The disk space presents as a single device. For example: `/dev/sdb`.
-   The storage is added to a volume group named `opscode` and is
    presented to the Chef Infra Server by mounting on `/var/opt/opscode`
    before a reconfiguration

The following commands properly set up disk configuration on the backend
server:

``` bash
pvcreate /dev/sdb
```

and:

``` bash
vgcreate opscode /dev/sdb
```

and:

``` bash
lvcreate -l 80%VG -n tiered opscode
```

### Mount Storage Device

To build and mount the storage device on the backend server, do the
following:

1.  Create the file system. For example, an `ext4` type named `tiered`:

    ``` bash
    mkfs.ext4 /dev/opscode/tiered
    ```

    then:

    ``` bash
    mkdir -p /var/opt/opscode
    ```

    and then:

    ``` bash
    mount /dev/opscode/tiered /var/opt/opscode
    ```

## Backend

Use the following steps to set up the backend Chef Infra Server:

1.  Download the packages from <https://downloads.chef.io/chef-server/>.
    For Red Hat and CentOS 6:

    ``` bash
    rpm -Uvh /tmp/chef-server-core-<version>.rpm
    ```

    For Ubuntu:

    ``` bash
    dpkg -i /tmp/chef-server-core-<version>.deb
    ```

    After a few minutes, the Chef Infra Server will be installed.

2.  Create a file named chef-server.rb that is located in the
    `/etc/opscode/` directory. See the chef-server.rb section below for
    an example of the settings and values that are required.

## chef-server.rb

The chef-server.rb file that is located in the `/etc/opscode/` directory
describes the topology of the tiered configuration. On the backend
server, create a file named chef-server.rb and save it in the
`/etc/opscode/` directory.

Add the following settings to the chef-server.rb file:

1.  Define the topology type:

    ``` ruby
    topology "tier"
    ```

2.  Define the backend server:

    ``` ruby
    server "FQDN",
      :ipaddress => "IP_ADDRESS",
      :role => "backend",
      :bootstrap => true
    ```

    Replace `FQDN` with the FQDN of the server and `IP_ADDRESS` with the
    IP address of the server. The role is a backend server is
    `"backend"`.

3.  Define the backend virtual IP address:

    ``` ruby
    backend_vip "FQDN",
      :ipaddress => "IP_ADDRESS",
      :device => "eth0"
    ```

    Replace `FQDN` with the FQDN of the server. Replace `IP_ADDRESS`
    with the virtual IP address of the server. The `:device` parameter
    should be the ethernet interface to which the virtual IP address
    will bind. This is typically the public interface of the server. In
    a typical tiered install, the config here could also be just the
    main FQDN and IP address that are already configured for the
    backend. Running `chef-server-ctl reconfigure` will not bind the
    `backend_vip` address to an interface, this must be done on startup
    of the machine.

4.  Define each frontend server:

    ``` ruby
    server "FQDN",
      :ipaddress => "IP_ADDRESS",
      :role => "frontend"
    ```

    Replace `FQDN` with the FQDN of the frontend server. Replace
    `IP_ADDRESS` with the IP address of the frontend server. Set `:role`
    to `"frontend"`.

    Add separate entry in the chef-server.rb file for each frontend
    server.

5.  Define the API FQDN:

    ``` ruby
    api_fqdn "FQDN"
    ```

    Replace `FQDN` with the FQDN of the load balanced virtual IP
    address, which should be equal to the FQDN for the service URI that
    is used by the Chef Infra Server.

6.  {{% install_chef_server_reconfigure %}}

## Frontend

For each frontend server, use the following steps to set up the Chef
Infra Server:

1.  Install the Chef Infra Server package. For Red Hat and CentOS 6:

    ``` bash
    rpm -Uvh /tmp/chef-server-core-<version>.rpm
    ```

    For Ubuntu:

    ``` bash
    dpkg -i /tmp/chef-server-core-<version>.deb
    ```

    After a few minutes, the Chef Infra Server will be installed.

2.  Create the `/etc/opscode/` directory, and then copy the entire
    contents of the `/etc/opscode` directory from the primary backend
    server, including all certificates and the chef-server.rb file.

3.  {{% install_chef_server_reconfigure %}}

4.  Start the Chef Infra Server:

    ``` bash
    chef-server-ctl start
    ```

On a single frontend server, create an administrator and an
organization:

1.  {{% ctl_chef_server_user_create_admin %}}
2.  {{% ctl_chef_server_org_create_summary %}}

## Enable Features

Enable additional features of the Chef Infra Server! The packages may be
downloaded directly as part of the installation process or they may be
first downloaded to a local directory, and then installed.

**Use Downloads**

The `install` subcommand downloads packages from
<https://packages.chef.io/> by default. For systems that are not behind
a firewall (and have connectivity to <https://packages.chef.io/>), the
Chef management console package can be installed as described below:

Chef Manage

:   Use Chef management console to manage data bags, attributes,
    run-lists, roles, environments, and cookbooks from a web user
    interface.

    On each front end server in the Chef Infra Server configuration,
    run:

    ``` bash
    chef-server-ctl install chef-manage
    ```

    then:

    ``` bash
    chef-server-ctl reconfigure
    ```

    and then:

    ``` bash
    chef-manage-ctl reconfigure
    ```

    To accept the [Chef MLSA](/chef_license/):

    ``` bash
    sudo chef-manage-ctl reconfigure --accept-license
    ```

    This updates the Chef Infra Server and creates the
    `/etc/opscode-manage/secrets.rb` file. When running the Chef
    management console 1.11 (or higher), copy the `secrets.rb` file in
    the `/etc/opscode-manage` directory on one of the frontend servers
    to the same directory on each of the other frontend servers, and
    then rerun `chef-manage-ctl reconfigure` so the copied
    `/etc/opscode-manage/secrets.rb` file gets used correctly.

**Use Local Packages**

{{% ctl_chef_server_install_features_manual %}}

## Reference

The following sections show an example chef-server.rb file and a list of
the ports that are required by the Chef Infra Server.

### chef-server.rb

A completed chef-server.rb configuration file for a four server tiered
Chef Infra Server configuration, consisting of:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 37%" />
<col style="width: 37%" />
</colgroup>
<thead>
<tr class="header">
<th>FQDN</th>
<th>Real IP Address</th>
<th>Role</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>be1.example.com</td>
<td>192.0.2.0</td>
<td>backend</td>
</tr>
<tr class="even">
<td>fe1.example.com</td>
<td>192.168.4.2</td>
<td>frontend</td>
</tr>
<tr class="odd">
<td>fe2.example.com</td>
<td>192.168.4.3</td>
<td>frontend</td>
</tr>
<tr class="even">
<td>fe3.example.com</td>
<td>192.168.4.4</td>
<td>frontend</td>
</tr>
<tr class="odd">
<td>chef.example.com</td>
<td></td>
<td>load balanced frontend VIP</td>
</tr>
<tr class="even">
<td>be.example.com</td>
<td>192.168.4.7</td>
<td>load balanced backend VIP</td>
</tr>
</tbody>
</table>

Looks like this:

``` ruby
topology "tier"

server "be1.example.com",
  :ipaddress => "192.0.2.0",
  :role => "backend",
  :bootstrap => true

backend_vip "be.example.com",
  :ipaddress => "192.168.4.7",
  :device => "eth0"

server "fe1.example.com",
  :ipaddress => "192.168.4.2",
  :role => "frontend"

server "fe2.example.com",
  :ipaddress => "192.168.4.3",
  :role => "frontend"

server "fe3.example.com",
  :ipaddress => "192.168.4.4",
  :role => "frontend"

api_fqdn "chef.example.com"
```

### Firewalls

{{% server_firewalls_and_ports_summary %}}

{{% server_firewalls_and_ports_listening %}}

{{% server_firewalls_and_ports_loopback %}}

#### Backend

{{% server_firewalls_and_ports_tiered %}}

#### Frontend

{{% server_firewalls_and_ports_fe %}}
