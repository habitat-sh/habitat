Values that are stored in a data bag are global to the organization and
are available to any environment. There are two main strategies that can
be used to store per-environment data within a data bag: by using a
top-level key that corresponds to the environment or by using separate
items for each environment.

A data bag that is storing a top-level key for an environment might look
something like this:

``` none
{
  "id": "some_data_bag_item",
  "production" : {
    # Hash with all your data here
  },
  "testing" : {
    # Hash with all your data here
  }
}
```

When using the data bag in a recipe, that data can be accessed from a
recipe using code similar to:

``` ruby
data_bag_item[node.chef_environment]['some_other_key']
```

The other approach is to use separate items for each environment.
Depending on the amount of data, it may all fit nicely within a single
item. If this is the case, then creating different items for each
environment may be a simple approach to providing per-environment values
within a data bag. However, this approach is more time-consuming and may
not scale to very large environments or when the data must be stored in
many data bag items.