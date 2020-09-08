+++
title = "Microsoft Azure CLI"
draft = false

aliases = ["/azure_chef_cli.html"]

[menu]
  [menu.infra]
    title = "Microsoft Azure Chef Extension"
    identifier = "chef_infra/getting_started/chef_on_azure_guide/azure_chef_cli.md Microsoft Azure Chef Extension"
    parent = "chef_infra/getting_started/chef_on_azure_guide"
    weight = 40
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/azure_chef_cli.md)

The Azure Chef Extension is an extension for Microsoft Azure to enable
Chef on virtual machine instances. The extension makes available two
Windows PowerShell cmdlets and two Microsoft Azure CLI commands.

## Azure CLI

If the Microsoft Azure [cross-platform command line tool
(Xplat-CLI)](https://github.com/Azure/azure-xplat-cli) is installed on
the workstation, along with the Azure Chef Extension, the `get-chef` and
`set-chef` extensions may be used to manage Chef running on virtual
machines in Microsoft Azure.

### get-chef

Use the `get-chef` command to get the details for the Azure Chef
Extension that is running on the named virtual machine.

#### Syntax

This command has the following syntax:

``` bash
azure vm extension get-chef VM_NAME
```

### set-chef

Use the `set-chef` command to enable Chef on any virtual machine running
on Microsoft Azure.

#### Syntax

This command has the following syntax:

``` bash
azure vm extension set-chef VM_NAME (options)
```

#### Options

This command has the following options:

`-a`, `--auto-update-client`

:   Auto-update Chef Infra Client. Set to `true` to auto update the
    version of the Azure Chef Extension when the virtual machine is
    restarted. For example, if this option is enabled, a virtual machine
    that has version `1205.12.2.0` will be updated automatically to
    `1205.12.2.1` when it is published. Default value: `false`.

`-b`, `--disable`

:   Disable the Azure Chef Extension extension.

`-c PATH_TO_CONFIG`, `--client-config PATH_TO_CONFIG`

:   The path to the `client.rb` file.

`-C CLIENT_PEM`, `--client-pem CLIENT_PEM`

:   The location of the file that contains the client key. Default
    value: `/etc/chef/client.pem`.

`-D`, `--delete-chef-config`

:   Disable the Azure Chef Extension extension.

`-j JSON`, `--bootstrap-options JSON`

:   A JSON string that is added to the first run of a Chef Infra Client.
    For example:

    ``` bash
    -j '{"chef_node_name":"test_node"}'
    ```

    Supported options: `"chef_node_name"`, `"chef_server_url"`
    (required), `"environment"`, `"secret"`, and
    `"validation_client_name"` (required).

`-O VALIDATOR_PEM`, `--validation-pem VALIDATOR_PEM`

:   The location of the file that contains the key used when a Chef
    Infra Client is registered with a Chef Infra Server. A validation
    key is signed using the `validation_client_name` for authentication.
    Default value: `/etc/chef/validation.pem`.

`-R RUN_LIST`, `--run-list RUN_LIST`

:   A comma-separated list of roles and/or recipes to be applied.

`-u`, `--uninstall`

:   Uninstall the Azure Chef Extension.

`-V NUMBER`, `--version NUMBER`

:   Specify the version number for the Azure Chef Extension extension.
    Default is to use the latest extension's version number.

#### Examples

The following examples show how to use this knife subcommand:

**Create a virtual machine**

``` bash
azure vm create your-vm-name MSFT__Windows-Server-2012 yourusername yourpassword --location "West US" -r
```

**Set the Chef extension without a run-list**

``` bash
azure vm extension set-chef your-vm-name --validation-pem ~/chef-repo/.chef/testorg-validator.pem --client-config ~/chef-repo/.chef/client.rb --version "1201.12"
```

**Set the Chef extension with a run-list**

``` bash
azure vm extension set-chef your-vm-name --validation-pem ~/chef-repo/.chef/testorg-validator.pem --client-config ~/chef-repo/.chef/client.rb --version "1201.12" -R 'recipe[your_cookbook_name::your_recipe_name]'
```

**Azure Resource Manager (ARM) Templates**

If you are using Azure Resource Manager templates to create your
infrastructure you can use the Chef extension to have Azure handle the
bootstrapping/configuration of your node to your Chef Infra Server.

### Options

The extension has the following options that can be provided in the
<span class="title-ref">settings</span> hash.

`runlist`

:   A comma-separated list of roles and/or recipes to be applied.

`client_rb`

:   A JSON escaped string containing the content of your `client.rb`
    file.

`validation_key_format`

:   Tells the extension whether the supplied validation key is
    `plaintext` or `base64encoded`.

    {{< note spaces=4 >}}

    If using the Chef extension in an ARM template, it is recommended
    that you base64 encode your validation key and set this option to
    `base64encoded`

    {{< /note >}}

`bootstrap_version`

:   The version of Chef Infra Client that will be installed on the
    system. **linux only**

    {{< note spaces=4 >}}

    Due to constraints in Azure, the `bootstrap_version` option is only
    available on the `LinuxChefClient` extension.

    {{< /note >}}

`bootstrap_channel`

:   Specify the [channel](/packages/) for installing the Chef Infra Client version.
    Options are `stable`, `current` or `unstable` release channels.

`chef_package_path`

:   Specifies a local path to install Chef Infra Client from.
    This feature is mainly used for cases where there are restrictions on
    internet access.
    {{< note spaces=4 >}}
    Azure extensions have network access limitations. See the
    [Azure documentation](https://docs.microsoft.com/en-us/azure/virtual-machines/extensions/features-linux#network-access)
    for more information.
    {{< /note >}}

`CHEF_LICENSE`

:   Chef Infra Client 15+ requires accepting the CHEF EULA license. Set `CHEF_LICENSE`
    to one of these values `accept`, `accept-silent` or `accept-no-persist`. Refer
    to [CHEF EULA license](/chef_license_accept/#accept-the-chef-eula)

`hints`

:   Specifies the [Ohai Hints](/ohai/#hints) to be set in the Ohai configuration of
    the target node.

`chef_package_url`

:    Specifies a URL to download and install the Chef Infra Client package (.msi .rpm .deb) from.

`bootstrap_options`

:   A hash of the following options: `chef_node_name`,
    `chef_server_url`, `environment`, `secret`, and
    `validation_client_name`.

    {{< note spaces=4 >}}

    Options that are supplied in the bootstrap items will take
    precedence over any conflicts found in the `client.rb` file.

    {{< /note >}}

`chef_node_name`

:   Determines which configuration should be applied and sets the
    `client_name`, which is the name used when authenticating to a Chef
    Infra Server. The default value is the the Chef Infra Client FQDN,
    as detected by Ohai. In general, Chef recommends that you leave this
    setting blank and let Ohai assign the FQDN of the node as the
    `node_name` during each Chef Infra Client run.

`chef_server_url`

:   The URL for the Chef Infra Server.

`environment`

:   The environment this machine will be placed in on your Chef Infra
    Server.

`secret`

:   The encryption key that is used for values contained within a data
    bag item.

`validation_client_name`

:   The name of the chef-validator key that Chef Infra Client uses to
    access the Chef Infra Server during the initial Chef Infra Client
    run.

`node_ssl_verify_mode`

:   Set the verify mode for HTTPS requests.

`node_verify_api_cert`

:   Verify the SSL certificate on the Chef Infra Server. When `true`,
    Chef Infra Client always verifies the SSL certificate. When `false`,
    Chef Infra Client uses the value of `ssl_verify_mode` to determine
    if the SSL certificate requires verification.

**Protected Settings**

The following options can be provided to the extension through the
`protectedSettings` hash:

`validation_key`

:   The contents of your organization validator key, the format is
    dependent on `validation_key_format`.

`chef_server_crt`

:   The SSL certificate of your Chef Infra Server that will be added to
    the trusted certificates.

`client_pem`

:   A client key that will be used to communication with the Chef Infra
    Server.

### Examples

The following examples show how Chef Infra Client can be installed and
configured from an ARM template.

**Installing the Azure Chef extension on a Linux system**

``` javascript
{
   "type": "Microsoft.Compute/virtualMachines/extensions",
   "name": "myVirtualMachine/LinuxChefClient",
   "apiVersion": "2015-05-01-preview",
   "location": "westus",
   "properties": {
     "publisher": "Chef.Bootstrap.WindowsAzure",
     "type": "LinuxChefClient",
     "typeHandlerVersion": "1210.12",
     "settings": {
       "bootstrap_options": {
         "chef_node_name": "node1",
         "chef_server_url": "https://api.chef.io/organizations/my-chef-organization",
         "validation_client_name": "my-chef-organization-validator"
       },
       "runlist": "recipe[awesome_customers_rhel],recipe[yum],role[base]",
       "validation_key_format": "plaintext"
     },
     "protectedSettings": {
       "validation_key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEpQIB..\n67VT3Dg=\n-----END RSA PRIVATE KEY-----"
     }
   }
 }
```

**Installing the Azure Chef extension on a Windows system**

``` javascript
{
  "type": "Microsoft.Compute/virtualMachines/extensions",
  "name": "myVirtualMachine/ChefClient",
  "apiVersion": "2015-05-01-preview",
  "location": "westus",
  "properties": {
    "publisher": "Chef.Bootstrap.WindowsAzure",
    "type": "ChefClient",
    "typeHandlerVersion": "1210.12",
    "settings": {
      "bootstrap_options": {
        "chef_node_name": "node12",
        "chef_server_url": "https://api.chef.io/organizations/my-chef-organization",
        "validation_client_name": "my-chef-organization-validator"
      },
      "runlist": "recipe[awesome_customers_windows],recipe[iis],role[windows_base]",
      "chef_package_url" : "https://my.packages.chef.io/chef-client-15.11.8-1-x64.msi",
      "validation_key_format": "plaintext"
    },
    "protectedSettings": {
      "validation_key": "-----BEGIN RSA PRIVATE KEY-----\nMIIEpQIB..\n67VT3Dg=\n-----END RSA PRIVATE KEY-----"
    }
  }
}
```

**Installing the Azure Chef extension on a Linux system with SSL peer
verification turned off and given a data bag secret**

``` javascript
{
   "type": "Microsoft.Compute/virtualMachines/extensions",
   "name": "myVirtualMachine/LinuxChefClient",
   "apiVersion": "2015-05-01-preview",
   "location": "westus",
   "properties": {
     "publisher": "Chef.Bootstrap.WindowsAzure",
     "type": "LinuxChefClient",
     "typeHandlerVersion": "1210.12",
     "settings": {
       "bootstrap_options": {
         "chef_node_name": "node1",
         "chef_server_url": "https://api.chef.io/organizations/my-chef-organization",
         "validation_client_name": "my-chef-organization-validator",
         "node_ssl_verify_mode": "none",
         "secret": "KCYWGXxSrkgR..."
       },
       "runlist": "recipe[awesome_customers_rhel],recipe[yum],role[base]",
       "validation_key_format": "base64encoded"
     },
     "protectedSettings": {
       "validation_key": "LS0tLS1CRUdJTiBSU0EgUFJ...FIEtFWS0tLS0t"
     }
   }
 }
```

{{< note >}}

Here we're also base64 encoding our validator key which is a recommended
approach when using the Azure Chef extension in an ARM template

{{< /note >}}
