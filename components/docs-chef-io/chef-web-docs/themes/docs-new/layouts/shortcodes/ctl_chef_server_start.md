The `start` subcommand is used to start all services that are enabled in
the Chef Infra Server. This command can also be run for an individual
service by specifying the name of the service in the command.

This subcommand has the following syntax:

``` bash
chef-server-ctl start SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand. When a service is
successfully started the output should be similar to:

``` bash
ok: run: service_name: (pid 12345) 1s
```

The supervisor for the Chef Infra Server is configured to wait seven
seconds for a service to respond to a command from the supervisor. If
you see output that references a timeout, it means that a signal has
been sent to the process, but that the process has yet to actually
comply. In general, processes that have timed out are not a big concern,
unless they are failing to respond to the signals at all. If a process
is not responding, use a command like the `kill` subcommand to stop the
process, investigate the cause (if required), and then use the `start`
subcommand to re-enable it.