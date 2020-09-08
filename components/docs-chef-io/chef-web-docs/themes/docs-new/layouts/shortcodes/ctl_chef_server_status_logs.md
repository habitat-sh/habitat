A typical status line for a service that is running any of the Chef
Infra Server front-end services is similar to the following:

``` bash
run: name_of_service: (pid 1486) 7819s; run: log: (pid 1485) 7819s
```

where:

-   `run` describes the state in which the supervisor attempts to keep
    processes. This state is either `run` or `down`. If a service is in
    a `down` state, it should be stopped
-   `name_of_service` is the service name, for example: `opscode-solr4`
-   `(pid 1486) 7819s;` is the process identifier followed by the amount
    of time (in seconds) the service has been running
-   `run: log: (pid 1485) 7819s` is the log process. It is typical for a
    log process to have a longer run time than a service; this is
    because the supervisor does not need to restart the log process in
    order to connect the supervised process

If the service is down, the status line will appear similar to the
following:

``` bash
down: opscode-solr4: 3s, normally up; run: log: (pid 1485) 8526s
```

where

-   `down` indicates that the service is in a down state
-   `3s, normally up;` indicates that the service is normally in a run
    state and that the supervisor would attempt to restart this service
    after a reboot