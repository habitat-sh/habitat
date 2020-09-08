To build a search query to use more than one attribute, use an
underscore (`_`) to separate each attribute. For example, the following
query will search for all nodes running a specific version of Ruby:

``` bash
knife search node "languages_ruby_version:2.7.0"
```