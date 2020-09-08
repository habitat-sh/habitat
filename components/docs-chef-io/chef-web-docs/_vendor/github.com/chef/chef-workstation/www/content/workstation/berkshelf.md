+++
title = "About Berkshelf"
draft = false

aliases = ["/berkshelf.html", "/berkshelf/"]

[menu]
  [menu.workstation]
    title = "Berkshelf"
    identifier = "chef_workstation/chef_workstation_tools/berkshelf.md Berkshelf"
    parent = "chef_workstation/chef_workstation_tools"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/berkshelf.md)

Berkshelf is a dependency manager for Chef cookbooks. With it, you can
easily depend on community cookbooks and have them safely included in
your workflow. You can also ensure that your CI systems reproducibly
select the same cookbook versions, and can upload and bundle cookbook
dependencies without needing a locally maintained copy. Berkshelf is
included in Chef Workstation.

{{< note >}}

For new users, we strongly recommend using
[Policyfiles](/policyfile/) rather than Berkshelf. Policyfiles
provide more predictability, since dependencies are only resolved once,
and a much improved way of promoting cookbooks from dev to testing, and
then to production.

{{< /note >}}

## Quick Start

Run `chef generate cookbook -b` or `--berks` to create a Berksfile in
the root of the cookbook. The Berksfile will be placed alongside the
cookbook's metadata.rb file. As usual, add your cookbook's dependencies
to the metadata.rb file:

``` ruby
name 'my_first_cookbook'
version '0.1.0'
depends 'apt', '~> 5.0'
```

The default `Berksfile` will contain the following:

``` ruby
source 'https://supermarket.chef.io'
metadata
```

Now, when you run `berks install`, the apt cookbook will be downloaded
from Supermarket into the cache:

``` shell
berks install
Resolving cookbook dependencies...
Fetching 'my_first_cookbook' from source at .
Fetching cookbook index from https://supermarket.chef.io...
Installing apt (5.0.0)
Using my_first_cookbook (0.1.0) from source at .
Installing compat_resource (12.16.2)
```

In this example, the `compat_resource` cookbook is also installed since
it's a dependency of the `apt` cookbook. Running the install command
also creates a `Berksfile.lock`, which represents exactly which cookbook
versions Berkshelf installed. This file ensures that someone else can
check the cookbook out of git and get exactly the same dependencies as
you.

You can now upload all cookbooks to your Chef Infra Server with
`berks upload`:

``` shell
berks upload
Uploaded apt (5.0.0) to: 'https://api.chef.io:443/organizations/example'
Uploaded compat_resource (12.16.2) to: 'https://api.chef.io:443/organizations/example'
Uploaded my_first_cookbook (0.1.0) to: 'https://api.chef.io:443/organizations/example'
```

## The Berksfile

A Berksfile describes the set of sources and dependencies needed to use
a cookbook. It is used in conjunction with the `berks` command.

### Syntax

A Berksfile is a Ruby file, in which sources, dependencies, and options
may be specified. Berksfiles are modeled closely on Bundler's Gemfile.
The syntax is as follows:

``` ruby
source "https://supermarket.chef.io"
metadata
cookbook "NAME" [, "VERSION_CONSTRAINT"] [, SOURCE_OPTIONS]
```

#### Source Keyword

A source defines where Berkshelf should look for cookbooks. Sources are
processed in the order that they are defined in, and processing stops as
soon as a suitable cookbook is found. Locations include a private or
public [Supermarket](/supermarket/), Chef Infra Server, or local
Chef repository.

By default, a Berksfile has a source for Chef's public Supermarket:

``` ruby
source "https://supermarket.chef.io"
```

To add a private Supermarket, which will be preferred:

``` ruby
source "https://supermarket.example.com"
source "https://supermarket.chef.io"
```

To add a Chef Infra Server:

``` ruby
source "https://supermarket.chef.io"
source :chef_server
```

To add a local Chef repository:

``` ruby
source "https://supermarket.chef.io"
source chef_repo: ".."
```

The location and authentication details for the Chef Infra Server will
be taken from the user's `config.rb` by default.

To add multiple Chef Infra Servers:

``` ruby
source "https://supermarket.chef.io"
source chef_server: "https://mychefserver.example.com/organizations/one", client_name: "one-client", client_key: "/path/to/one-client.pem"
source chef_server: "https://mychefserver.example.com/organizations/two", client_name: "two-client", client_key: "/path/to/two-client.pem"
```

To use an [Artifactory
server](https://www.jfrog.com/confluence/display/RTF/Chef+Cookbook+Repositories)
as a source:

``` ruby
source artifactory: "https://artifactory.example.com/api/chef/nameofrepo", api_key: "XZg6tefUZYfiglmSbQ1s1e6habk"
```

If the `api_key` option is not given, it will use the value of the
`$ARTIFACTORY_API_KEY` environment variable by default.

#### Metadata Keyword

The `metadata` keyword causes Berkshelf to process the local cookbook
metadata. This ensures that the dependencies of the cookbook are
resolved by Berkshelf. Using the `metadata` keyword requires that the
Berksfile be placed in the root of the cookbook, next to `metadata.rb`.

#### Cookbook Keyword

The `cookbook` keyword allows the user to define where a cookbook is
installed from, or to set additional version constraints. It can also be
used to install additional cookbooks, for example to use during testing.

The format of a `cookbook` stanza is as follows:

``` ruby
cookbook "NAME" [, "VERSION_CONSTRAINT"] [, SOURCE_OPTIONS]
```

The simplest form is:

``` ruby
cookbook "library-cookbook"
```

This ensures that a cookbook named `library-cookbook` is installed by
berkshelf.

Version constraints are the second parameter:

``` ruby
cookbook "library-cookbook", "~> 0.1.1"
```

These are identical to the version constraints in a [cookbook metadata
file](/config_rb_metadata/#cookbook-version-constraints).

Source options are used to specify the location to acquire a cookbook
from, or to place a cookbook in a group. By default, cookbooks are
acquired from the default sources, but it's possible to override this on
a case by case basis. Often this is used to get a development cookbook
from Git, or to use another cookbook in a monolithic cookbook
repository.

**Path Location**

The path location enables Berkshelf to use a cookbook located on the
same system. It does not cache the target cookbook, ensuring that the
latest version is always used. The target must be a single cookbook with
a `metadata.rb`.

``` ruby
cookbook "library-cookbook", "~> 0.1.1", path: "../library-cookbook"
```

**Git Location**

The git location enables Berkshelf to use acquire a cookbook from a git
repository.

``` ruby
cookbook "library-cookbook", "~> 0.1.1", git: "https://github.com/example/library-cookbook.git"
```

The user can specify a git branch or a tag (the options are synonymous)
using an optional argument:

``` ruby
cookbook "library-cookbook", "~> 0.1.1", git: "https://github.com/example/library-cookbook.git", branch: "smartos-dev"
cookbook "library-cookbook", "~> 0.1.1", git: "https://github.com/example/library-cookbook.git", tag: "1.2.3"
```

The user can also specify a revision:

``` ruby
cookbook "library-cookbook", "~> 0.1.1", git: "https://github.com/example/library-cookbook.git", ref: "eef7e65806e7ff3bdbe148e27c447ef4a8bc3881"
```

If a git repository contains many cookbooks, the user can specify the
path to the desired cookbook using the `rel` option:

``` ruby
cookbook "library-cookbook", "~> 0.1.1", git: "https://github.com/example/cookbook-repo.git", rel: "library-cookbook"
```

**GitHub Location**

If a cookbook is in GitHub, you can use the `github:` shorthand to refer
to it:

``` ruby
cookbook "library-cookbook", "~> 0.1.1", github: "example/library-cookbook"
```

Any other git options are valid for a GitHub location.

#### Groups

Adding cookbooks to a group is useful should you wish to exclude certain
cookbooks from upload or vendoring.

Groups can be defined via blocks:

``` ruby
group :test do
  cookbook "test-cookbook", path: "test/fixtures/test"
end
```

Groups can also be specified inline:

``` ruby
cookbook "test-cookbook", path: "test/fixtures/test", group: :test
```

To exclude a group when using `berks`, use the `--except` flag:

``` bash
berks install --except test
```

#### Solver Keyword

It is possible to configure which engine to use for the
[solve](https://github.com/berkshelf/solve) dependency resolution
system.

By default, the solver selection depends on your environment. When the
`dep_selector` gem is installed, as in the case of Chef Workstation, the
`gecode` solver is used. Otherwise, the `ruby` solver is utilized by
default.

The `gecode` solver matches the engine used by the Chef Infra Server, so
will more closely reflect the behavior of the Chef Infra Server in
selecting cookbooks:

``` ruby
solver :gecode
```

The `ruby` solver can give better results in some situations, notably
when Berkshelf times out when trying to build a dependency set.

``` ruby
solver :ruby
```

## Berkshelf CLI

The Berkshelf CLI is the interface to Berkshelf.

### Common Options

`-c PATH`, `--config PATH`

:   The path to the Berkshelf configuration file.

`-d`, `--debug`

:   Use to print debug information. Default value: `false`.

`-F JSON`, `--format JSON`

:   Use to specify the output format to be used. Default value: `human`
    Possible values: `base`, `human`, `json`, and `null`.

`-q`, `--quiet`

:   Use to silence all informational output. Default value: `false`.

### berks apply

Use `berks apply` to apply Berksfile version locks to the named
environment on the Chef Infra Server.

#### Syntax

This subcommand has the following syntax:

``` bash
berks apply ENVIRONMENT (options)
```

#### Options

This command has the following options:

`-b LOCK_FILE_PATH`, `--lockfile LOCK_FILE_PATH`

:   The path to the Berksfile lock file from which Berksfile version
    locks are applied.

`-f JSON_FILE_PATH`, `--envfile PATH`

:   The path to an environment file (in JSON format) to which Berksfile
    version locks are applied.

`--ssl-verify`

:   Use to enable (`true`) or disable (`false`) SSL verification when
    applying Berksfile version locks to an environment.

### berks contingent

Use `berks contingent` to list all cookbooks in a Berksfile that depend
on the named cookbook.

#### Syntax

This subcommand has the following syntax:

``` bash
berks contingent COOKBOOK (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the cookbook is located.

### berks cookbook

Use `berks cookbook` to create a skeleton for a new cookbook.

{{< warning >}}

This command is deprecated. Please use `chef generate cookbook` instead.

{{< /warning >}}

### berks info

Use `berks info` to display the name, author, copyright, and dependency
information for the named cookbook.

#### Syntax

This subcommand has the following syntax:

``` bash
berks info COOKBOOK (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the cookbook is located.

### berks init

Use `berks init` to initialize Berkshelf to the specified directory.

{{< warning >}}

This command is deprecated. Please use `chef generate cookbook` instead.

{{< /warning >}}

### berks install

Use `berks install` to install cookbooks into the cache. This command
generates the Berkshelf lock file that ensures consistency.

#### Syntax

This subcommand has the following syntax:

``` bash
berks install (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the cookbook is located.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be listed.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be listed. When this option is used,
    cookbooks that exist in groups not listed will not be listed.

### berks list

Use `berks list` to list cookbooks and their dependencies.

#### Syntax

This subcommand has the following syntax:

``` bash
berks list (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the cookbook is located.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be listed.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be listed. When this option is used,
    cookbooks that exist in groups not listed will not be listed.

### berks outdated

Use `berks outdated` to list dependencies for the named cookbook, and
then check if there are new versions available for version constraints
that may exist.

#### Syntax

This subcommand has the following syntax:

``` bash
berks outdated COOKBOOK (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the cookbook is located.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be checked for version
    constraints.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be checked for version constraints.
    When this option is used, cookbooks that exist in groups not listed
    will not be checked for version constraints.

### berks package

Use `berks package` to vendor, and then archive the dependencies of a
Berksfile.

#### Syntax

This subcommand has the following syntax:

``` bash
berks package PATH (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile to be vendored, and then archived.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be vendored, and then
    archived.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be vendored, and then archived. When
    this option is used, cookbooks that exist in groups not listed will
    not be vendored or archived.

### berks search

Use `berks search` to search the remote source for cookbooks that match
the search query. The query itself will match partial cookbook names.

#### Syntax

This subcommand has the following syntax:

``` bash
berks search QUERY (options)
```

#### Options

This command has the following options:

`--source URL`

:   The URL at which remote cookbooks are located. Default value:
    `https://supermarket.chef.io`.

### berks show

Use `berks show` to show the path to the named cookbook.

#### Syntax

This subcommand has the following syntax:

``` bash
berks show COOKBOOK (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the named cookbook is defined.

### berks update

Use `berks update` to update the named cookbook or cookbooks (and any
dependencies).

#### Syntax

This subcommand has the following syntax:

``` bash
berks update COOKBOOK (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the named cookbook is defined.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be updated.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be updated. When this option is used,
    cookbooks that exist in groups not listed will not be updated.

### berks upload

Use `berks upload` to upload the named cookbook to the Chef Infra
Server.

#### Syntax

This subcommand has the following syntax:

``` bash
berks upload COOKBOOK (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile in which the named cookbook is defined.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be uploaded.

`--force`

:   Use to upload any named cookbook even if that cookbook exists on the
    Chef Infra Server and is frozen.

`--halt-on-frozen`

:   Use to exit the command with a non-zero exit code if this version of
    a cookbook already exists on the Chef Infra Server.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be uploaded. When this option is
    used, cookbooks that exist in groups not listed will not be
    uploaded.

`--no-freeze`

:   A frozen cookbook requires changes to that cookbook to be submitted
    as a new version of that cookbook. Use this option to prevent this
    cookbook from being frozen. Default value: `false` (i.e. "frozen").

`--ssl-verify`

:   Use to enable (`true`) or disable (`false`) SSL verification when
    uploading cookbooks to the Chef Infra Server.

`-s`, `--skip-syntax-check`

:   Use to skip Ruby syntax checking when uploading a cookbook to the
    Chef Infra Server. Default value: `false`.

### berks vendor

Use `berks vendor` to vendor groups of cookbooks (as specified by group
name) into a directory.

#### Syntax

This subcommand has the following syntax:

``` bash
berks vendor PATH (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile from which cookbooks will be vendored.

`--delete`

:   Use to clean the directory in which vendored cookbooks will be
    placed prior to executing this command.

`-e [GROUP, GROUP, ...]`, `--except [GROUP, GROUP, ...]`

:   An array of cookbook groups that will not be vendored.

`-o [GROUP, GROUP, ...]`, `--only [GROUP, GROUP, ...]`

:   An array of cookbook groups to be vendored. When this option is
    used, cookbooks that exist in groups not listed will not be
    vendored.

### berks verify

Use `berks verify` to perform a validation of the contents of resolved
cookbooks.

#### Syntax

This subcommand has the following syntax:

``` bash
berks verify (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile from which resolved cookbooks are
    validated.

### berks version

Use `berks version` to display the version of Berkshelf.

### berks viz

Use `berks viz` to generate a dependency graph image file.

#### Syntax

This subcommand has the following syntax:

``` bash
berks viz (options)
```

#### Options

This command has the following options:

`-b PATH`, `--berksfile PATH`

:   The path to the Berksfile for which the dependency graph is built.

`-o NAME`, `--outfile NAME`

:   The name of the file to which output is saved. Default value:
    `graph.png`.
