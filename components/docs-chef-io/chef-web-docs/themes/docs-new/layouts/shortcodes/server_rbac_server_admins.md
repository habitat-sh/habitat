The `server-admins` group is a global group that grants its members
permission to create, read, update, and delete user accounts, with the
exception of superuser accounts. The `server-admins` group is useful for
users who are responsible for day-to-day administration of the Chef
Infra Server, especially user management via the `knife user`
subcommand. Before members can be added to the `server-admins` group,
they must already have a user account on the Chef Infra Server.