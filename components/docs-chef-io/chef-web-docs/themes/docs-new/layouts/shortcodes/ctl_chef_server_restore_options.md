This subcommand has the following options:

`-c`, `--cleanse`

:   Use to remove all existing data on the Chef Infra Server; it will be
    replaced by the data in the backup archive.

`-d DIRECTORY`, `--staging-dir DIRECTORY`

:   Use to specify that the path to an empty directory to be used during
    the restore process. This directory must have enough disk space to
    expand all data in the backup archive.