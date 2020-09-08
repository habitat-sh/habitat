+++
title = "Chef for Microsoft Windows"
draft = false

aliases = ["/windows.html"]

[menu]
  [menu.infra]
    title = "Chef for Microsoft Windows"
    identifier = "chef_infra/getting_started/chef_on_windows_guide/windows.md Chef for Microsoft Windows"
    parent = "chef_infra/getting_started/chef_on_windows_guide"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/windows.md)

## Overview

The Chef Infra Client has specific components that are designed to
support unique aspects of the Microsoft Windows platform, including
PowerShell, PowerShell DSC, and Internet Information Services (IIS).

{{% windows_install_overview %}}

## Setting up Windows Workstations

To set up your Windows workstation follow the steps on [Chef for
Microsoft Windows](/workstation/install_workstation/)

## Install Chef Infra Client on Windows Nodes

{{% chef_client_summary %}}

This command has the following syntax:

``` bash
chef-client OPTION VALUE OPTION VALUE ...
```

This command has the following option specific to Microsoft Windows:

`-A`, `--fatal-windows-admin-check`

:   Cause a Chef Infra Client run to fail when Chef Infra Client does
    not have administrator privileges in Microsoft Windows.

### System Requirements

The recommended minimum amount of RAM available to Chef Infra Client
during a Chef Infra Client run is 512MB. Each node and workstation must
have access to Chef Infra Server via HTTPS. The Chef Infra Client can be
used to manage machines that run on the following versions of Microsoft
Windows:

<table>
<colgroup>
<col style="width: 33%" />
<col style="width: 33%" />
<col style="width: 33%" />
</colgroup>
<thead>
<tr class="header">
<th>Operating System</th>
<th>Architecture</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Windows</td>
<td><code>x86</code>, <code>x64</code></td>
<td><code>8.1</code>, <code>2012</code>, <code>2012 R2</code>, <code>2016</code>, <code>10 (all channels except "insider" builds)</code>, <code>2019 (Long-term servicing channel (LTSC), both Desktop Experience and Server Core)</code></td>
</tr>
</tbody>
</table>

After Chef Infra Client is installed, it is located at `C:\opscode`. The
main configuration file for Chef Infra Client is located at
`C:\chef\client.rb`.

### Information for Windows Users

#### Run With Elevated Privileges

{{% ctl_chef_client_elevated_privileges %}}

{{% ctl_chef_client_elevated_privileges_windows %}}

#### Spaces and Directories

{{% windows_spaces_and_directories %}}

#### Top-level Directory Names

{{% windows_top_level_directory_names %}}

#### PATH System Variable

{{% windows_environment_variable_path %}}

#### Proxy Settings

{{% proxy_windows %}}

### Remotely administering nodes

{{% knife_windows_summary %}}

Se the [knife windows](/workstation/knife_windows/) for more information.

#### Ports

{{% knife_windows_winrm_ports %}}

### Install Chef Infra Client using the MSI Installer

A Microsoft Installer Package (MSI) is available for installing Chef
Infra Client on a Microsoft Windows machine from [Chef
Downloads](https://downloads.chef.io/).

#### Msiexec.exe

{{% windows_msiexec %}}

#### ADDLOCAL Options

{{% windows_msiexec_addlocal %}}

#### Enable as a Scheduled Task

{{% install_chef_client_windows_as_scheduled_task %}}

### Install Chef Infra Client using an Existing Process

{{% windows_install_system_center %}}

## Windows Cookbooks

Some of the most popular Chef-maintained cookbooks that contain custom
resources useful when configuring machines running Microsoft Windows are
listed below:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Cookbook</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="https://github.com/chef-cookbooks/iis">iis Cookbook</a></td>
<td>The <code>iis</code> cookbook is used to install and configure Internet Information Services (IIS).</td>
</tr>
<tr class="even">
<td><a href="https://github.com/chef-cookbooks/iis_urlrewrite">iis_urlrewrite Cookbook</a></td>
<td>This cookbook downloads and installs the IIS URL Rewrite 2.0 extension into Microsoft Internet Information Server.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/chef-cookbooks/powershell">PowerShell Cookbook</a></td>
<td>Installs and configures PowerShell 2.0, 3.0, 4.0 or 5.0.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/chef-cookbooks/vcruntime">Microsoft Visual C++ Runtime Cookbook</a></td>
<td>Installs Microsoft Visual C++ runtime version 6 (2005), 9 (2008), 10 (2010), 11 (2012), 12 (2013), 14 (2015) or 15 (2017) on Windows.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/chef-cookbooks/mingw">Mingw Cookbook</a></td>
<td>Installs <code>msys/mingw</code> compiler toolchains on windows.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/chef-cookbooks/webpi">Webpi Cookbook</a></td>
<td>The <code>webpi</code> cookbook is used to run the Microsoft Web Platform Installer (WebPI).</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/chef-cookbooks/windows">Windows Cookbook</a></td>
<td>The <code>windows</code> cookbook is used to configure auto run, batch, reboot, enable built-in operating system packages, configure Microsoft Windows packages, reboot machines, and more.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/chef-cookbooks/windows_dns">Windows_dns Cookbook</a></td>
<td>This cookbook provides a resource for managing DNS on Windows hosts.</td>
</tr>
</tbody>
</table>

### Community Supported Windows Projects

Two community supports two provisioners for Kitchen:

-   [kitchen-dsc](https://github.com/test-kitchen/kitchen-dsc)
-   [kitchen-pester](https://github.com/test-kitchen/kitchen-pester)

## Windows Resources

{{% resources_common %}}

### Windows Resources

Chef Infra provides a growing number of Windows-specific resources.

-   [Chocolatey_config](/resources/chocolatey_config/)
-   [Chocolatey_package](/resources/chocolatey_package/)
-   [Chocolatey_source](/resources/chocolatey_package/)
-   [dsc_resource](/resources/dsc_resource/)
-   [resource_registry_key](/resources/registry_key/)
-   [Windows_ad_join](/resources/windows_ad_join/)
-   [Windows_auto_run](/resources/windows_auto_run/)
-   [Windows_certificate](/resources/windows_certificate/)
-   [Windows_dfs_folder](/resources/windows_dfs_folder/)
-   [Windows_dfs_namespace](/resources/windows_dfs_namespace/)
-   [Windows_dfs_server](/resources/windows_dfs_server/)
-   [Windows_dns_record](/resources/windows_dns_record/)
-   [Windows_dns_zone](/resources/windows_dns_zone/)
-   [Windows_env](/resources/windows_env/)
-   [Windows_feature_dism](/resources/windows_feature_dism/)
-   [Windows_feature_powershell](/resources/windows_feature_powershell/)
-   [Windows_feature](/resources/windows_feature/)
-   [Windows_firewall_rule](/resources/windows_firewall_rule/)
-   [Windows_font](/resources/windows_font/)
-   [Windows_package](/resources/windows_package/)
-   [Windows_pagefile](/resources/windows_pagefile/)
-   [Windows_path](/resources/windows_path/)
-   [Windows_windows_printer_port](/resources/windows_printer_port/)
-   [Windows_printer](/resources/windows_printer/)
-   [Windows_service](/resources/windows_service/)
-   [Windows_share](/resources/windows_share/)
-   [Windows_shortcut](/resources/windows_shortcut/)
-   [Windows_task](/resources/windows_task/)
-   [Windows_uac](/resources/windows_uac/)
-   [Windows_workgroup](/resources/windows_workgroup/)

### Windows Compatible Resources

The most popular core resources in Chef Infra Client work the same way
in Microsoft Windows as they do on any UNIX- or Linux-based platform.

-   [cookbook_file](/resources/cookbook_file/)
-   [directory](/resources/directory/)
-   [env](/resources/env/)
-   [execute](/resources/execute/)
-   [file](/resources/file/)
-   [group](/resources/group/)
-   [http_request](/resources/http_request/)
-   [link](/resources/link/)
-   [mount](/resources/mount/)
-   [package](/resources/package/)
-   [remote_directory](/resources/remote_directory/)
-   [remote_file](/resources/remote_file/)
-   [ruby_block](/resources/ruby_block/)
-   [service](/resources/service/)
-   [template](/resources/template/)
-   [user](/resources/user/)

The file-based resources have attributes that support unique
requirements within the Microsoft Windows platform, including `inherits`
(for file inheritance), `mode` (for octal modes), and `rights` (for
access control lists, or ACLs).

-   [cookbook_file](/resources/cookbook_file/)
-   [file](/resources/file/)
-   [remote_file](/resources/remote_file/)
-   [template](/resources/template/)
