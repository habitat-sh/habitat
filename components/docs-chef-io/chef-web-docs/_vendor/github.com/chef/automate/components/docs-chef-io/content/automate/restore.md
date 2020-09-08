+++
title = "Restore"

date = 2018-03-26T15:27:52-07:00
draft = false
[menu]
  [menu.automate]
    title = "Restore"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/restore.md Restore"
    weight = 80
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/restore.md)

Restore Chef Automate from a [filesystem backup]({{< ref "restore.md#restore-from-a-filesystem-backup" >}}), an [Amazon S3 bucket backup]({{< ref "restore.md#restore-from-an-aws-s3-backup" >}}), or a [Google Cloud Storage (GCS) bucket backup]({{< ref "restore.md#restore-from-a-google-cloud-storage-backup" >}}).

## Prerequisites

1. On the restore host, download and unzip the Chef Automate command-line tool:

   ```shell
        curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64.zip | gunzip - > chef-automate && chmod +x chef-automate
    ```

1. To restore from **filesystem backups**, Chef Automate requires access to a backup directory in the [configured location]({{< ref "backup.md#backup-to-a-filesystem" >}}).
Ensure access for the backup type used:

     1. To restore [a network-attached filesystem backup]({{< ref "backup.md#backup-to-a-filesystem" >}}), mount the shared backup directory to the same mount point configured at the time of the backup.
     1. To restore [a backup directory that is not a network-attached filesystem]({{< ref "backup.md#backup-to-a-filesystem" >}}), copy the backup directory to the configured location at the time of the backup.
     1. To restore a [single-file backup archive]({{< ref "backup.md#store-a-filesystem-backup-in-a-single-file-archive" >}}), copy your archive to the restore host and extract it to the configured backup directory.

1. To restore a backup to a host with a different fully qualified domain name (FQDN) than the original backup host, create a `patch.toml` file that specifies the new FQDN and provide it at restore time:

    ```toml
         [global.v1]
         fqdn = "<new-fqdn>"

         # To provide a cert and key for the restore host, uncomment and fill
         # these sections.
         # [[global.v1.frontend_tls]]
         # The TLS certificate for the load balancer frontend.
         # cert = """-----BEGIN CERTIFICATE-----
         # <certificate-for-new-fqdn>
         # -----END CERTIFICATE-----
         # """

         # The TLS RSA key for the load balancer frontend.
         # key = """-----BEGIN RSA PRIVATE KEY-----
         # <key-for-new-fqdn>
         # -----END RSA PRIVATE KEY-----
         # """
    ```

## Restore From a Filesystem Backup

Meet the required [prerequisites]({{< ref "restore.md#prerequisites" >}}) before beginning your restore process.

### Restore in an Internet-Connected Environment

If you have [configured the backup directory]({{< relref "backup.md#backup-to-a-filesystem" >}}) to a directory other than the default directory (`/var/opt/chef-automate/backups`), you must supply the backup directory.
Without a backup ID, Chef Automate uses the most recent backup in the backup directory.

To restore on a new host, run:

```shell
chef-automate backup restore </path/to/backups/>BACKUP_ID
```

To restore on an existing Chef Automate host, run:

```shell
chef-automate backup restore </path/to/backups/>BACKUP_ID --skip-preflight
```

Use the `--patch-config` option with a [configuration patch file]({{< relref "backup.md#prerequisites" >}}) to restore to a host with a different FQDN than that of the backup host:

```shell
chef-automate backup restore </path/to/backups/>BACKUP_ID --patch-config </path/to/patch.toml> --skip-preflight
```

Restores from a filesystem backup may fail with incorrect directory permissions.
Run the [`fix-repo-permissions` command]({{< ref "cli_chef_automate/#chef-automate-backup-fix-repo-permissions" >}}) to address such issues:

```shell
sudo chef-automate backup fix-repo-permissions <path>
```

### Restore in an Airgapped Environment

To restore a backup of an [airgapped installation]({{< relref "airgapped_installation.md" >}}), you must specify the [Airgap Installation Bundle]({{< relref "airgapped_installation.md#create-an-airgap-installation-bundle" >}}) used by the installation.
If you have [configured the backup directory]({{< relref "backup.md#backup-to-a-filesystem" >}}) to a directory that is not the default `/var/opt/chef-automate/backups`, then you must supply the backup directory.
If you do not provide a backup ID, Chef Automate uses the most recent backup in the backup directory.

To restore on a new host, run:

```shell
chef-automate backup restore --airgap-bundle </path/to/bundle> </path/to/backups/>BACKUP_ID
```

To restore on an existing Chef Automate host, run:

```shell
chef-automate backup restore --airgap-bundle </path/to/bundle> </path/to/backups/>BACKUP_ID --skip-preflight
```

To restore using AWS S3 on an existing Chef Automate host, run:

```shell
chef-automate backup restore --airgap-bundle </path/to/bundle> s3://bucket_name/</path/to/backups/>BACKUP_ID --skip-preflight
```

To restore using Google Cloud Storage (GCS) on an existing Chef Automate host, run:

```shell
chef-automate backup restore --airgap-bundle </path/to/bundle> gs://bucket_name/</path/to/backups/>BACKUP_ID --skip-preflight
```

Use the `--patch-config` option with a [configuration patch file]({{< relref "backup.md#prerequisites" >}}) to restore to a host with a different FQDN than that of the backup host.

Restores from a filesystem backup may fail with incorrect directory permissions.
Run the [`fix-repo-permissions` command]({{< ref "cli_chef_automate/#chef-automate-backup-fix-repo-permissions" >}}) to address such issues:

```shell
sudo chef-automate backup fix-repo-permissions <path>
```

## Restore From an AWS S3 Backup

Meet the required [prerequisites]({{< ref "restore.md#prerequisites" >}}) before beginning your restore process.

See how to [back up to AWS S3]({{< ref "backup/#backup-to-aws-s3" >}}).

To restore from an AWS S3 bucket backup on a new host, run:

```shell
chef-automate backup restore s3://bucket_name/path/to/backups/BACKUP_ID
```

To restore from an AWS S3 bucket backup on an existing Chef Automate host, run:

```shell
chef-automate backup restore s3://bucket_name/path/to/backups/BACKUP_ID --skip-preflight
```

Use the `--patch-config` option with a [configuration patch file]({{< relref "backup.md#prerequisites" >}}) to restore to a host with a different FQDN than that of the backup host:

```shell
chef-automate backup restore s3://bucket_name/path/to/backups/BACKUP_ID --patch-config </path/to/patch.toml> --skip-preflight
```

A successful restore shows the timestamp of the backup used at the end of the status output:

```shell
Success: Restored backup 20180517223558
```

## Restore From a Google Cloud Storage Backup

Meet the required [prerequisites]({{< ref "restore.md#prerequisites" >}}) before beginning your restore process.

See how to [back up to GCS]({{< ref "backup/#backup-to-gcs" >}}).

To restore from a Google Cloud Storage (GCS) bucket backup on a new host, run:

```shell
chef-automate backup restore gs://bucket_name/path/to/backups/BACKUP_ID
```

To restore from a Google Cloud Storage (GCS) bucket backup on an existing Chef Automate host, run:

```shell
chef-automate backup restore gs://bucket_name/path/to/backups/BACKUP_ID --skip-preflight
```

Use the `--patch-config` option with a [configuration patch file]({{< relref "backup.md#prerequisites" >}}) to restore to a host with a different FQDN than that of the backup host:

```shell
chef-automate backup restore gs://bucket_name/path/to/backups/BACKUP_ID --patch-config </path/to/patch.toml> --skip-preflight
```

A successful restore shows the timestamp of the backup used at the end of the status output:

```shell
Success: Restored backup 20180517223558
```

## Troubleshooting

Set the log level to `debug` before re-running a failed restore to output debug info to the Chef Automate log:

```shell
chef-automate debug set-log-level deployment-service debug
```

## References

See the [`chef-automate backup restore` command reference]({{< ref "cli_chef_automate/#chef-automate-backup-restore" >}}).
