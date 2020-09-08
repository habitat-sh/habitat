+++
title = "knife cookbook"
draft = false

aliases = ["/knife_cookbook.html", "/knife_cookbook/"]

[menu]
  [menu.workstation]
    title = "knife cookbook"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_cookbook.md knife cookbook"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_cookbook.md)

{{% cookbooks_summary %}}

{{% knife_cookbook_summary %}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## bulk delete

Use the `bulk delete` argument to delete cookbook files that match a
pattern defined by a regular expression. The regular expression must be
within quotes and not be surrounded by forward slashes (/).

### Syntax

This argument has the following syntax:

``` bash
knife cookbook bulk delete REGEX (options)
```

### Options

This argument has the following options:

`-p`, `--purge`

:   Entirely remove a cookbook (or cookbook version) from the Chef Infra
    Server. Use this action carefully because only one copy of any
    single file is stored on the Chef Infra Server. Consequently,
    purging a cookbook disables any other cookbook that references one
    or more files from the cookbook that has been purged.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Bulk delete many cookbooks**

Use a regular expression to define the pattern used to bulk delete
cookbooks:

``` bash
knife cookbook bulk delete "^[0-9]{3}$" -p
```

## delete

Use the `delete` argument to delete a specified cookbook or cookbook
version on the Chef Infra Server (and not locally).

### Syntax

This argument has the following syntax:

``` bash
knife cookbook delete COOKBOOK_NAME [COOKBOOK_VERSION] (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Delete all cookbooks (and cookbook versions).

`COOKBOOK_VERSION`

:   The version of a cookbook to be deleted. If a cookbook has only one
    version, this option does not need to be specified. If a cookbook
    has more than one version and this option is not specified, knife
    prompts for a version.

`-p`, `--purge`

:   Entirely remove a cookbook (or cookbook version) from the Chef Infra
    Server. Use this action carefully because only one copy of any
    single file is stored on the Chef Infra Server. Consequently,
    purging a cookbook disables any other cookbook that references one
    or more files from the cookbook that has been purged.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Delete a cookbook**

``` bash
knife cookbook delete cookbook_name version
```

For example:

``` bash
knife cookbook delete smartmon 0.8
```

Type `Y` to confirm a deletion.

## download

Use the `download` argument to download a cookbook from the Chef Infra
Server to the current working directory.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook download COOKBOOK_NAME [COOKBOOK_VERSION] (options)
```

### Options

This argument has the following options:

`-d DOWNLOAD_DIRECTORY`, `--dir DOWNLOAD_DIRECTORY`

:   The directory in which cookbooks are located.

`-f`, `--force`

:   Overwrite an existing directory.

`-N`, `--latest`

:   Download the most recent version of a cookbook.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Download a cookbook**

To download a cookbook named `smartmon`, enter:

``` bash
knife cookbook download smartmon
```

## list

Use the `list` argument to view a list of cookbooks that are currently
available on the Chef Infra Server. The list will contain only the most
recent version for each cookbook by default.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook list (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Return all available versions for every cookbook.

`-w`, `--with-uri`

:   Show the corresponding URIs.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**View a list of cookbooks**

To view a list of cookbooks:

``` bash
knife cookbook list
```

## metadata

Use the `metadata` argument to generate the metadata for one or more
cookbooks.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook metadata (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Generate metadata for all cookbooks.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Generate metadata**

``` bash
knife cookbook metadata -a
```

## metadata from file

Use the `metadata from file` argument to load the metadata for a
cookbook from a file.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook metadata from file FILE
```

### Options

This command does not have any specific options.

### Examples

The following examples show how to use this knife subcommand:

**View metadata**

``` bash
knife cookbook metadata from file /path/to/file
```

## show

Use the `show` argument to view information about a cookbook, parts of a
cookbook (attributes, definitions, files, libraries, providers, recipes,
resources, and templates), or a file that is associated with a cookbook
(including attributes such as checksum or specificity).

### Syntax

This argument has the following syntax:

``` bash
knife cookbook show COOKBOOK_NAME [COOKBOOK_VERSION] [PART...] [FILE_NAME] (options)
```

### Options

This argument has the following options:

`COOKBOOK_VERSION`

:   The version of a cookbook to be shown. If a cookbook has only one
    version, this option does not need to be specified. If a cookbook
    has more than one version and this option is not specified, a list
    of cookbook versions is returned.

`-f FQDN`, `--fqdn FQDN`

:   The FQDN of the host.

`FILE_NAME`

:   The name of a file that is associated with a cookbook.

`-p PLATFORM`, `--platform PLATFORM`

:   The platform for which a cookbook is designed.

`PART`

:   The part of the cookbook to show: `attributes`, `definitions`,
    `files`, `libraries`, `providers`, `recipes`, `resources`, or
    `templates`. More than one part can be specified.

`-V PLATFORM_VERSION`, `--platform-version PLATFORM_VERSION`

:   The version of the platform.

`-w`, `--with-uri`

:   Show the corresponding URIs.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Show cookbook data**

To get the list of available versions of a cookbook named
`getting-started`, enter:

``` bash
knife cookbook show getting-started
```

to return something like:

``` none
getting-started   0.3.0  0.2.0
```

**Show cookbook versions**

To show a list of data about a cookbook using the name of the cookbook
and the version, enter:

``` bash
knife cookbook show getting-started 0.3.0
```

to return something like:

``` none
attributes:
  checksum:     fa0fc4abf3f6787aeb5c3c5c35de667c
  name:         default.rb
  path:         attributes/default.rb
  specificity:  default
  url:          https://somelongurlhere.com
chef_type:      cookbook_version
cookbook_name:  getting-started
definitions:    []
files:          []
frozen?:        false
json_class:     Chef::CookbookVersion
libraries:      []
```

**Show a cookbook version**

To only view data about templates, enter:

``` bash
knife cookbook show getting-started 0.3.0 templates
```

to return something like:

``` none
checksum:     a29d6f254577b830091f140c3a78b1fe
name:         chef-getting-started.txt.erb
path:         templates/default/chef-getting-started.txt.erb
specificity:  default
url:          https://someurlhere.com
```

**Show cookbook data as JSON**

To view information in JSON format, use the `-F` common option as part
of the command like this:

``` bash
knife cookbook show devops -F json
```

Other formats available include `text`, `yaml`, and `pp`.

## test

Use the `test` argument to test a cookbook for syntax errors. This
argument uses Ruby syntax checking to verify every file in a cookbook
that ends in .rb and Embedded Ruby (ERB). This argument will respect
chefignore files when determining which cookbooks to test for syntax
errors.

{{< warning >}}

This feature is deprecated in favor of [Cookstyle](/workstation/cookstyle/) and
[ChefSpec](/workstation/chefspec/)

{{< /warning >}}

### Syntax

This argument has the following syntax:

``` bash
knife cookbook test COOKBOOK_NAME (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Test all cookbooks.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Test a cookbook**

``` bash
knife cookbook test cookbook_name
```

## upload

Use the `upload` argument to upload one or more cookbooks (and any files
that are associated with those cookbooks) from a local repository to the
Chef Infra Server. Only files that do not already exist on the Chef
Infra Server will be uploaded.

{{< note >}}

Use a chefignore file to prevent the upload of specific files and file
types, such as temporary files or files placed in folders by version
control systems. The chefignore file must be located in the root of the
cookbook repository and must use rules similar to filename globbing (as
defined by the Ruby `File.fnmatch` syntax).

{{< /note >}}

{{< note >}}

Empty directories are not uploaded to the Chef Infra Server. To upload
an empty directory, create a "dot" file---e.g. `.keep`---in that
directory to ensure that the directory itself is not empty.

{{< /note >}}

### Syntax

This argument has the following syntax:

``` bash
knife cookbook upload [COOKBOOK_NAME...] (options)
```

### Options

This argument has the following options:

`-a`, `--all`

:   Upload all cookbooks.

`--concurrency`

:   The number of allowed concurrent connections. Default: `10`.

`-d`, `--include-dependencies`

:   Ensure that when a cookbook has a dependency on one (or more)
    cookbooks, those cookbooks are also uploaded.

`-E ENVIRONMENT`, `--environment ENVIRONMENT`

:   Use to set the environment version dependency to the cookbook
    version being uploaded.

`--force`

:   Update a cookbook even if the `--freeze` flag has been set.

`--freeze`

:   Require changes to a cookbook be included as a new version. Only the
    `--force` option can override this setting.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Upload a cookbook**

``` bash
knife cookbook upload cookbook_name
```

**Freeze a cookbook**

To upload a cookbook, and then prevent other users from being able to
make changes to it, enter:

``` bash
knife cookbook upload redis --freeze
```

to return something like:

``` none
Uploading redis...
Upload completed
```

If a cookbook is frozen and the `--force` option is not specified, knife
will return an error message similar to the following:

``` none
Uploading redis...
ERROR: Version 0.1.6 of cookbook redis is frozen. Use --force to override.
```
