A **log** resource block adds messages to the log file based on events
that occur during a Chef Infra Client run:

``` ruby
log 'message' do
  message 'A message add to the log.'
  level :info
end
```

The full syntax for all of the properties that are available to the
**log** resource is:

``` ruby
log 'name' do
  level        Symbol # default value: :info
  message      String # default value: 'name' unless specified
  action       Symbol # defaults to :write if not specified
end
```

where:

-   `log` is the resource.
-   `name` is the name given to the resource block.
-   `action` identifies which steps Chef Infra Client will take to bring
    the node into the desired state.
-   `level` and `message` are the properties available to this resource.