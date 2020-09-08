To use a range search to find IP addresses within a subnet, enter the
following:

``` bash
knife search node 'ipaddress:[192.168.0.* TO 192.0.2.*]'
```

where `192.168.0.* TO 192.0.2.*` defines the subnet range.