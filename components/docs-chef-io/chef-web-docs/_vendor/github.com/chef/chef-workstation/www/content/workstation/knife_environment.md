+++
title = "knife environment"
draft = false

aliases = ["/knife_environment.html", "/knife_environment/"]

[menu]
  [menu.workstation]
    title = "knife environment"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_environment.md knife environment"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_environment.md)

{{% environment %}}

{{% knife_environment_summary %}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## compare

Use the `compare` argument to compare the cookbook version constraints
that are set on one (or more) environments.

### Syntax

This argument has the following syntax:

``` bash
knife environment compare [ENVIRONMENT_NAME...] (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Upload all environments found at the specified path.

`-m`, `--mismatch`

:   Show only matching versions.

### Examples

The following examples show how to use this knife subcommand:

**Compare cookbook versions in a single environment**

To compare cookbook versions for a single environment:

``` bash
knife environment compare development
```

to return something similar to:

``` bash
        development
apache  2.3.1
windows 4.1.2
```



**Compare cookbook versions for multiple environments**

To compare cookbook versions for multiple environments:

``` bash
knife environment compare development staging
```

to return something similar to:

``` bash
            development     staging
apache      2.3.1           1.2.2
windows     4.1.2           1.0.0
postgresql  1.0.0           1.0.0
```

**Compare cookbook versions for all environments**

To compare all cookbook versions for all environments:

``` bash
knife environment compare --all
```

to return something similar to:

``` bash
                staging   development
ulimit          latest    latest
redisio         latest    latest
journly         latest    latest
aws             latest    latest
test            latest    latest
unicorn         latest    latest
sensu           latest    latest
runit           latest    latest
templater       latest    latest
powershell      latest    latest
openssl         latest    latest
rbenv           latest    latest
rabbitmq        latest    latest
postgresql      latest    latest
mysql           latest    latest
ohai            latest    latest
git             latest    latest
erlang          latest    latest
ssh_known_hosts latest    latest
nginx           latest    latest
database        latest    latest
yum             latest    latest
xfs             latest    latest
apt             latest    latest
dmg             latest    latest
chef_handler    latest    latest
windows         1.0.0     4.1.2
```

## create

Use the `create` argument to add an environment object to the Chef Infra
Server. When this argument is run, knife will open \$EDITOR to enable
editing of the `ENVIRONMENT` description field (unless a description is
specified as part of the command). When finished, knife will add the
environment to the Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife environment create ENVIRONMENT_NAME -d --description ENVIRONMENT_DESCRIPTION
```

### Options

This argument has the following options:

`--description DESCRIPTION`

:   The description of the environment. This value populates the
    description field for the environment on the Chef Infra Server.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Create an environment**

To create an environment named `dev` with a description of
`The development environment.`:

``` bash
knife environment create dev -d --description "The development environment."
```

## delete

Use the `delete` argument to delete an environment from a Chef Infra
Server.

### Syntax

This argument has the following syntax:

``` bash
knife environment delete ENVIRONMENT_NAME
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Delete an environment**

To delete an environment named `dev`, enter:

``` bash
knife environment delete dev
```

Type `Y` to confirm a deletion.

## edit

Use the `edit` argument to edit the attributes of an environment. When
this argument is run, knife will open \$EDITOR to enable editing of
`ENVIRONMENT` attributes. When finished, knife will update the Chef
Infra Server with those changes.

### Syntax

This argument has the following syntax:

``` bash
knife environment edit ENVIRONMENT_NAME
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**Edit an environment**

To edit an environment named `devops`, enter:

``` bash
knife environment edit devops
```

## from file

Use the `from file` argument to add or update an environment using a
JSON or Ruby DSL description.

### Syntax

This argument has the following syntax:

``` bash
knife environment from file FILE (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Upload all environments found at the specified path.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Create an environment from a JSON file**

To add an environment using data contained in a JSON file:

``` bash
knife environment from file "path to JSON file"
```

## list

Use the `list` argument to list all of the environments that are
currently available on the Chef Infra Server.

### Syntax

This argument has the following syntax:

``` bash
knife environment list -w
```

### Options

This argument has the following options:

`-w`, `--with-uri`

:   Show the corresponding URIs.

### Examples

The following examples show how to use this knife subcommand:

**View a list of environments**

To view a list of environments:

``` bash
knife environment list -w
```

## show

Use the `show` argument to display information about the specified
environment.

### Syntax

This argument has the following syntax:

``` bash
knife environment show ENVIRONMENT_NAME
```

### Options

This argument has the following options:

`-a ATTR`, `--attribute ATTR`

:   The attribute (or attributes) to show.

### Examples

The following examples show how to use this knife subcommand:

**Show environments**

To view information about the `dev` environment enter:

``` bash
knife environment show dev
```

to return:

``` bash
% knife environment show dev
chef_type:            environment
cookbook_versions:
default_attributes:
description:
json_class:           Chef::Environment
name:                 dev
override_attributes:

\\
\\
\\
\\
```

**Show environments as JSON**

To view information in JSON format, use the `-F` common option as part
of the command like this:

``` bash
knife environment show devops -F json
```

Other formats available include `text`, `yaml`, and `pp`.
