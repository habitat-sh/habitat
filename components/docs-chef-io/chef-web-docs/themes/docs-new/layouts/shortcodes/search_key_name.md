To see the available keys for a node, enter the following (for a node
named `staging`):

``` bash
knife node show staging -Fj | less
```

to return a full JSON description of the node and to view the available
keys with which any search query can be based.