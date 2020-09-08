+++
title = "About Chef Workstation"
draft = false

aliases = ["/about_workstation.html", "/about_chefdk.html", "/chef_dk.html", "/about_workstation/"]

[menu]
  [menu.workstation]
    title = "About Chef Workstation"
    identifier = "chef_workstation/about_workstation.md About Chef Workstation"
    parent = "chef_workstation"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/_index.md)

{{% chef_workstation %}}

Chef Workstation replaces ChefDK, combining all the existing features
with new features, such as ad-hoc task support and the new Chef
Workstation desktop application. Chef will continue to maintain ChefDK,
but new development will take place in Chef Workstation without
back-porting features.

## Getting Started

Chef Infra is a systems and cloud infrastructure automation framework
that makes it easy to deploy servers and applications to any physical,
virtual, or cloud location, no matter the size of the infrastructure.
Each organization is comprised of one (or more) Chef Workstation
installations, a single server, and every node that will be configured
and maintained by Chef Infra Client. Cookbooks (and recipes) are used to
tell Chef Infra Client how each node in your organization should be
configured. Chef Infra Client---which is installed on every node---does
the actual configuration.

-   [An Overview of Chef Infra](/chef_overview/)
-   [Install Chef Workstation](/workstation/install_workstation/)

### Cookbook Development Workflow

Chef Infra defines a common workflow for cookbook development:

1.  Create a skeleton cookbook by running <span class="title-ref">chef
    generate cookbook MY_COOKBOOK_NAME</span>. This will generate a
    cookbook with a single recipe and testing configuration with
    ChefSpec and Delivery Local Mode.
2.  Write cookbook recipes or resources and debug those recipes as they
    are being written using Cookstyle and Test Kitchen. This is
    typically an iterative process, where cookbooks are tested as they
    are developed, bugs are fixed quickly, and then cookbooks are tested
    again. A text editor---Visual Studio Code, Atom, vim, or any other
    preferred text editor---is used to author the files in the cookbook.
3.  Perform acceptance tests. These tests are not done in a development
    environment, but rather are done against a full Chef Infra Server
    using an environment that matches the production environment as
    closely as possible.
4.  When the cookbooks pass all the acceptance tests and have been
    verified to work in the desired manner, deploy the cookbooks to the
    production environment.

## Tools

Chef Workstation packages all the tools necessary to be successful with
Chef Infra and InSpec. These tools are combined into native packages for
common operating systems and include all the dependencies you need to
get started.

The most important tools included in Chef Workstation are:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Tool</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef CLI</td>
<td>A workflow tool for Chef.</td>
</tr>
<tr class="even">
<td>Chef Infra Client</td>
<td>The agent that runs Chef.</td>
</tr>
<tr class="odd">
<td>ChefSpec</td>
<td>A unit testing framework that tests resources locally.</td>
</tr>
<tr class="even">
<td>Cookstyle</td>
<td>A linting tool that helps you write better Chef Infra cookbooks by detecting and automatically correcting style, syntax, and logic mistakes in your code.</td>
</tr>
<tr class="odd">
<td>Delivery CLI</td>
<td>A command-line tool for continuous delivery workflow. Is used to setup and execute phase jobs on a Chef Automate server.</td>
</tr>
<tr class="even">
<td>Test Kitchen</td>
<td>An integration testing framework tool that tests cookbooks across platforms.</td>
</tr>
<tr class="odd">
<td>kitchen-dokken</td>
<td>A Test Kitchen plugin that provides a driver, transport, and provisioner for rapid cookbook testing and container development using Docker and Chef.</td>
</tr>
<tr class="even">
<td>kitchen-vagrant</td>
<td>A Test Kitchen plugin for local cookbook testing using Vagrant with VirtualBox, Parallels, VMware, and Hyper-V hypervisors</td>
</tr>
<tr class="odd">
<td>kitchen-ec2, kitchen-google, kitchen-azurerm</td>
<td>Test Kitchen drivers for cookbook testing on common cloud providers</td>
</tr>
<tr class="even">
<td>Ruby</td>
<td>The reference language for Chef Infra and InSpec.</td>
</tr>
</tbody>
</table>
