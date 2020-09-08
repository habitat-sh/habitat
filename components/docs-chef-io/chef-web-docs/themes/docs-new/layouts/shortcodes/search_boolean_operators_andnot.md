Operators must be in ALL CAPS. Parentheses can be used to group clauses
and to form sub-queries.

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

Using `AND NOT` together may trigger an error. For example:

``` bash
ERROR: knife search failed: invalid search query:
'datacenter%3A123%20AND%20NOT%20hostname%3Adev-%20AND%20NOT%20hostanem%3Asyslog-'
Parse error at offset: 38 Reason: Expected one of \ at line 1, column 42 (byte 42) after AND
```

Use `-` instead of `NOT`. For example:

``` bash
knife search sample "id:foo AND -id:bar"
```



</div>

</div>