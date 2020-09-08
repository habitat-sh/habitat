Creating and editing the contents of a data bag or a data bag item from
a recipe is not recommended. The recommended method of updating a data
bag or a data bag item is to use knife and the `knife data bag`
subcommand. If this action must be done from a recipe, please note the
following:

-   If two operations concurrently attempt to update the contents of a
    data bag, the last-written attempt will be the operation to update
    the contents of the data bag. This situation can lead to data loss,
    so organizations should take steps to ensure that only one Chef
    Infra Client is making updates to a data bag at a time.
-   Altering data bags from the node when using the open source Chef
    Infra Server requires the node's API client to be granted admin
    privileges. In most cases, this is not advisable.

and then take steps to ensure that any subsequent actions are done
carefully. The following examples show how a recipe can be used to
create and edit the contents of a data bag or a data bag item using the
`Chef::DataBag` and `Chef::DataBagItem` objects.

To create a data bag from a recipe:

``` ruby
users = Chef::DataBag.new
users.name('users')
users.create
```

To create a data bag item from a recipe:

``` ruby
sam = {
  'id' => 'sam',
  'Full Name' => 'Sammy',
  'shell' => '/bin/zsh'
}
databag_item = Chef::DataBagItem.new
databag_item.data_bag('users')
databag_item.raw_data = sam
databag_item.save
```

To edit the contents of a data bag item from a recipe:

``` ruby
sam = data_bag_item('users', 'sam')
sam['Full Name'] = 'Samantha'
sam.save
```