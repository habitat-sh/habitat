+++
title = "Getting Started"
draft = false

aliases = ["/workstation_setup.html", "/chefdk_setup.html", "/workstation.html", "/workstation_setup/"]

[menu]
  [menu.workstation]
    title = "Getting Started"
    identifier = "chef_workstation/getting_started.md Getting Started"
    parent = "chef_workstation"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/getting_started.md)

This guide contains common configuration options used when setting up a
new Chef Workstation installation. If you do not have Chef Workstation
installed, see its [installation guide](/workstation/install_workstation/)
before proceeding further.

## Configure Ruby Environment

For many users of Chef, the version of Ruby that is included in Chef
Workstation should be configured as the default version of Ruby on your
system.

{{< note >}}

These instructions are intended for macOS and Linux users.

{{< /note >}}

1.  Open a terminal and enter the following:

    ``` bash
    which ruby
    ```

    which will return something like `/usr/bin/ruby`.

2.  To use Chef Workstation-provided Ruby as the default Ruby on your
    system, edit the `$PATH` and `GEM` environment variables to include
    paths to Chef Workstation. For example, on a machine that runs Bash,
    run:

    ``` bash
    echo 'eval "$(chef shell-init bash)"' >> ~/.bash_profile
    ```

    where `bash` and `~/.bash_profile` represents the name of the shell.

    If zsh is your preferred shell then run the following:

    ``` bash
    echo 'eval "$(chef shell-init zsh)"' >> ~/.zshrc
    ```

3.  Run `which ruby` again. It should return
    `/opt/chef-workstation/embedded/bin/ruby`.

{{< note >}}

Using Chef Workstation-provided Ruby as your system Ruby is optional.
For many users, Ruby is primarily used for authoring Chef cookbooks. If
that's true for you, then using the Chef Workstation-provided Ruby is
recommended.

{{< /note >}}

Add Ruby to \$PATH
==================

Chef Infra Client includes a stable version of Ruby as part of its
installer. The path to this version of Ruby must be added to the `$PATH`
environment variable and saved in the configuration file for the command
shell (Bash, csh, and so on) that is used on the machine running Chef
Workstation. In a command window, type the following:

``` bash
echo 'export PATH="/opt/chef-workstation/embedded/bin:$PATH"' >> ~/.configuration_file && source ~/.configuration_file
```

where `configuration_file` is the name of the configuration file for the
specific command shell. For example, if Bash were the command shell and
the configuration file were named `bash_profile`, the command would look
something like the following:

``` bash
echo 'export PATH="/opt/chef-workstation/embedded/bin:$PATH"' >> ~/.bash_profile && source ~/.bash_profile
```

{{< warning >}}

On Microsoft Windows, `C:/opscode/Chef Workstation/bin` must be before
`C:/opscode/Chef Workstation/embedded/bin` in the `PATH`.

{{< /warning >}}

## Create the Chef repository

Use [the chef generate repo]({{< relref "ctl_chef.md#chef-generate-repo" >}}) to
create the Chef repository. For example, to create a repository called
`chef-repo`:

``` bash
chef generate repo chef-repo
```

### Create .chef Directory

The `.chef` directory is used to store three files:

-   `config.rb`
-   `ORGANIZATION-validator.pem`
-   `USER.pem`

Where `ORGANIZATION` and `USER` represent strings that are unique to
each organization. These files must be present in the `.chef` directory
in order for Chef Workstation to be able to connect to a Chef Infra
Server.

To create the `.chef` directory:

1.  In a command window, enter the following:

    ``` bash
    mkdir -p ~/chef-repo/.chef
    ```

    Note that you'll need to replace `chef-repo` with the name of the
    repository you created previously.

2.  After the `.chef` directory has been created, the following folder
    structure will be present on the local machine:

        chef-repo/
           .chef/        << the hidden directory
           certificates/
           config/
           cookbooks/
           data_bags
           environments/
           roles/

3.  Add `.chef` to the `.gitignore` file to prevent uploading the
    contents of the `.chef` folder to GitHub. For example:

    ``` bash
    echo '.chef' >> ~/chef-repo/.gitignore
    ```

### Install a Code Editor

A good visual code editor is not a requirement for working with Chef
Infra, but a good code editor can save you time. A code editor should
support the following: themes, plugins, snippets, syntax Ruby code
coloring/highlighting, multiple cursors, a tree view of the entire
folder/repository you are working with, and a Git integration.

These are a few common editors:

-   [Visual Studio Code (free/open
    source)](http://code.visualstudio.com)
-   [GitHub Atom - (free/open source)](http://atom.io)

Chef Infra support in editors:

-   [VSCode Chef Infra
    Extension](https://marketplace.visualstudio.com/items?itemName=chef-software.Chef)
-   [Chef on Atom](https://atom.io/packages/language-chef)

### Starter Kit

If you have access to Chef Infra Server through Automate or Chef Manage,
you can download the starter kit. The starter kit will create the
necessary configuration files: the `.chef` directory, `config.rb`,
`ORGANIZATION-validator.pem`, and `USER.pem`. Simply download the
starter kit and then move it to the desired location on your Chef
Workstation machine.

## Configure the Chef Repository

### With WebUI

Use the following steps to manually set up the chef-repo and to use the
Chef management console to get the `.pem` and `config.rb` files.

#### Get Config Files

For a Chef Workstation installation that will interact with the Chef
Infra Server (including the hosted Chef Infra Server), log on and
download the following files:

-   `config.rb`. This configuration file can be downloaded from the
    **Organizations** page.
-   `ORGANIZATION-validator.pem`. This private key can be downloaded
    from the **Organizations** page.
-   `USER.pem`. This private key can be downloaded from the **Change
    Password** section of the **Account Management** page.

#### Move Config Files

The `config.rb`, `ORGANIZATION-validator.pem`, and `USER.pem` files must
be moved to the `.chef` directory after they are downloaded from the
Chef Infra Server.

To move files to the `.chef` directory:

1.  In a command window, enter each of the following:

    ``` bash
    cp /path/to/config.rb ~/chef-repo/.chef
    ```

    and:

    ``` bash
    cp /path/to/ORGANIZATION-validator.pem ~/chef-repo/.chef
    ```

    and:

    ``` bash
    cp /path/to/USERNAME.pem ~/chef-repo/.chef
    ```

    where `/path/to/` represents the path to the location in which these
    three files were placed after they were downloaded.

2.  Verify that the files are in the `.chef` folder.

### Without WebUI

Use the following steps to manually set up the Chef repository: On your
Chef Infra Server, create the `ORGANIZATION-validator.pem` and
`USER.pem` files with the `chef-server-ctl` command line tool. Then, on
your workstation create the `config.rb` file with the `knife` tool.

#### Create an Organization

On the Chef Infra Server machine create the `ORGANIZATION-validator.pem`
from the command line using `chef-server-ctl`. Run the following
command:

``` bash
chef-server-ctl org-create ORG_NAME ORG_FULL_NAME -f FILE_NAME
```

where

-   The name must begin with a lower-case letter or digit, may only
    contain lower-case letters, digits, hyphens, and underscores, and
    must be between 1 and 255 characters. For example: `chef`
-   The full name must begin with a non-white space character and must
    be between 1 and 1023 characters. For example:
    `"Chef Software, Inc."`
-   `-f FILE_NAME`: Write the `ORGANIZATION-validator.pem` to
    `FILE_NAME` instead of printing it to `STDOUT`. For example:
    `/tmp/chef.key`.

For example, an organization named `chef`, with a full name of
`Chef Software, Inc.`, and with the ORGANIZATION-validator.pem file
saved to `/tmp/chef.key`:

``` bash
chef-server-ctl org-create chef "Chef Software, Inc." -f /tmp/chef.key
```

#### Create a User

On the Chef Infra Server machine create the `USER.pem` from the command
line using `chef-server-ctl`. Run the following command:

``` bash
chef-server-ctl user-create USER_NAME FIRST_NAME LAST_NAME EMAIL PASSWORD -f FILE_NAME
```

where

-   `-f FILE_NAME` writes the `USER.pem` to a file instead of `STDOUT`.
    For example: `/tmp/grantmc.key`.

For example: a user named `grantmc`, with a first and last name of
`Grant McLennan`, an email address of `grantmc@chef.io`, a poorly-chosen
password, and a `USER.pem` file saved to `/tmp/grantmc.key`:

``` bash
chef-server-ctl user-create grantmc Grant McLennan grantmc@chef.io p@s5w0rD! -f /tmp/grantmc.key
```

#### Move .pem Files

Download the `ORGANIZATION-validator.pem` and `USER.pem` files from the
Chef Infra Server and move them to the `.chef` directory.

To move files to the .chef directory:

1.  In a command window, enter each of the following:

    ``` bash
    cp /path/to/ORGANIZATION-validator.pem ~/chef-repo/.chef
    ```

    and:

    ``` bash
    cp /path/to/USERNAME.pem ~/chef-repo/.chef
    ```

    where `/path/to/` represents the path to the location in which these
    three files were placed after they were downloaded.

2.  Verify that the files are in the `.chef` folder.

#### Create the config.rb File

Navigate to the `~/chef-repo/.chef` directory and create the `config.rb`
using the `knife configure` tool. The file must be created in the
`.chef` folder. It should look similar to:

``` ruby
current_dir = File.dirname(__FILE__)
log_level                :info
log_location             STDOUT
node_name                'node_name'
client_key               "#{current_dir}/USER.pem"
validation_client_name   'ORG_NAME-validator'
validation_key           "#{current_dir}/ORGANIZATION-validator.pem"
chef_server_url          'https://api.chef.io/organizations/ORG_NAME'
cache_type               'BasicFile'
cache_options( :path => "#{ENV['HOME']}/.chef/checksums" )
cookbook_path            ["#{current_dir}/../cookbooks"]
```

At a minimum, you must update the following settings with the
appropriate values:

-   `client_key` should point to the location of the Chef Infra Server
    user's `.pem` file on your Chef Workstation machine.
-   `validation_client_name` should be updated with the name of the
    desired organization that was created on the Chef Infra Server.
-   `validation_key` should point to the location of your organization's
    `.pem` file on your Chef Workstation machine.
-   `chef_server_url` must be updated with the domain or IP address used
    to access the Chef Infra Server.

See the [knife config.rb documentation](/workstation/config_rb/) for more
details.

## Get SSL Certificates

Chef Server 12 and later enables SSL verification by default for all
requests made to the server, such as those made by knife and Chef Infra
Client. The certificate that is generated during the installation of the
Chef Infra Server is self-signed, which means there isn't a signing
certificate authority (CA) to verify. In addition, this certificate must
be downloaded to any machine from which knife and/or Chef Infra Client
will make requests to the Chef Infra Server.

Use the `knife ssl fetch` subcommand to pull the SSL certificate down
from the Chef Infra Server:

``` bash
knife ssl fetch
```

See [SSL Certificates](/chef_client_security/#ssl-certificates) for
more information about how knife and Chef Infra Client use SSL
certificates generated by the Chef Infra Server.

## Verify Server Communication

To verify that Chef Workstation can connect to the Chef Infra Server:

1.  In a command window, navigate to the Chef repository:

    ``` bash
    cd ~/chef-repo
    ```

2.  In a command window, enter the following:

    ``` bash
    knife client list
    ```

    to return a list of clients (registered nodes and Chef Workstation
    installations) that have access to the Chef Infra Server. For
    example:

    ``` bash
    chefdk_machine
    registered_node
    ```

## Ad-hoc remote execution with `chef-run`

The `chef-run` utility allows you to execute ad-hoc configuration updates on the systems you manage without setting up a Chef server. With `chef-run`, you connect to servers over SSH or WinRM, and you can apply single resources, recipes, or even entire cookbooks directly from the command line.

### Example: Installing NTP Server

Chef Workstation combines the power of InSpec and `chef-run`, giving you the ability to easily detect and correct issues on any target instance. One common task that administrators perform in their environments is installing the Network Time Protocol (NTP), which keeps the clocks in sync between servers. InSpec allows us to check if the package is installed with a query, using the InSpec `package` resource:

```ruby
describe package('ntp') do
  it { should be_installed }
end
 ```

Chef also provides a single-resource solution to install the Network Time Protocol package:

```ruby
package 'ntp' do
  action :install
end
```

With `chef-run`, you can run the resource directly from the command-line, converging your targets with a single resource, without creating a cookbook or recipe:

```bash
chef-run myhost package ntp action=install
```

Combined with executing an InSpec scan to validate successful package installation, we have everything we need to define our requirements, and make sure they're met with two simple commands, either locally or remotely.

```ruby
inspec exec ntp-check -t ssh://myuser@myhost -i ~/.ssh/mykey
```

```bash
chef-run -i ~/.ssh/mykey myuser@myhost package ntp action=install
```

![Chef Run NTP Installation](/images/chef-workstation/chef-run.gif)

### Recipe and Multi-Node Convergence

Use `chef-run` to execute Chef recipes and cookbooks as well, and run it against multiple targets in parallel. Here are a few  examples of chef-run in action:

#### Example: Recipe execution on multiple targets

Run the default recipe from the defined cookbook against two resources: myhost1 & myhost2.

```bash
chef-run myhost1,myhost2 /path/to/my/cookbook
```

#### Example: Alternate Recipe syntax and targets defined by a range

Run the `my_cookbook::my_recipe` cookbook against twenty resources: myhost1 through myhost20

```bash
chef-run myhost[1:20] my_cookbook::my_recipe
```

## Further Reading

* [Chef Run CLI Reference]({{< ref "chef_run.md" >}})
* [Introducing Chef Workstation](https://blog.chef.io/2018/05/23/introducing-chef-workstation/)
* [Chef Workstation - How We Made that Demo](https://blog.chef.io/2018/06/25/chef-workstation-how-we-made-that-demo/)
