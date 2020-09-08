+++
title = "System Requirements"
draft = false

aliases = ["/chef_system_requirements.html"]

[menu]
  [menu.infra]
    title = "System Requirements"
    identifier = "chef_infra/getting_started/chef_system_requirements.md System Requirements"
    parent = "chef_infra/getting_started"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_system_requirements.md)

Before installing Chef Infra:

-   Ensure that each system you will be managing is running a [supported
    platform](/platforms/)
-   Ensure that the machine that will run the Chef Infra Server is
    sufficiently powerful
-   Ensure that any network and firewall settings are configured
    correctly

Install and configure the Chef Infra Server, then install and configure
Chef Workstation, and then run the bootstrap command from Chef
Workstation to install Chef Infra Client on each node.

## The Chef Infra Server

The following sections describe the various requirements for the Chef
Infra Server.

### Hosted Chef Infra Server

The hosted Chef Infra Server has the following requirements:

-   **Browser** --- Firefox, Google Chrome, Safari, or Internet Explorer
    (versions 9 or better)
-   Every node that will be configured by Chef Infra Client and every
    workstation that will upload data to the Chef Infra Server must be
    able to communicate with the hosted Chef server

### Chef Infra Server, On-premises or in Cloud Environment

{{% system_requirements_server_hardware %}} {{% system_requirements_server_software %}}

## Chef Infra Client

-   The recommended amount of RAM available to Chef Infra Client during
    a Chef Infra Client run is 512MB
-   The Chef Infra Client binaries are stored in the `/opt/chef`
    directory, which requires a minimum of 200MB of disk space. On
    Windows, the Chef Infra Client binaries can be found in
    `C:\opscode\`, and they require a minimum of 600MB of disk space.
-   Chef Infra Client caches to `/var/chef/cache` during a Chef Infra
    Client run. This is the location in which downloaded cookbooks,
    packages required by those cookbooks, and other large files are
    stored. This directory requires enough space to save all of this
    data and should be generously sized. 5GB is a safe number as a
    starting point, but tune the size of `/var/chef/cache` as necessary.
    This location is tunable in a node's
    [client.rb](/config_rb_client/) file via the
    `file_cache_path` setting.

## Chef Workstation

-   64-bit architecture
-   4 GB of RAM or more
-   2 GB of free disk space
