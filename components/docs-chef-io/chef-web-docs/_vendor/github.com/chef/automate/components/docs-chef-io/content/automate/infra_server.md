+++
title = "Install Chef Infra Server With Automate"

date = 2020-02-11T14:24:00-08:00
weight = 20
draft = false
[menu]
  [menu.automate]
    title = "Install Chef Infra Server With Automate"
    parent = "automate/getting_started"
    identifier = "automate/getting_started/infra_server.md Install Chef Infra Server With Automate"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/infra_server.md)

{{% warning %}}

Chef Automate will not deploy the Chef Infra Server add-ons Chef Manage and Push Jobs Server.

{{% /warning %}}

{{% warning %}}

Supermarket cannot authenticate users on Chef Infra Server deployed
with Chef Automate.

{{% /warning %}}

Use Chef Automate to install Chef Infra Server either for a single-host installation that contains both Chef Infra Server and Chef Automate, or for a standalone Chef Infra Server instance.
See the [Chef Infra Server documentation](https://docs.chef.io/server_overview/) for instructions and guidance on using and managing your Chef Infra Server.

## System Requirements and Prerequisites

Before beginning your installation, check the [System Requirements]({{< relref "system_requirements.md" >}}) for Chef Automate, and ensure that the `chef-automate` command line tool installed.

To download the `chef-automate` command line tool, run the following command in your command line interface:

```shell
curl https://packages.chef.io/files/current/latest/chef-automate-cli/chef-automate_linux_amd64.zip | gunzip - > chef-automate && chmod +x chef-automate
```

## Install Chef Automate and Chef Infra Server on the Same Host

Use either a command line interface or a configuration file to install Chef Automate and Chef Infra Server on the same host.
Installations require elevated privileges, so run the commands as the superuser or use `sudo` at the start of each command.

### Hardware Requirements for Single-Host Installation

For a single-host installation that contains Chef Infra Server and Chef Automate, we recommend the following memory and vCPU minimums:

* up to 200 managed nodes: 8GB RAM, 2 vCPUs
* between 200 and 500 managed nodes: 30GB RAM, 4 vCPUs
* between 500 and 5000 managed nodes: 61GB RAM, 8vCPUs

A single-host installation that contains Chef Infra Server and Chef Automate requires a `/hab` directory that contains 80 GB of disk space for software artifacts plus 2 MB of disk space per managed node.

### Command Line Install of Chef Automate and Infra Server

Install Chef Automate and Chef Infra Server on the same host with this command:

```shell
sudo chef-automate deploy --product automate --product infra-server
```

Then, [set up `knife`]({{< relref "infra_server.md#use-knife-with-chef-infra-server" >}}) for use with Chef Infra Server.

### Configuration File Install of Chef Automate and Infra Server

Installing Chef Automate and Chef Infra Server on the same host using a configuration file also requires the use of the Chef Automate CLI.
Installations require elevated privileges, so run the commands as the superuser or use `sudo` at the start of each command.

1. First, generate a skeleton configuration file by running this command:

    ```shell
      sudo chef-automate init-config
    ```

1. Add a stanza to the configuration file to deploy Chef Automate and Chef Infra Server and bump the maximum report size:

    ```toml
      [deployment.v1.svc]
      products=["automate", "infra-server"]
    ```

1. Make any other configuration changes desired.

1. Run the `chef-automate deploy` command with your configuration file:

    ```shell
      sudo chef-automate deploy config.toml
    ```

1. [Set up `knife`]({{< relref "infra_server.md#use-knife-with-chef-infra-server" >}}) for use with Chef Infra Server.

## Install A Standalone Chef Infra Server

Use either a command line interface or a configuration file to install Chef Infra Server using the Chef Automate `deploy` command.

Refer to the Chef Infra Server [hardware requirements](https://docs.chef.io/install_server_pre/) for guidance on memory and number of CPUs.

### Command Line Install of Standalone Chef Infra Server

When Chef Automate deploys the Chef Infra Server, it automatically configures the Chef Infra Server to collect data to send to Chef Automate.
To deploy a standalone Chef Infra Server with Chef Automate, you must turn off data collection in the configuration.
Installations require elevated privileges, so run the commands as the superuser or use `sudo` at the start of each command.

1. First, generate a skeleton configuration file by running:

    ```shell
       sudo chef-automate init-config
    ```

1. Add a stanza to the configuration file to disable Chef Automate data collection and bump the maximum report size:

    ```toml
       [erchef.v1.sys.data_collector]
       enabled = false
    ```

1. Use the configuration file to deploy Chef Infra Server by running the following command:

    ```shell
       sudo chef-automate deploy --product infra-server <configuration_file>
    ```

1. [Set up `knife`]({{< relref "infra_server.md#use-knife-with-chef-infra-server" >}}) for use with Chef Infra Server.

### Install a Standalone Chef Infra Server with a Configuration File

Installing Chef Infra Server through Chef Automate using a configuration file also requires the use of the Chef Automate CLI.
When Chef Automate deploys the Chef Infra Server, it automatically configures the Chef Infra Server to collect data to send to Chef Automate.
To deploy a standalone Chef Infra Server with Chef Automate, you must turn off data collection in the configuration.
Installations require elevated privileges, so run the commands as the superuser or use `sudo` at the start of each command.

1. First, generate a skeleton configuration file by running the following command:

    ```shell
      sudo chef-automate init-config
    ```

1. Add a stanza to the configuration file to deploy Chef Infra Server and bump the maximum report size:

    ```toml
       [deployment.v1.svc]
       products=["infra-server"]

       # Disable Automate data collection as Automate will not be deployed
       [erchef.v1.sys.data_collector]
       enabled = false
    ```

1. Run the `chef-automate deploy` command with your configuration file (config.toml):

    ```shell
      sudo chef-automate deploy config.toml
    ```

1. [Set up `knife`]({{< relref "infra_server.md#use-knife-with-chef-infra-server" >}}) for use with Chef Infra Server.

## Add a New Chef Infra Server to an Existing Chef Automate Installation

Patch an existing Chef Automate installation to add Chef Infra Server:

1. Create a `patch.toml` file to add `infra-server` to the list of products to deploy and bump the maximum report size:

    ```toml
       [deployment.v1.svc]
       products=["automate", "infra-server"]
    ```

2. Apply the patch to the Chef Automate installation:

    ```shell
       sudo chef-automate config patch ./patch.toml
    ```

   The command output shows the added Chef Infra Server services:

    ```shell
       Updating deployment configuration

       Applying deployment configuration
         Installed automate-cs-bookshelf
         Installed automate-cs-oc-bifrost
         Installed automate-cs-oc-erchef
         Installed automate-cs-nginx
         Started automate-cs-bookshelf
         Started automate-cs-oc-bifrost
         Started automate-cs-oc-erchef
         Started automate-cs-nginx
         Started automate-load-balancer
       Success: Configuration patched
    ```

## Set Up the Chef Infra Server

The [`knife` command-line utility](https://docs.chef.io/workstation/knife/) provides an interface to interact with a Chef Infra Server from a workstation.

On the Chef Infra Server host:

1. Run the following command to create a user:

    ```shell
      sudo chef-server-ctl user-create USER_NAME FIRST_NAME LAST_NAME EMAIL 'PASSWORD' --filename USER_NAME.pem
    ```

    An RSA private key generates automatically and is the chef-validator key.
    Save this RSA private key to a safe location.
    The `--filename` option will save the RSA private key to the specified absolute path.

1. Run the following command to create an organization, generate its validator key, and assign the user created in the previous step as an administrator:

    ```shell
      sudo chef-server-ctl org-create SHORT_NAME 'FULL_ORGANIZATION_NAME' --association_user USER_NAME --filename ORGANIZATION-validator.pem
    ```

    The short name must begin with a lower-case letter or digit, may contain lower-case letters, digits, hyphens, and underscores, and must be between 1 and 255 characters. For example: `4thcoffee`.

    The full organization name must begin with a non-white space character and must be between 1 and 1023 characters. For example: `'Fourth Coffee, Inc.'`.

    The `--association_user` option will associate the `USER_NAME` with the `admins` security group on the Chef Infra Server.

    An RSA private key generates automatically and is the chef-validator key.
    Save this RSA private key to a safe location.
    The `--filename` option will save the RSA private key to the specified absolute path.

On the workstation:

1. Install [Chef Workstation](https://docs.chef.io/workstation/install_workstation/).

1. Create a Chef repository by using the `chef generate repo` subcommand.
     For example, create a Chef repository named `chef-repo` by running:

     ```shell
     chef generate repo chef-repo
     ```

     Replace `chef-repo` with your desired repository name.

1. Within your named Chef repository, create a `.chef` directory with the `mkdir` command. For example:

    ```shell
      mkdir /chef-repo/.chef
    ```

1. Copy `ORGANIZATION-validator.pem` and `USER_NAME.pem` to the `.chef` directory.

1. In the `.chef` directory, create a `config.rb` file that contains:

    ```shell
      current_dir = File.dirname(__FILE__)
      log_level                :info
      log_location             STDOUT
      node_name                'USER_NAME'
      client_key               "#{current_dir}/USER_NAME.pem"
      validation_client_name   'ORGANIZATION-validator'
      validation_key           "#{current_dir}/ORGANIZATION.pem"
      chef_server_url          'https://{{< example_fqdn "automate" >}}/organizations/ORGANIZATION'
      cache_type               'BasicFile'
      cache_options( :path => "#{ENV['HOME']}/.chef/checksums" )
      cookbook_path            ["#{current_dir}/../cookbooks"]
    ```

    For airgapped installations, [create a bootstrap
    template](https://docs.chef.io/install_chef_air_gap/#create-a-bootstrap-template) and
    [add it](https://docs.chef.io/install_chef_air_gap/#configure-knife) to your `config.rb`.

1. Run `knife ssl fetch` to get the SSL certificates from Chef Infra Server and make them available to `knife`.

For more information on how to set up the workstation, see [the Chef Workstation documentation](https://docs.chef.io/workstation/getting_started/).
