+++
title = "knife cookbook site"
draft = false

aliases = ["/knife_cookbook_site.html", "/knife_cookbook_site/"]

[menu]
  [menu.workstation]
    title = "knife cookbook site"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_cookbook_site.md knife cookbook site"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_cookbook_site.md)

{{% supermarket_api_summary %}}

Use the `knife cookbook site` subcommand to interact with cookbooks that
are available in the [Chef Supermarket](https://supermarket.chef.io/). A
user account is required for any community actions that write data to
this site. The following arguments do not require a user account:
`download`, `search`, `install`, and `list`.

{{< warning >}}

`knife cookbook site` has been deprecated in favor of the [knife
supermarket](/workstation/knife_supermarket/) command.

{{< /warning >}}

{{< warning >}}

{{% notes_knife_cookbook_site_use_devkit_berkshelf %}}

{{< /warning >}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## Private Supermarket

To use the `knife cookbook site` command with a private Supermarket
installation, you must first add the URL of your Supermarket to your
`config.rb` file:

``` ruby
knife[:supermarket_site] = 'https://supermarket.example.com'
```

If this value is not specified, knife will use
`https://supermarket.chef.io` by default.

## download

Use the `download` argument to download a cookbook from the community
website. A cookbook will be downloaded as a tar.gz archive and placed in
the current working directory. If a cookbook (or cookbook version) has
been deprecated and the `--force` option is not used, knife will alert
the user that the cookbook is deprecated and then will provide the name
of the most recent non-deprecated version of that cookbook.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site download COOKBOOK_NAME [COOKBOOK_VERSION] (options)
```

### Options

This argument has the following options:

`COOKBOOK_VERSION`

:   The version of a cookbook to be downloaded. If a cookbook has only
    one version, this option does not need to be specified. If a
    cookbook has more than one version and this option is not specified,
    the most recent version of the cookbook is downloaded.

`-f FILE`, `--file FILE`

:   The file to which a cookbook download is written.

`--force`

:   Overwrite an existing directory.

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Download a cookbook**

To download the cookbook `getting-started`, enter:

``` bash
knife cookbook site download getting-started
```

to return something like:

``` bash
Downloading getting-started from the cookbooks site at version 1.2.3 to
  /Users/grantmc/chef-support/getting-started-1.2.3.tar.gz
Cookbook saved: /Users/grantmc/chef-support/getting-started-1.2.3.tar.gz
```

## install

Use the `install` argument to install a cookbook that has been
downloaded from the community site to a local git repository . This
action uses the git version control system in conjunction with the the
[Chef Supermarket](https://supermarket.chef.io/cookbooks) site to
install community-contributed cookbooks to the local chef-repo. Using
this argument does the following:

1.  A new "pristine copy" branch is created in git for tracking the
    upstream.
2.  All existing versions of a cookbook are removed from the branch.
3.  The cookbook is downloaded from the [Chef
    Supermarket](https://supermarket.chef.io/cookbooks) in the tar.gz
    format.
4.  The downloaded cookbook is untarred and its contents are committed
    to git and a tag is created.
5.  The "pristine copy" branch is merged into the master branch.

This process allows the upstream cookbook in the master branch to be
modified while letting git maintain changes as a separate patch. When an
updated upstream version becomes available, those changes can be merged
while maintaining any local modifications.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site install COOKBOOK_NAME [COOKBOOK_VERSION] (options)
```

### Options

This argument has the following options:

`-b`, `--use-current-branch`

:   Ensure that the current branch is used.

`-B BRANCH`, `--branch BRANCH`

:   The name of the default branch. This defaults to the master branch.

`COOKBOOK_VERSION`

:   The version of the cookbook to be installed. If a version is not
    specified, the most recent version of the cookbook is installed.

`-D`, `--skip-dependencies`

:   Ensure that all cookbooks to which the installed cookbook has a
    dependency are not installed.

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Install a cookbook**

To install the cookbook `getting-started`, enter:

``` bash
knife cookbook site install getting-started
```

to return something like:

``` bash
Installing getting-started to /Users/grantmc/chef-support/.chef/../cookbooks
Checking out the master branch.
Creating pristine copy branch chef-vendor-getting-started
Downloading getting-started from the cookbooks site at version 1.2.3 to
  /Users/grantmc/chef-support/.chef/../cookbooks/getting-started.tar.gz
Cookbook saved: /Users/grantmc/chef-support/.chef/../cookbooks/getting-started.tar.gz
Removing pre-existing version.
Uncompressing getting-started version /Users/grantmc/chef-support/.chef/../cookbooks.
removing downloaded tarball
1 files updated, committing changes
Creating tag cookbook-site-imported-getting-started-1.2.3
Checking out the master branch.
Updating 4d44b5b..b4c32f2
Fast-forward
 cookbooks/getting-started/README.rdoc              |    4 +++
 cookbooks/getting-started/attributes/default.rb    |    1 +
 cookbooks/getting-started/metadata.json            |   29 ++++++++++++++++++++
 cookbooks/getting-started/metadata.rb              |    6 ++++
 cookbooks/getting-started/recipes/default.rb       |   23 +++++++++++++++
 .../templates/default/chef-getting-started.txt.erb |    5 +++
 6 files changed, 68 insertions(+), 0 deletions(-)
 create mode 100644 cookbooks/getting-started/README.rdoc
 create mode 100644 cookbooks/getting-started/attributes/default.rb
 create mode 100644 cookbooks/getting-started/metadata.json
 create mode 100644 cookbooks/getting-started/metadata.rb
 create mode 100644 cookbooks/getting-started/recipes/default.rb
 create mode 100644 cookbooks/getting-started/templates/default/chef-getting-started.txt.erb
Cookbook getting-started version 1.2.3 successfully installed
```

## list

Use the `list` argument to view a list of cookbooks that are currently
available at the [Chef
Supermarket](https://supermarket.chef.io/cookbooks).

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site list
```

### Options

This argument has the following options:

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

`-w`, `--with-uri`

:   Show the corresponding URIs.

### Examples

The following examples show how to use this knife subcommand:

**View a list of cookbooks**

To view a list of cookbooks at the [Chef
Supermarket](https://supermarket.chef.io/cookbooks) server, enter:

``` bash
knife cookbook site list
```

to return a list similar to:

``` bash
1password             homesick              rabbitmq
7-zip                 hostname              rabbitmq-management
AmazonEC2Tag          hosts                 rabbitmq_chef
R                     hosts-awareness       rackspaceknife
accounts              htop                  radiant
ack-grep              hudson                rails
activemq              icinga                rails_enterprise
ad                    id3lib                redis-package
ad-likewise           iftop                 redis2
ant                   iis                   redmine
[...truncated...]
```

## search

Use the `search` argument to search for a cookbook at the [Chef
Supermarket](https://supermarket.chef.io/cookbooks). A search query is
used to return a list of cookbooks at the [Chef
Supermarket](https://supermarket.chef.io/cookbooks) and uses the same
syntax as the `knife search` subcommand.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site search SEARCH_QUERY (options)
```

### Options

This argument has the following options:

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

### Examples

The following examples show how to use this knife subcommand:

**Search for cookbooks**

To search for all of the cookbooks that can be used with Apache, enter:

``` bash
knife cookbook site search 'apache*'
```

to return something like:

``` bash
apache2:
  cookbook:             https://supermarket.chef.io/api/v1/cookbooks/apache2
  cookbook_description: Installs and configures apache2
  cookbook_maintainer:  sous-chefs
  cookbook_name:        apache2
apache_hadoop:
  cookbook:             https://supermarket.chef.io/api/v1/cookbooks/apache_hadoop
  cookbook_description: Installs/Configures the Apache Hadoop distribution
  cookbook_maintainer:  dowlingj
  cookbook_name:        apache_hadoop
apache_kafka:
  cookbook:             https://supermarket.chef.io/api/v1/cookbooks/apache_kafka
  cookbook_description: Installs/Configures Apache Kafka >= 0.7.0
  cookbook_maintainer:  mathyourlife
  cookbook_name:        apache_kafka
[...truncated...]
```

## share

Use the `share` argument to add a cookbook to the [Chef
Supermarket](https://supermarket.chef.io/cookbooks). This action will
require a user account and a certificate for [Chef
Supermarket](https://supermarket.chef.io/). By default, knife will use
the user name and API key that is identified in the configuration file
used during the upload; otherwise these values must be specified on the
command line or in an alternate configuration file. If a cookbook
already exists on the [Chef
Supermarket](https://supermarket.chef.io/cookbooks), then only an owner
or maintainer of that cookbook can make updates.

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site share COOKBOOK_NAME CATEGORY (options)
```

### Options

This argument has the following options:

`CATEGORY`

:   The cookbook category: `"Databases"`, `"Web Servers"`,
    `"Process Management"`, `"Monitoring & Trending"`,
    `"Programming Languages"`, `"Package Management"`, `"Applications"`,
    `"Networking"`, `"Operating Systems & Virtualization"`,
    `"Utilities"`, or `"Other"`.

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

`-n`, `--dry-run`

:   Take no action and only print out results. Default: `false`.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

{{< note >}}

{{% knife_common_see_all_config_options %}}

{{< /note >}}

### Examples

The following examples show how to use this knife subcommand:

**Share a cookbook**

To share a cookbook named `apache2`:

``` bash
knife cookbook site share "apache2" "Web Servers"
```

## show

Use the `show` argument to view information about a cookbook on the
[Chef Supermarket](https://supermarket.chef.io/cookbooks).

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site show COOKBOOK_NAME [COOKBOOK_VERSION]
```

### Options

This argument has the following options:

`COOKBOOK_VERSION`

:   The version of a cookbook to be shown. If a cookbook has only one
    version, this option does not need to be specified. If a cookbook
    has more than one version and this option is not specified, a list
    of cookbook versions is returned.

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

### Examples

The following examples show how to use this knife subcommand:

**Show cookbook data**

To show the details for a cookbook named `haproxy`:

``` bash
knife cookbook site show haproxy
```

to return something like:

``` bash
average_rating:
category:        Other
created_at:      2009-10-25T23:51:07.000Z
deprecated:      false
description:     Installs and configures haproxy
external_url:    https://github.com/sous-chefs/haproxy
issues_url:      https://github.com/sous-chefs/haproxy/issues
latest_version:  https://supermarket.chef.io/api/v1/cookbooks/haproxy/versions/6.2.3
maintainer:      sous-chefs
metrics:
  collaborators: 3
  downloads:
    total:    29114892
    versions:
      0.7.0: 1258890
      0.8.0: 1258804
      [...truncated...]
  followers:     139
name:            haproxy
source_url:      https://github.com/sous-chefs/haproxy
up_for_adoption:
updated_at:      2018-08-08T20:09:52.334Z
versions:
  https://supermarket.chef.io/api/v1/cookbooks/haproxy/versions/6.2.3
  https://supermarket.chef.io/api/v1/cookbooks/haproxy/versions/6.2.2
  [...truncated...]
```

**Show cookbook data as JSON**

To view information in JSON format, use the `-F` common option as part
of the command like this:

``` bash
knife cookbook site show devops -F json
```

Other formats available include `text`, `yaml`, and `pp`.

## unshare

Use the `unshare` argument to stop the sharing of a cookbook at the
[Chef Supermarket](https://supermarket.chef.io/cookbooks). Only the
maintainer of a cookbook may perform this action.

{{< note >}}

Unsharing a cookbook will break a cookbook that has set a dependency on
that cookbook or cookbook version.

{{< /note >}}

### Syntax

This argument has the following syntax:

``` bash
knife cookbook site unshare COOKBOOK_NAME/versions/VERSION
```

### Options

This argument has the following options:

`-m SUPERMARKET_SITE`, `--supermarket-site SUPERMARKET_SITE`

:   The URL at which the Chef Supermarket is located. Default value:
    <https://supermarket.chef.io>.

### Examples

The following examples show how to use this knife subcommand:

**Unshare a cookbook**

To unshare a cookbook named `getting-started`, enter:

``` bash
knife cookbook site unshare "getting-started"
```

**Unshare a cookbook version**

To unshare cookbook version `0.10.0` for the `getting-started` cookbook,
enter:

``` bash
knife cookbook site unshare "getting-started/versions/0.10.0"
```
