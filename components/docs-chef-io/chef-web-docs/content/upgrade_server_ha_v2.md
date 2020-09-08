+++
title = "High Availability: Upgrade to Chef Backend 2"
draft = false

aliases = ["/upgrade_server_ha_v2.html"]

[menu]
  [menu.infra]
    title = "Upgrade HA Cluster"
    identifier = "chef_infra/managing_chef_infra_server/upgrade_server_ha_v2.md Upgrade HA Cluster"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 130
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/upgrade_server_ha_v2.md)

This topic describes the process of upgrading a high availability Chef
Infra Server cluster.

## Overview

These instructions cover the process of upgrading a Chef Backend
cluster. Please refer to the appropriate directions for the version of
Chef Backend that you are using and the version that you intend to
upgrade to:

-   [Update Chef Backend](#update-chef-backend)
-   [Chef Backend 1.x to 2.x Upgrade](#chef-backend-1.x-to-2.x-upgrade)
    (downtime upgrade)
-   [DRBD/Keepalived HA to Chef Backend
    2.x](#drbdkeepalived-ha-to-chef-backend-2.x) (migration)

## Update Chef Backend

The minor version update is appropriate for all upgrades of a Chef
Backend cluster within a version. For example, updating from 1.x to 1.x or 2.x to 2.x.

{{< note >}}

The procedure assumes that the new chef-backend package has been copied
to all of the nodes.

{{< /note >}}

### Step 1: Block Failover

We don't want the cluster to fail over to a follower that is in the
process of being updated. So we start by disabling failover

1.  Run `chef-backend-ctl set-cluster-failover off`

### Step 2: Update the followers

Followers should be updated sequentially. Upgrading them simultaneously
is not supported and may result in data loss. Verify the successful
rejoin after each upgrade.

1.  Install the new chef-backend package

    RHEL and CentOS:

    ``` bash
    yum install PATH_TO_FILE.rpm
    ```

    Debian and Ubuntu:

    ``` bash
    dpkg -i PATH_TO_FILE.deb
    ```

    You may also want to look at the chef-ingredient cookbook to
    automate downloading and installing the latest package.

2.  Run the upgrade command

    ``` bash
    chef-backend-ctl upgrade
    ```

The update command will make any changes necessary to start the new
service and verify that the updated node has rejoined the cluster.

Repeat the previous steps in this section for each remaining follower.

### Step 3: Update the leader

Unblock failover, trigger failover, block it again.

``` bash
% chef-backend-ctl set-cluster-failover on
% chef-backend-ctl upgrade --failover
% chef-backend-ctl set-cluster-failover off
```

### Step 4: Re-enable failover

Allow failover again:

``` bash
% chef-backend-ctl set-cluster-failover on
```

### Step 5: Verify the cluster is stable

Check the status of the cluster:

``` bash
chef-backend-ctl status
```

## Chef Backend 1.x to 2.x Upgrade

{{< warning >}}

Upgrading from Chef Backend 1.x to Chef Backend 2.x requires full
cluster downtime.

{{< /warning >}}

1.  Identify the node with the **leader** role using the
    `chef-backend-ctl cluster-status` command:

    ``` none
    Name       IP              GUID                              Role      PG        ES
    backend-1  192.168.33.215  dc0c6ea77a751f94037cd950e8451fa3  leader    leader    not_master
    backend-2  192.168.33.216  008782c59d3628b6bb7f43556ac0c66c  follower  follower  not_master
    backend-3  192.168.33.217  1af654172b1830927a571d9a5ba7965b  follower  follower  master
    ```

    In this example, `backend-1` is the **leader** node, as indicated by
    its role in the **Role** column.

2.  Install the new Chef Backend package on all nodes in the cluster:

    RHEL and CentOS:

    ``` bash
    yum install PATH_TO_FILE.rpm
    ```

    Debian and Ubuntu:

    ``` bash
    dpkg -i PATH_TO_FILE.deb
    ```

3.  On the leader, run the following command to take the node down for
    the upgrade:

    ``` bash
    chef-backend-ctl down-for-upgrade
    ```

4.  Then issue the same command on the follower nodes:

    ``` bash
    chef-backend-ctl down-for-upgrade
    ```

5.  Initiate the upgrade on the follower nodes first:

    ``` bash
    chef-backend-ctl upgrade
    ```

6.  Then initiate the upgrade on the leader node:

    ``` bash
    chef-backend-ctl upgrade
    ```

7.  On any Chef Infra Server frontend nodes using the Chef Backend
    cluster upgraded in the previous steps, run:

    ``` bash
    chef-server-ctl reconfigure
    ```

8.  To continue the upgrades on Chef Infra Server frontend nodes using
    this backend cluster, see [Upgrade Frontends Associated with a Chef
    Backend
    Cluster](/install_server_ha/#upgrading-chef-infra-server-on-the-frontend-machines)

## DRBD/Keepalived HA to Chef Backend 2.x

DRBD configurations are no longer supported. See [End of Life
Products](/versions/#end-of-life-eol)

For a guide to migrating to Chef Backend from DRBD see the [Best Best
Practices for Migrating Your Chef
Server](https://blog.chef.io/2018/04/06/best-practices-for-migrating-your-chef-server/)
webinar from the [Chef Blog](https://blog.chef.io/)
