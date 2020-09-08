+++
title = "Chef Automate Backups"
draft = false
robots = "noindex"


aliases = ["/delivery_server_backup.html"]

[menu]
  [menu.legacy]
    title = "Delivery Server Backup"
    identifier = "legacy/workflow/managing_workflow/delivery_server_backup.md Delivery Server Backup"
    parent = "legacy/workflow/managing_workflow"
    weight = 150
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/delivery_server_backup.md)

Chef Automate provides tools for creating, managing and restoring backup
archives and Elasticsearch snapshots of your Chef Automate data.

`automate-ctl create-backup` will create a compressed backup archives of
the PostgreSQL database, configuration files, user keys, license file,
git repository data, Chef Compliance Server profiles and RabbitMQ queues. It
also utilizes the snapshot capability of Elasticsearch to create
incremental snapshots of your Chef Automate Elasticsearch indexes.
Paired together, backup archives and Elasticsearch snapshots make it
possible to take complete backups of a Chef Automate cluster without
disrupting service.

`automate-ctl list-backups` will list existing backup archives and
snapshots in either human or machine readable format.

`automate-ctl delete-backups` will delete specific backups or snapshots.
It's also capable of taking backup and snapshot limit parameters to
prune the backups to specified limits.

`automate-ctl restore-backup` will perform full or partial restorations
of a backup archive or elasticsearch snapshot.

## Configuration

By default the Chef Automate cluster is configured to store
near-complete backup archives and snapshots on the local filesystem.
When you create backups they will include all Chef Automate data and
configuration except for the RabbitMQ queues. This was determined to be
a safe choice as the RabbitMQ queues are commonly quite small and
require taking the Chef Automate cluster offline in order to back them
up. As they are not required to restore a functional Chef Automate
cluster the services disruption is rarely worth the value of the
RabbitMQ queues.

All backup commands can be configured by changing the default setting in
`/etc/delivery/delivery.rb`. Several configuration options can also be
set at runtime by using the appropriate command line switch to pass the
configuration option. Configuration options passed via command line
flags will always supersede any default configuration.

The Chef Automate [optional
settings](/config_rb_delivery_optional_settings/#backup) page
contains a full reference of all backup configuration options that are
available.

### Local Backups

Local storage mode is the default configuration for both backup archives
and snapshots. Backups are created and exported into the
`/var/opt/delivery/backups` and
`/var/opt/delivery/elasticsearch_backups` directories. You can configure
the storage locations by setting the `backup['location']` and
`backup['elasticsearch']['location']` options in `delivery.rb`.

When using local backups it is advised to mount a remote backup storage
device to the aforementioned locations.

The staging directory is a local directory that will be used for
temporarily storing the backup archive, database dump, and configuration
data during the backup procedure. When left unconfigured, the Ruby
temporary directory will be used. The Ruby temporary directory is
usually nested in `/tmp` on Linux systems, but the value of the `TMPDIR`
environment variable will also be honored. You can configure the staging
directory by using the `backup['staging_dir']` setting in `delivery.rb`.

{{< note >}}

The backup create will clear any existing files in the staging directory
at the beginning of the backup procedure. Only use a directory that does
not contain any other system data.

{{< /note >}}

### S3 Backups

Using Amazon Web Services (AWS) S3 as a storage location for both Chef
Automate backup archives and the Elasticsearch snapshot repository is
natively supported. In this mode the backup archives and snapshots will
be uploaded to the bucket of your choice.

To enable this functionality, first configure the machine with access to
the desired S3 bucket using either an instance profile with a valid S3
policy or a standard [shared credentials
file](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-getting-started.html#cli-config-files)
located at `/root/.aws/credentials`.

Below is an example Amazon Web Services (AWS) instance profile policy
with the required permissions to create an S3 bucket called
`example-backups`. A policy with these permissions is sufficient for the
backup commands to function as expected.

``` json
{
  "Statement": [
    {
      "Action": [
        "s3:CreateBucket",
        "s3:ListBucket",
        "s3:GetBucketLocation",
        "s3:ListBucketMultipartUploads",
        "s3:ListBucketVersions"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::example-backups"
      ]
    },
    {
      "Action": [
        "s3:GetObject",
        "s3:PutObject",
        "s3:DeleteObject",
        "s3:AbortMultipartUpload",
        "s3:ListMultipartUploadParts"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::example-backups/*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
```

Next, configure Chef Automate to use S3 for both the backups and
snapshots. For example:

``` ruby
backup['bucket']                    = 'example-backups'
backup['region']                    = 'us-west-2'
backup['type']                      = 's3'
backup['elasticsearch']['bucket']   = 'example-backups'
backup['elasticsearch']['region']   = 'us-west-2'
backup['elasticsearch']['type']     = 's3'
```

`automate-ctl reconfigure`

{{< note >}}

Using the same bucket for backup archives and snapshots is supported but
both must be configured independently.

{{< /note >}}

SSE-S3 AES256 Server side encryption is supported and enabled by default
for both backup archives and snapshots. Backup archives can also be
encrypted with SSE-KMS or SSE-C, though snapshots are currently limited
to SSE-S3.

{{< note >}}

While the backup utility currently supports encrypting backups with with
SSE-S3, SSE-KMS, and SSE-C, only SSE-S3 is currently supported for
restoration.

{{< /note >}}

See below for valid examples of `delivery.rb` configurations for server
side encryption.

``` ruby
# Elasticsearch snapshot SSE-S3 AES256
backup['elasticsearch']['server_side_encryption'] = true # default
backup['elasticsearch']['server_side_encryption'] = false

# Backup archive SSE-S3 AES256
backup['server_side_encryption'] = 'AES256' # default

# Backup archive SSE-KMS
backup['server_side_encryption'] = 'aws:kms'
backup['ssekms_key_id'] = 'XXXX'

# Backup archive SSE-C
backup["sse_customer_algorithm"] = "AES256"
backup["sse_customer_key"] = "XXXX"
backup["sse_customer_key_md5"] = "XXXX"
```

### Backup Cron

To enable a backup cron job that will create new backups and prune older
backups and snapshots, configure the following settings in
`delivery.rb`:

``` ruby
backup['cron']['enabled']       = true
backup['cron']['max_archives']  = 7
backup['cron']['max_snapshots'] = 7
backup['cron']['notation']      = "0 0 0/1 1/1 * ? * "
```

If omitted, the default `max_archives`, `max_snapshots`, and `notation`
settings will create daily backups and keep the most recent seven. Any
standard cron notation is supported. If you wish to keep all backups or
snapshots you can set both `max_snapshots` and/or `max_archives` options
to `nil`.

## Create Backups

{{% automate_ctl_create_backup %}}

## List Backups

The [list-backups](/ctl_automate_server/#list-backups) command is
used to list Chef Automate backup archives and Elasticsearch snapshots
in either human or machine readable outputs.

## Delete Backups

The [delete-backups](/ctl_automate_server/#delete-backups) command
is used to delete Chef Automate backup archives and Elasticsearch
snapshots. The command matches a given regular expression and prompts
the user to confirm deletion of each matched backup or snapshot. It can
also be passed maximum archive and snapshot limits and prune the backup
repositories to conform to those limits.

## Restore Backups

The [restore-backup](/ctl_automate_server/#restore-backup) command
is used to fully or partially restore a Chef Automate cluster from
backup archives and/or Elasticsearch snapshots.

{{< note >}}

Backups created with the older `automate-ctl backup-data` command are
not supported with this command. If you wish to restore an older backup
please install the version of Chef Automate that took the backup and use
`automate-ctl restore-data`

{{< /note >}}

### Local Backups

Follow the process below for an example of restoring a Chef Automate
cluster from a local backup archive and a shared filesystem
Elasticsearch snapshot:

1.  Copy the Chef Automate backup archive to a directory that is large
    enough to expand the the archive, e.g.:

    `scp user@backup-server:2016-10-14-08-38-55-chef-automate-backup.zst /mnt/ephemeral/`

2.  Install the same version of Chef Automate that was used to take the
    backup. If the versions do not match you be prompted with a
    compatibility warning but can still proceed with the restore if you
    choose to do so.

    `dpkg -i delivery.rpm`

3.  Mount the Elasticsearch shared filesystem to the same mount point.

    `mount backup-server:/export/chef-automate/elasticsearch_backups /var/opt/delivery/elasticsearch_backups`

4.  Restore the backup archive and snapshot:

    `automate-ctl restore-backup /mnt/ephemeral/2016-10-14-08-38-55-chef-automate-backup.zst 2016-10-14-08-38-55-chef-automate-backup --staging-dir /mnt/ephemeral/restore`

{{< note >}}

Specifying a staging directory is not mandatory but when given it will
clear **all** existing data from it.

{{< /note >}}

### S3 Backups

Follow the process below for an example of restoring a Chef Automate
cluster from a backup archive and Elasticsearch snapshot in Amazon Web
Services (AWS) S3:

1.  Install the same version of Chef Automate that was used to take the
    backup. If the versions do not match you can still proceed with the
    restore but we cannot guarantee compatibility.

    `dpkg -i delivery.rpm`

2.  Restore the backup archive and snapshot by specifying the region,
    bucket, backup artifact name and snapshot name:

    `automate-ctl restore-backup us-east-1:your-s3-bucket:2016-10-14-08-38-55-chef-automate-backup.zst 2016-10-14-08-38-55-chef-automate-backup`

### Partial Restoration

It is possible to restore only specific data from a Chef Automate backup
artifact. Below is an example of restoring only the PostgreSQL database
and git repositories from a backup archive in S3:

1.  Determine the archive you want to restore

    `automate-ctl list-backups --automate`

2.  Restore it

    `automate-ctl restore-backup us-east-1:your-s3-bucket:2016-10-14-08-38-55-chef-automate-backup.zst --no-census --no-license --no-config`

It is also possible to restore a functional Chef Automate cluster to a
specific Elasticsearch snapshot. Below is an example of restoring only
an Elasticsearch snapshot:

1.  Determine the snapshot you want to restore

    `automate-ctl list-backups --elasticsearch`

2.  Restore it

    `automate-ctl restore-backup 2016-10-14-08-38-55-chef-automate-backup`
