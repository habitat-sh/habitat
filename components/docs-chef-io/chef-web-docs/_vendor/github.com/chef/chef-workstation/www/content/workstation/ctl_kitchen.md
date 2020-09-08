+++
title = "kitchen (executable)"
draft = false

aliases = ["/ctl_kitchen.html", "/ctl_kitchen/"]

[menu]
  [menu.workstation]
    title = "kitchen (executable)"
    identifier = "chef_workstation/chef_workstation_tools/test_kitchen/ctl_kitchen.md kitchen (executable)"
    parent = "chef_workstation/chef_workstation_tools/test_kitchen"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/ctl_kitchen.md)

{{% ctl_kitchen_summary %}}

{{< note >}}

This topic details functionality that is packaged with Chef Workstation.
See <https://kitchen.ci/docs/getting-started/> for more information
about Test Kitchen.

{{< /note >}}

## Fuzzy Matching

Fuzzy matching can be used with all commands because kitchen uses
regular expressions to search. For example, a fully qualified name:

``` bash
kitchen list client-ubuntu-1804 --bare
```

will return something similar to:

``` bash
client-ubuntu-1804
```

A partial name:

``` bash
kitchen list ubuntu --bare
```

will return something similar to:

``` bash
client-ubuntu-1804
server-ubuntu-1804
```

A short string:

``` bash
kitchen list ub --bare
```

will return something similar to:

``` bash
client-ubuntu-1804
server-ubuntu-1804
```

An integer:

``` bash
kitchen list 4 --bare
```

will return something similar to:

``` bash
client-ubuntu-1804
server-ubuntu-1804
```

A single-quoted Ruby regular expression:

``` bash
kitchen list '^cli.*-65$' --bare
```

will return something similar to:

``` bash
client-centos-65
```

## kitchen converge

Use the `converge` subcommand to converge one (or more) instances.
Instances are based on the list of platforms in the kitchen.yml file.
This process will install Chef Infra Client on an instance using the
Chef installer, upload cookbook files and minimal configuration to the
instance, and then start a Chef Infra Client run using the run-list and
attributes specified in the kitchen.yml file.

Test Kitchen will skip unnecessary steps. For example, if Chef Infra
Client is already installed to the instance, Test Kitchen will not
re-install Chef Infra Client. That said, Test Kitchen will always upload
the cookbook files and minimal configuration. This ensures that cookbook
testing is being done correctly.

The following exit codes are used by Test Kitchen:

-   `0` means the operation was successful
-   Any non-zero value means at least one part of the operation was
    unsuccessful

In general, use the `test` subcommand to verify the end-to-end quality
of a cookbook. Use the `converge` and `verify` subcommands during the
normal the day-to-day development of a cookbook.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen converge PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-c`, `--concurrency`

:   The number of allowed concurrent connections. Default: `9999` (all
    instances, effectively).

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**Converge the default CentOS instance**

To converge the default CentOS instance, run the following:

``` bash
kitchen converge default-centos-7
```

Chef Infra Client is downloaded the first time this command is run. The
output of the command is similar to:

``` bash
-----> Starting Kitchen (v1.4.2)
-----> Converging <default-centos-7>...
       Preparing files for transfer
       Preparing cookbooks from project directory
       Removing non-cookbook files before transfer
       Preparing nodes
-----> Installing Chef Omnibus (true)
       downloading https://www.chef.io/chef/install.sh
         to file /tmp/install.sh
       ...
       Downloading Chef ...
       Installing Chef ...
       Thank you for installing Chef!
       Transferring files to <default-centos-7>
       [2014-06-27T18:41:04+00:00] INFO: Forking chef instance to converge...
       Starting Chef Client, version 12.4.1
       [2014-06-27T18:45:18+00:00] INFO: *** Chef 12.4.1 ***
       [2014-06-27T18:45:18+00:00] INFO: Chef-client pid: 3226
       [2014-06-27T18:45:25+00:00] INFO: Setting the run_list to ["recipe[chef-repo::default]"] from CLI options
       [2014-06-27T18:45:25+00:00] INFO: Run List is [recipe[chef-repo::default]]
       [2014-06-27T18:45:25+00:00] INFO: Run List expands to [chef-repo::default]
       [2014-06-27T18:45:25+00:00] INFO: Starting Chef Run for default-centos-7
       [2014-06-27T18:45:25+00:00] INFO: Running start handlers
       [2014-06-27T18:42:40+00:00] INFO: Start handlers complete.
       Compiling Cookbooks...
       Converging 1 resources
       Recipe: chef-repo::default
         * file[/root/test.txt] action create... INFO: Processing file[/root/test.txt]
           action create (chef-repo::default line 10)
       [2014-06-27T18:42:40+00:00] INFO: file[/root/test.txt] created file /root/test.txt
         - create new file /root/test.txt... INFO: file[/root/test.txt] updated file contents /root/test.txt
         - update content in file /root/test.txt from none to d9c88f
       --- /root/test.txt    2014-06-27 18:42:40.695889276 +0000
       +++ /tmp/.test.txt20140627-2810-1xdx98p   2014-06-27 18:42:40.695889276 +0000
       @@ -1 +1,2 @@
       +This file created by Chef!
         - restore selinux security context
       [2014-06-27T18:42:40+00:00] INFO: Chef Run complete in 0.168252291 seconds
       Running handlers:
       [2014-06-27T18:42:40+00:00] INFO: Running report handlers
       Running handlers complete
       [2014-06-27T18:42:40+00:00] INFO: Report handlers complete
       Chef Client finished, 1/1 resources updated in 7.152725504 seconds
       Finished converging <default-centos-7> (0m8.43s).
-----> Kitchen is finished. (0m15.96s)
```

**Converge the default Ubuntu instance**

To converge the default Ubuntu instance, run the following:

``` bash
kitchen converge default-ubuntu-1804
```

Chef Infra Client is downloaded the first time this command is run. The
output of the command is similar to:

``` bash
-----> Starting Kitchen (v1.4.2)
-----> Converging <default-ubuntu-1804>...
       Preparing files for transfer
       Preparing cookbooks from project directory
       Removing non-cookbook files before transfer
       Preparing nodes
-----> Installing Chef Omnibus (true)
       downloading https://www.chef.io/chef/install.sh
         to file /tmp/install.sh
       ...
       Downloading Chef ...
       Installing Chef ...
       Thank you for installing Chef!
       Transferring files to <default-ubuntu-1804>
       [2014-06-27T18:48:01+00:00] INFO: Forking chef instance to converge...
       Starting Chef Client, version 12.4.1
       [2014-06-27T18:48:01+00:00] INFO: *** Chef 12.4.1 ***
       [2014-06-27T18:48:01+00:00] INFO: Chef-client pid: 1246
       [2014-06-27T18:48:03+00:00] INFO: Setting the run_list to ["recipe[chef-repo::default]"] from CLI options
       [2014-06-27T18:48:03+00:00] INFO: Run List is [recipe[chef-repo::default]]
       [2014-06-27T18:48:03+00:00] INFO: Run List expands to [chef-repo::default]
       [2014-06-27T18:48:03+00:00] INFO: Starting Chef Run for default-ubuntu-1804
       [2014-06-27T18:48:03+00:00] INFO: Running start handlers
       [2014-06-27T18:48:03+00:00] INFO: Start handlers complete.
       Compiling Cookbooks...
       Converging 1 resources
       Recipe: chef-repo::default
         * file[/home/vagrant/test.txt] action create... INFO: Processing file[/home/vagrant/test.txt]
           action create (chef-repo::default line 10)
       [2014-06-27T18:48:03+00:00] INFO: file[/home/vagrant/test.txt] created file /home/vagrant/test.txt
         - create new file /home/vagrant/test.txt... INFO: file[/home/vagrant/test.txt] updated file contents /home/vagrant/test.txt
         - update content in file /home/vagrant/test.txt from none to d9c88f
       --- /home/vagrant/test.txt    2014-06-27 18:48:03.233096345 +0000
        +++ /tmp/.test.txt20140627-1246-178u9dg  2014-06-27 18:48:03.233096345 +0000
       @@ -1 +1,2 @@
       +This file created by Chef!
       [2014-06-27T18:48:03+00:00] INFO: Chef Run complete in 0.015439954 seconds
       Running handlers:
       [2014-06-27T18:48:03+00:00] INFO: Running report handlers
       Running handlers complete
       [2014-06-27T18:48:03+00:00] INFO: Report handlers complete
       Chef Client finished, 1/1 resources updated in 1.955915841 seconds
       Finished converging <default-ubuntu-1804> (0m15.67s).
-----> Kitchen is finished. (0m15.96s)
```

## kitchen create

Use the `create` subcommand to create one (or more) instances. Instances
are based on the list of platforms and suites in the kitchen.yml file.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen create PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-c`, `--concurrency`

:   The number of allowed concurrent connections. Default: `9999` (all
    instances, effectively).

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**Create the default CentOS instance**

To create the default CentOS instance, run the following:

``` bash
kitchen create default-centos-7
```

CentOS is downloaded the first time this command is run, after which
Vagrant is started. (This may take a few minutes.)

The output of the command is similar to:

``` bash
-----> Starting Kitchen (v1.4.2)
-----> Creating <default-centos-7>...
       Bringing machine 'default' up with 'virtualbox' provider...
       ==> default: Box 'opscode-centos-7' could not be found. Attempting to find and install...
           default: Box Provider: virtualbox
           default: Box Version: >= 0
       ==> default: Adding box 'opscode-centos-7' (v0) for provider: virtualbox
           default: Downloading: https://opscode-vm-bento.s3.amazonaws.com/vagrant/virtualbox/opscode_centos-6.5_chef-provisionerless.box
       ==> default: Successfully added box 'opscode-centos-7' (v0) for 'virtualbox'!
       ==> default: Importing base box 'opscode-centos-7'...
       ==> default: Matching MAC address for NAT networking...
       ==> default: Setting the name of the VM: default-centos-7_default_1403650129063_53517
       ==> default: Clearing any previously set network interfaces...
       ==> default: Preparing network interfaces based on configuration...
           default: Adapter 1: nat
       ==> default: Forwarding ports...
           default: 22 => 2222 (adapter 1)
       ==> default: Booting VM...
       ==> default: Waiting for machine to boot. This may take a few minutes...
           default: SSH address: 127.0.0.1:2222
           default: SSH username: vagrant
           default: SSH auth method: private key
           default: Warning: Connection timeout. Retrying...
       ==> default: Machine booted and ready!
       ==> default: Checking for guest additions in VM...
       ==> default: Setting hostname...
       ==> default: Machine not provisioning because `--no-provision` is specified.
       Vagrant instance <default-centos-7> created.
       Finished creating <default-centos-7> (4m0.59s).
-----> Kitchen is finished. (11m29.76s)
```

**Create the default Ubuntu instance**

To create the default Ubuntu instance, run the following:

``` bash
kitchen create default-ubuntu-1804
```

Ubuntu is downloaded the first time this command is run, after which
Vagrant is started. (This may take a few minutes.)

The output of the command is similar to:

``` bash
-----> Starting Kitchen (v1.4.2)
-----> Creating <default-ubuntu-1804>...
       Bringing machine 'default' up with 'virtualbox' provider...
       ==> default: Box 'opscode-ubuntu-12.04' could not be found. Attempting to find and install...
           default: Box Provider: virtualbox
           default: Box Version: >= 0
       ==> default: Adding box 'opscode-ubuntu-12.04' (v0) for provider: virtualbox
           default: Downloading: https://opscode-vm-bento.s3.amazonaws.com/vagrant/virtualbox/opscode_ubuntu-12.04_chef-provisionerless.box
       ==> default: Successfully added box 'opscode-ubuntu-12.04' (v0) for 'virtualbox'!
       ==> default: Importing base box 'opscode-ubuntu-12.04'...
       ==> default: Matching MAC address for NAT networking...
       ==> default: Setting the name of the VM: default-ubuntu-1804_default_1403651715173_54200
       ==> default: Fixed port collision for 22 => 2222. Now on port 2200.
       ==> default: Clearing any previously set network interfaces...
       ==> default: Preparing network interfaces based on configuration...
           default: Adapter 1: nat
       ==> default: Forwarding ports...
           default: 22 => 2200 (adapter 1)
       ==> default: Booting VM...
==> default: Waiting for machine to boot. This may take a few minutes...
           default: SSH username: vagrant
           default: SSH auth method: private key
           default: Warning: Connection timeout. Retrying...
       ==> default: Machine booted and ready!
       ==> default: Checking for guest additions in VM...
       ==> default: Setting hostname...
       ==> default: Machine not provisioning because `--no-provision` is specified.
       Vagrant instance <default-ubuntu-1804> created.
       Finished creating <default-ubuntu-1804> (4m1.59s).
-----> Kitchen is finished. (10m58.24s)
```

## kitchen destroy

Use the `destroy` subcommand to delete one (or more) instances.
Instances are based on the list of platforms and suites in the
kitchen.yml file.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen destroy PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-c`, `--concurrency`

:   The number of allowed concurrent connections. Default: `9999` (all
    instances, effectively).

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

None.

## kitchen diagnose

Use the `diagnose` subcommand to show a computed diagnostic
configuration for one (or more) instances. This subcommand will make all
implicit configuration settings explicit because it echoes back all of
the configuration data as YAML.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen diagnose PLATFORMS (options)
```

### Options

This subcommand has the following options:

`--all`

:   Include all diagnostics. Default: `false`.

`--instances`

:   Include instance diagnostics. Default: `true`.

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`--loader`

:   Include data loader diagnostics. Default: `false`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**Diagnose an instance**

Use the `--loader` option to include diagnostic data in the output:

``` yaml
loader:
  combined_config:
    filename:
    raw_data:
      driver:
        name: vagrant
        socket: tcp://192.0.2.0:1234
    provisioner:
     #...
```

or:

``` yaml
---
loader:
  global_config:
    filename: "/Users/username/.kitchen/config.yml"
    raw_data: #...
  project_config:
    filename: "/Users/username/Projects/sandbox/path/to/kitchen.yml"
    raw_data: #...
  local_config:
```

**Diagnose an instance using --instances option**

Use the `--instances` option to track instances, which are based on the
list of platforms and suites in the kitchen.yml file:

``` yaml
---
instances
  default-ubuntu-1804
    busser:
      root_path: /tmp/busser
      ruby_bindir: /opt/chef/embedded/bin
      sudo: true
```

**Diagnose an instance using --loader option**

This command returns data as YAML:

``` yaml
---
timestamp: 2014-04-15 18:59:58.460470000 Z
kitchen-version: 1.2.2.dev
instances:
  default-ubuntu-1804
    # ...
  default-centos-8
    # ...
```

When Test Kitchen is being used to test cookbooks, Test Kitchen will
track state data:

``` yaml
---
instances:
  default-ubuntu-1804
    state_file:
      hostname: 192.0.2.0
      last_action: create
      port: '22'
      ssh_key: "/Users/username/path/to/key"
      username: vagrant
  default-centos-65
    # ...
```

and will track information that was given to a driver:

``` yaml
---
instances:
  default-ubuntu-1804
    driver:
      box: opscode-ubuntu-18.04
      box_url: https://URL/path/to/filename.box
      kitchen_root: "/Users/username/Projects/sandbox/"
```

and will track information about provisioners:

``` yaml
---
instances:
  default-ubuntu-1804
    provisioner:
      attributes: {}
      chef_omnibus_url: https://www.chef.io/chef/install.sh
      clients_path:
      name: chef_zero
```

## kitchen driver create

Use the `driver create` subcommand to create a new Test Kitchen driver
in the RubyGems project.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen driver create NAME
```

### Options

This subcommand has the following options:

`-l`, `--license`

:   The license for the RubyGems file. Possible values: `apachev2`,
    `lgplv3`, `mit`, and `reserved`. Default: `apachev2`.

### Examples

None.

## kitchen exec

Use the `exec` subcommand to execute a command on a remote instance.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen exec PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-c REMOTE_COMMAND`

:   Use to specify a remote command to be run via SSH.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

None.

## kitchen init

Use the `init` subcommand to create an initial Test Kitchen environment,
including:

-   Creating a kitchen.yml file
-   Appending Test Kitchen to the RubyGems file, .gitignore, and .thor
-   Creating the `test/integration/default` directory

### Syntax

This subcommand has the following syntax:

``` bash
kitchen init
```

### Options

This subcommand has the following options:

`--create_gemfile`

:   Create a RubyGems file, if one does not already exist. Default:
    `false`.

`-D`, `--driver`

:   Add one (or more) Test Kitchen drivers to a RubyGems file. Default:
    `kitchen-vagrant`.

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`-P`, `--provisioner`

:   The default provisioner that is used by Test Kitchen.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**Create the Test Kitchen environment**

``` bash
kitchen init --driver=kitchen-vagrant
```

will return something similar to:

``` bash
create kitchen.yml
create test/integration/default
create .gitignore
append .gitignore
append .gitignore
run    gem install kitchen-vagrant from "."
Fetching: kitchen-vagrant-0.12.0.gem (100%)
Successfully installed kitchen-vagrant-0.12.0
1 gem installed
```

## kitchen list

Use the `list` subcommand to view the list of instances. Instances are
based on the list of platforms in the kitchen.yml file. Test Kitchen
will auto-name instances by combining a suite name with a platform name.
For example, if a suite is named `default` and a platform is named
`ubuntu-18.04`, then the instance would be `default-ubuntu-1804`. This
ensures that Test Kitchen instances have safe DNS and hostname records.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen list PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-b`, `--bare`

:   Print the name of each instance, one instance per line. Default:
    `false`.

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**View a list of Test Kitchen instances**

To view the list of Test Kitchen instances:

``` bash
kitchen list
```

A list will be returned, similar to:

``` bash
Instance              Driver   Provisioner   Last Action
default-ubuntu-18.04  vagrant  chef_zero     created
default-centos-8      vagrant  chef_zero     created
```

or:

``` bash
Instance              Driver   Provisioner   Last Action
default-ubuntu-18.04  vagrant  chef_zero     converged
default-centos-8      vagrant  chef_zero     created
```

or:

``` bash
Instance              Driver   Provisioner   Last Action
default-ubuntu-18.04  vagrant  chef_zero     verified
default-centos-8      vagrant  chef_zero     created
```

or:

``` bash
Instance              Driver   Provisioner   Last Action
default-ubuntu-18.04  vagrant  chef_zero     created
default-centos-8      vagrant  chef_zero     <not created>
```

or if there are multiple suites defined, such as `default` and `test`:

``` bash
Instance              Driver   Provisioner   Last Action
default-ubuntu-18.04  vagrant  chef_zero     <not created>
default-centos-8      vagrant  chef_zero     <not created>
test-ubuntu-18.04     vagrant  chef_zero     <not created>
test-centos-8         vagrant  chef_zero     <not created>
```

## kitchen login

Use the `login` subcommand to log in to a single instance. Instances are
based on the list of platforms and suites in the kitchen.yml file. After
logging in successfully, the instance can be interacted with just like
any other virtual machine, including adding or removing packages,
starting or stopping services, and so on. It's a sandbox. Make any
change necessary to help improve the coverage for cookbook testing.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen login PLATFORM (options)
```

### Options

This subcommand has the following options:

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

To login to the default Ubuntu instance, run the following:

``` bash
kitchen login default-ubuntu-1804
```

to return something similar to:

``` bash
Welcome to Ubuntu 18.04.2 LTS (GNU/Linux 4.15.0-51-generic x86_64)

Last login: Wed Jul  3 18:21:09 2019 from 10.0.2.2
vagrant@default-ubuntu-1804:~$
```

## kitchen setup

Use the `setup` subcommand to set up one (or more) instances. Instances
are based on the list of platforms in the kitchen.yml file.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen setup PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-c`, `--concurrency`

:   The number of allowed concurrent connections. Default: `9999` (all
    instances, effectively).

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

None.

## kitchen test

Use the `test` subcommand to test one (or more) verified instances.
Instances are based on the list of platforms and suites in the
kitchen.yml file. This subcommand will create a new instance (cleaning
up a previous instance, if necessary), converge that instance, set up
the test harness, verify the instance using that test harness, and then
destroy the instance.

In general, use the `test` subcommand to verify the end-to-end quality
of a cookbook. Use the `converge` and `verify` subcommands during the
normal day-to-day development of a cookbook.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen test PLATFORMS (options)
```

### Options

This subcommand has the following options:

`--auto-init`

:   Invoke the `init` command if kitchen.yml is missing. Default:
    `false`.

`-c NUMBER`, `--concurrency NUMBER`

:   The number of allowed concurrent connections. Use this option to
    limit the number of instances that are tested concurrently. For
    example, `--concurrency 6` will set this limit to six concurrent
    instances. Default: `9999` (all instances, effectively).

`-d`, `--destroy`

:   The destroy strategy used at the conclusion of a Test Kitchen run.
    Possible values: `always` (always destroy instances), `never` (never
    destroy instances), or `passing` (only destroy instances that
    passed). Default: `passing`.

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**Test the default Ubuntu instance**

To test the default Ubuntu instance, run the following:

``` bash
kitchen test default-ubuntu-1804
```

to return something similar to:

``` bash
-----> Starting Kitchen (v2.2.5)
-----> Cleaning up any prior instances of <default-ubuntu-1804>
-----> Destroying <default-ubuntu-1804>...
...
       Finished destroying <config-ubuntu-1804> (0m4.92s).
-----> Testing <default-ubuntu-1804>
-----> Creating <default-ubuntu-1804>...
       Bringing machine 'default' up with 'virtualbox' provider...
...
       Vagrant instance <default-ubuntu-1804> created.
       Finished creating <default-ubuntu-1804> (0m34.01s).
-----> Converging <default-ubuntu-1804>...
...
-----> Installing Chef install only if missing package
       Downloading https://omnitruck.chef.io/install.sh to file /tmp/install.sh
...
       Installing chef
...
       Setting up chef (15.1.36-1) ...
       Thank you for installing Chef Infra Client! For help getting started visit https://learn.chef.io
...
       Starting Chef Infra Client, version 15.1.36
...
       Converging 2 resources
       Recipe: git::default
         * package[git] action install[date/time] INFO: Processing package[git] action install (git::default line 10)
           - install version 1:2.3.4.5-6 of package git

         * log[log_description] action write[date/time] INFO: Processing log[log_description] action write (git::default line 5)
...
       Chef Infra Client finished finished, 2 resources updated
       Finished converging <default-ubuntu-1804> (0m45.17s).
-----> Setting up <default-ubuntu-1804>...
       Finished setting up <default-ubuntu-1804> (0m0.00s).
-----> Verifying <default-ubuntu-1804>...
...
Package: `git`
   ✓  should exist

Test Summary: 1 successful, 0 failures, 0 skipped
     Finished verifying <default-ubuntu-1804> (0m1.25s).
-----> Destroying <default-ubuntu-1804>...
...
       Finished destroying <default-ubuntu-1804> (0m4.68s).
       Finished testing <default-ubuntu-1804> (0m57.80s).
```

**Test an instance using --concurrency option**

Use the `--concurrency` option to control the number of instances that
are tested concurrently on the local workstation. The default setting is
to test all instances, but the practical setting depends on the
capabilities of the local machine itself. The following examples will
limit the number of instances to four:

``` bash
kitchen test --concurrency=4
```

or:

``` bash
kitchen test --concurrency 4
```

or:

``` bash
kitchen test -c=4
```

or:

``` bash
kitchen test -c 4
```

## kitchen verify

Use the `verify` subcommand to verify one (or more) instances. Instances
are based on the list of platforms and suites in the kitchen.yml file.

In general, use the `test` subcommand to verify the end-to-end quality
of a cookbook. Use the `converge` and `verify` subcommands during the
normal day-to-day development of a cookbook.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen verify PLATFORMS (options)
```

### Options

This subcommand has the following options:

`-c`, `--concurrency`

:   The number of allowed concurrent connections. Default: `9999` (all
    instances, effectively).

`-l`, `--log-level`

:   The level of logging to be stored in a log file. Options (in order
    of priority): `debug`, `info`, `warn`, `error`, and `fatal`.
    Default: `info`.

`PLATFORMS`

:   Run Test Kitchen against one or more platforms listed in the
    kitchen.yml file. Use `all` to run Test Kitchen against all
    platforms. Use a Ruby regular expression to glob two or more
    platforms into a single run.

    {{< readFile_shortcode file="ctl_kitchen_common_option_platforms.md" >}}

### Examples

**Verify the default Ubuntu instance**

To verify the default Ubuntu instance, run the following:

``` bash
kitchen verify default-ubuntu-18.04
```

to return something similar to:

``` bash
-----> Starting Kitchen (v2.2.5)
-----> Setting up <default-ubuntu-18.04>
Fetching: <name of test tool> (100%)
Successfully installed <name of test tool>
# gems installed
-----> Setting up <name of test tool>
...
-----> Running <name of test tool> test suite
 ✓ <test result>

2 tests, 0 failures
     Finished verifying <default-ubuntu-18.04> (2m1.12s).
-----> Kitchen is finished. (2m3.45s)
echo $?
0
```

or:

``` bash
-----> Starting Kitchen (v2.2.5)
-----> Setting up <default-ubuntu-18.04>
Fetching: <name of test tool> (100%)
Successfully installed <name of test tool>
# gems installed
-----> Setting up <name of test tool>
...
-----> Running <name of test tool> test suite
 - <test result>

2 tests, 1 failures
... exit code was 1
echo $?
10
```

## kitchen version

Use the `version` subcommand to print the version of Kitchen.

### Syntax

This subcommand has the following syntax:

``` bash
kitchen version
```

### Options

This subcommand does not have any options.

### Examples

**Verify the version of Test Kitchen**

To view the version of Test Kitchen:

``` bash
kitchen version
```

will return something similar to:

``` bash
Test Kitchen version 2.2.5
```
