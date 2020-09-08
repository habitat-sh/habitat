+++
title = "About the chef-repo"
draft = false

aliases = ["/chef_repo.html"]

[menu]
  [menu.infra]
    title = "chef-repo"
    identifier = "chef_infra/concepts/chef_repo.md chef-repo"
    parent = "chef_infra/concepts"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_repo.md)

{{% chef_repo_description %}}

## Directory Structure

The chef-repo contains several directories, each with a README file that
describes what it is for and how to use that directory when managing
systems.

The sub-directories in the chef-repo are:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Directory</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>.chef/</code></td>
<td>A hidden directory that is used to store validation key files and optionally a <a href="/config_rb/">config.rb</a> file.</td>
</tr>
<tr class="even">
<td><code>cookbooks/</code></td>
<td>Contains cookbooks that have been downloaded from the <a href="https://supermarket.chef.io/">Chef Supermarket</a> or created locally.</td>
</tr>
<tr class="odd">
<td><code>data_bags/</code></td>
<td>Stores data bags (and data bag items) in JSON (.json) format.</td>
</tr>
<tr class="even">
<td><code>policyfiles/</code></td>
<td>Stores Policyfiles in Ruby (.rb) format.</td>
</tr>
</tbody>
</table>

### .chef/

{{% all_directory_chef %}}

### cookbooks/

The `cookbooks/` directory is used to store the cookbooks that Chef
Infra Client uses when configuring the various systems in the
organization. This directory contains the cookbooks that are used to
configure systems in the infrastructure. Each cookbook can be configured
to contain cookbook-specific copyright, email, and license data.

### data_bags/

The `data_bags/` directory is used to store all of the data bags that
exist for an organization. Each sub-directory corresponds to a single
data bag on the Chef Infra Server and contains a JSON file for each data
bag item. If a sub-directory does not exist, then create it using SSL
commands. After a data bag item is created, it can then be uploaded to
the Chef Infra Server.

### policyfiles/

The `policyfiles/` directory is used to store Policyfiles that define
the set of cookbooks and attributes to apply to specific systems managed
by the Chef Infra Server.

## chefignore Files

The chefignore file is used to tell knife which cookbook files in the
chef-repo should be ignored when uploading data to the Chef Infra
Server. The type of data that should be ignored includes swap files,
version control data, build output data, and so on. The chefignore file
uses the `File.fnmatch` Ruby syntax to define the ignore patterns using
`*`, `**`, and `?` wildcards.

-   A pattern is relative to the cookbook root
-   A pattern may contain relative directory names
-   A pattern may match all files in a directory

The chefignore file can be located in any subdirectory of a chef-repo:
`/`, `/cookbooks`, `/cookbooks/COOKBOOK_NAME/`, etc. It should contain
sections similar to the following:

``` none
## section
*ignore_pattern

## section
ignore_pattern*

## section
**ignore_pattern

## section
ignore_pattern**

## section
?ignore_pattern

## section
ignore_pattern?
```

### Examples

The following examples show how to add entries to the `chefignore` file.

**Ignore editor swap files**

Many text editors leave files behind. To prevent these files from being
uploaded to the Chef Infra Server, add an entry to the chefignore file.
For Emacs, do something like:

``` none
*~
```

and for vim, do something like:

``` none
*.sw[a-z]
```

**Ignore top-level Subversion data**

If using Subversion as the version source control application, it is
important not to upload certain files that Subversion uses to maintain
the version history of each file. This is because Chef Infra Client will
never use it while configuring nodes, plus the amount of data in an
upload that includes top-level Subversion data could be significant.

To prevent the upload of top-level Subversion data, add something like
the following to the chefignore file:

``` none
*/.svn/*
```

To verify that the top-level Subversion data is not being uploaded to
the Chef Infra Server, use knife and run a command similar to:

``` bash
knife cookbook show name_of_cookbook cookbook_version | grep .svn
```

**Ignore all files in a directory**

The chefignore file can be used to ignore all of the files in a
directory. For example:

``` none
files/default/subdirectory/*
```

or:

``` none
files/default/subdirectory/**
```

## Many Users, Same Repo

{{% chef_repo_many_users_same_knife %}}

## Create the chef-repo

Use the [chef generate repo command](/ctl_chef/#chef-generate-repo)
to create your chef-repo directory along with the base folder structure.
This command uses the `chef` command-line tool that is packaged as part
of Chef Workstation to create a chef-repo.

``` bash
chef generate repo REPO_NAME
```

{{< note >}}

By default the chef generate repo command generates a base chef-repo
configured for Policyfiles. To create a repository setup for Roles and
Environments instead use the `--roles` flag when running the command.

{{< /note >}}
