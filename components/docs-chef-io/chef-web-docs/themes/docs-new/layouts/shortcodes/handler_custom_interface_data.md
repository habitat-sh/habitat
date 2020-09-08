The `data` method is used to return the Hash representation of the
`run_status` object. For example:

``` ruby
def data
  @run_status.to_hash
end
```