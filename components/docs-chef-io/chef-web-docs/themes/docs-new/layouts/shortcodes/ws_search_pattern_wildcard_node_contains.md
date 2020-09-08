To search for a node using a partial name, enter one of the following:

``` bash
knife search node 'name:app*'
```

or:

``` bash
knife search node 'name:app1*.example.com'
```

or:

``` bash
knife search node 'name:app?.example.com'
```

or:

``` bash
knife search node 'name:app1.example.???'
```

to return `app1.example.com` (and any other node that matches any of the
string searches above).