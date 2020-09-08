+++
title = "Supermarket Backup and Restore"
draft = false

aliases = ["/supermarket_backup_restore.html"]

[menu]
  [menu.infra]
    title = "Backup and Restore"
    identifier = "chef_infra/setup/supermarket/supermarket_backup_restore.md Backup and Restore"
    parent = "chef_infra/setup/supermarket"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/supermarket_backup_restore.md)

Periodic backups of Supermarket data are an essential part of managing
and maintaining a healthy configuration, and to help ensure that
important data can be restored if required. In a typical installation of
Supermarket, both the cookbook store and the database need to be backed
up on a regular basis.

## Backup

### Cookbook Backup

If Supermarket is not configured to use AWS S3 storage for cookbooks,
then the local cookbook storage location on the Supermarket server will
need to be backed up.

The default location is: `/var/opt/supermarket/data/cookbook_versions`.

For example, a cookbook backup command:

``` bash
cd /var/opt/supermarket/data/
tar cvzf ~/supermarket_cookbook_versions.tar.gz cookbook_versions
```

### Database Backup

A database export can be made in several formats.

For example, a database export in a .dump format can be made with the
following syntax:

``` bash
/opt/supermarket/embedded/bin/pg_dump --host localhost --username supermarket --dbname supermarket --port 15432 --format c --blobs --verbose --file ~/supermarket_database_backup.dump
```

where, in a typical installation:

:   -   `/opt/supermarket/embedded/bin/pg_dump` is the path to the
        database export utility included in the Supermarket installation
    -   `localhost` may alternatively be 127.0.0.1
    -   `15432` is the PostgreSQL port number, which may need to be
        modified
    -   `--format c` sets the output to PostgreSQL's "custom" binary
        file format

Be sure to update the various local values in the `pg_dump` command as
necessary to match your infrastructure. For documentation about the
pg_dump utility, see:
<https://www.postgresql.org/docs/9.3/app-pgdump.html>

To find local variables, look at
`/etc/supermarket/supermarket-running.json`. This file lives next to
`supermarket.rb` and `supermarket.json` where their configuration is
set. `supermarket-running.json` contains the final values the system is
operating with after running `sudo supermarket-ctl reconfigure`.

There's a "database" key in `supermarket-running.json`:

``` javascript
{ "supermarket": {
     ...
     "database": {
          "user": "supermarket",
          "name": "supermarket",
          "host": "127.0.0.1",
          "port": "15432",
          "pool": "30",
          "extensions": {
               "plpgsql": true,
               "pg_trgm": true
          },
          "password": "sup3rs3cr3t"
     },
     ...
}
```

## Restore

### Cookbook Restore

When restoring cookbooks, **make sure the cookbook directory is writable
by the Supermarket user.**

For example, to restore your cookbook files, run:

``` bash
cd /var/opt/supermarket/data/
tar xvzf /supermarket_cookbook_versions.tar.gz
```

### Database Restore

{{< note >}}

The restore does not support transferring backups across different
versions of Supermarket. Backups taken must be restored to the same
version of Supermarket that was in use when they were created.

{{< /note >}}

For example, to restore a backup in a .dump format, run:

``` bash
pg_restore --host localhost --port 15432 --clean --no-acl --no-owner --dbname supermarket_production --verbose supermarket_database_backup.dump
```
