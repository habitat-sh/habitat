The `stop` subcommand is used to stop all services enabled on the Chef
Infra Server. This command can also be run for an individual service by
specifying the name of the service in the command.

This subcommand has the following syntax:

``` bash
chef-server-ctl stop SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand. When a service is
successfully stopped the output should be similar to:

``` bash
ok: down: service_name: 0s, normally up
```

For example:

``` bash
chef-server-ctl stop
```

will return something similar to:

``` bash
ok: down: nginx: 393s, normally up
ok: down: opscode-chef: 391s, normally up
ok: down: opscode-erchef: 391s, normally up
ok: down: opscode-expander: 390s, normally up
ok: down: opscode-expander-reindexer: 389s, normally up
ok: down: opscode-solr4: 389s, normally up
ok: down: postgresql: 388s, normally up
ok: down: rabbitmq: 388s, normally up
ok: down: redis_lb: 387s, normally up
```