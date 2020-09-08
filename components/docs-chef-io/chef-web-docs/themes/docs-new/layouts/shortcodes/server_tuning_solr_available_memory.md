Use the following configuration setting to help ensure that Apache Solr
does not run out of memory:

`opscode_solr4['heap_size']`

:   The amount of memory (in MBs) available to Apache Solr. If there is
    not enough memory available, search queries made by nodes to Apache
    Solr may fail. The amount of memory that must be available also
    depends on the number of nodes in the organization, the frequency of
    search queries, and other characteristics that are unique to each
    organization. In general, as the number of nodes increases, so does
    the amount of memory.

If Apache Solr is running out of memory, the
`/var/log/opscode/opscode-solr4/current` log file will contain a message
similar to:

``` bash
SEVERE: java.lang.OutOfMemoryError: Java heap space
```

The default value for `opscode_solr4['heap_size']` should work for many
organizations, especially those with fewer than 25 nodes. For
organizations with more than 25 nodes, set this value to 25% of system
memory or `1024`, whichever is smaller. For very large configurations,
increase this value to 25% of system memory or `4096`, whichever is
smaller. This value should not exceed `8192`.