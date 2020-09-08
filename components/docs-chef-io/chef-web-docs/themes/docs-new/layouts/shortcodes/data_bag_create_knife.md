knife can be used to create data bags and data bag items when the
`knife data bag` subcommand is run with the `create` argument. For
example:

``` bash
knife data bag create DATA_BAG_NAME (DATA_BAG_ITEM)
```

knife can be used to update data bag items using the `from file`
argument:

``` bash
knife data bag from file BAG_NAME ITEM_NAME.json
```

As long as a file is in the correct directory structure, knife will be
able to find the data bag and data bag item with only the name of the
data bag and data bag item. For example:

``` bash
knife data bag from file BAG_NAME ITEM_NAME.json
```

will load the following file:

``` none
data_bags/BAG_NAME/ITEM_NAME.json
```

Continuing the example above, if you are in the "admins" directory and
make changes to the file charlie.json, then to upload that change to the
Chef Infra Server use the following command:

``` bash
knife data bag from file admins charlie.json
```

In some cases, such as when knife is not being run from the root
directory for the chef-repo, the full path to the data bag item may be
required. For example:

``` bash
knife data bag from file BAG_NAME /path/to/file/ITEM_NAME.json
```