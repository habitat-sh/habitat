Use the `chef undelete` subcommand to recover a deleted policy or policy
group. This command:

-   Does not detect conflicts. If a deleted item has been recreated,
    running this command will overwrite it
-   Does not include cookbooks that may be referenced by policy files;
    cookbooks that are cleaned after running this command may not be
    fully restorable to their previous state
-   Does not store access control data