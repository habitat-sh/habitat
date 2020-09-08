Use the `knife download` subcommand to download roles, cookbooks,
environments, nodes, and data bags from the Chef Infra Server to the
current working directory. It can be used to back up data on the Chef
Infra Server, inspect the state of one or more files, or to extract
out-of-process changes users may have made to files on the Chef Infra
Server, such as if a user made a change that bypassed version source
control. This subcommand is often used in conjunction with `knife diff`,
which can be used to see exactly what changes will be downloaded, and
then `knife upload`, which does the opposite of `knife download`.