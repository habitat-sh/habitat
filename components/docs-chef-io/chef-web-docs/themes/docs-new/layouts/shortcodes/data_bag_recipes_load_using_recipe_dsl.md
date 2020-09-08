The Recipe DSL provides access to data bags and data bag items
(including encrypted data bag items) with the following methods:

-   `data_bag(bag)`, where `bag` is the name of the data bag.
-   `data_bag_item('bag_name', 'item', 'secret')`, where `bag` is the
    name of the data bag and `item` is the name of the data bag item. If
    `'secret'` is not specified, Chef Infra Client will look for a
    secret at the path specified by the `encrypted_data_bag_secret`
    setting in the client.rb file.

The `data_bag` method returns an array with a key for each of the data
bag items that are found in the data bag.

Some examples:

To load the secret from a file:

``` ruby
data_bag_item('bag', 'item', IO.read('secret_file'))
```

To load a single data bag item named `admins`:

``` ruby
data_bag('admins')
```

The contents of a data bag item named `justin`:

``` ruby
data_bag_item('admins', 'justin')
```

will return something similar to:

``` ruby
# => {'comment'=>'Justin Currie', 'gid'=>1005, 'id'=>'justin', 'uid'=>1005, 'shell'=>'/bin/zsh'}
```

If `item` is encrypted, `data_bag_item` will automatically decrypt it
using the key specified above, or (if none is specified) by the
`Chef::Config[:encrypted_data_bag_secret]` method, which defaults to
`/etc/chef/encrypted_data_bag_secret`.