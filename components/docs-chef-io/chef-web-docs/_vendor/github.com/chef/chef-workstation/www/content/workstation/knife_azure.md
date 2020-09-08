+++
title = "Knife Azure"
draft = false

aliases = ["/knife_azure.html", "/knife_azure/"]

[menu]
  [menu.workstation]
    title = "knife azure"
    identifier = "chef_workstation/chef_workstation_tools/knife/knife_azure.md knife azure"
    parent = "chef_workstation/chef_workstation_tools/knife"
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/knife_azure.md)

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

### Installation

knife-azure ships in Chef Workstation. Install the latest version of Chef
Workstation from [Chef Downloads](https://downloads.chef.io/chef-workstation)

### Configuration

#### ASM Mode

The `knife azure` (ASM mode) subcommand uses a management certificate
for secure communication with Microsoft Azure. The management
certificate is required for secure communication with the Microsoft
Azure platform via the REST APIs. To generate the management certificate
(.pem file):

1.  Download the settings file:
    <http://go.microsoft.com/fwlink/?LinkId=254432>.

2.  Extract the data from the `ManagementCertificate` field into a
    separate file named `cert.pfx`.

3.  Decode the certificate file with the following command:

    ``` bash
    base64 -d cert.pfx > cert_decoded.pfx
    ```

4.  Convert the decoded PFX file to a PEM file with the following
    command:

    ``` bash
    openssl pkcs12 -in cert_decoded.pfx -out managementCertificate.pem -nodes
    ```

{{< note >}}

It is possible to generate certificates, and then upload them. See the
following link for more information:
www.windowsazure.com/en-us/manage/linux/common-tasks/manage-certificates/.

{{< /note >}}

### Knife Azure Commands

#### ag create

Use the `ag create` argument to create an affinity group.

**Syntax**

This argument has the following syntax:

``` bash
knife azure ag create (options)
```

**Options**

This argument has the following options:

`-a`, `--azure-affinity-group GROUP`

:   The affinity group to which the virtual machine belongs. Required
    when not using a service location. Required when not using
    `--azure-service-location`.

`--azure-ag-desc DESCRIPTION`

:   The description of the Microsoft Azure affinity group.

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-m LOCATION`, `--azure-service-location LOCATION`

:   The geographic location for a virtual machine and its services.
    Required when not using `--azure-affinity-group`.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

#### ag list

Use the `ag list` argument to get a list of affinity groups.

**Syntax**

This argument has the following syntax:

``` bash
knife azure ag list (options)
```

**Options**

This argument has the following options:

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

#### image list

Use the `image list` argument to get a list of images that exist in a
Microsoft Azure environment. Any image in this list may be used for
provisioning.

**Syntax**

This argument has the following syntax:

``` bash
knife azure image list (options)
```

**Options**

This argument has the following options:

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`--full`

:   Show all fields for all images.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

#### internal lb create

Use the `internal lb create` argument to create a new internal load
balancer within a cloud service.

**Syntax**

This argument has the following syntax:

``` bash
knife azure internal lb create (options)
```

**Options**

This argument has the following options:

`--azure-dns-name DNS_NAME`

:   The DNS prefix name that will be used to add this load balancer to.
    This must be an existing service/deployment.

`--azure-lb-static-vip VIP`

:   The Virtual IP that will be used for the load balancer.

`--azure-publish-settings-file FILENAME`

:   Your Azure Publish Settings File

`--azure-subnet-name SUBNET_NAME`

:   Required if static VIP is set. Specifies the subnename the load
    balancer is located in.

`-c`, `--config CONFIG`

:   The configuration file to use.

`--chef-zero-host HOST`

:   Host for starting chef-zero.

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges lik1000,1010
    or 8889-9999 will try all given ports until one works.

`--config-option OPTION=VALUE`

:   Override a single configuration option.

`--[no-]color`

:   Use colored output, defaults to enabled.

`-d`, `--disable-editing`

:   Do not open EDITOR, just accept the data as is.

`--defaults`

:   Accept default values for all questions

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment (except for in searches where this will be
    flagrantly ignored)

`-F`, `--format FORMAT`

:   Which format to use for output.

`--[no-]fips`

:   Enable fips mode.

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port.

`-h`, `--help`

:   Show help message.

`-H HOSTNAME`, `--azure-api-host-name`

:   Your Azure host name

`-k`, `--key KEY`

:   API Client Key

`-n`, `--azure-load-balancer NAME`

:   Required. Specifies new load balancer name.

`-p`, `--azure-mgmt-cert FILENAME`

:   Your Azure PEM file name.

`-s`, `--server-url URL`

:   Chef Infra Server URL.

`-S`, `--azure-subscription-id ID`

:   Your Azure subscription ID

`--print-after`

:   Show the data after a destructive operation

`--profile PROFILE`

:   The credentials profile to select

`-u`, `--user USER API`

:   Client Username.

`-v`, `--version`

:   Show Chef version.

`-V`, `--verbose`

:   More verbose output. Use twice for maximum verbosity.

`--verify-ssl-cert`

:   Verify SSL Certificates for communication over HTTPS.

`-y`, `--yes`

:   Say yes to all prompts for confirmation.

`-z`, `--local-mode`

:   Point knife commands at local repository instead of server.

#### internal lb list

Use the `internal lb create` argument to a list of defined load
balancers for all cloud services. Does not show public facing load
balancers.

**Syntax**

This argument has the following syntax:

``` bash
knife azure internal lb create (options)
```

**Options**

This argument has the following options:

`--azure-publish-settings-file FILENAME`

:   Your Azure Publish Settings File

`-c`, `--config CONFIG`

:   The configuration file to use.

`--chef-zero-host HOST`

:   Host for starting chef-zero.

`--chef-zero-port PORT`

:   Port (or port range) to start chef-zero on. Port ranges lik1000,1010
    or 8889-9999 will try all given ports until one works.

`--config-option OPTION=VALUE`

:   Override a single configuration option.

`--[no-]color`

:   Use colored output, defaults to enabled.

`-d`, `--disable-editing`

:   Do not open EDITOR, just accept the data as is.

`--defaults`

:   Accept default values for all questions

`-e`, `--editor EDITOR`

:   Set the editor to use for interactive commands

`-E`, `--environment ENVIRONMENT`

:   Set the Chef environment (except for in searches where this will be
    flagrantly ignored)

`-F`, `--format FORMAT`

:   Which format to use for output.

`--[no-]fips`

:   Enable fips mode.

`--[no-]listen`

:   Whether a local mode (-z) server binds to a port.

`-h`, `--help`

:   Show help message.

`-H HOSTNAME`, `--azure-api-host-name`

:   Your Azure host name

`-k`, `--key KEY`

:   API Client Key

`-p`, `--azure-mgmt-cert FILENAME`

:   Your Azure PEM file name.

`-s`, `--server-url URL`

:   Chef Infra Server URL.

`-S`, `--azure-subscription-id ID`

:   Your Azure subscription ID

`--print-after`

:   Show the data after a destructive operation

`--profile PROFILE`

:   The credentials profile to select

`-u`, `--user USER API`

:   Client Username.

`-v`, `--version`

:   Show Chef version.

`-V`, `--verbose`

:   More verbose output. Use twice for maximum verbosity.

`--verify-ssl-cert`

:   Verify SSL Certificates for communication over HTTPS.

`-y`, `--yes`

:   Say yes to all prompts for confirmation.

`-z`, `--local-mode`

:   Point knife commands at local repository instead of server.

#### server create

Use the `server create` argument to create a new Microsoft Azure cloud
instance. This will provision a new image in Microsoft Azure, perform a
bootstrap (using the SSH protocol), and then install Chef Infra Client
on the target system so that it can be used to configure the node and to
communicate with a Chef Infra Server.

**Syntax**

This argument has the following syntax:

``` bash
knife azure server create (options)
```

**Options**

This argument has the following options:

`-a`, `--azure-affinity-group GROUP`

:   The affinity group to which the virtual machine belongs. Required
    when not using a service location. Required when not using
    `--azure-service-location`.

`--auto-update-client`

:   Enable automatic updates for Chef Infra Client in Microsoft Azure.
    This option may only be used when `--bootstrap-protocol` is set to
    `cloud-api`. Default value: `false`.

`--azure-availability-set NAME`

:   The name of the availability set for the virtual machine.

`--azure-dns-name DNS_NAME`

:   Required. The name of the DNS prefix that is used to access the
    cloud service. This name must be unique within Microsoft Azure. Use
    with `--azure-connect-to-existing-dns` to use an existing DNS
    prefix.

`--azure-network-name NETWORK_NAME`

:   The network for the virtual machine.

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`--azure-storage-account STORAGE_ACCOUNT_NAME`

:   The name of the storage account used with the hosted service. A
    storage account name may be between 3 and 24 characters (lower-case
    letters and numbers only) and must be unique within Microsoft Azure.

`--azure-subnet-name SUBNET_NAME`

:   The subnet for the virtual machine.

`--azure-vm-name NAME`

:   The name of the virtual machine. Must be unique within Microsoft
    Azure. Required for advanced server creation options.

`--azure-vm-ready-timeout TIMEOUT`

:   A number (in minutes) to wait for a virtual machine to reach the
    `provisioning` state. Default value: `10`.

`--azure-vm-startup-timeout TIMEOUT`

:   A number (in minutes) to wait for a virtual machine to transition
    from the `provisioning` state to the `ready` state. Default value:
    `15`.

`--bootstrap-protocol PROTOCOL`

:   The protocol used to bootstrap on a machine that is running Windows
    Server: `cloud-api`, `ssh`, or `winrm`. Default value: `winrm`.

    Use the `cloud-api` option to bootstrap a machine in Microsoft
    Azure. The bootstrap operation will enable the guest agent to
    install, configure, and run Chef Infra Client on a node, after which
    Chef Infra Client is configured to run as a daemon/service. (This is
    a similar process to using the Azure portal.)

    Microsoft Azure maintains images of Chef Infra Client on the guest,
    so connectivity between the guest and the workstation from which the
    bootstrap operation was initiated is not required, after a
    `cloud-api` bootstrap is started.

    During the `cloud-api` bootstrap operation, knife does not print the
    output of a Chef Infra Client run like it does when the `winrm` and
    `ssh` options are used. knife reports only on the status of the
    bootstrap process: `provisioning`, `installing`, `ready`, and so on,
    along with reporting errors.

`--bootstrap-version VERSION`

:   The version of Chef Infra Client to install.

`-c`, `--azure-connect-to-existing-dns`

:   Add a new virtual machine to the existing deployment and/or service.
    Use with `--azure-dns-name` to ensure the correct DNS is used.

`--cert-passphrase PASSWORD`

:   The password for the SSL certificate.

`--cert-path PATH`

:   The path to the location of the SSL certificate.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the virtual machine.

`--hint HINT_NAME[=HINT_FILE]`

:   An Ohai hint to be set on the target node. See the
    [Ohai](/ohai/#hints) documentation for more information.
    `HINT_FILE` is the name of the JSON file. `HINT_NAME` is the name of
    a hint in a JSON file. Use multiple `--hint` options to specify
    multiple hints.

`--host-name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-I IMAGE`, `--azure-source-image IMAGE`

:   The name of the disk image to be used to create the virtual machine.

`--identity-file IDENTITY_FILE`

:   The SSH identity file used for authentication. Key-based
    authentication is recommended.

`--identity-file_passphrase PASSWORD`

:   The passphrase for the SSH key. Use only with `--identity-file`.

`-j JSON_ATTRIBS`, `--json-attributes JSON_ATTRIBS`

:   A JSON string that is added to the first run of a Chef Infra Client.

`-m LOCATION`, `--azure-service-location LOCATION`

:   The geographic location for a virtual machine and its services.
    Required when not using `--azure-affinity-group`.

`-N NAME`, `--node-name NAME`

:   The name of the node. Node names, when used with Microsoft Azure,
    must be 91 characters or shorter.

`--[no-]host-key-verify`

:   Use `--no-host-key-verify` to disable host key verification. Default
    setting: `--host-key-verify`.

`-o DISK_NAME`, `--azure-os-disk-name DISK_NAME`

:   The operating system type of the Microsoft Azure OS image: `Linux`
    or `Windows`.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-P PASSWORD`, `--ssh-password PASSWORD`

:   The SSH password. This can be used to pass the password directly on
    the command line. If this option is not specified (and a password is
    required) knife prompts for the password.

`--prerelease`

:   Install pre-release gems.

`-r RUN_LIST`, `--run-list RUN_LIST`

:   A comma-separated list of roles and/or recipes to be applied.

`-R ROLE_NAME`, `--role-name ROLE_NAME`

:   The name of the virtual machine.

`--ssh-port PORT`

:   The SSH port. Default value: `22`.

`-t PORT_LIST`, `--tcp-endpoints PORT_LIST`

:   A comma-separated list of local and public TCP ports that are to be
    opened. For example: `80:80,433:5000`.

`--template-file TEMPLATE`

:   The path to a template file to be used during a bootstrap operation.

    Deprecated in Chef Client 12.0.

`--thumbprint THUMBPRINT`

:   The thumbprint of the SSL certificate.

`-u PORT_LIST`, `---udp-endpoints PORT_LIST`

:   A comma-separated list of local and public UDP ports that are to be
    opened. For example: `80:80,433:5000`.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

`--windows-auth-timeout MINUTES`

:   The amount of time (in minutes) to wait for authentication to
    succeed. Default value: `25`.

`-x USER_NAME`, `--ssh-user USER_NAME`

:   The SSH user name.

`-z SIZE`, `--azure-vm-size SIZE`

:   The size of the virtual machine: `ExtraSmall`, `Small`, `Medium`,
    `Large`, or `ExtraLarge`. Default value: `Small`.

**Examples**

**Provision an instance using new hosted service and storage accounts**

To provision a medium-sized CentOS machine configured as a web server in
the `West US` data center, while reusing existing hosted service and
storage accounts, enter something like:

``` bash
knife azure server create -r "role[webserver]" --service-location "West US"
  --hosted-service-name webservers --storage-account webservers-storage --ssh-user foo
  --ssh--password password --role-name web-apache-0001 --host-name web-apache
  --tcp-endpoints 80:80,8080:8080 --source-image name_of_source_image --role-size Medium
```

**Provision an instance using new hosted service and storage accounts**

To provision a medium-sized CentOS machine configured as a web server in
the `West US` data center, while also creating new hosted service and
storage accounts, enter something like:

``` bash
knife azure server create -r "role[webserver]" --service-location "West US" --ssh-user foo
  --ssh--password password --role-name web-apache-0001 --host-name web-apache
  --tcp-endpoints 80:80,8080:8080 --source-image name_of_source_image --role-size Medium
```

#### server delete

Use the `server delete` argument to delete one or more instances that
are running in the Microsoft Azure cloud. To find a specific cloud
instance, use `knife azure server list`. Use the `--purge` option to
delete all associated node and client objects from the Chef Infra Server
or use the `knife node delete` and `knife client delete` subcommands to
delete specific node and client objects.

**Syntax**

This argument has the following syntax:

``` bash
knife azure server delete [SERVER...] (options)
```

**Options**

This argument has the following options:

`--azure-dns-name NAME`

:   The name of the DNS server (also known as the Hosted Service Name).

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`--delete-azure-storage-account`

:   Delete any corresponding storage account. When this option is
    `true`, any storage account not used by any virtual machine is
    deleted.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-N NODE_NAME`, `--node-name NODE_NAME`

:   The name of the node to be deleted, if different from the server
    name. This must be used with the `-p` (purge) option.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-P`, `--purge`

:   Destroy all corresponding nodes and clients on the Chef Infra
    Server, in addition to the Microsoft Azure node itself. This action
    (by itself) assumes that the node and client have the same name as
    the server; if they do not have the same names, then the
    `--node-name` option must be used to specify the name of the node.

`--preserve-azure-dns-name`

:   Preserve the DNS entries for the corresponding cloud services. If
    this option is `false`, any service not used by any virtual machine
    is deleted.

`--preserve-azure-os-disk`

:   Preserve the corresponding operating system disk.

`--preserve-azure-vhd`

:   Preserve the underlying virtual hard disk (VHD).

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

`--wait`

:   Pause the console until the server has finished processing the
    request.

**Examples**

**Delete an instance**

To delete an instance named `devops12`, enter:

``` bash
knife azure server delete devops12
```

#### server list

Use the `server list` argument to find instances that are associated
with a Microsoft Azure account. The results may show instances that are
not currently managed by the Chef Infra Server.

**Syntax**

This argument has the following syntax:

``` bash
knife azure server list (options)
```

**Options**

This argument has the following options:

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

#### server show

Use the `server show` argument to show the details for the named server
(or servers).

**Syntax**

This argument has the following syntax:

``` bash
knife azure server show SERVER [SERVER...] (options)
```

**Options**

This argument has the following options:

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

#### vnet create

Use the `vnet create` argument to create a virtual network.

**Syntax**

This argument has the following syntax:

``` bash
knife azure vnet create (options)
```

**Options**

This argument has the following options:

`-a`, `--azure-affinity-group GROUP`

:   The affinity group to which the virtual machine belongs. Required
    when not using a service location.

`--azure-address-space CIDR`

:   The address space of the virtual network. Use with classless
    inter-domain routing (CIDR) notation.

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`--azure-subnet-name CIDR`

:   The subnet for the virtual machine. Use with classless inter-domain
    routing (CIDR) notation.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-n`, `--azure-network-name NETWORK_NAME`

:   The network for the virtual machine.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.

#### vnet list

Use the `vnet list` argument to get a list of virtual networks.

**Syntax**

This argument has the following syntax:

``` bash
knife azure vnet list (options)
```

**Options**

This argument has the following options:

`--azure-publish-settings-file FILE_NAME`

:   The name of the Azure Publish Settings file, including the path. For
    example: `"/path/to/your.publishsettings"`.

`-H HOST_NAME`, `--azure_host_name HOST_NAME`

:   The host name for the Microsoft Azure environment.

`-p FILE_NAME`, `--azure-mgmt-cert FILE_NAME`

:   The name of the file that contains the SSH public key that is used
    when authenticating to Microsoft Azure.

`-S ID`, `--azure-subscription-id ID`

:   The subscription identifier for the Microsoft Azure portal.

`--verify-ssl-cert`

:   The SSL certificate used to verify communication over HTTPS.
