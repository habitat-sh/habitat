+++
title = "Chef Workstation in Azure Cloud Shell"
draft = false

aliases = ["/azure_cwa_cloud_shell.html"]

[menu]
  [menu.infra]
    title = "Chef Workstation in Azure Cloud Shell"
    identifier = "chef_infra/getting_started/chef_on_azure_guide/azure_cwa_cloud_shell.md Chef Workstation in Azure Cloud Shell"
    parent = "chef_infra/getting_started/chef_on_azure_guide"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/azure_cwa_cloud_shell.md)

Chef Workstation is available in Azure Cloud Shell, allowing users to
run ad-hoc configurations on target systems and any other Chef command
when connected to an Azure subscription. This makes all of the Chef
command line tools available, without installing software on a local
machine.

Chef Workstation on Azure Cloud Shell lets you use:

-   [chef](/ctl_chef/)
-   [kitchen](/workstation/ctl_kitchen/)
-   [inspec](/docs/reference/cli/)
-   [knife](/workstation/knife/)
-   [cookstyle](/workstation/cookstyle/)
-   [chef-run](https://www.chef.sh/docs/chef-workstation/getting-started/)

Combine the Chef Workstation command utilities with the other tools
available in Cloud Shell, such as git, az-cli, terraform to write your
infrastructure and compliance automation from the browser--without the
need for a local shell.

## Azure Cloud Shell Installation

Ensure you have an accessible Azure Cloud Shell instance. You may need
to create a storage account to use Azure Cloud Shell if you haven't used
it before in this tenant. For more information on accessing, setting up,
and using Azure Cloud Shell, see the [Cloud Shell
Documentation](https://docs.microsoft.com/en-us/azure/cloud-shell/quickstart).
