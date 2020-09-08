+++
title = "Microsoft Azure PowerShell"
draft = false

aliases = ["/azure_powershell.html"]


[menu]
  [menu.infra]
    title = "Microsoft Azure PowerShell"
    identifier = "chef_infra/getting_started/chef_on_azure_guide/azure_powershell.md Microsoft Azure PowerShell"
    parent = "chef_infra/getting_started/chef_on_azure_guide"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/azure_powershell.md)

## PowerShell

### PowerShell Cmdlets

If Windows PowerShell is installed on the workstation, along with the
Azure Chef Extension, the `Get-AzureVMChefExtension` and
`Set-AzureVMChefExtension` extensions may be used to manage Chef running
on virtual machines in Microsoft Azure.

#### Get-AzureVMChefExtension

Use the `Get-AzureVMChefExtension` cmdlet to get the details for the
Azure Chef Extension that is running on the named virtual machine.

**Syntax**

This cmdlet has the following syntax:

``` bash
Get-AzureVMChefExtension -VM <string>
```

**Example**

The following examples show how to use the `Get-AzureVMChefExtension`
cmdlet:

**Get details for a virtual machine**

``` bash
Get-AzureVM -ServiceName cloudservice1 -Name azurevm1 | Get-AzureVMExtension
```

#### Set-AzureVMChefExtension

Use the `Set-AzureVMChefExtension` cmdlet to enable Chef on any virtual
machine running on Microsoft Azure.

**Syntax**

This cmdlet has the following syntax.

For Microsoft Windows:

``` bash
Set-AzureVMChefExtension -ValidationPem <String> -VM <IPersistentVM> -Windows [-ChefServerUrl <String> ] [-ClientRb <String> ] [-OrganizationName <String> ] [-RunList <String> ] [-ValidationClientName <String> ] [-Version <String> ] [ <CommonParameters>]
```

For Linux:

``` bash
Set-AzureVMChefExtension -Linux -ValidationPem <String> -VM <IPersistentVM> [-ChefServerUrl <String> ] [-ClientRb <String> ] [-OrganizationName <String> ] [-RunList <String> ] [-ValidationClientName <String> ] [-Version <String> ] [ <CommonParameters>]
```

**Options**

This cmdlet has the following options:

`-AutoUpdateChefClient`

:   Auto-update . Set to `true` to auto update the version of the Azure
    Chef Extension when the virtual machine is restarted. For example,
    if this option is enabled, a virtual machine that has version
    `1205.12.2.0` will be updated automatically to `1205.12.2.1` when it
    is published.

`-BootstrapOptions <string>`

:   A JSON string that is added to the first run of a Chef Infra Client.
    For example:

    ``` bash
    -BootstrapOptions '{"chef_node_name":"test_node"}'
    ```

    Supported options: `"chef_node_name"`, `"chef_server_url"`
    (required), `"environment"`, `"secret"`, and
    `"validation_client_name"` (required).

`-ChefServerUrl <string>`

:   The URL for the Chef Infra Server.

`-ClientRb <string>`

:   The path to the `client.rb` file.

`-DeleteChefConfig`

:   Disable the Azure Chef Extension extension.

`-Linux`

:   Sets the Azure Chef Extension to run Linux.

`-OrganizationName <string>`

:   The name of the organization on the Chef Infra Server.

`-RunList <string>`

:   A comma-separated list of roles and/or recipes to be applied.

`-ValidationClientName <string>`

:   The name of the chef-validator key Chef Infra Client uses to access
    the Chef Infra Server during the initial Chef Infra Client run.

`-ValidationPem  <string>`

:   The location of the file that contains the key used when a Chef
    Infra Client is registered with a Chef Infra Server. A validation
    key is signed using the `validation_client_name` for authentication.
    Default value: `/etc/chef/validation.pem`.

`-Version <string>`

:   Specify the version number for the Azure Chef Extension extension.
    Default is to use the latest extension's version number.

`-Windows`

:   Sets the Azure Chef Extension to run Microsoft Windows.

**Examples**

The following examples show how to use the `Set-AzureVMChefExtension`
cmdlet:

**Create Windows virtual machine**

``` bash
$vm1 = "azurechefwin"
$svc = "azurechefwin"
$username = 'azure'
$password = 'azure@123'

$img = "a699494373c04fc0bc8f2bb1389d6106__Windows-Server-2012-R2-201406.01-en.us-127GB.vhd"

$vmObj1 = New-AzureVMConfig -Name $vm1 -InstanceSize Small -ImageName $img

$vmObj1 = Add-AzureProvisioningConfig -VM $vmObj1 -Password $password -AdminUsername $username -Windows

# set azure chef extension
$vmObj1 = Set-AzureVMChefExtension -VM $vmObj1 -ValidationPem "C:\\users\\azure\\msazurechef-validator.pem" -ClientRb
"C:\\users\\azure\\client.rb" -RunList "getting-started" -Windows

New-AzureVM -Location 'West US' -ServiceName $svc -VM $vmObj1
```

**Create CentOS virtual machine**

``` bash
$vm1 = "azurecheflnx"
$svc = "azurecheflnx"
$username = 'azure'
$password = 'azure@123'

# CentOS image id
$img = "5112500ae3b842c8b9c604889f8753c3__OpenLogic-CentOS-71-20150605"

$vmObj1 = New-AzureVMConfig -Name $vm1 -InstanceSize Small -ImageName $img

$vmObj1 = Add-AzureProvisioningConfig -VM $vmObj1 -Password $password -Linux -LinuxUser $username

# set azure chef extension
$vmObj1 = Set-AzureVMChefExtension -VM $vmObj1 -ValidationPem "C:\\users\\azure\\msazurechef-validator.pem" -ClientRb
"C:\\users\\azure\\client.rb" -RunList "getting-started" -Linux

New-AzureVM -Location 'West US' -ServiceName $svc -VM $vmObj1
```

**Create Ubuntu virtual machine**

``` bash
$vm1 = "azurecheflnx"
$svc = "azurecheflnx"
$username = 'azure'
$password = 'azure@123'

$img = "b39f27a8b8c64d52b05eac6a62ebad85__ubuntu-20_04_5-LTS-amd64-server-20150127-en-us-30GB"

$vmObj1 = New-AzureVMConfig -Name $vm1 -InstanceSize Small -ImageName $img

$vmObj1 = Add-AzureProvisioningConfig -VM $vmObj1 -Password $password -Linux -LinuxUser $username

# set azure chef extension
$vmObj1 = Set-AzureVMChefExtension -VM $vmObj1 -ValidationPem "C:\\users\\azure\\msazurechef-validator.pem" -ClientRb
"C:\\users\\azure\\client.rb" -RunList "getting-started" -Linux

New-AzureVM -Location 'West US' -ServiceName $svc -VM $vmObj1
```
