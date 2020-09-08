A special character can be used to fine-tune a search query and to
increase the accuracy of the search results. The following characters
can be included within the search query syntax, but each occurrence of a
special character must be escaped with a backslash (`\`), also (`/`)
must be escaped against the Elasticsearch:

``` ruby
+  -  &&  | |  !  ( )  { }  [ ]  ^  "  ~  *  ?  :  \  /
```

For example:

``` ruby
\(1\+1\)\:2
```