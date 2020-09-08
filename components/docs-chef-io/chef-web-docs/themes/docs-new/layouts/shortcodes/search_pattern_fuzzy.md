A fuzzy matching search pattern is used to search based on the proximity
of two strings of characters. An (optional) integer may be used as part
of the search query to more closely define the proximity. A fuzzy
matching search pattern has the following syntax:

``` ruby
"search_query"~edit_distance
```

where `search_query` is the string that will be used during the search
and `edit_distance` is the proximity. A tilde ("\~") is used to separate
the edit distance from the search query.