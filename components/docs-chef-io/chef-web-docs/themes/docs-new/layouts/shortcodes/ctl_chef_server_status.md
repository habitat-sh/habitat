The `status` subcommand is used to show the status of all services
available to the Chef Infra Server. The results will vary based on the
configuration of a given server. This subcommand has the following
syntax:

``` bash
chef-server-ctl status
```

and will return the status for all services. Status can be returned for
individual services by specifying the name of the service as part of the
command:

``` bash
chef-server-ctl status SERVICE_NAME
```

where `SERVICE_NAME` represents the name of any service that is listed
after running the `service-list` subcommand.

When service status is requested, the output should be similar to:

``` bash
run: service_name: (pid 12345) 12345s; run: log: (pid 1234) 67890s
```

where

-   `run:` is the state of the service (`run:` or `down:`)
-   `service_name:` is the name of the service for which status is
    returned
-   `(pid 12345)` is the process identifier
-   `12345s` is the uptime of the service, in seconds

For example:

``` bash
down: opscode-erchef: (pid 35546) 10s
```

By default, runit will restart services automatically when the services
fail. Therefore, runit may report the status of a service as `run:` even
when there is an issue with that service. When investigating why a
particular service is not running as it should be, look for the services
with the shortest uptimes. For example, the list below indicates that
the **opscode-erchef** should be investigated further:

``` bash
run: oc-id
run: opscode-chef: (pid 4327) 13671s; run: log: (pid 4326) 13671s
run: opscode-erchef: (pid 5383) 5s; run: log: (pid 4382) 13669s
run: opscode-expander: (pid 4078) 13694s; run: log: (pid 4077) 13694s
run: opscode-expander-reindexer: (pid 4130) 13692s; run: log: (pid 4114) 13692s
```