+++
title = "Chef Habitat Builder on-prem + PostreSQL"
description = "Using Postgres as your Chef Habitat Builder on-prem backend"

[menu]
  [menu.habitat]
    title = "Postgres"
    identifier = "habitat/builder-on-prem/postgres"
    parent = "habitat"

+++

Managing your Postgres Installation

## Managing Builder On-Prem PostgreSQL Data

The data that Builder stores is luckily fairly lightweight and thus the backup and DR strategy is pretty straightforward. On-Prem Builder has two types of data that should be backed up case of a disaster:

1. PostgreSQL package and user metadata
1. [Minio habitat artifacts](./minio.md#minio-artifact-backups)

Ideally, you should coordinate the backup of the entire Builder on-prem cluster to happen together. However, the type of data that Builder stores (metadata and artifacts) permits some flexibility in the timing of your backup operations. In the worst case, if a package's metadata is missing from PostgreSQL, you can repopulate it by re-uploading the package with the `--force` flag, for example: `hab pkg upload <path to hartfile> -u <on-prem_url> --force`.

### PostgreSQL Data Backups

Backing up Builder's PostgreSQL database is the same as for any PostgreSQL database. The process is a [pg_dump](https://www.postgresql.org/docs/11/app-pgdump.html). If you have a backup strategy for other production instances of PostgreSQL, then apply your backup pattern to the `builder` database. To backup your `builder` database manually, follow these steps:

1. Shut down the API to ensure no active transactions are occurring. (Optional but preferred)
        `hab svc stop habitat/builder-api`
1. Switch to user `hab`
        `sudo su - hab`
1. Find your Postgres password
        `sudo cat /hab/svc/builder-api/config/config.toml`
1. Export as envvar
        `export PGPASSWORD=<pw>`
1. Run pgdump
        `/hab/pkgs/core/postgresql/<version>/<release>/bin/pg_dump --file=builder.dump --format=custom --host=<ip_of_pg_host> --dbname=builder`
1. Start the api and verify
        `sudo hab svc start habitat/builder-api`

Once the backup finishes,  your will find it as the `builder.dump` file on your filesystem. Move and store this file according to your local policies. We recommend storing it remotely--either physically or virtually--so it will be useable in a worst-case scenario. For most, storing the dump file in an AWS bucket or Azure storage is enough, but you should follow the same strategy for all database backups.

### Restoring PostgreSQL Data

Restoring a `builder` database is exactly like restoring any other database--which is to say, there is no magical solution. If you already have a restoration strategy in place at your organization, follow that to restore your `builder` database.  To restore your  data `builder` database manually, follow these steps:

1. Switch to user `hab`
        `sudo su - hab`
1. Find your Postgres password
        `sudo cat /hab/svc/builder-api/config/config.toml`
1. Export as envvar
        `export PGPASSWORD=<pw>`
1. Create the new builder database *
        `/hab/pkgs/core/postgresql/<version>/<release>/bin/createdb -w -h <url_of_pg_host> -p <configured_pg_port> -U hab builder`
1. Verify connectivity to the new database instance
        `/hab/pkgs/core/postgresql/<version>/<release>/bin/psql --host=<url_of_pg_host> --dbname=builder`
1. Restore the dump into the new DB
        `/hab/pkgs/core/postgresql/<version>/<release>/bin/pg_restore --host=<url_of_pg_host> --dbname=builder builder.dump`
1. Start the on-prem Builder services

    > Note: In some cases your version of Postgres might not have a `createdb` binary in which case you'll want to connect to database to run the create db command.

Your database data should be restored and ready for use!

## Merging PostgreSQL Database Shards

This following sections on "Merging Database Shards" and "Merging Databases" is for installations of On-Premise Depot that were done *prior* to
August 17th 2018. If you re-install or upgrade to a newer version of the
On-Premise Depot, you will be required to also merge your database shards into
the `public` Postgres database schema. Please follow the steps below.

### Shard Migration Pre-requisites

1. The password to your Postgres database. By default, this is located at
   `/hab/svc/builder-datastore/config/pwfile`
1. A fresh backup of the two databases present in the On-Premise Depot,
   `builder_sessionsrv` and `builder_originsrv`. You can create such a backup
   with `pg_dump`:

   ```shell
   PGPASSWORD=$(sudo cat /hab/svc/builder-datastore/config/pwfile) hab pkg exec core/postgresql pg_dump -h 127.0.0.1 -p 5432 -U hab builder_originsrv > builder-originsrv.sql
   ```

### Shard Migration

1. Uninstall existing services by running `sudo -E ./uninstall.sh`
1. Install new services by running `./install.sh`
1. If you check your logs at this point, you will likely see lines like this:
   `Shard migration hasn't been completed successfully` repeated over and over
   again, as the supervisor tries to start the new service, but the service
   dies because the migration hasn't been run.
1. Optionally, if you want to be extra sure that you're in a good spot to perform the
   migration, log into the Postgres console and verify that you have empty
   tables in the `public` schema. A command to do this might look like:

   ```shell
   PGPASSWORD=$(sudo cat /hab/svc/builder-datastore/config/pwfile) hab pkg exec core/postgresql psql -h 127.0.0.1 -p 5432 -U hab builder_originsrv
   ```

   That should drop you into a prompt where you can type `\d` and hopefully see
   a list of tables where the schema says `public`. If you try to select data
   from any of those tables, they should be empty. Note that this step is
   definitely not required, but can be done if it provides you extra peace of
   mind.
1. Now you are ready to migrate the data itself. The following command will do
   that for `builder-originsrv`:

   ```shell
   PGPASSWORD=$(sudo cat /hab/svc/builder-datastore/config/pwfile) ./scripts/merge-shards.sh originsrv migrate
   ```

   After confirming that you have fresh database backups, the script
   should run and at the end, you should see several notices that everything is
   great, row counts check out, and your database has been marked as migrated.
1. Do the same migration for `builder-sessionsrv`.

   ```shell
   PGPASSWORD=$(sudo cat /hab/svc/builder-datastore/config/pwfile) ./scripts/merge-shards.sh sessionsrv migrate
   ```

1. Double check the logs for `builder-originsrv` and `builder-sessionsrv` to
   make sure things look normal again. If there are still errors, restart the
   services.
1. At this point, all data is stored in the `public` schema. All of the other
   schemas, from `shard_0` up to `shard_127` will still be present in your
   database, and the data in them will remain intact, but the services will no
   longer reference those shards.

## Merging PostgreSQL Databases

This section is for installations of On-Premise Depot that were done *after*
the database shard migration listed above. If upgrade to a newer version of the
On-Premise Depot, you will be required to also merge databases into
the `builder` Postgres database. Please follow the steps below.

### Database Merge Pre-requisites

1. The password to your Postgres database. By default, this is located at
   `/hab/svc/builder-datastore/config/pwfile`
1. A fresh backup of the two databases present in the On-Premise Depot,
   `builder_sessionsrv` and `builder_originsrv`. You can create such a backup
   with `pg_dump`:

   ```shell
   PGPASSWORD=$(sudo cat /hab/svc/builder-datastore/config/pwfile) hab pkg exec core/postgresql pg_dump -h 127.0.0.1 -p 5432 -U hab builder_originsrv > builder-originsrv.sql
   ```

### Database Merge Migration

1. With all services running your *current* versions, execute the following command from the root of the repo directory:

   ```shell
   PGPASSWORD=$(sudo cat /hab/svc/builder-datastore/config/pwfile) ./scripts/merge-databases.sh
   ```

   After confirming that you have fresh database backups, the script
   should run and create a new 'builder' database, and then migrate the data.
1. At this point, all data is stored in the `builder` database. Both of the other
   databases (`builder_originsrv` and `builder_sessionsrv`) will still be present,
   and the data in them will remain intact, but new services will no
   longer reference those databases.
1. Now, stop and uninstall the existing services by running `sudo -E ./uninstall.sh`
1. Install new services by running `./install.sh`
1. Once the new services come up, you should be able to log back into the depot UI and confirm that everything is as expected.
