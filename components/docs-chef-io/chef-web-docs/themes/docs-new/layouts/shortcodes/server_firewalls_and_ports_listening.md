All services must be listening on the appropriate ports. Most monitoring
systems provide a means of testing whether a given port is accepting
connections and service-specific tools may also be available. In
addition, the generic system tool Telnet can also be used to initiate
the connection:

``` bash
telnet HOST_NAME PORT
```