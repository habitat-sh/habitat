The `backup` subcommand is used to back up all Chef Infra Server data.
This subcommand:

-   Requires rsync to be installed on the Chef Infra Server prior to
    running the command
-   Requires a `chef-server-ctl reconfigure` prior to running the
    command
-   Should not be run in a Chef Infra Server configuration with an
    external PostgreSQL database; [use knife ec
    backup](https://github.com/chef/knife-ec-backup) instead
-   Puts the initial backup in the `/var/opt/chef-backup` directory as a
    tar.gz file; move this backup to a new location for safe keeping