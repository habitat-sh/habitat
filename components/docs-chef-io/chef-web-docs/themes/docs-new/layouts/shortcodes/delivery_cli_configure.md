Before you use the Delivery CLI from a workstation, you need to provide
it with details such as the URL of the Chef Automate server, and the
names of the relevant enterprise, organization, and user. The
`delivery setup` subcommand creates a configuration file named
`.delivery/cli.toml` with the required information.

The placement of the `.delivery` directory in your file hierarchy is
significant. Like git, Delivery CLI commands search the current
directory and parent directories to locate server settings. Because
server settings are unique to an organization, we recommend that you
create a directory for each organization you belong to and run the
`delivery setup` command from that directory.

``` bash
delivery setup --server=DELIVERY_SERVER_IP_ADDR --ent=ENTERPRISE --org=ORGANIZATION --user=USERNAME
```

The following settings may be added to the `.delivery/cli.toml` file:

`auto_bump`

:   Bumps the cookbook metadata version number automatically when
    `delivery review` is run. Default value: `false`.