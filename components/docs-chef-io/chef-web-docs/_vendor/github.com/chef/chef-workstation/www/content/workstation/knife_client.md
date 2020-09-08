+++
title = "knife client"
draft = false

aliases = ["/knife_client.html", "/knife_client/"]

[menu]
  [menu.workstation]
    title = "knife client"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_client.md knife client"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_client.md)

{{% knife_client_summary %}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## bulk delete

Use the `bulk delete` argument to delete any API client that matches a
pattern defined by a regular expression. The regular expression must be
within quotes and not be surrounded by forward slashes (`/`).

### Syntax

This argument has the following syntax:

``` bash
knife client bulk delete REGEX
```

### Options

This argument has the following options:

`-D`, `--delete-validators`

:   Force the deletion of the client when it is also a chef-validator.

### Examples

None.

## create

Use the `create` argument to create a new API client. This process will
generate an RSA key pair for the named API client. The public key will
be stored on the Chef Infra Server and the private key will be displayed
on `STDOUT` or written to a named file.

-   For Chef Infra Client, the private key should be copied to the
    system as `/etc/chef/client.pem`.
-   For knife, the private key is typically copied to
    `~/.chef/client_name.pem` and referenced in the config.rb
    configuration file.

### Syntax

This argument has the following syntax:

``` bash
knife client create CLIENT_NAME (options)
```

### Options

This argument has the following options:

`-a`, `--admin`

:   Create a client as an admin client.

`-f FILE`, `--file FILE`

:   Save a private key to the specified file name.

`-k`, `--prevent-keygen`

:   Create a user without a public key. This key may be managed later by
    using the `knife user key` subcommands.

    {{< note spaces=4 >}}

    This option is valid only with Chef Infra Server API, version 1.0,
    which was released with Chef Server 12.1. If this option or the
    `--user-key` option are not passed in the command, the Chef Infra
    Server will create a user with a public key named `default` and will
    return the private key. For the Chef Server versions earlier than
    12.1, this option will not work; a public key is always generated
    unless `--user-key` is passed in the command.

    {{< /note >}}

`-p FILE`, `--public-key FILE`

:   The path to a file that contains the public key. This option may not
    be passed in the same command with `--prevent-keygen`. When using
    Chef a default key is generated if this option is not passed in the
    command. For Chef Server version 12.x, see the `--prevent-keygen`
    option.

`--validator`

:   Create the client as the chef-validator. Default value: `true`.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Create an admin client**

To create a Chef Infra Client that can access the Chef Infra Server API
as an administrator---sometimes referred to as an "API Chef Infra
Client"---with the name "exampleorg" and save its private key to a file,
enter:

``` bash
knife client create exampleorg -a -f "/etc/chef/client.pem"
```

## delete

Use the `delete` argument to delete a registered API client. If using
Chef Client 12.17 or later, you can delete multiple clients using this
subcommand.

### Syntax

This argument has the following syntax:

``` bash
knife client delete CLIENT_NAME
```

### Options

This argument has the following options:

`-D`, `--delete-validators`

:   Force the deletion of the client when it is also a chef-validator.

### Examples

The following examples show how to use this knife subcommand:

**Delete a client**

To delete a client with the name "client_foo", enter:

``` bash
knife client delete client_foo
```

Type `Y` to confirm a deletion.

## edit

Use the `edit` argument to edit the details of a registered API client.
When this argument is run, knife will open \$EDITOR to enable editing of
the `admin` attribute. (None of the other attributes should be changed
using this argument.) When finished, knife will update the Chef Infra
Server with those changes.

### Syntax

This argument has the following syntax:

``` bash
knife client edit CLIENT_NAME
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Edit a client**

To edit a client with the name "exampleorg", enter:

``` bash
knife client edit exampleorg
```

## key create

Use the `key create` argument to create a public key.

### Syntax

This argument has the following syntax:

``` bash
knife client key create CLIENT_NAME (options)
```

### Options

This argument has the following options:

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

## key delete

Use the `key delete` argument to delete a public key.

### Syntax

This argument has the following syntax:

``` bash
knife client key delete CLIENT_NAME KEY_NAME
```

### Examples

None.

## key edit

Use the `key edit` argument to modify or rename a public key.

### Syntax

This argument has the following syntax:

``` bash
knife client key edit CLIENT_NAME KEY_NAME (options)
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
client.

### Syntax

This argument has the following syntax:

``` bash
knife client key list CLIENT_NAME (options)
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
knife client key show CLIENT_NAME KEY_NAME
```

### Examples

None.

## list

Use the `list` argument to view a list of registered API client.

### Syntax

This argument has the following syntax:

``` bash
knife client list (options)
```

### Options

This argument has the following options:

`-w`, `--with-uri`

:   Show the corresponding URIs.

### Examples

The following examples show how to use this knife subcommand:

**View a list of clients**

To verify the API client list for the Chef Infra Server, enter:

``` bash
knife client list
```

to return something similar to:

``` none
exampleorg
i-12345678
rs-123456
```

To verify that an API client can authenticate to the Chef Infra Server
correctly, try getting a list of clients using `-u` and `-k` options to
specify its name and private key:

``` bash
knife client list -u ORGNAME -k .chef/ORGNAME.pem
```

## reregister

Use the `reregister` argument to regenerate an RSA key pair for an API
client. The public key will be stored on the Chef Infra Server and the
private key will be displayed on `STDOUT` or written to a named file.

{{< note >}}

Running this argument will invalidate the previous RSA key pair, making
it unusable during authentication to the Chef Infra Server.

{{< /note >}}

### Syntax

This argument has the following syntax:

``` bash
knife client reregister CLIENT_NAME (options)
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

**Re-register a client**

To re-register the RSA key pair for a client named "testclient" and save
it to a file named "rsa_key", enter:

``` bash
knife client reregister testclient -f rsa_key
```

## show

Use the `show` argument to show the details of an API client.

### Syntax

This argument has the following syntax:

``` bash
knife client show CLIENT_NAME (options)
```

### Options

This argument has the following options:

`-a ATTR`, `--attribute ATTR`

:   The attribute (or attributes) to show.

### Examples

The following examples show how to use this knife subcommand:

**Show clients**

To view a client named "testclient", enter:

``` bash
knife client show testclient
```

to return something like:

``` none
admin:       false
chef_type:   client
json_class:  Chef::ApiClient
name:        testclient
public_key:
```

To view information in JSON format, use the `-F` common option as part
of the command like this:

``` bash
knife client show devops -F json
```

Other formats available include `text`, `yaml`, and `pp`.
