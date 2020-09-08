+++
title = "Install Chef Infra Client on Windows Nodes"
draft = false

aliases = ["/install_windows.html"]

[menu]
  [menu.infra]
    title = "Chef Infra Client on Windows"
    identifier = "chef_infra/getting_started/chef_on_windows_guide/install_windows.md Chef Infra Client on Windows"
    parent = "chef_infra/getting_started/chef_on_windows_guide"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/install_windows.md)

## Nodes

{{% node %}}

{{% windows_install_overview %}}

### Use knife windows

{{% knife_windows_summary %}}

#### Ports

{{% knife_windows_winrm_ports %}}

### Msiexec.exe

{{% windows_msiexec %}}

#### ADDLOCAL Options

{{% windows_msiexec_addlocal %}}

### Use MSI Installer

A Microsoft Installer Package (MSI) is available for installing Chef
Infra Client on a Microsoft Windows machine at [Chef
Downloads](https://downloads.chef.io/)

#### Run as a Scheduled Task

Chef Infra Client can be run as a scheduled task. On the Microsoft
Windows platform, a scheduled task provides more visibility,
configurability, and reliability around log rotation and permissions, as
compared to running Chef Infra Client as a service. A scheduled task:

-   Prevents log rotation issues, such as filling a disk partition
-   Does not have an indeterminate status, such as a service for which
    only the watcher is "running"
-   Prevents insufficient permissions related to service context
-   Prevents issues related to a system reboot

#### Scheduled Task Options

{{% install_chef_client_windows_as_scheduled_task %}}

### Use an Existing Process

{{% windows_install_system_center %}}

### PATH System Variable

{{% windows_environment_variable_path %}}
