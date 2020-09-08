+++
title = "knife user"
draft = false

aliases = ["/knife_user.html", "/knife_user/"]

[menu]
  [menu.workstation]
    title = "knife user"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_user.md knife user"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_user.md)

{{% knife_user_summary %}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## create

Use the `create` argument to create a user. This process will generate
an RSA key pair for the named user. The public key will be stored on the
Chef Infra Server and the private key will be displayed on `STDOUT` or
written to a named file.

-   For the user, the private key should be copied to the system as
    `/etc/chef/client.pem`.
-   For knife, the private key is typically copied to
    `~/.chef/client_name.pem` and referenced in the config.rb
    configuration file.

### Syntax

This argument has the following syntax:

``` bash
knife user create USERNAME DISPLAY_NAME FIRST_NAME LAST_NAME EMAIL PASSWORD (options)
```

### Options

This argument has the following options:

`-a`, `--admin`

:   Create a client as an admin client.

`-f FILE_NAME`, `--file FILE_NAME`

:   Save a private key to the specified file name.

`-p PASSWORD`, `--password PASSWORD`

:   The user password.

`--user-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, the Chef Infra Server will generate a public/private
    key pair.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Create a user**

``` bash
knife user create rbirdman "Radio Birdman" Radio Birdman radio@bird.man -f /keys/radio_birdman
```

## delete

Use the `delete` argument to delete a registered user.

### Syntax

This argument has the following syntax:

``` bash
knife user delete USER_NAME
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Delete a user**

``` bash
knife user delete "Steve Danno"
```

## edit

Use the `edit` argument to edit the details of a user. When this
argument is run, knife will open \$EDITOR. When finished, knife will
update the Chef Infra Server with those changes.

### Syntax

This argument has the following syntax:

``` bash
knife user edit USER_NAME
```

### Options

This command does not have any specific options.

### Examples

None.

## key create

Use the `key create` argument to create a public key.

### Syntax

This argument has the following syntax:

``` bash
knife user key create USER_NAME (options)
```

### Options

This argument has the following options:

`-e DATE`, `--expiration-date DATE`

:   The expiration date for the public key, specified as an ISO 8601
    formatted string: `YYYY-MM-DDTHH:MM:SSZ`. If this option is not
    specified, the public key will not have an expiration date. For
    example: `2013-12-24T21:00:00Z`.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name.

`-k NAME`, `--key-name NAME`

:   The name of the public key.

`-p FILE_NAME`, `--public-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, and only if `--key-name` is specified, the Chef Infra
    Server will generate a public/private key pair.

### Examples

None.

## key delete

Use the `key delete` argument to delete a public key.

### Syntax

This argument has the following syntax:

``` bash
knife user key delete USER_NAME KEY_NAME
```

### Examples

None.

## key edit

Use the `key edit` argument to modify or rename a public key.

### Syntax

This argument has the following syntax:

``` bash
knife user key edit USER_NAME KEY_NAME (options)
```

### Options

This argument has the following options:

`-c`, `--create-key`

:   Generate a new public/private key pair and replace an existing
    public key with the newly-generated public key. To replace the
    public key with an existing public key, use `--public-key` instead.

`-e DATE`, `--expiration-date DATE`

:   The expiration date for the public key, specified as an ISO 8601
    formatted string: `YYYY-MM-DDTHH:MM:SSZ`. If this option is not
    specified, the public key will not have an expiration date. For
    example: `2013-12-24T21:00:00Z`.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name. If the `--public-key`
    option is not specified the Chef Infra Server will generate a
    private key.

`-k NAME`, `--key-name NAME`

:   The name of the public key.

`-p FILE_NAME`, `--public-key FILE_NAME`

:   The path to a file that contains the public key. If this option is
    not specified, and only if `--key-name` is specified, the Chef Infra
    Server will generate a public/private key pair.

### Examples

None.

## key list

Use the `key list` argument to view a list of public keys for the named
user.

### Syntax

This argument has the following syntax:

``` bash
knife user key list USER_NAME (options)
```

### Options

This argument has the following options:

`-e`, `--only-expired`

:   Show a list of public keys that have expired.

`-n`, `--only-non-expired`

:   Show a list of public keys that have not expired.

`-w`, `--with-details`

:   Show a list of public keys, including URIs and expiration status.

### Examples

None.

## key show

Use the `key show` argument to view details for a specific public key.

### Syntax

This argument has the following syntax:

``` bash
knife user key show USER_NAME KEY_NAME
```

### Examples

None.

## list

Use the `list` argument to view a list of registered users.

### Syntax

This argument has the following syntax:

``` bash
knife user list (options)
```

### Options

This argument has the following options:

`-w`, `--with-uri`

:   Show the corresponding URIs.

### Examples

None.

## reregister

Use the `reregister` argument to regenerate an RSA key pair for a user.
The public key will be stored on the Chef Infra Server and the private
key will be displayed on `STDOUT` or written to a named file.

{{< note >}}

Running this argument will invalidate the previous RSA key pair, making
it unusable during authentication to the Chef Infra Server.

{{< /note >}}

### Syntax

This argument has the following syntax:

``` bash
knife user reregister USER_NAME (options)
```

### Options

This argument has the following options:

`-f FILE_NAME`, `--file FILE_NAME`

:   Save a private key to the specified file name.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Regenerate the RSA key-pair**

``` bash
knife user reregister "Robert Younger"
```

## show

Use the `show` argument to show the details of a user.

### Syntax

This argument has the following syntax:

``` bash
knife user show USER_NAME (options)
```

### Options

This argument has the following options:

`-a ATTR`, `--attribute ATTR`

:   The attribute (or attributes) to show.

### Examples

The following examples show how to use this knife subcommand:

**Show user data**

To view a user named `Dennis Teck`, enter:

``` bash
knife user show "Dennis Teck"
```

to return something like:

``` bash
chef_type:   user
json_class:  Chef::User
name:        Dennis Teck
public_key:
```

**Show user data as JSON**

To view information in JSON format, use the `-F` common option as part
of the command like this:

``` bash
knife user show "Dennis Teck" -F json
```

(Other formats available include `text`, `yaml`, and `pp`, e.g.
`-F yaml` for YAML.)
