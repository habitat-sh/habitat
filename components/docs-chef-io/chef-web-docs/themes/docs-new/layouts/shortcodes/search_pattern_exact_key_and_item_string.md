To search in a specific data bag using a string to find any matching
data bag item, enter the following:

``` bash
knife search admins 'comment:"Charlie the Unicorn"'
```

where `admins` is the name of the data bag and `Charlie the Unicorn` is
the string that will be used during the search. Something similar to the
following will be returned:

``` bash
1 items found
_rev:       1-39ff4099f2510f477b4c26bef81f75b9
chef_type:  data_bag_item
comment:    Charlie the Unicorn
data_bag:   admins
gid:        ops
id:         charlie
shell:      /bin/zsh
uid:        1005
```