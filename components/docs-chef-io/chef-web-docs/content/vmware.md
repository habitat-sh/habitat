+++
title = "Chef and VMware"
draft = false

aliases = ["/vmware.html"]

[menu]
  [menu.infra]
    title = "VMware"
    identifier = "chef_infra/setup/integrations/vmware.md VMware"
    parent = "chef_infra/setup/integrations"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/vmware.md)

VMware, Inc. is a subsidiary of Dell Technologies that provides cloud
computing and platform virtualization software and services. This page
outlines the different tools that can be used to integrate Chef with the
VMware platform.

For discussions on VMware and Chef, visit the
[VMware{code}](https://code.vmware.com/web/code/join) Slack team,
located in the **\#chef** channel.

## knife

There are multiple knife plugins that interact with the VMware stack in
different ways. The following knife plugins are directly supported by
Chef:

### knife-vsphere

[\[GitHub\]](https://github.com/chef-partners/knife-vsphere)

-   Supports vCenter \> 5.0
-   Most VMware compute use cases are covered
-   The main starting point for Chef and VMware

These are the necessary settings for your `config.rb` file:

``` ruby
knife[:vsphere_host] = "vcenter-hostname"
knife[:vsphere_user] = "privileged username" # Domain logins may need to be "user@domain.com"
knife[:vsphere_pass] = "password"       # or %Q(mypasswordwithfunnycharacters)
knife[:vsphere_dc] = "your-datacenter"
knife[:vsphere_insecure] = true              # Set this if you have self signed certs
```

#### Usage Examples

**Clone from a VMware template and bootstrap Chef with generic DHCP
options:**

``` bash
knife vsphere vm clone MACHINENAME --template TEMPLATENAME --bootstrap --cips dhcp
```

**Clone a virtual machine from a VMware template, use a customization
template called "SPEC" to assist the bootstrapping process, and specify
the SSH user and password:**

``` bash
knife vsphere vm clone MACHINENAME --template TEMPLATENAME --bootstrap --cips dhcp \
--cspec SPEC --connection-user USER --connection-password PASSWORD
```

{{< note >}}

Add a `-f FOLDERNAME` if you put your `--template` in a directory other
than the root folder. Use `--dest-folder FOLDERNAME` if you want your VM
created in `FOLDERNAME` instead of the root folder.

{{< /note >}}

**Clone from a folder into the "Datacenter Root" directory:**

``` bash
knife vsphere vm clone MACHINENAME --template TEMPLATENAME -f /path/to/template \
--bootstrap --start --cips dhcp --dest-folder /
```

**List the available VMware templates:**

``` bash
knife vsphere template list
Template Name: ubuntu16-template
knife vsphere template list -f FOLDERNAME
Template Name: centos7-template
```

**Delete a machine:**

``` bash
knife vsphere vm delete MACHINENAME
```

This command can be used with the `-P` option to remove the machine from
the Chef Infra Server.

### knife-vcenter

[\[GitHub\]](https://github.com/chef/knife-vcenter)

-   Supports vCenter \>= 6.5 REST API
-   Supports the main use cases of knife: `bootstrap`, `create`,
    `destroy`, and `list`
-   If you have the
    [VCSA](https://docs.vmware.com/en/VMware-vSphere/6.5/com.vmware.vsphere.vcsa.doc/GUID-223C2821-BD98-4C7A-936B-7DBE96291BA4.html)
    or are planning on upgrading to vCenter 6.5+, this is the plugin to
    use

The main settings for your `config.rb`:

``` ruby
knife[:vcenter_username] = "user"
knife[:vcenter_password] = "password"
knife[:vcenter_host] = "172.16.20.2"
knife[:vcenter_disable_ssl_verify] = true # if you want to disable SSL checking
```

#### Usage Examples

**Clone a machine:**

``` bash
knife vcenter vm clone example-01 --targethost 172.16.20.3 --folder example --connection-password \
P@ssw0rd! --datacenter Datacenter --template ubuntu16-template -N example-01
Creating new machine
Waiting for network interfaces to become available...
ID: vm-183
Name: example-01
Power State: POWERED_ON
Bootstrapping the server by using bootstrap_protocol: ssh and image_os_type: linux

Waiting for sshd to host (10.0.0.167)
...
```

**Delete a machine:**

``` bash
knife vcenter vm delete example-01 -N example-01 --purge
Creating new machine
Waiting for network interfaces to become available...
ID: vm-183
Name: example-01
Power State: POWERED_ON
Bootstrapping the server by using bootstrap_protocol: ssh and image_os_type: linux

Waiting for sshd to host (10.0.0.167)
WARNING: Deleted node example-01
WARNING: Deleted client example-01
```

### knife-vrealize

[\[GitHub\]](https://github.com/chef-partners/knife-vrealize)

-   Supports both vRealize Automation and vRealize Orchestrator
-   Supports vRealize Automation 7.0+
-   If you have vRealize Automation \< 7.0, you will need to downgrade
    the
    [vmware-vra-gem](https://github.com/chef-partners/vmware-vra-gem) to
    version `1.7.0`
-   Supports the main use cases of knife: `bootstrap`, `create`,
    `destroy`, and `list`
-   Directly integrates with vRA to call out predetermined blueprints or
    catalogs
-   Can integrate directly with vRO to call out predetermined workflows

The main settings for your `config.rb`:

``` ruby
knife[:vra_username] = 'user'
knife[:vra_password] = 'password'
knife[:vra_base_url] = 'https://vra.corp.local'
knife[:vra_tenant]   = 'tenant'
knife[:vra_disable_ssl_verify] = true # if you want to disable SSL checking.
```

Additional `config.rb` settings are required to integrate with vRO:

``` ruby
knife[:vro_username] = 'user'
knife[:vro_password] = 'password'
knife[:vro_base_url] = 'https://vra.corp.local:8281'
```

A basic clone example for vRA is:

Creates a server from a catalog blueprint. Find the catalog ID with the
`knife vra catalog list` command. After the resource is created, knife
will attempt to bootstrap it.

Each blueprint may require different parameters to successfully complete
provisioning. See your vRA administrator with questions. Knife will
attempt to provide any helpful error messages from vRA if they're
available.

Common parameters to specify are:

-   `--cpus`: number of CPUs
-   `--memory`: amount of RAM in MB
-   `--requested-for`: vRA login that should be listed as the owner
-   `--lease-days`: number of days for the resource lease
-   `--notes`: any optional notes you'd like to be logged with your
    request
-   `--subtenant-id`: all resources must be tied back to a Business
    Group, or "subtenant." If your catalog item is tied to a specific
    Business Group, you do not need to specify this. However, if your
    catalog item is a global catalog item, then the subtenant ID is not
    available to knife; you will need to provide it. It usually looks
    like a UUID. See your vRA administrator for assistance in
    determining your subtenant ID.
-   `--connection-password`: for Linux hosts, the password to use during
    bootstrap
-   `--winrm-password`: for Windows hosts, the password to use during
    bootstrap

<!-- -->

``` bash
knife vra server create 5dcd1900-3b89-433d-8563-9606ae1249b8 --cpus 1 --memory 512 \
--requested-for devmgr@corp.local --connection-password my_password --lease-days 5
Catalog request d282fde8-6fd2-406c-998e-328d1b659078 submitted.
Waiting for request to complete.
Current request status: PENDING_PRE_APPROVAL.
Current request status: IN_PROGRESS..
...
```

#### Usage Examples

**Delete a server from vRA:**

``` bash
knife vra server delete 2e1f6632-1613-41d1-a07c-6137c9639609 --purge
Server ID: 2e1f6632-1613-41d1-a07c-6137c9639609
Server Name: hol-dev-43
IP Addresses: 192.168.110.203
Status: ACTIVE
Catalog Name: CentOS 6.6

Do you really want to delete this server? (Y/N) Y
Destroy request f2aa716b-ab24-4232-ac4a-07635a03b4d4 submitted.
Waiting for request to complete.
Current request status: PENDING_PRE_APPROVAL.
Current request status: IN_PROGRESS...
...
```

If you supply the `--purge` option, the server will also be removed from
the Chef Infra Server

**Execute a vRO workflow:**

``` bash
knife vro workflow execute "knife testing" key1=value1
Starting workflow execution...
Workflow execution 4028eece4effc046014f27da864d0187 started. Waiting for it to complete...
Workflow execution complete.

Output Parameters:
outkey1: some value (string)

Workflow Execution Log:
2015-08-13 09:17:57 -0700 info: cloudadmin: Workflow 'Knife Testing' has started
2015-08-13 09:17:58 -0700 info: cloudadmin: Workflow 'Knife Testing' has completed
```

This requires the workflow name. You may supply any input parameters, as
well. If your workflow name is not unique in your vRO workflow list, you
can specify a workflow to use with `--vro-workflow-id ID`. You can find
the workflow ID from within the vRO UI. However, a workflow name is
still required by the API.

## test-kitchen

The following test-kitchen drivers for VMware are directly supported by
Chef:

### kitchen-vsphere (chef-provisioning-vsphere)

[\[GitHub\]](https://github.com/chef-partners/chef-provisioning-vsphere)

-   Built into the chef-provisioning-vsphere driver
-   A community driven project, with Chef Partners maintaining the
    releases
-   Leverages the typical test-kitchen workflow for vCenter \> 5.0+
-   There is a
    [kitchen-vsphere](https://rubygems.org/gems/kitchen-vsphere) gem,
    but it is not supported at this time

#### Usage Examples

There is an [example
cookbook](https://github.com/jjasghar/vsphere_testing) that attempts to
capture everything required. The following is a basic `kitchen.yml`
example:

``` yaml
---
driver:
name: vsphere
driver_options:
  host: FQDN or IP of vCenter
  user: 'administrator@vsphere.local'
  password: 'PASSWORD'
  insecure: true
machine_options:
 start_timeout: 600
 create_timeout: 600
 ready_timeout: 90
 bootstrap_options:
   use_linked_clone: true
   datacenter: 'Datacenter'
   template_name: 'ubuntu16'
   template_folder: 'Linux'
   resource_pool: 'Cluster'
   num_cpus: 2
   memory_mb: 4096
   ssh:
     user: ubuntu
     paranoid: false
     password: PASSWORD
     port: 22

provisioner:
  name: chef_zero
  sudo_command: sudo

verifier:
  name: inspec

transport:
  username: root or ssh enabled user
  password: PASSWORD for root or user

platforms:
  - name: ubuntu-18.04
  - name: centos-8

suites:
  - name: default
    run_list:
      - recipe[COOKBOOK::default]
    attributes:
```

### kitchen-vcenter

[\[GitHub\]](https://github.com/chef/kitchen-vcenter)

-   Supports vCenter \>= 6.5 REST API
-   Leverages the typical test-kitchen workflow for vCenter \>= 6.5+
-   If you have the
    [VCSA](https://docs.vmware.com/en/VMware-vSphere/6.5/com.vmware.vsphere.vcsa.doc/GUID-223C2821-BD98-4C7A-936B-7DBE96291BA4.html)
    or are planning on upgrading to vCenter 6.5+, use this plugin

#### Usage Examples

The following is a basic `kitchen.yml` for vCenter:

``` yaml
driver:
  name: vcenter
  vcenter_username: <%= ENV['VCENTER_USER'] || "administrator@vsphere.local" %>
  vcenter_password: <%= ENV['VCENTER_PASSWORD'] || "password" %>
  vcenter_host: vcenter.chef.io
  vcenter_disable_ssl_verify: true
  driver_config:
    targethost: 172.16.20.41
    datacenter: "Datacenter"

platforms:
  - name: ubuntu-2004
    driver_config:
      template: ubuntu16-template
  - name: centos-8
    driver_config:
      template: centos7-template
```

### kitchen-vra

[\[GitHub\]](https://github.com/chef-partners/kitchen-vra)

-   An integration point with vRA and test-kitchen
-   For companies required to use vRA this is a natural progression for
    Chef Development

#### Usage Examples

The following is a basic `kitchen.yml` example:

``` yaml
driver:
  name: vra
  username: user@corp.local
  password: password
  tenant: tenant
  base_url: https://vra.corp.local
  verify_ssl: true

platforms:
- name: centos6
  driver:
    catalog_id: e9db1084-d1c6-4c1f-8e3c-eb8f3dc574f9
- name: centos7
  driver:
    catalog_id: c4211950-ab07-42b1-ba80-8f5d3f2c8251
```

### kitchen-vro

[\[GitHub\]](https://github.com/chef-partners/kitchen-vro)

-   An integration point with vRO and test-kitchen
-   Leverages specific Workflows in vRO if it's required by VMware
    admins

#### Usage Examples

The following is a basic `kitchen.yml` example:

``` yaml
driver:
  name: vro
  vro_username: user@domain.com
  vro_password: password
  vro_base_url: https://vra.corp.local:8281
  create_workflow_name: Create TK Server
  destroy_workflow_name: Destroy TK Server

platforms:
  - name: centos
    driver:
      create_workflow_parameters:
        os_name: centos
        os_version: 6.7
  - name: windows
    driver:
      create_workflow_parameters:
        os_name: windows
        os_version: server2012
        cpus: 4
        memory: 4096
```

## Chef InSpec

The Chef InSpec VMware plugin is used to verify the vCenter and ESXi
VMware stack.

### inspec-vmware

[\[GitHub\]](https://github.com/chef/inspec-vmware)

-   Supports vCenter \> 5.0
-   11 resources available at the time of writing, with more planned

#### Usage Examples

An example demo control:

``` ruby
control "vmware-1" do
  impact 0.7
  title 'Checks that soft power off is disabled'
  describe vmware_vm_advancedsetting({datacenter: 'ha-datacenter', vm: 'testvm'}) do
    its('softPowerOff') { should cmp 'false' }
  end
end
```

## Chef integrations inside of the VMware Suite

### vRA Example Blueprints

-   [Linux](https://code.vmware.com/samples?id=1371)
-   [Windows](https://code.vmware.com/samples?id=1390)

### vRO plugin

-   The [Chef plugin for vRealize
    Orchestrator](https://solutionexchange.vmware.com/store/products/chef-plugin-for-vrealize-orchestrator)
    (vRO) is a VMware-supplied plugin
-   If you use vRO this provides the majority of the necessary features

For more information, see the plugin demo on
[YouTube](https://www.youtube.com/watch?v=HlvoZ4Zdwc4).
