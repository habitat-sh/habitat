+++
title = "knife data bag"
draft = false

aliases = ["/knife_data_bag.html", "/knife_data_bag/"]

[menu]
  [menu.workstation]
    title = "knife data bag"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_data_bag.md knife data bag"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_data_bag.md)

{{% data_bag %}}

{{% data_bag_encryption %}}

{{% knife_data_bag_summary %}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## create

Use the `create` argument to add a data bag to the Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife data bag create DATA_BAG_NAME [DATA_BAG_ITEM] (options)
```

### Options

This argument has the following options:

`DATA_BAG_ITEM`

:   The name of a specific item within a data bag. `DATA_BAG_NAME` may
    not be `client`, `environment`, `node` or `role`, as these are
    reserved words for [knife search](/workstation/knife_search/).

`--secret SECRET`

:   The encryption key that is used for values contained within a data
    bag item. If `secret` is not specified, Chef Infra Client looks for
    a secret at the path specified by the `encrypted_data_bag_secret`
    setting in the client.rb file. For encrypted data bag items, use
    **either** `--secret` or `--secret-file`, not both.

`--secret-file FILE`

:   The path to the file that contains the encryption key.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Create a data bag**

To create a data bag named "admins", enter:

``` bash
knife data bag create admins
```

to return:

``` none
Created data_bag[admins]
```

## delete

Use the `delete` argument to delete a data bag or a data bag item from a
Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife data bag delete DATA_BAG_NAME [DATA_BAG_ITEM] (options)
```

### Options

This argument has the following options:

`DATA_BAG_ITEM`

:   The name of a specific item within a data bag.

### Examples

The following examples show how to use this knife subcommand:

**Delete a data bag**

``` bash
knife data bag delete data_bag_name
```

**Delete a data bag item**

To delete an item named "charlie", enter:

``` bash
knife data bag delete admins charlie
```

Type `Y` to confirm a deletion.

## edit

{{% knife_data_bag_edit %}}

### Syntax

This argument has the following syntax:

``` bash
knife data bag edit DATA_BAG_NAME [DATA_BAG_ITEM] (options)
```

### Options

This argument has the following options:

`DATA_BAG_ITEM`

:   The name of a specific item within a data bag.

`--secret SECRET`

:   The encryption key that is used for values contained within a data
    bag item. If `secret` is not specified, Chef Infra Client looks for
    a secret at the path specified by the `encrypted_data_bag_secret`
    setting in the client.rb file.

`--secret-file FILE`

:   The path to the file that contains the encryption key.

{{< note >}}

For encrypted data bag items, use *either* `--secret` or
`--secret-file`, not both.

{{< /note >}}

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Edit a data bag**

To edit the contents of a data bag, enter:

``` bash
knife data bag edit dogs tibetanspaniel
```

where `dogs` is the name of the data bag and `tibetanspaniel` is the
name of the data bag item. This will return something similar to the
following in the knife editor:

``` javascript
{
  "name":"data_bag_item_dogs_tibetanspaniel",
  "json_class":"Chef::DataBagItem",
  "chef_type":"data_bag_item",
  "data_bag":"dogs",
  "raw_data":
    {
      "description":"small dog that likes to sit in windows",
      "id":"tibetanspaniel"
    }
}
```

Make the necessary changes to the key-value pairs under `raw_data` and
save them.

**Edit a data bag item**

{{% knife_data_bag_edit_item %}}

## from file

Use the `from file` argument to:

-   Add a data bag item to a data bag
-   Update the contents of an existing data bag item

The data bag itself must already exist on the Chef Infra Server and must
be specified as part of the command. The contents of the data bag item
are specified using a JSON file. This JSON file may be located at a
relative or absolute path; its location must be specified as part of the
command. The JSON file that defines the contents of the data bag item
must at least contain the name of the data bag item---`"id": "name"`.

### Syntax

This argument has the following syntax:

``` bash
knife data bag from file DATA_BAG_NAME_or_PATH JSON_FILE
```

### Options

This argument has the following options:

`-a`, `--all`

:   Upload all data bags found at the specified path.

`--secret SECRET`

:   The encryption key that is used for values contained within a data
    bag item. If `secret` is not specified, Chef Infra Client looks for
    a secret at the path specified by the `encrypted_data_bag_secret`
    setting in the client.rb file.

`--secret-file FILE`

:   The path to the file that contains the encryption key.

{{< note >}}

For encrypted data bag items, use *either* `--secret` or
`--secret-file`, not both.

{{< /note >}}

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Create a data bag from a file**

To create a data bag on the Chef Infra Server from a file:

``` bash
knife data bag from file "path to JSON file"
```

**Create an encrypted data bag from a file**

To create a data bag named "devops_data" that contains encrypted data,
enter:

``` bash
knife data bag from file devops_data --secret-file "path to decryption file"
```

**Create an encrypted data bag for use with Chef Infra Client local
mode**

{{% knife_data_bag_from_file_create_encrypted_local_mode %}}

## list

Use the `list` argument to view a list of data bags that are currently
available on the Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife data bag list
```

### Options

This argument has the following options:

`-w`, `--with-uri`

:   Show the corresponding URIs.

### Examples

The following examples show how to use this knife subcommand:

**View a list of data bags**

``` bash
knife data bag list
```

## show

Use the `show` argument to view the contents of a data bag.

### Syntax

This argument has the following syntax:

``` bash
knife data bag show DATA_BAG_NAME (options)
```

### Options

This argument has the following options:

`DATA_BAG_ITEM`

:   The name of a specific item within a data bag.

`--secret SECRET`

:   The encryption key that is used for values contained within a data
    bag item. If `secret` is not specified, Chef Infra Client looks for
    a secret at the path specified by the `encrypted_data_bag_secret`
    setting in the client.rb file.

`--secret-file FILE`

:   The path to the file that contains the encryption key.

{{< note >}}

For encrypted data bag items, use *either* `--secret` or
`--secret-file`, not both.

{{< /note >}}

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Show a data bag**

``` bash
knife data bag show admins
```

to return something like:

``` none
charlie
```

**Show a data bag item**

To show the contents of a specific item within data bag, enter:

``` bash
knife data bag show admins charlie
```

to return:

``` none
comment:  Crazy Charlie
gid:      ops
id:       charlie
shell:    /bin/zsh
uid:      1005
```

**Show a data bag, encrypted**

To show the contents of a data bag named `passwords` with an item that
contains encrypted data named `mysql`, enter:

``` bash
knife data bag show passwords mysql
```

to return:

``` javascript
{
  "id": "mysql",
  "pass": "trywgFA6R70NO28PNhMpGhEvKBZuxouemnbnAUQsUyo=\n",
  "user": "e/p+8WJYVHY9fHcEgAAReg==\n"
}
```

**Show a data bag, decrypted**

To show the decrypted contents of the same data bag, enter:

``` bash
knife data bag show --secret-file /path/to/decryption/file passwords mysql
```

to return:

``` javascript
{
  "id": "mysql",
  "pass": "thesecret123",
  "user": "fred"
}
```

**Show a data bag as JSON**

To view information in JSON format, use the `-F` common option as part
of the command like this:

``` bash
knife data bag show admins -F json
```

Other formats available include `text`, `yaml`, and `pp`.
