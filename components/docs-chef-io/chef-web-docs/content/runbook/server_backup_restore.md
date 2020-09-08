+++
title = "Backup and Restore a Standalone or Frontend install"
draft = false

aliases = ["/server_backup_restore.html"]

runbook_weight = 70

[menu]
  [menu.infra]
    title = "Backup & Restore"
    identifier = "chef_infra/managing_chef_infra_server/server_backup_restore.md Backup & Restore"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/runbook/server_backup_restore.md)

Periodic backups of Chef Infra Server data are an essential part of
managing and maintaining a healthy configuration and ensuring that
important data can be restored, if required.

## chef-server-ctl

For the majority of use cases, `chef-server-ctl backup` is the
recommended way to take backups of the Chef Infra Server. Use the
following commands for managing backups of Chef Infra Server data, and
for restoring those backups.

### backup

{{% ctl_chef_server_backup %}}

**Options**

{{% ctl_chef_server_backup_options %}}

**Syntax**

{{% ctl_chef_server_backup_syntax %}}

### restore

{{% ctl_chef_server_restore %}}

**Options**

{{% ctl_chef_server_restore_options %}}

**Syntax**

{{% ctl_chef_server_restore_syntax %}}

**Examples**

``` bash
chef-server-ctl restore /path/to/tar/archive.tar.gz
```

## Backup and restore a Chef Backend install

In a disaster recovery scenario, the backup and restore processes allow
you to restore a data backup into a newly built cluster. It is not
intended for the recovery of an individual machine in the chef-backend
cluster or for a point-in-time rollback of an existing cluster.

### Backup

Restoring your data in the case of an emergency depends on having
previously made backups of:

-   the data in your Chef Backend cluster
-   the configuration from your Chef server

To make backups for future use in disaster scenarios:

1.  On a follower chef-backend node, run `chef-backend-ctl backup`
2.  On a Chef Infra Server node run: `chef-server-ctl backup --config-only`
3.  Move the tar archives created in steps (1) and (2) to a long-term
    storage location.

### Restore

To restore a Chef Backend-based Chef Infra Server cluster:

1.  Restore the node and an IP address that can be used to reach the
    node on the first machine that you want to use in your new Chef
    Backend cluster. The argument to the `--publish_address` option
    should be the IP address for reaching the node you are restoring.

    ``` bash
    chef-backend-ctl restore --publish_address X.Y.Z.W /path/to/backup.tar.gz
    ```

2.  Join additional nodes to your Chef Backend cluster. (If you are only
    testing and verifying your restore process you can test against a
    single Chef Backend node and a single Chef Infra Server node.)

    ``` bash
    chef-backend-ctl join-cluster IP_OF_FIRST_NODE --publish_address IP_OF_THIS_NODE
    ```

3.  Restore Chef Infra Server from your backed up Infra Server configuration
    (See step 2 in the backup instructions above). Alternatively, you
    can generate new configuration for this node and reconfigure it
    using the steps found in [the installation
    instructions.](/install_server_ha/#step-5-install-and-configure-first-frontend).

    ``` bash
    chef-server-ctl restore /path/to/chef-server-backup.tar.gz
    ```

4.  Run the `reindex` command to re-populate your search index

    ``` bash
    chef-server-ctl reindex --all
    ```

### Verify

We recommend periodically verifying your backup by restoring a single
Chef Backend node, a single Chef Infra Server node, and ensuring that
various knife commands and Chef Infra Client runs can successfully
complete against your backup.
