A search query is comprised of two parts: the key and the search
pattern. A search query has the following syntax:

``` ruby
key:search_pattern
```

where `key` is a field name that is found in the JSON description of an
indexable object on the Chef Infra Server (a role, node, client,
environment, or data bag) and `search_pattern` defines what will be
searched for, using one of the following search patterns: exact,
wildcard, range, or fuzzy matching. Both `key` and `search_pattern` are
case-sensitive; `key` has limited support for multiple character
wildcard matching using an asterisk ("\*") (and as long as it is not the
first character).