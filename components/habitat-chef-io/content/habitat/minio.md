+++
title = "Chef Habitat Builder on-prem + Minio"
description = "Using Minio as your Chef Habitat Builder on-prem backend"

[menu]
  [menu.habitat]
    title = "Minio"
    identifier = "habitat/builder-on-prem/minio"
    parent = "habitat"

+++

[MinIO](https://min.io/) is an open source object storage server. Chef Habitat Builder on-prem uses Minio to store habitat artifacts (.harts).

## Managing Builder On-Prem Artifacts

The data that Builder stores is luckily fairly lightweight and thus the backup and DR or Warm Spare strategy is pretty straightforward. On-Prem Builder has two types of data that should be backed up case of a disaster:

1. [PostgreSQL package and user metadata](./postgres.md#postgresql-data-backups)
1. Minio habitat artifacts (.harts)

Chef Habitat Builder on-prem supports only Minio artifact repositories.

Ideally, you should coordinate the backup of the entire Builder on-prem cluster to happen together. However, the type of data that Builder stores (metadata and artifacts) permits some flexibility in the timing of your backup operations.

### Minio Artifact Backups

The process of artifact backups is quite a bit more environmentally subjective than Postgres if only because we support more than one artifact storage backend. For the sake of these docs we will focus on Minio backups.

Backing up Minio is also a bit subjective but more or less amounts to a filesystem backup. Because Minio stores its files on the filesystem (unless you're using a non-standard configuration) any filesystem backup strategy you want to use should be fine whether taking disk snapshots of some kind or data  mirroring, and rsync. Minio however also has the [minio client](https://docs.min.io/docs/minio-client-quickstart-guide.html) which provides a whole boatload of useful features and specifically allows the user to mirror a bucket to an alternative location on the filesystem or even a remote S3 bucket! Ideally you should _never_ directly/manually manipulate the files within Minio's buckets while it could be performing IO. Which means you should _always_ use the Minio client mentioned above to manipulate Minio data.

A simple backup strategy might look like this:

1. Shut down the API to ensure no active transactions are occurring. (Optional but preferred)
        `hab svc stop habitat/builder-api`
1. Mirror Minio data to an AWS S3 bucket. **
        `mc mirror <local/minio/object/dir> <AWS_/S3_bucket>`
** Another option here is to mirror to a different part of the filesystem, perhaps one that's NFS mounted or the like and then taking snapshots of it:
        `mc mirror <local/minio/object/dir> <new/local/path>

As mentioned before since this operation could be dramatically different for different environments Minio backup cannot be 100% prescriptive. But This should give you some ideas to explore.

## Migrating off of local filesystem into S3/Minio

This section is for installations of On-Premise Depot that were done *prior* to June 15, 2018. If you re-install or upgrade to a newer version of the On-Premise Depot, you will be required to also migrate your package artifacts to a local instance of Minio (the new object store we are using). Please follow the steps below.

## Pre-requisites

1. Install the following Habitat packages:

```bash
hab pkg install -b core/aws-cli
hab pkg install -b core/jq-static
hab pkg install -b habitat/s3-bulk-uploader
```

If you are running in an "air-gapped" environment, you may need to download the hart files and do a `hab pkg install -b <HART FILE>` instead.  Don't forget the `-b` parameter to binlink the binaries into your path.

1. Please make sure that you have appropriate values for Minio in your `bldr.env`.  Check the 'bldr.env.sample' for the new required values.

## Migration

1. Run the `install.sh` script so that Minio is appropriately configured
1. Check that you can log into your Minio instance at the URL specified in the `bldr.env`
1. If all looks good, run the artifact migration script: `sudo ./scripts/s3migrate.sh minio`

Once the migration script starts, you will be presented with some questions to specify the Minio instance, the credentials, and the Minio bucket name to migrate your package artifacts to. The script will attempt to automatically detect all of these from the running service, so you can usually just accept the defaults. Please refer to your `bldr.env` file if you need to explicitly type in any values.

The migration script may take a while to move over the artifacts into Minio. During the script migration, the Depot services will continue to run as normal, however packages will not be downloadable until the artifacts are migrated over to Minio.

Once the migration is complete, you will be presented with an option to remove the files in your `hab/svc/builder-api/data/pkgs` directory. You may want to preserve the files until you have verified that all operations are completing successfully.
