+++
title = "Backup & Restore"

date = 2018-03-26T15:27:52-07:00
draft = false
[menu]
  [menu.automate]
    title = "Backup & Restore"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/backup.md Backup & Restore"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/backup.md)

Backups are crucial for protecting your data from catastrophic loss and preparing a recovery procedure.
The `chef-automate backup create` command creates a single backup that contains data for all products deployed with Chef Automate, including [Chef Infra Server]({{< ref "infra_server.md" >}}) and [Chef Habitat Builder on-prem]({{< ref "on_prem_builder.md" >}}).
By default, Chef Automate stores backups to the filesystem in the directory `/var/opt/chef-automate/backups`.
You can also configure Chef Automate to store backups in AWS S3 buckets.

## Backup Space Requirements

This amount of space needed for a backup varies depending on your Chef Automate use. You need enough free space for:

* Complete copies of each Chef Automate service PostgreSQL database
* Complete copies of your configuration files
* Elasticsearch snapshots of your Chef Automate configuration and data, such as converge, scan, and report data. You will need enough disk space for the each Elasticsearch snapshot and the delta--or the list of changes--for each successive snapshot
* Chef Habitat Builder artifacts

## Backup to a Filesystem

To store backups in a configurable backup directory, the backup directory should be on network-attached storage or synced periodically to a disk on another machine.
This best practice ensures that you can restore from your backup data during a hardware failure.

The default backup directory is `/var/opt/chef-automate/backups`. If it does not exist, the deployment process creates it.

To configure your Chef Automate installation's backup directory to another location:

1. Create a `backup_config.toml` file in your current directory with the following content. Replace `/path/to/backups` with the path to your backup directory:

    ```toml
    [global.v1.backups.filesystem]
      path = "/path/to/backups"
    ```

1. Run the following command to apply your configuration:

    ```shell
    chef-automate config patch backup_config.toml
    ```

1. Remove the now-redundant `backup_config.toml` file.

### Store a Filesystem Backup in a Single-file Archive

To store backups offline in single-file archives, single-file archives must include both the configuration data and the reporting data contained in the standard backup.

The [configured backup directory]({{< ref "backup.md#backup-to-a-filesystem" >}}) contains both the timestamp-based directory for the configuration and the reporting data stored in the `automate-elasticsearch-data` directory.

A timestamp-based directory has a date-based name, such as `20180518010336`, in the `automate-elasticsearch-data` directory.

To provide externally-deployed Elasticsearch nodes access to Chef Automate's built-in backup storage services, you must [configure Elasticsearch backup]({{< relref "install.md#configuring-external-elasticsearch" >}}) settings separately from Chef Automate's primary backup settings.

## Backup to AWS S3

To store backups in an existing AWS S3 bucket, use the supported S3-related settings below:

```toml
[global.v1.backups]
  location = "s3"
[global.v1.backups.s3.bucket]
  # name (required): The name of the bucket
  name = "<bucket name>"

  # endpoint (required): The endpoint for the region the bucket lives in.
  # See https://docs.aws.amazon.com/general/latest/gr/rande.html#s3_region
  endpoint = "https://<region endpoint>"

  # base_path (optional):  The path within the bucket where backups should be stored
  # If base_path is not set, backups will be stored at the root of the bucket.
  base_path = "<base path>"

[global.v1.backups.s3.credentials]
  # Optionally, AWS credentials may be provided. If these are not provided, IAM instance
  # credentials will be used. It's also possible for these to be read through the standard
  # AWS environment variables or through the shared AWS config files.
  access_key = "<access_key>"
  secret_key = "<secret_key>"
  session_key = "<session_key>"

[global.v1.backups.s3.ssl]
  # root_cert (optional): The root certificate used for SSL validation.
  # For S3 compatible APIs, you can set the SSL root cert if needed
  root_cert = """
  -----BEGIN CERTIFICATE-----
  ...
  -----END CERTIFICATE-----
```

See how to [restore from AWS S3]({{< ref "restore/#restore-from-an-aws-s3-backup" >}}).

### AWS S3 Permissions

The following IAM policy describes the basic permissions Chef Automate requires to run backup and restore operations.

```json
{
  "Statement": [
    {
      "Action": [
        "s3:ListBucket",
        "s3:GetBucketLocation",
        "s3:ListBucketMultipartUploads",
        "s3:ListBucketVersions"
      ],
      "Effect": "Allow",
      "Resource": [
        "arn:aws:s3:::automate-backups.example.com"
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
        "arn:aws:s3:::automate-examples.example.com/*"
      ]
    }
  ],
  "Version": "2012-10-17"
}
```

## Backup to GCS

To store backups in an existing Google Cloud Storage (GCS) bucket, [generate a service account key](https://cloud.google.com/iam/docs/creating-managing-service-account-keys) with the `storage.admin` permission for the associated project and GCS bucket, and use the supported GCS-related settings below:

```toml
[global.v1.backups]
  location = "gcs"
[global.v1.backups.gcs.bucket]
  # name (required): The name of the bucket
  name = "<bucket name>"

  # base_path (optional):  The path within the bucket where backups should be stored.
  # If base_path is not set, backups will be stored at the root of the bucket.
  base_path = "<base path>"

[global.v1.backups.gcs.credentials]
# This is the JSON credentials file you generate during service account
# creation, you must copy/paste the entire contents here (this is just an example)
json = '''
  {
  "type": "service_account",
  "project_id": "my-favorite-project",
  "private_key_id": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "private_key": "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  "client_email": "chef@my-favorite-project.iam.gserviceaccount.com",
  "client_id": "XXXXXXXXXXXXXXXXXXXXX",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token",
  "auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
  "client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/chef%40my-favorite-project.iam.gserviceaccount.com"
}
'''
```

See how to [restore from GCS]({{< ref "restore/#restore-from-a-google-cloud-storage-backup" >}}).

## Backup Commands

### Create a Backup

Make a backup with the [`backup create`]({{< ref "cli_chef_automate/#chef-automate-backup-create" >}}) command:

```shell
chef-automate backup create
```

The output shows the backup progress for each service. A successful backup displays a success message containing the timestamp of the backup:

```shell
Success: Created backup 20180518010336
```

Restores from a filesystem backup may fail with incorrect directory permissions.
Run the [`fix-repo-permissions` command]({{< ref "cli_chef_automate/#chef-automate-backup-fix-repo-permissions" >}}) to address such issues:

```shell
sudo chef-automate backup fix-repo-permissions <path>
```

### List Backups

You can list existing backups with the [`backup list`]({{< ref "cli_chef_automate/#chef-automate-backup-list" >}}) command:

```shell
chef-automate backup list
```

The output shows each backup and its age:

```shell
        Backup        State  Age
20180508201548    completed  8 minutes old
20180508201643    completed  8 minutes old
20180508201952    completed  4 minutes old
```

By default, this command communicates with your running Chef Automate installation to list the backups. If the Chef Automate installation is down, you can still list the backups.

To list filesystem backups:

```shell
chef-automate backup list /var/opt/chef-automate/backups
Listing backups from local directory /var/opt/chef-automate/backups
        Backup        State  Age
20180508201548    completed  12 minutes old
20180508201643    completed  11 minutes old
20180508201952    completed  8 minutes old
```

For backups stored in an AWS S3 bucket, use:

```shell
chef-automate backup list s3://bucket_name/base_path
```

where `bucket_name` is the name of the S3 bucket and `base_path` is an optional path within the bucket where the backups live.

For backups stored in a Google Cloud Storage (GCS) bucket, use:

```shell
chef-automate backup list gs://bucket_name/base_path
```

where `bucket_name` is the name of the GCS bucket and `base_path` is an optional path within the bucket where the backups live.

## Delete Backups

To delete backups from a running instance of Chef Automate, first find the relevant backup ID with `chef-automate backup list` and then delete the backup using [`chef automate backup delete ID`]({{< ref "cli_chef_automate/#chef-automate-backup-delete" >}}).

```shell
chef-automate backup list
        Backup        State  Age
20181026183901    completed  1 minute old
20181026183954    completed  33 seconds old
20181026184012    completed  15 seconds old
```

Delete a single backup with `chef-automate backup delete`:

```shell
chef-automate backup delete 20181026183901
The following backups will be permanently deleted:
20181026183901
Are you sure you want to continue? (y/n)
y
Success: Backups deleted
```

To delete two or more backups, use `chef-automate backup delete` followed by the backup IDs:

```shell
chef-automate backup delete 20181026183954 20181026184012
The following backups will be permanently deleted:
20181026183954
20181026184012
Are you sure you want to continue? (y/n)
y
Success: Backups deleted
```

To prune all but a certain number of the most recent backups manually, parsing the output of `chef-automate backup list` and applying the command `chef-automate backup delete`.
For example:

```bash
export KEEP=10; chef-automate backup list --result-json backup.json > /dev/null && jq "[.result.backups[].id] | sort | reverse | .[]" -rM backup.json | tail -n +$(($KEEP+1)) | xargs -L1 -i chef-automate backup delete --yes {}
```

## Troubleshooting

To debug a failed backup, set the log level to `debug` and re-run the backup. This outputs the debug information to the Chef Automate log:

```shell
chef-automate debug set-log-level deployment-service debug
```

## References

See the [`chef-automate backup` command reference]({{< ref "cli_chef_automate/#chef-automate-backup" >}}).
