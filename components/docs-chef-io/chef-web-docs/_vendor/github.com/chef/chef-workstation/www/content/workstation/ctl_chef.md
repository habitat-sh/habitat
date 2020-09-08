+++
title = "chef (executable)"
draft = false

aliases = ["/ctl_chef.html", "/ctl_chef/"]

[menu]
  [menu.workstation]
    title = "chef executable (full page)"
    identifier = "chef_workstation/chef_workstation_tools/chef_(executable)/ctl_chef.md chef (executable)"
    parent = "chef_workstation/chef_workstation_tools/chef_(executable)"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/workstation/blob/master/www/content/workstation/ctl_chef.md)

The chef executable is a command-line tool that does the following:

-   Generates repositories, cookbooks, recipes, attributes, templates,
    and custom resources.
-   Installs gems into the Chef development environment's Ruby
    installation.

## chef capture

Use the `chef capture` subcommand to capture a node's state as a local [chef-repo](/chef_repo/). Use the generated repository if you need to converge a node locally.

### Syntax

This subcommand has the following syntax:

``` bash
chef capture NODE-NAME [options]
```

### Options

`-c`, `--credentials`

:   Credentials file (default $HOME/.chef/credentials).

`-d`, `--with-data-bags`

:   Download all data bags as part of node capture.

`-h`, `--help`

:   Show help for the command.

`-k`, `--client-key`

:   Chef Infra Server API client key.

`-n`, `--client-name`

:   Chef Infra Server API client name.

`-o`, `--ssl-no-verify`

:   Do not verify SSL when connecting to Chef Infra Server (default: verify).

`-p`, `--profile`

:   Profile to use from credentials file (default "default").

`-s`, `--chef-server-url`

:   Chef Infra Server URL.

### Examples

**Capture a node**

To capture a node in a local repository, run a command similar to:

``` bash
chef capture test-server
```

This will return something similar to:

``` bash
- Setting up local repository
 - Capturing node object 'test-server'
 - Capturing policy data...
 - Capturing cookbook artifacts...
 - Writing kitchen configuration...

Repository has been created in './node-test-server-repo'.

Next, locate version-controlled copies of the cookbooks. This is
important so that you can track changes to the cookbooks as you
edit them. You may have one or more existing paths where you have
checked out cookbooks. If not, now is a good time to open a
separate terminal and clone or check out the cookbooks.

If all cookbooks are not available in the same base location,
you will have a chance to provide additional locations.

Press Enter to Continue:

You're ready to begin!

Start with 'cd ./node-test-server-repo; kitchen converge'.

As you identify issues, you can modify cookbooks in their
original checkout locations or in the repository's cookbooks
directory and they will be picked up on subsequent runs
of 'kitchen converge'.

```

See the [Upgrade Lab](/workstation/upgrade_lab/#create-an-upgrade-environment-with-chef-capture) documentation for additional
information on upgrading Chef Infra Client.

## chef env

Use the `chef env` subcommand to configure the environment for Chef
Workstation.

### Syntax

This subcommand has the following syntax:

``` bash
chef env
```

### Options

This command does not have any specific options.

### Examples

None.

## chef exec

Use the `chef exec` subcommand to run arbitrary shell commands with the
`PATH` environment variable and the `GEM_HOME` and `GEM_PATH` Ruby
environment variables pointed at Chef Workstation.

### Syntax

This subcommand has the following syntax:

``` bash
chef exec SYSTEM_COMMAND (options)
```

### Options

This subcommand has the following options:

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

None.

## chef gem

The `chef gem` subcommand is a wrapper around the `gem` command in
RubyGems and is used by Chef to install RubyGems into Chef Workstation
development environment. All knife plugins, drivers for Kitchen, and
other Ruby applications that are not packaged within Chef Workstation
will be installed to the `.Chef Workstation` path in the home directory:
`~/.Chef Workstation/gem/ruby/ver.si.on/bin` (where `ver.si.on` is the
version of Ruby that is packaged within Chef Workstation).

### Syntax

This subcommand has the following syntax:

``` bash
chef gem GEM_COMMAND GEM_OPTIONS (options)
```

### Options

This subcommand has the following options:

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

**Show an existing gem in Chef Workstation**

To show a gem, run a command similar to:

``` bash
chef gem list cookstyle
```

to return something similar to:

``` bash
*** LOCAL GEMS ***

cookstyle (5.20.0)
```

**List all local gems**

To list all of the installed gems on your development environment, use
the `list` command without any arguments:

``` bash
chef gem list
```

**Search for local gems**

The `list` command can also be used to search for locally installed
gems. For example, to list all of the gems with `knife` in their title:

``` bash
chef gem list knife
```

which returns the following output:

``` bash
*** LOCAL GEMS ***

knife-opc (0.3.2)
knife-push (1.0.2)
knife-windows (1.9.0)
```

**Search remote gems**

Use the `search` command to search for remote gems available for
installation:

``` bash
chef gem search kitchen
```

to return something similar to:

``` bash
*** REMOTE GEMS ***

chefkitchen_cli (0.0.1)
gst-kitchen (0.9.0)
guard-kitchen (0.0.2)
jackal-kitchen (0.1.2)
jackal-kitchen-slack (0.1.2)
kitchen (0.0.3)
```

**Install a gem**

To install a gem, run a command similar to:

``` bash
chef gem install knife-config
```

to return something similar to:

``` bash
Successfully installed knife-config-1.1.0
1 gem installed
```

**Uninstall a gem**

To uninstall a gem from Chef Workstation environment:

``` bash
chef gem uninstall knife-config
```

to return something similar to:

``` bash
Successfully uninstalled knife-config-1.1.0
```

**View the contents of a gem**

To view the contents of a gem, run a command similar to:

``` bash
chef gem content knife-config
```

to return something similar to:

``` bash
/Users/user/.chefdk/gem/ruby/2.1.0/gems/knife-config-1.1.0/LICENSE
/Users/user/.chefdk/gem/ruby/2.1.0/gems/knife-config-1.1.0/README.md
/Users/user/.chefdk/gem/ruby/2.1.0/gems/knife-config-1.1.0/lib/chef/knife/config.rb
/Users/user/.chefdk/gem/ruby/2.1.0/gems/knife-config-1.1.0/lib/knife-config.rb
```

## chef generate attribute

Use the `chef generate attribute` subcommand to generate an attribute
file in the `/attributes` directory.

### Syntax

This subcommand has the following syntax:

``` bash
chef generate attribute COOKBOOK_PATH NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

**Create an attribute**

To generate an attribute, run a command similar to:

``` bash
chef generate attribute /path/to/cookbook FOO
```

will return something similar to:

``` bash
Recipe: code_generator::attribute
  * directory[/Users/grantmc/chef-repo/cookbooks/chef-repo/attributes] action create
    - create new directory /Users/grantmc/chef-repo/cookbooks/chef-repo/attributes

  * template[/Users/grantmc/chef-repo/cookbooks/chef-repo/attributes/FOO.rb] action create
    - create new file /Users/grantmc/chef-repo/cookbooks/chef-repo/attributes/FOO.rb
```

## chef generate cookbook

Use the `chef generate cookbook` subcommand to generate a cookbook.

{{< note >}}

{{% ruby_style_patterns_hyphens %}}

{{< /note >}}

### Syntax

This subcommand has the following syntax:

``` bash
chef generate cookbook COOKBOOK_PATH/COOKBOOK_NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-b`, `--berks`

:   Create a Berksfile in the cookbook. Default: disabled. Optional.
    Use to create a berksfile by overriding default creation of a Policyfile.

`-C COPYRIGHT`, `--copyright COPYRIGHT`

:   Specify the copyright holder for copyright notices in generated
    files. Default value: `The Authors`

`-d`, `--delivery`

:   Generate a delivery config file and build cookbook inside the new
    cookbook. Default value: disabled. This option is disabled. It has
    no effect and exists only for compatibility with past releases

`-m EMAIL`, `--email EMAIL`

:   Specify the email address of the author. Default value:
    `you@example.com`.

`-a KEY=VALUE`, `--generator-arg KEY=VALUE`

:   Sets a property named `KEY` to the given `VALUE` on the generator
    context object in the generator cookbook. This allows custom
    generator cookbooks to accept optional user input on the command
    line.

`-I LICENSE`, `--license LICENSE`

:   Sets the license. Valid values are `all_rights`, `apache2`, `mit`,
    `gplv2`, or `gplv3`. Default value: `all_rights`.

`-P`, `--policy`

:   Create a Policyfile in the cookbook instead of a Berksfile.
    Default: enabled.

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

**Create a cookbook**

To generate a cookbook, run a command similar to:

``` bash
chef generate cookbook chefdocs
```

will return something similar to:

``` bash
Recipe: code_generator::cookbook
  * directory[/Users/grantmc/chefdocs] action create
    - create new directory /Users/grantmc/chefdocs

  * template[/Users/grantmc/chefdocs/metadata.rb] action create
    - create new file /Users/grantmc/chefdocs/metadata.rb

  * template[/Users/grantmc/chefdocs/README.md] action create
    - create new file /Users/grantmc/chefdocs/README.md

  * cookbook_file[/Users/grantmc/chefdocs/chefignore] action create
    - create new file /Users/grantmc/chefdocs/chefignore

  * cookbook_file[/Users/grantmc/chefdocs/Berksfile] action create
    - create new file /Users/grantmc/chefdocs/Berksfile

  * template[/Users/grantmc/chefdocs/kitchen.yml] action create
    - create new file /Users/grantmc/chefdocs/kitchen.yml

  * directory[/Users/grantmc/chefdocs/recipes] action create
    - create new directory /Users/grantmc/chefdocs/recipes

  * template[/Users/grantmc/chefdocs/recipes/default.rb] action create
    - create new file /Users/grantmc/chefdocs/recipes/default.rb

  * execute[initialize-git] action run
    - execute git init .

  * cookbook_file[/Users/grantmc/chefdocs/.gitignore] action create
    - create new file /Users/grantmc/chefdocs/.gitignore
```

and which creates a directory structure similar to:

    /chefdocs
      /.git
      .gitignore
      kitchen.yml
      Berksfile
      chefignore
      metadata.rb
      README.md
      /recipes
        default.rb

**Create a cookbook using a custom skeleton cookbook**

If a custom skeleton cookbook is located on a macOS desktop (and in this
example, the `chef_generator` cookbook is simply a copy of the same
cookbook that ships in Chef Workstation), the following command will use
the skeleton cookbook at the custom location to generate a cookbook into
the repository from which the `chef` command is run:

``` bash
chef generate cookbook --generator-cookbook ~/Desktop testcookbook
```

{{< note >}}

The `code_generator` cookbook itself is not specified as part of the
path.

{{< /note >}}

will return something similar to:

``` bash
Compiling Cookbooks...
Recipe: code_generator::cookbook
  * directory[/Users/grantmc/Desktop/chef-repo/test-cookbook] action create
    - create new directory /Users/grantmc/Desktop/chef-repo/test-cookbook

  * template[/Users/grantmc/Desktop/chef-repo/test-cookbook/metadata.rb] action create
    - create new file /Users/grantmc/Desktop/chef-repo/test-cookbook/metadata.rb

  * template[/Users/grantmc/Desktop/chef-repo/test-cookbook/README.md] action create
    - create new file /Users/grantmc/Desktop/chef-repo/test-cookbook/README.md

  * cookbook_file[/Users/grantmc/Desktop/chef-repo/test-cookbook/chefignore] action create
    - create new file /Users/grantmc/Desktop/chef-repo/test-cookbook/chefignore

  * cookbook_file[/Users/grantmc/Desktop/chef-repo/test-cookbook/Berksfile] action create
    - create new file /Users/grantmc/Desktop/chef-repo/test-cookbook/Berksfile

  * template[/Users/grantmc/Desktop/chef-repo/test-cookbook/kitchen.yml] action create
    - create new file /Users/grantmc/Desktop/chef-repo/test-cookbook/kitchen.yml

  * directory[/Users/grantmc/Desktop/chef-repo/test-cookbook/recipes] action create
    - create new directory /Users/grantmc/Desktop/chef-repo/test-cookbook/recipes

  * template[/Users/grantmc/Desktop/chef-repo/test-cookbook/recipes/default.rb] action create
    - create new file /Users/grantmc/Desktop/chef-repo/test-cookbook/recipes/default.rb
```

## chef generate build-cookbook

Use the `chef generate build-cookbook` subcommand to generate a delivery
configuration file and build cookbook.

### Syntax

This subcommand has the following syntax:

``` bash
chef generate build-cookbook COOKBOOK_PATH/COOKBOOK_NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-C COPYRIGHT`, `--copyright COPYRIGHT`

:   Specify the copyright holder for copyright notices in generated
    files. Default value: `The Authors`

`-m EMAIL`, `--email EMAIL`

:   Specify the email address of the author. Default value:
    `you@example.com`.

`-a KEY=VALUE`, `--generator-arg KEY=VALUE`

:   Sets a property named `KEY` to the given `VALUE` on the generator
    context object in the generator cookbook. This allows custom
    generator cookbooks to accept optional user input on the command
    line.

`-I LICENSE`, `--license LICENSE`

:   Sets the license. Valid values are `all_rights`, `apache2`, `mit`,
    `gplv2`, or `gplv3`. Default value: `all_rights`.

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

None.

## chef generate file

Use the `chef generate file` subcommand to generate a file in the
`/files` directory.

### Syntax

This subcommand has the following syntax:

``` bash
chef generate file COOKBOOK_PATH NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-h`, `--help`

:   Show help for the command.

`-s SOURCE_FILE`, `--source SOURCE_FILE`

:   Copy the contents from a source file.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

None.

## chef generate resource

Use the `chef generate resource` subcommand to generate a custom
resource in the `/resources` directory.

{{< note >}}

{{% ruby_style_patterns_hyphens %}}

{{< /note >}}

### Syntax

This subcommand has the following syntax:

``` bash
chef generate resource COOKBOOK_PATH NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

None.

## chef generate recipe

Use the `chef generate recipe` subcommand to generate a recipe in the
`/recipes` directory.

### Syntax

This subcommand has the following syntax:

``` bash
chef generate recipe COOKBOOK_PATH NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

None.

## chef generate repo

{{% ctl_chef_generate_repo %}}

### Syntax

{{% ctl_chef_generate_repo_syntax %}}

### Options

{{% ctl_chef_generate_repo_options %}}

### Examples

None.

## chef generate template

Use the `chef generate template` subcommand to generate a template in
the `/templates` directory.

### Syntax

This subcommand has the following syntax:

``` bash
chef generate template COOKBOOK_PATH NAME (options)
```

### Options

This subcommand has the following options:

`-g GENERATOR_COOKBOOK_PATH`, `--generator-cookbook GENERATOR_COOKBOOK_PATH`

:   The path at which a cookbook named `code_generator` is located. This
    cookbook is used by the `chef generate` subcommands to generate
    cookbooks, cookbook files, templates, attribute files, and so on.
    Default value: `lib/chef-dk/skeletons`, under which is the default
    `code_generator` cookbook that is included as part of Chef
    Workstation.

`-h`, `--help`

:   Show help for the command.

`-s SOURCE_FILE`, `--source SOURCE_FILE`

:   Copy the contents from a source file.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

None.

## chef report cookbooks

Use the `chef report cookbooks` subcommand to generate a cookbook-oriented report for your Chef Infra environment.
This report provides details about upgrade compatibility errors and node cookbook usage.

### Syntax

This subcommand has the following syntax:

``` bash
chef report cookbooks [options]
```

### Options

This subcommand has the following options:

`-a`, `--anonymize`

:   Replace cookbook and node names with hash values to protect the sensitive information.

`-c`, `--credentials string`

:   Credentials file (default $HOME/.chef/credentials).

`-f`, `--format`

:   Output format: txt is human readable, csv is machine readable (default "txt").

`-F`, `--node-filter`

:   Search filter to apply to nodes.

`-h`, `--help`

:   Show help for the command.

`-k`, `--client-key`

:   Chef Infra Server API client key.

`-n`, `--client-name`

:   Chef Infra Server API client name.

`-o`, `--ssl-no-verify`

:   Do not verify SSL when connecting to Chef Infra Server (default: verify).

`-p`, `--profile`

:   Profile to use from credentials file (default "default").

`-s`, `--chef-server-url`

:   Chef Infra Server URL.

`-u`, `--only-unused`

:   Generate a report that only includes cookbooks that are not included in any node's runlist.

`-V`, `--verify-upgrade`

:   Verify the upgrade compatibility of every cookbook.

`-w`, `--workers`

:   Maximum number of parallel workers at once (default 50).


### Examples

**Generate a report**

To generate a cookbook oriented report, run a command similar to:

``` bash
chef report cookbooks
```

will return something similar to:

``` bash
Finding available cookbooks... (5 found)
Analyzing cookbooks...
5 / 5 [----------------------------------------------------------------------------------------------------------------] 100.00% 13 p/s

-- REPORT SUMMARY --

  Cookbook   Version   Policy Group   Policy   Nodes Affected
-----------+---------+--------------+--------+-----------------
  apt        2.7.0                             2
  Go         0.1.0                             3
  java       0.3.0                             4
  java                 Production     K8s      1
  K8s                  Production     K8s      1


Cookbooks report saved to .chef-workstation/reports/cookbooks-20200722120938.txt
```

More details of this summary report can be viewed in the text file, the location for which is given at the end of the summary.

See the [Upgrade Lab](/workstation/upgrade_lab/#chef-report-cookbooks) documentation for additional
information on upgrading Chef Infra Client.

## chef report nodes

Use the `chef report nodes` subcommand to generate a nodes-oriented report for your chef environment. This report provides details about nodes, applied policies and the cookooks used during the most recent chef-client run.

### Syntax

This subcommand has the following syntax:

``` bash
chef report nodes [options]
```

### Options

This subcommand has the following options:

`-a`, `--anonymize`

:   Replace cookbook and node names with hash values to protect the sensitive information.

`-c`, `--credentials string`

:   Credentials file (default $HOME/.chef/credentials).

`-f`, `--format`

:   Output format: txt is human readable, csv is machine readable (default "txt").

`-F`, `--node-filter`

:   Search filter to apply to nodes.

`-h`, `--help`

:   Show help for the command.

`-k`, `--client-key`

:   Chef Infra Server API client key.

`-n`, `--client-name`

:   Chef Infra Server API client name.

`-o`, `--ssl-no-verify`

:   Do not verify SSL when connecting to Chef Infra Server (default: verify).

`-p`, `--profile`

:   Profile to use from credentials file (default "default").

`-s`, `--chef-server-url`

:   Chef Infra Server URL.


### Examples

**Generate a report**

To generate a nodes oriented report, run a command similar to:

``` bash
chef report nodes
```

will return something similar to:

``` bash
Analyzing nodes...

-- REPORT SUMMARY --

      Node Name        Chef Version   Operating System   Number Cookbooks
---------------------+--------------+------------------+-------------------
  dev-server-backend   16.1.16        redhat v8.2                       3
  dev-server-web       16.1.16        redhat v8.2                       1
  production-server    16.1.16        redhat v8.2                       2
  staging-server       16.1.16        redhat v8.2                       4
  test-server          16.1.16        redhat v8.2                       3

Nodes report saved to .chef-workstation/reports/nodes-20200722130502.txt

```
More details of this summary report can be viewed in the text file, the location for which is given at the end of the summary.

[Upgrade Lab](https://docs.chef.io/workstation/upgrade_lab) provides further details on how this command can be used in Chef client upgrade process.


## chef shell-init

Use the `chef shell-init` subcommand to set the Ruby included in Chef
Workstation as the system Ruby. Chef Workstation is designed to allow
the isolation of applications used by Chef Workstation from other Ruby
development tools that may be present on the workstation. This supports
Bash, fish, Windows PowerShell (posh), and zsh.

bash zsh fish PowerShell (posh)

### Syntax

This subcommand has the following syntax:

``` bash
chef shell-init SHELL_NAME (options)
```

### Options

This subcommand has the following options:

`-h`, `--help`

:   Show help for the command.

`-v`, `--version`

:   The Chef Infra Client version.

### Examples

**Set PowerShell**

You can use the `chef shell-init` command with Windows PowerShell to add
the appropriate variables to your environment.

To try it in your current session:

``` bash
chef shell-init powershell | Invoke-Expression
```

To enable it permanently:

``` bash
"chef shell-init powershell | Invoke-Expression" >> $PROFILE
```

**Set the execution policy on new machines**

On new Windows machines, PowerShell scripts will not work until an
administrator runs the following command:

``` bash
Set-ExecutionPolicy RemoteSigned
```

**Create a \$PROFILE on new machines**

On new Windows machines, commands cannot be appended to `$PROFILE` if
the folder does not exist, or if there is a new user profile. This will
result in an error similar to the following:

``` bash
PS C:\Users\Stuart> "chef shell-init powershell | Invoke-Expression" >> $PROFILE
out-file : Could not find a part of the path
'C:\Users\Stuart\Documents\WindowsPowerShell\Microsoft.PowerShell_profile.ps1'.
At line:1 char:1
+ "chef shell-init powershell | Invoke-Expression" >> $PROFILE
+ ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
    + CategoryInfo          : OpenError: (:) [Out-File], DirectoryNotFoundException
    + FullyQualifiedErrorId : FileOpenFailure,Microsoft.PowerShell.Commands.OutFileCommand
```

In this situation, run the following `chef shell-init` command instead:

``` bash
if(Test-Path $PROFILE){ chef shell-init powershell | Add-Content $PROFILE } else { New-Item -Force -ItemType File $PROFILE; chef shell-init powershell | Add-Content $PROFILE }
```

## Policyfile Commands

{{% policyfile_chef_commands %}}

### chef clean-policy-cookbooks

{{% ctl_chef_clean_policy_cookbooks %}}

#### Syntax

{{% ctl_chef_clean_policy_cookbooks_syntax %}}

#### Options

{{% ctl_chef_clean_policy_cookbooks_options %}}

#### Examples

None.

### chef clean-policy-revisions

{{% ctl_chef_clean_policy_revisions %}}

#### Syntax

{{% ctl_chef_clean_policy_revisions_syntax %}}

#### Options

{{% ctl_chef_clean_policy_revisions_options %}}

#### Examples

None.

### chef delete-policy

{{% ctl_chef_delete_policy %}}

#### Syntax

{{% ctl_chef_delete_policy_syntax %}}

#### Options

{{% ctl_chef_delete_policy_options %}}

#### Examples

None.

### chef delete-policy-group

{{% ctl_chef_delete_policy_group %}}

#### Syntax

{{% ctl_chef_delete_policy_group_syntax %}}

#### Options

{{% ctl_chef_delete_policy_group_options %}}

#### Examples

None.

### chef diff

{{% ctl_chef_diff %}}

#### Syntax

{{% ctl_chef_diff_syntax %}}

#### Options

{{% ctl_chef_diff_options %}}

#### Examples

**Compare current lock to latest commit on latest branch**

{{% ctl_chef_diff_current_lock_latest_branch %}}

**Compare current lock with latest commit on master branch**

{{% ctl_chef_diff_current_lock_master_branch %}}

**Compare current lock to specified revision**

{{% ctl_chef_diff_current_lock_specified_revision %}}

**Compare lock on master branch to lock on revision**

{{% ctl_chef_diff_master_lock_revision_lock %}}

**Compare lock for version with latest commit on master branch**

{{% ctl_chef_diff_version_lock_master_branch %}}

**Compare current lock with latest lock for policy group**

{{% ctl_chef_diff_current_lock_policy_group %}}

**Compare locks for two policy groups**

{{% ctl_chef_diff_two_policy_groups %}}

### chef export

{{% ctl_chef_export %}}

#### Syntax

{{% ctl_chef_export_syntax %}}

#### Configuration Settings

{{% ctl_chef_export_config %}}

#### Options

{{% ctl_chef_export_options %}}

#### Examples

None.

### chef generate policyfile

{{% ctl_chef_generate_policyfile %}}

#### Syntax

{{% ctl_chef_generate_policyfile_syntax %}}

#### Options

{{% ctl_chef_generate_policyfile_options %}}

#### Examples

None.

### chef install

{{% ctl_chef_install %}}

#### Syntax

{{% ctl_chef_install_syntax %}}

#### Options

{{% ctl_chef_install_options %}}

#### Policyfile.lock.json

{{% policyfile_lock_json %}}

{{% policyfile_lock_json_example %}}

#### Examples

None.

### chef push

{{% ctl_chef_push %}}

#### Syntax

{{% ctl_chef_push_syntax %}}

#### Options

{{% ctl_chef_push_options %}}

#### Examples

None.

### chef push-archive

{{% ctl_chef_push_archive %}}

#### Syntax

{{% ctl_chef_push_archive_syntax %}}

#### Options

{{% ctl_chef_push_archive_options %}}

#### Examples

None.

### chef show-policy

{{% ctl_chef_show_policy %}}

#### Syntax

{{% ctl_chef_show_policy_syntax %}}

#### Options

{{% ctl_chef_show_policy_options %}}

#### Examples

None.

### chef undelete

{{% ctl_chef_undelete %}}

#### Syntax

{{% ctl_chef_undelete_syntax %}}

#### Options

{{% ctl_chef_undelete_options %}}

#### Examples

None.

### chef update

{{% ctl_chef_update %}}

#### Syntax

{{% ctl_chef_update_syntax %}}

#### Options

{{% ctl_chef_update_options %}}

#### Examples

None.
