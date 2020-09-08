``` ruby
data_bag('users') #=> ['sandy', 'jill']
```

Iterate over the contents of the data bag to get the associated
`data_bag_item`:

``` ruby
data_bag('users').each do |user|
  data_bag_item('users', user)
end
```

The `id` for each data bag item will be returned as a string.