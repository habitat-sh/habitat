A node's initial run-list is specified using a JSON file on the host
system. When running Chef Infra Client as an executable, use the `-j`
option to tell Chef Infra Client which JSON file to use. For example:

``` bash
chef-client -j /etc/chef/file.json --environment _default
```

where `file.json` is similar to:

``` javascript
{
  "resolver": {
    "nameservers": [ "10.0.0.1" ],
    "search":"int.example.com"
  },
  "run_list": [ "recipe[resolver]" ]
}
```

and where `_default` is the name of the environment that is assigned to
the node.

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

This approach may be used to update
[normal](/attributes.html#attribute-types) attributes, but should never
be used to update any other attribute type, as all attributes updated
using this option are treated as `normal` attributes.



</div>

</div>