The `int` subcommand is used to send a `SIGINT` to all services. This
command can also be run for an individual service by specifying the name
of the service in the command.

This subcommand has the following syntax:

``` bash
chef-server-ctl int SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.