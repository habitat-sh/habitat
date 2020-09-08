At the end of every Chef Infra Client run, the node object is saved to
the Chef Infra Server. From the Chef Infra Server, each node object is
then added to the `SOLR` search index. This process is asynchronous. By
default, node objects are committed to the search index every 60 seconds
or per 1000 node objects, whichever occurs first.

When data is committed to the Apache Solr index, all incoming updates
are blocked. If the duration between updates is too short, it is
possible for the rate at which updates are asked to occur to be faster
than the rate at which objects can be actually committed.

Use the following configuration setting to improve the indexing
performance of node objects:

`opscode_solr4['commit_interval']`

:   The frequency (in seconds) at which node objects are added to the
    Apache Solr search index. Default value: `60000` (every 60 seconds).

`opscode_solr4['max_commit_docs']`

:   The frequency (in documents) at which node objects are added to the
    Apache Solr search index. Default value: `1000` (every 1000
    documents).