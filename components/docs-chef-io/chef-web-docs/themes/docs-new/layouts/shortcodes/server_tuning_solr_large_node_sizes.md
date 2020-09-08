The maximum field length setting for Apache Solr should be greater than
any expected node object file sizes in order for them to be successfully
added to the search index. If a node object file is greater than the
maximum field length, the node object will be indexed up to the maximum,
but the part of the file past that limit will not be indexed. If this
occurs, it will seem as if nodes disappear from the search index. To
ensure that large node file sizes are indexed properly, verify the
following configuration settings:

`nginx['client_max_body_size']`

:   The maximum accepted body size for a client request, as indicated by
    the `Content-Length` request header. When the maximum accepted body
    size is greater than this value, a `413 Request Entity Too Large`
    error is returned. Default value: `250m`.

and

`opscode_erchef['max_request_size']`

:   When the request body size is greater than this value, a 413 Request
    Entity Too Large error is returned. Default value: `2000000`.

to ensure that those settings are not part of the reasons for incomplete
indexing, and then update the following setting so that its value is
greater than the expected node file sizes:

`opscode_solr4['max_field_length']`

:   The maximum field length (in number of tokens/terms). If a field
    length exceeds this value, Apache Solr may not be able to complete
    building the index. Default value: `100000` (increased from the
    Apache Solr default value of `10000`).

Use the `wc` command to get the byte count of a large node object file.
For example:

``` bash
wc -c NODE_NAME.json
```

and then ensure there is a buffer beyond that value. For example, verify
the size of the largest node object file:

``` bash
wc -c nodebsp2016.json
```

which returns `154516`. Update the `opscode_solr4['max_field_length']`
setting to have a value greater than the returned value. For example:
`180000`.

If you don't have a node object file available then you can get an
approximate size of the node data by running the following command on a
node.

``` bash
ohai | wc -c
```