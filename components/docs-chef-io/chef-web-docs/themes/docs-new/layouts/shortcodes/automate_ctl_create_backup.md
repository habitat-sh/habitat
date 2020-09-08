The `create-backup` subcommand is used to create Chef Automate backups.
By default, it creates Automate backup archives and Elasticsearch
snapshots.

**Syntax**

``` none
automate-ctl create-backup [NAME] [options]
     --chef-server-config         Backup up the Chef Infra Server config if present
     --digest [int]               The SHA digest length to output. 256, 384, and 512 are valid
     --force                      Agree to all warnings and prompts
     --name [string]              The output name of the backup
     --no-census                  Do not back up Chef Automate's census data
     --no-compliance-profiles     Do not back up Chef Automate's compliance profiles
     --no-config                  Do not back up Chef Automate's configuration directory
     --no-db                      Do not back up Chef Automate's database
     --no-elasticsearch           Do not snapshot Chef Automate's Elasticsearch
     --no-git                     Do not back up Chef Automate's git repositories
     --no-license                 Do not back up Chef Automate's license file
     --no-notifications           Do not back up Chef Automate's notifications rulestore
     --no-wait                    Do not wait for non-blocking backup operations
     --no-wait-for-lock           Do not wait for Elasticsearch lock
     --quiet                      Do not output non-error information
     --rabbit                     Back up Chef Automate's RabbitMQ queues
     --retry-limit                Maximum number of times to retry archive uploads to S3
     --staging-dir [string]       The path to use for temporary files during backup
 -h, --help                       Show the usage message
```

The `NAME` value is optional. If omitted, a default name with the
current time will be used.

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

In rare circumstances, jobs that are running at the time of backup
creation may be left in an unrecoverable state. For this reason, it's
recommended to take a backup when no critical jobs are running.



</div>

</div>

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

`create-backup` should be run outside of root-only directories like
`/root`, as it tries to chpst to the user chef-pgsql. This user will
have problems running with a current working directory owned by root.



</div>

</div>

**Examples**

Complete backup:

:   `automate-ctl create-backup`

Elasticsearch snapshot only:

:   `automate-ctl create-backup --no-census --no-config --no-db --no-license --no-git`

Automate archive only

:   `automate-ctl create-backup --no-elasticsearch`