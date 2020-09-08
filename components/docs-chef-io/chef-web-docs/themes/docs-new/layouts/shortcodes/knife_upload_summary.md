Use the `knife upload` subcommand to upload data to the Chef Infra
Server from the current working directory in the chef-repo. The
following types of data may be uploaded with this subcommand:

-   Cookbooks
-   Data bags
-   Roles stored as JSON data
-   Environments stored as JSON data

(Roles and environments stored as Ruby data will not be uploaded.) This
subcommand is often used in conjunction with `knife diff`, which can be
used to see exactly what changes will be uploaded, and then
`knife download`, which does the opposite of `knife upload`.