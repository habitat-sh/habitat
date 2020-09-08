A single loopback interface should be configured using the `127.0.0.1`
address. This ensures that all of the services are available to the Chef
Infra Server, in the event that the Chef Infra Server attempts to
contact itself from within a front or back end machine. All ports should
be accessible through the loopback interface of their respective hosts.