Use `:filter_result` as part of a search query to filter the search
output based on the pattern specified by a Hash. Only attributes in the
Hash will be returned.

The syntax for the `search` method that uses `:filter_result` is as
follows:

``` ruby
search(:index, 'query',
  :filter_result => { 'foo' => [ 'abc' ],
                      'bar' => [ '123' ],
                      'baz' => [ 'sea', 'power' ]
                    }
      ).each do |result|
  puts result['foo']
  puts result['bar']
  puts result['baz']
end
```

where:

-   `:index` is of name of the index on the Chef Infra Server against
    which the search query will run: `:client`, `:data_bag_name`,
    `:environment`, `:node`, and `:role`
-   `'query'` is a valid search query against an object on the Chef
    server
-   `:filter_result` defines a Hash of values to be returned

For example:

``` ruby
search(:node, 'role:web',
  :filter_result => { 'name' => [ 'name' ],
                      'ip' => [ 'ipaddress' ],
                      'kernel_version' => [ 'kernel', 'version' ]
                    }
      ).each do |result|
  puts result['name']
  puts result['ip']
  puts result['kernel_version']
end
```