The `grant-server-admin-permissions` subcommand is used to add a user to
the `server-admins` group. Run the command once per user added.

This subcommand has the following syntax:

``` bash
chef-server-ctl grant-server-admin-permissions USER_NAME
```

where `USER_NAME` is the user to add to the list of server
administrators.

For example:

``` bash
chef-server-ctl grant-server-admin-permissions bob
```

returns:

``` bash
User bob was added to server-admins. This user can now list,
read, and create users (even for orgs they are not members of)
for this Chef Infra Server.
```