A range matching search pattern is used to query for values that are
within a range defined by upper and lower boundaries. A range matching
search pattern can be inclusive or exclusive of the boundaries. Use
square brackets ("\[ \]") to denote inclusive boundaries and curly
braces ("{ }") to denote exclusive boundaries and with the following
syntax:

``` ruby
boundary TO boundary
```

where `TO` is required (and must be capitalized).