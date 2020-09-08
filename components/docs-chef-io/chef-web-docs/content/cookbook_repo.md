+++
title = "Cookbook Directory"
draft = false

aliases = ["/cookbook_repo.html"]

[menu]
  [menu.infra]
    title = "Cookbook Repo"
    identifier = "chef_infra/cookbook_reference/cookbook_repo.md Cookbook Repo"
    parent = "chef_infra/cookbook_reference"
    weight = 100
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/cookbook_repo.md)

The `cookbooks/` directory of your Chef Infra repository is used to
store the cookbooks that Chef Infra Client uses in configuring the
various systems in the organization.

## Working with Cookbooks

Use the following knife subcommands to create, install, and/or download
cookbooks.

### Create

To create a cookbook (including all default components), run the
following command:

``` bash
chef generate cookbook COOKBOOK_NAME
```

where `COOKBOOK_NAME` is the name of the cookbook that will be created.
Any unneeded directory components can be left unused or deleted, if
preferred.

### Install

To download a cookbook when git is used for version source control, run
the following command:

``` bash
knife supermarket install COOKBOOK_NAME
```

where `COOKBOOK_NAME` is the name of a cookbook on [Chef
Supermarket](https://supermarket.chef.io/). This will start a process
that:

-   downloads the cookbook from [Chef
    Supermarket](https://supermarket.chef.io/) as a tar.gz archive
-   ensures that its using the git master branch, and then checks out
    the cookbook from a vendor branch (creating a new vendor branch, if
    required)
-   removes the old (existing) version
-   expands the tar.gz archive and adds the expanded files to the git
    index and commits
-   creates a tag for the version that was downloaded
-   checks out the master branch
-   merges the cookbook into the master (to ensure that any local
    changes or modifications are preserved)

### Download

To download a cookbook when git is not used for version source control,
run the following command:

``` bash
knife supermarket download COOKBOOK_NAME
```

where `COOKBOOK_NAME` is the name of a cookbook on [Chef
Supermarket](https://supermarket.chef.io/). This will download the
tar.gz file associated with the cookbook and will create a file named
`COOKBOOK_NAME.tar.gz` in the current directory (e.g., `~/chef-repo`).
Once downloaded, using a version source control system is recommended.

## Cookbook Metadata

{{% cookbooks_metadata %}}

Each cookbook can be configured to contain cookbook-specific copyright,
email, and license data which is stored in the
[metadata.rb](/config_rb_metadata/) file.

You can configure default values for the copyright, email, and license
of new cookbooks by adding the following to the config.rb file in the
chef-repo:

``` bash
cookbook_copyright "Example, Com."
cookbook_email     "cookbooks@example.com"
cookbook_license   "apachev2"
```

where the `cookbook_copyright` and `cookbook_email` are specific to the
organization and `cookbook_license` is either `apachev2` or `none`.
These settings will be used in the default recipe and in corresponding
values in the metadata.rb file, but can be modified in those locations
as well (if they should be different from the default values contained
in the config.rb file.)

For a full explanation of working with cookbook metadata, see
[metadata.rb](/config_rb_metadata/).
