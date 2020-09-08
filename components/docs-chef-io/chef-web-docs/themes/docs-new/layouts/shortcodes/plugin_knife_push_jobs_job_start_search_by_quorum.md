To search for nodes assigned the role `webapp`, and where 90% of those
nodes must be available, run the following command:

``` bash
knife job start --quorum 90% 'chef-client' --search 'role:webapp'
```