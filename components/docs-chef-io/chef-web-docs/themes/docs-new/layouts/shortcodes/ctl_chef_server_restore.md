The `restore` subcommand is used to restore Chef Infra Server data from
a backup that was created by the `backup` subcommand. This subcommand
may also be used to add Chef Infra Server data to a newly-installed
server. This subcommand:

-   Requires rsync to be installed on the Chef Infra Server prior to
    running the command
-   Requires a `chef-server-ctl reconfigure` prior to running the
    command
-   Should not be run in a Chef Infra Server configuration with an
    external PostgreSQL database; [use knife ec
    backup](https://github.com/chef/knife-ec-backup) instead