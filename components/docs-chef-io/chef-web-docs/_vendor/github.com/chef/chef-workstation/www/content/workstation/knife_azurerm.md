+++
title = "Knife Azurerm"
draft = false

aliases = ["/knife_azurerm.html", "/knife_azurerm/"]

[menu]
  [menu.workstation]
    title = "knife azurerm"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_azurerm.md knife azurerm"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_azurerm.md)

## Knife Azure Overview

{{% knife_azure %}}

{{< note >}}

Review the list of [common options](/workstation/knife_options/) available to
this (and all) knife subcommands and plugins.

{{< /note >}}

`knife-azure` version 1.6.0 and later supports Azure Resource Manager.
Commands starting with `knife azurerm` use the Azure Resource Manager
API. Commands starting with `knife azure` use the Azure Service
Management API. While you can switch between the two command sets, they
are not designed to work together.

### Install Chef Workstation

Install the latest version of Chef Workstation from [Chef
Downloads](https://downloads.chef.io/chef-workstation).

### Installation

knife-azure ships in Chef Workstation. Install the latest version of Chef
Workstation from [Chef Downloads](https://downloads.chef.io/chef-workstation)

### Configuration

The `knife azurem` (ARM mode) requires setting up a service principal
for authentication and permissioning. For setting up a service principal
from the command line, see [Create service principal with PowerShell /
Azure CLI
2.0](https://docs.microsoft.com/en-us/azure/azure-resource-manager/resource-group-authenticate-service-principal).

{{< note >}}

When creating your user following the example in the Microsoft
documentation, change <span class="title-ref">-o Reader</span> to <span
class="title-ref">-o Contributor</span>, otherwise you will not be able
to spin up or delete machines.

{{< /note >}}

After creating the service principal, you will have the values:

-   client id (GUID)
-   client secret(string)
-   tenant id (GUID).

Put the following in your <span class="title-ref">knife.rb</span>

``` ruby
knife[:azure_tenant_id] # found via: tenantId=$(azure account show -s <subscriptionId> --json | jq -r '.[0].tenantId')
knife[:azure_subscription_id] # found via: <subscriptionId>
knife[:azure_client_id] # appId=$(azure ad app show --search <principleappcreated> --json | jq -r '.[0].appId')
knife[:azure_client_secret] # password you set at the beginning
```

Microsoft Azure encourages the use of Azure CLI 2.0. If you are still
using [azure-xplat-cli](https://github.com/Azure/azure-xplat-cli) _then
simply run `azure login` and skip creating the service principal.

### Knife Azurerm Commands

#### server create

Use the `server create` argument to provision a new server in Azure and
then perform a Chef bootstrap.

**Syntax**

This argument has the following syntax:

``` bash
knife azurerm server create (options)
```

**Options**

This argument has the following options:

`-a`, `--azure-storage-account NAME`

:   Required for advanced server-create option. A name for the storage
    account that is unique within Windows Azure. Storage account names
    must be between 3 and 24 characters in length and use numbers and
    lower-case letters only. This name is the DNS prefix name and can be
    used to access blobs, queues, and tables in the storage account. For
    example: <http://ServiceName.blob.core.windows.net/mycontainer/>

`--azure-availability-set NAME`

:   Name of availability set to add virtual machine into.

`--azure-extension-client-config CLIENT_PATH`

:   Path to a client.rb file for use by the bootstrapped node.

`--azure-image-os-type OSTYPE`

:   Specifies the image OS Type for which server needs to be created.
    Accepted values: `ubuntu`, `centos`, `rhel`, `debian`, `windows`.

`--azure-image-reference-offer OFFER`

:   Specifies the offer of the image used to create the virtual machine.
    eg. CentOS, UbuntuServer, WindowsServer.

`--azure-image-reference-publisher PUBLISHER_NAME`

:   Specifies the publisher of the image used to create the virtual
    machine. eg. OpenLogic Canonical, MicrosoftWindowsServer.

`--azure-image-reference-sku SKU`

:   Specifies the SKU of the image used to create the virtual machine.

`--azure-image-reference-version VERSION`

:   Specifies the version of the image used to create the virtual
    machine. Default: 'latest'.

`--azure-resource-group-name RESOURCE_GROUP_NAME`

:   The Resource Group name.

`--azure-storage-account-type TYPE`

:   One of the following account types (case-sensitive): `Standard_LRS`
    (Standard Locally-redundant storage); `Standard_ZRS` (Standard
    Zone-redundant storage); `Standard_GRS` (Standard Geo-redundant
    storage); `Standard_RAGRS` (Standard Read access geo-redundant
    storage); `Premium_LRS` (Premium Locally-redundant storage).

`--azure-vm-name NAME`

:   Required. Specifies the name for the virtual machine. The name must
    be unique within the ResourceGroup. Maximum length: 15 characters.

`--azure-vm-size SIZE`

:   Size of virtual machine. Values: `ExtraSmall`, `Small`, `Medium`,
    `Large`, `ExtraLarge`.

`--azure-vnet-name VNET_NAME`

:   Specifies the virtual network name. This may be the name of an
    existing vnet present under the given resource group or this may be
    the name of a new vnet to be added in the given resource group. If
    not specified then azure-vm-name will be taken as the default name
    for vnet name as well. Along with this option, the
    `azure-vnet-subnet-name` option can also be specified or skipped.

`--azure-vnet-subnet-name VNET_SUBNET_NAME`

:   Specifies the virtual network subnet name. Must be specified only
    with `azure-vnet-name` option. This may be the name of an existing
    subnet present under the given virtual network or this may be the
    name of a new subnet to be added virtual network. If not specified
    then `azure-vm-name` will be taken as name for subnet name as well.
    Note: `GatewaySubnet` cannot be used as the name for the
    `--azure-vnet-subnet-name` option.

`--bootstrap-proxy PROXY_URL`

:   The proxy server for the node being bootstrapped.

`--bootstrap-version VERSION`

:   The version of Chef to install.

`-c`, `--config CONFIG`

:   The configuration file to use.

`--cert-passphrase PASSWORD`

:   SSL Certificate Password.

`--cert-path PATH`

:   SSL Certificate Path.

`--chef-daemon-interval INTERVAL`

:   It specifies the frequency (in minutes) at which the chef-service
    runs. Pass 0 if you don't want the chef-service to be installed on
    the target machine.

`--chef-zero-host HOST`

:   Host for starting chef-zero.

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges like
    1000,1010 or 8889-9999 will try all given ports until one works.

`--[no-]color`

:   Use colored output. Default: `enabled`

`--config-option OPTION=VALUE`

:   Override a single configuration option.

`-d`, `--disable-editing`

:   Accept the data without opening the editor.

`--daemon DAEMON`

:   Configures the Chef Infra Client service for unattended execution.
    Requires `--bootstrap-protocol` to be `cloud-api` and the node
    platform to be `Windows`. Options: 'none' or 'service' or 'task'.
    'none' - Currently prevents the Chef Infra Client service from being
    configured as a service. 'service' - Configures Chef Infra Client to
    run automatically in the background as a service. 'task' -
    Configures Chef Infra Client to run automatically in the background
    as a scheduled task.

`--defaults`

:   Accept default values for all questions

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment (except for in searches, where this will be
    flagrantly ignored)

`--extended-logs`

:   It shows chef convergence logs in detail.

`-F`, `--format FORMAT`

:   Which format to use for output

`--[no-]fips`

:   Enable fips mode

`-h`, `--help`

:   Show this message

`-j`, `--json-attributes JSON`

:   A JSON string to be added to the first run of Chef Infra Client

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port

`-k`, `--key KEY`

:   API Client Key

`-m LOCATION`, `--azure-service-location`

:   Required if not using an Affinity Group. Specifies the geographic
    location - the name of the data center location that is valid for
    your subscription. Eg: westus, eastus, eastasia, southeastasia,
    northeurope, westeurope

`-N`, `--node-name NAME`

:   The Chef node name for your new node

`-o DISKNAME`, `--azure-os-disk-name`

:   Specifies the friendly name of the disk containing the guest OS
    image in the image repository.

`--node-ssl-verify-mode [peer|none]`

:   Whether or not to verify the SSL cert for all HTTPS requests.

`--[no-]node-verify-api-cert`

:   Verify the SSL cert for HTTPS requests to the Chef Infra Server API.

`--ohai-hints HINT_OPTIONS`

:   Hint option names to be set in Ohai configuration the target node.
    Values: `vm_name`, `public_fqdn` and platform. User can pass any
    comma separated combination of these values like
    `vm_name,public_fqdn`. Default: `default` which corresponds to
    supported values list mentioned here.

`--print-after`

:   Show the data after a destructive operation

`--profile PROFILE`

:   The credentials profile to select

`-r`, `--run-list RUN_LIST`

:   Comma separated list of roles/recipes to apply

`-s`, `--secret`

:   The secret key to use to encrypt data bag item values. Can also be
    defaulted in your config with the key 'secret'

`--secret-file SECRET_FILE`

:   A file containing the secret key to use to encrypt data bag item
    values. Can also be defaulted in your config with the key
    'secret_file'

`--server-count COUNT`

:   Number of servers to create with same configuration. Maximum: 5.
    Default: 1.

`--server-url URL`

:   Chef Infra Server URL

`--ssh-password PASSWORD`

:   The ssh password

`--ssh-port PORT`

:   The ssh port. Default: 22.

`--ssh-public-key FILENAME`

:   It is the ssh-rsa public key path. Specify either `ssh-password` or
    `ssh-public-key`.

`--ssh-user USERNAME`

:   The ssh username

`-t`, `--tcp-endpoints PORT_LIST`

:   Comma-separated list of TCP ports to open e.g. '80,433'

`--thumbprint THUMBPRINT`

:   The thumprint of the ssl certificate

`-u`, `--user USER`

:   API Client Username

`-v`, `--version`

:   Show Chef version

`-V`, `--verbose`

:   More verbose output. Use twice for max verbosity.

`-P`, `--winrm-password PASSWORD`

:   The WinRM password

`-x`, `--winrm-user USERNAME`

:   The WinRM username

`-y`, `--yes`

:   Say yes to all prompts for confirmation

`-z`, `--local-mode`

:   Point knife commands at local repository instead of server

knife azurerm server delete SERVER \[SERVER\] (options)

#### server delete

Use the `server delete` argument to delete existing ARM servers
configured in the Azure account.

**Syntax**

This argument has the following syntax:

``` bash
knife azurerm server delete (options)
```

**Options**

This argument has the following options:

`-c`, `--config CONFIG`

:   The configuration file to use.

`--chef-zero-host HOST`

:   Host for starting chef-zero.

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges like
    1000,1010 or 8889-9999 will try all given ports until one works.

`--[no-]color`

:   Use colored output, defaults to enabled.

`--config-option OPTION=VALUE`

:   Override a single configuration option.

`-d`, `--disable-editing`

:   Accept the data without opening the editor.

`--defaults`

:   Accept default values for all questions.

`--delete-resource-group`

:   Deletes corresponding resource group along with VirtualMachine.

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands.

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment, except for use in searching.

`-F`, `--format FORMAT`

:   Which format to use for output.

`--[no-]fips`

:   Enable fips mode.

`-h`, `--help`

:   Show the help message

`-k`, `--key KEY`

:   API Client Key.

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port.

`-N`, `--node-name NAME`

:   The name of the node and client to delete, if it differs from the
    server name. Only has meaning when used with the '--purge' option.

`--print-after`

:   Show the data after a destructive operation.

`--profile PROFILE`

:   The credentials profile to select.

`-P`, `--purge`

:   Destroy corresponding node and client on the ChefServer, in addition
    to destroying the Windows Azure node itself. Assumes node and client
    have the same name as the server (if not, add the '--node-name'
    option).

`-r RESOURCE_GROUP_NAME`, `--azure-resource-group-name`

:   The Resource Group name.

`-s`, `--server-url URL`

:   Chef Infra Server URL.

`-u`, `--user USER`

:   API Client Username

`-v`, `--version`

:   Show chef version

`-V`, `--verbose`

:   More verbose output. Use twice for maximum verbosity.

`-y`, `--yes`

:   Say yes to all prompts for confirmation.

`-z`, `--local-mode`

:   Point knife commands at local repository instead of at the server.

#### server list

Use the `server list` argument to output a list of all ARM
servers--including those not managed by the Chef server---in the Azure
account.

**Syntax**

This argument has the following syntax:

``` bash
knife azurerm server list (options)
```

**Options**

This argument has the following options:

`-c`, `--config CONFIG`

:   The configuration file to use.

`--chef-zero-host HOST`

:   Host for starting chef-zero.

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges like
    1000,1010 or 8889-9999 will try all given ports until one works.

`--[no-]color`

:   Use colored output, defaults to enabled.

`--config-option OPTION=VALUE`

:   Override a single configuration option.

`-d`, `--disable-editing`

:   Accept the data without opening the editor.

`--defaults`

:   Accept default values for all questions.

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands.

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment, except for use in searching.

`-F`, `--format FORMAT`

:   Which format to use for output.

`--[no-]fips`

:   Enable fips mode.

`-h`, `--help`

:   Show the help message

`-k`, `--key KEY`

:   API Client Key.

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port.

`--print-after`

:   Show the data after a destructive operation.

`--profile PROFILE`

:   The credentials profile to select.

`-r RESOURCE_GROUP_NAME`, `--azure-resource-group-name`

:   The Resource Group name.

`-s`, `--server-url URL`

:   Chef Infra Server URL.

`-u`, `--user USER`

:   API Client Username

`-v`, `--version`

:   Show chef version

`-V`, `--verbose`

:   More verbose output. Use twice for maximum verbosity.

`-y`, `--yes`

:   Say yes to all prompts for confirmation.

`-z`, `--local-mode`

:   Point knife commands at local repository instead of at the server.

#### server show

Use the `server show` argument to output the details of an ARM server in
the Azure account.

**Syntax**

This argument has the following syntax:

``` bash
knife azurerm server show (options)
```

**Options**

This argument has the following options:

`-c`, `--config CONFIG`

:   The configuration file to use.

`--chef-zero-host HOST`

:   Host for starting chef-zero.

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges like
    1000,1010 or 8889-9999 will try all given ports until one works.

`--[no-]color`

:   Use colored output, defaults to enabled.

`--config-option OPTION=VALUE`

:   Override a single configuration option.

`-d`, `--disable-editing`

:   Accept the data without opening the editor.

`--defaults`

:   Accept default values for all questions.

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands.

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment, except for use in searching.

`-F`, `--format FORMAT`

:   Which format to use for output.

`--[no-]fips`

:   Enable fips mode.

`-h`, `--help`

:   Show the help message

`-k`, `--key KEY`

:   API Client Key.

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port.

`--print-after`

:   Show the data after a destructive operation.

`--profile PROFILE`

:   The credentials profile to select.

`-r RESOURCE_GROUP_NAME`, `--azure-resource-group-name`

:   The Resource Group name.

`-s`, `--server-url URL`

:   Chef Infra Server URL.

`-u`, `--user USER`

:   API Client Username

`-v`, `--version`

:   Show chef version

`-V`, `--verbose`

:   More verbose output. Use twice for maximum verbosity.

`-y`, `--yes`

:   Say yes to all prompts for confirmation.

`-z`, `--local-mode`

:   Point knife commands at local repository instead of at the server.
