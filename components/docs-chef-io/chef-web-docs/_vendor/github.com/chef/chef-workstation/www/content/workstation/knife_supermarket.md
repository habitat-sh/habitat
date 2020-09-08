+++
title = "knife supermarket"
draft = false

aliases = ["/knife_supermarket.html", "/plugin_knife_supermarket.html", "/knife_supermarket/"]

[menu]
  [menu.workstation]
    title = "knife supermarket"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_supermarket.md knife supermarket"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_supermarket.md)

The `knife supermarket` subcommand is used to interact with cookbooks
that are located in on the public Supermarket as well as private Chef
Supermarket sites. A user account is required for any community actions
that write data to the Chef Supermarket; however, the following
arguments do not require a user account: `download`, `search`,
`install`, and `list`.

{{< note >}}

{{% notes_knife_cookbook_site_use_devkit_berkshelf %}}

{{< /note >}}

{{< note >}}

{{% knife_common_see_common_options_link %}}

{{< /note >}}

## download

Use the `download` argument to download a cookbook from Chef
Supermarket. A cookbook will be downloaded as a tar.gz archive and
placed in the current working directory. If a cookbook (or cookbook
version) has been deprecated and the `--force` option is not used, knife
will alert the user that the cookbook is deprecated and then will
provide the name of the most recent non-deprecated version of that
cookbook.

### Syntax

This argument has the following syntax:

``` bash
knife supermarket download COOKBOOK_NAME [COOKBOOK_VERSION] (options)
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

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

### Examples

The following examples show how to use this knife subcommand:

**Download a cookbook**

To download the cookbook `mysql`, enter:

``` bash
knife supermarket download mysql
```

## install

Use the `install` argument to install a cookbook that has been
downloaded from Chef Supermarket to a local git repository . This action
uses the git version control system in conjunction with Chef Supermarket
site to install community-contributed cookbooks to the local chef-repo.
Using this argument does the following:

1.  A new "pristine copy" branch is created in git for tracking the
    upstream.
2.  All existing versions of a cookbook are removed from the branch.
3.  The cookbook is downloaded from Chef Supermarket in the tar.gz
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
knife supermarket install COOKBOOK_NAME [COOKBOOK_VERSION] (options)
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

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

### Examples

The following examples show how to use this knife subcommand:

**Install a cookbook**

To install the cookbook `mysql`, enter:

``` bash
knife supermarket install mysql
```

## list

Use the `list` argument to view a list of cookbooks that are currently
available at Chef Supermarket.

### Syntax

This argument has the following syntax:

``` bash
knife supermarket list (options)
```

### Options

This argument has the following options:

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

`-w`, `--with-uri`

:   Show the corresponding URIs.

### Examples

The following examples show how to use this knife subcommand:

**View a list of cookbooks**

To view a list of cookbooks at the [Chef
Supermarket](https://supermarket.chef.io/cookbooks) server, enter:

``` bash
knife supermarket list
```

to return a list similar to:

``` bash
1password                            minecraft
301                                  mineos
7-zip                                minidlna
AWS_see_spots_run                    minitest
AmazonEC2Tag                         minitest-handler
Appfirst-Cookbook                    mirage
CVE-2014-3566-poodle                 mlocate
CVE-2015-0235                        mod_security
Obfsproxy                            mod_security2
R                                    modcloth-hubot
Rstats                               modcloth-nad
SysinternalsBginfo                   modman
VRTSralus                            modules
abiquo                               mogilefs
acadock                              mongodb
accel-ppp                            mongodb-10gen
accounts                             mongodb-agents
accumulator                          monit
...
```

## search

Use the `search` argument to search for a cookbooks located at Chef
Supermarket. A search query is used to return a list of these cookbooks
and uses the same syntax as the `knife search` subcommand.

### Syntax

This argument has the following syntax:

``` bash
knife supermarket search SEARCH_QUERY (options)
```

### Options

This argument has the following options:

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

### Examples

The following examples show how to use this knife subcommand:

**Search for cookbooks**

To search for a cookbook, use a command similar to:

``` bash
knife supermarket search mysql
```

where `mysql` is the search term. This will return something similar to:

``` bash
mysql:
  cookbook:             https://supermarket.chef.io/api/v1/cookbooks/mysql
  cookbook_description: Provides mysql_service, mysql_config, and mysql_client resources
  cookbook_maintainer:  chef
  cookbook_name:        mysql
mysql-apt-config:
  cookbook:             https://supermarket.chef.io/api/v1/cookbooks/mysql-apt-config
  cookbook_description: Installs/Configures mysql-apt-config
  cookbook_maintainer:  tata
  cookbook_name:        mysql-apt-config
mysql-multi:
  cookbook:             https://supermarket.chef.io/api/v1/cookbooks/mysql-multi
  cookbook_description: MySQL replication wrapper cookbook
  cookbook_maintainer:  rackops
  cookbook_name:        mysql-multi
```

## share

Use the `share` argument to add a cookbook to Chef Supermarket. This
action will require a user account and a certificate for [Chef
Supermarket](https://supermarket.chef.io/). By default, knife will use
the user name and API key that is identified in the configuration file
used during the upload; otherwise these values must be specified on the
command line or in an alternate configuration file. If a cookbook
already exists in Chef Supermarket, then only an owner or maintainer of
that cookbook can make updates.

### Syntax

This argument has the following syntax:

``` bash
knife supermarket share COOKBOOK_NAME CATEGORY (options)
```

### Options

This argument has the following options:

`CATEGORY`

:   The cookbook category: `"Databases"`, `"Web Servers"`,
    `"Process Management"`, `"Monitoring & Trending"`,
    `"Programming Languages"`, `"Package Management"`, `"Applications"`,
    `"Networking"`, `"Operating Systems & Virtualization"`,
    `"Utilities"`, or `"Other"`.

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

`-o PATH:PATH`, `--cookbook-path PATH:PATH`

:   The directory in which cookbooks are created. This can be a
    colon-separated path.

### Examples

The following examples show how to use this knife subcommand:

**Share a cookbook**

To share a cookbook named `my_apache2_cookbook` and add it to the
`Web Servers` category in Chef Supermarket:

``` bash
knife supermarket share "my_apache2_cookbook" "Web Servers"
```

## show

Use the `show` argument to view information about a cookbook located at
Chef Supermarket.

### Syntax

This argument has the following syntax:

``` bash
knife supermarket show COOKBOOK_NAME [COOKBOOK_VERSION] (options)
```

### Options

This argument has the following options:

`COOKBOOK_VERSION`

:   The version of a cookbook to be shown. If a cookbook has only one
    version, this option does not need to be specified. If a cookbook
    has more than one version and this option is not specified, a list
    of cookbook versions is returned.

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

### Examples

The following examples show how to use this knife subcommand:

**Show cookbook data**

To show the details for a cookbook named `mysql`:

``` bash
knife supermarket show mysql
```

to return something similar to:

``` bash
average_rating:
category:        Other
created_at:      2009-10-28T19:16:54.000Z
deprecated:      false
description:     Provides mysql_service, mysql_config, and mysql_client resources
external_url:    https://github.com/chef-cookbooks/mysql
issues_url:      https://github.com/chef-cookbooks/mysql/issues
latest_version:  https://supermarket.chef.io/api/v1/cookbooks/mysql/versions/8.5.1
maintainer:      sous-chefs
metrics:
  collaborators: 2
  downloads:
    total:    128998032
  versions:
    0.10.0: 927561
    0.15.0: 927536
    0.20.0: 927321
    0.21.0: 927298
    0.21.1: 927311
    0.21.2: 927424
    0.21.3: 927441
    0.21.5: 927326
    0.22.0: 927297
    0.23.0: 927353
    0.23.1: 927862
    0.24.0: 927316
```

**Show cookbook version data**

To show the details for a cookbook version, run a command similar to:

``` bash
knife supermarket show mysql 8.5.1
```

where `mysql` is the cookbook and `8.5.1` is the cookbook version. This
will return something similar to:

``` bash
average_rating:
cookbook:          https://supermarket.chef.io/api/v1/cookbooks/mysql
file:              https://supermarket.chef.io/api/v1/cookbooks/mysql/versions/8.5.1/download
license:           Apache-2.0
published_at:      2017-08-23T19:01:28Z
quality_metrics:
  failed:   false
  feedback: passed the No Binaries metric. Contains no obvious binaries.
  name:     No Binaries

  failed:   false
  feedback: mysql passed the publish metric
  name:     Publish

  failed:   false
  feedback: mysql supports at least one platform.
  name:     Supported Platforms

  failed:   false
  feedback: passed the Collaborators Metric with 2 collaborators.
  name:     Collaborator Number

  failed:   false
  feedback:
  Run with Foodcritic Version 14.0.0 with tags metadata,correctness ~FC031 ~FC045 and failure tags any
  name:     Foodcritic

  failed:   false
  feedback: passed the CONTRIBUTING.md file metric.
  name:     Contributing File

  failed:   false
  feedback: passed the version tag metric.
  name:     Version Tag

  failed:   false
  feedback: passed the TESTING.md file metric.
  name:     Testing File
supports:
  amazon:       >= 0.0.0
  centos:       >= 6.0
  debian:       >= 7.0
  fedora:       >= 0.0.0
  opensuseleap: >= 0.0.0
  oracle:       >= 6.0
  redhat:       >= 6.0
  scientific:   >= 6.0
  suse:         >= 12.0
  ubuntu:       >= 12.04
tarball_file_size: 23763
version:           8.5.1
```

## unshare

Use the `unshare` argument to stop the sharing of a cookbook located at
Chef Supermarket. Only the maintainer of a cookbook may perform this
action.

{{< note >}}

Unsharing a cookbook will break a cookbook that has set a dependency on
that cookbook or cookbook version.

{{< /note >}}

### Syntax

This argument has the following syntax:

``` bash
knife supermarket unshare COOKBOOK_NAME/versions/VERSION (options)
```

### Options

This argument has the following options:

`-m`, `--supermarket-site`

:   The URL at which the Chef Supermarket is located. Default value:
    `https://supermarket.chef.io`.

### Examples

The following examples show how to use this knife subcommand:

**Unshare a cookbook**

To unshare a cookbook named `my_apache2_cookbook`, enter:

``` bash
knife supermarket unshare "my_apache2_cookbook" "Web Servers"
```

**Unshare a cookbook version**

To unshare cookbook version `0.10.0` for the `my_apache2_cookbook`
cookbook, enter:

``` bash
knife supermarket unshare "my_apache2_cookbook/versions/0.10.0"
```
