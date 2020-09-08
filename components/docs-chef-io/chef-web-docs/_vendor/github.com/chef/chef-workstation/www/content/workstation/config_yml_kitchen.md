+++
title = "kitchen.yml"
draft = false

aliases = ["/config_yml_kitchen.html", "/config_yml_kitchen/"]

[menu]
  [menu.workstation]
    title = "kitchen.yml"
    identifier = "chef_workstation/chef_workstation_tools/test_kitchen/config_yml_kitchen.md kitchen.yml"
    parent = "chef_workstation/chef_workstation_tools/test_kitchen"
    weight = 30
+++

[\[edit on GitHub\]](https://github.com/chef/chef-workstation/blob/master/www/content/workstation/config_yml_kitchen.md)

Use [Test Kitchen](https://kitchen.ci/) to automatically test cookbook
data across any combination of platforms and test suites:

-   Defined in a kitchen.yml file
-   Uses a driver plugin architecture
-   Supports cookbook testing across many cloud providers and
    virtualization technologies
-   Supports all common testing frameworks that are used by the Ruby
    community
-   Uses a comprehensive set of base images provided by
    [Bento](https://github.com/chef/bento)

{{% test_kitchen_yml %}}

{{< note >}}

This topic details functionality that is packaged with Chef Workstation.
See <https://kitchen.ci/docs/getting-started/> for more information
about Test Kitchen.

{{< /note >}}

## Syntax

{{% test_kitchen_yml_syntax %}}

## Provisioner Settings

Test Kitchen's provisioner settings will be changing in a future
version. See [Chef RFC
091](https://github.com/chef/chef-rfc/blob/master/rfc091-deprecate-kitchen-settings.md)
for details. Settings that will be deprecated are listed in the
descriptions below. The new recommended settings are listed in the [New
Provisioner Settings](/workstation/config_yml_kitchen/#new-provisioner-settings)
table.

Kitchen can configure the chef-zero provisioner with the following
Chef-specific settings:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>attributes</code></td>
<td>Chef attributes for use in the suite</td>
</tr>
<tr class="even">
<td><code>chef_client_path</code></td>
<td>Chef Infra Client provisioner only.</td>
</tr>
<tr class="odd">
<td><code>chef_metadata_url</code></td>
<td><strong>This will be deprecated in a future version.</strong></td>
</tr>
<tr class="even">
<td><code>chef_omnibus_install_options</code></td>
<td>Use to specify the package to be installed. Possible values: <code>-P chef</code> (for Chef Infra Client) and <code>-P chef-workstation</code> (for the Chef Infra Client that is packaged as part of Chef Workstation). Use <code>-n</code> to specify the nightly build. For example: <code>-P chef-workstation</code> or <code>-n -P chef-workstation</code>. <strong>This will be deprecated in a future version.</strong> See the <code>product_name</code>, <code>product_version</code>, and <code>channel</code> settings instead.</td>
</tr>
<tr class="odd">
<td><code>chef_omnibus_root</code></td>
<td>Default value: <code>/etc/opt</code> for UNIX and Linux, <code>$env:systemdrive\\opscode\\chef</code> on Microsoft Windows.</td>
</tr>
<tr class="even">
<td><code>chef_omnibus_url</code></td>
<td>The URL for an <code>install.sh</code> script that will install Chef Infra Client on the machine under test. Default value: <code>https://www.chef.io/chef/install.sh</code>. <strong>This will be deprecated in a future version.</strong></td>
</tr>
<tr class="odd">
<td><code>chef_solo_path</code></td>
<td>chef-solo provisioner only.</td>
</tr>
<tr class="even">
<td><code>chef_zero_host</code></td>
<td>Chef Infra Client provisioner only.</td>
</tr>
<tr class="odd">
<td><code>chef_zero_port</code></td>
<td>Chef Infra Client provisioner only. The port on which chef-zero is to listen.</td>
</tr>
<tr class="even">
<td><p><code>client_rb</code></p></td>
<td><p>Chef Infra Client provisioner only. A list of client.rb file settings. For example:</p>
<div class="sourceCode" id="cb1"><pre class="sourceCode yaml"><code class="sourceCode yaml"><span id="cb1-1"><a href="#cb1-1"></a><span class="fu">client_rb</span><span class="kw">:</span></span>
<span id="cb1-2"><a href="#cb1-2"></a><span class="at">  </span><span class="fu">log_level</span><span class="kw">:</span><span class="at"> :warn</span></span></code></pre></div></td>
</tr>
<tr class="odd">
<td><code>clients_path</code></td>
<td>The relative path to the directory in which client data is located. This data must be defined as JSON.</td>
</tr>
<tr class="even">
<td><code>cookbook_files_glob</code></td>
<td>A file glob (pattern) that matches files considered to be part of the cookbook. (Typically, this value does not need to be modified from the default.)</td>
</tr>
<tr class="odd">
<td><code>data_path</code></td>
<td>Use to specify the path from which non-cookbook files are copied to a Kitchen instance.</td>
</tr>
<tr class="even">
<td><code>data_bags_path</code></td>
<td>The relative path to a directory in which data bags and data bag items are defined. This data must be structured as if it were in the chef-repo.</td>
</tr>
<tr class="odd">
<td><code>deprecations_as_errors</code></td>
<td>Set to <span class="title-ref">true</span> to treat deprecation warning messages as error messages.</td>
</tr>
<tr class="even">
<td><code>driver</code></td>
<td>Use to specify a driver for a platform. This will override the default driver.</td>
</tr>
<tr class="odd">
<td><code>enforce_idempotency</code></td>
<td>Use with <code>multiple_converge</code> &gt; 1. Set to <code>true</code> to force test-kitchen to fail if last converge has any updated resources.</td>
</tr>
<tr class="even">
<td><code>encrypted_data_bag_secret_key_path</code></td>
<td>The path to an RSA key file that is used to decrypt encrypted data bag items.</td>
</tr>
<tr class="odd">
<td><code>environments_path</code></td>
<td>The relative path to the directory in which environment data is located. This data must be defined as JSON.</td>
</tr>
<tr class="even">
<td><code>http_proxy</code></td>
<td>The proxy server for HTTP connections.</td>
</tr>
<tr class="odd">
<td><code>https_proxy</code></td>
<td>The proxy server for HTTPS connections.</td>
</tr>
<tr class="even">
<td><code>no_proxy</code></td>
<td>The comma-separated exception list of host patterns to exclude from proxying.</td>
</tr>
<tr class="odd">
<td><code>install_msi_url</code></td>
<td>An alternate URL for a Windows MSI package that will install Chef Infra Client on the machine under test. <strong>This will be deprecated in a future version.</strong> Use the <code>download_url</code> setting instead.</td>
</tr>
<tr class="even">
<td><code>json_attributes</code></td>
<td>Chef Infra Client provisioner only.</td>
</tr>
<tr class="odd">
<td><code>log_file</code></td>
<td></td>
</tr>
<tr class="even">
<td><code>multiple_converge</code></td>
<td>Number of times to converge the node. Defaults to 1.</td>
</tr>
<tr class="odd">
<td><code>nodes_path</code></td>
<td>The relative path to the directory in which node data is located. This data must be defined as JSON.</td>
</tr>
<tr class="even">
<td><code>require_chef_omnibus</code></td>
<td>Use to install the latest version of Chef Infra Client on a node. Set to <code>true</code> to install the latest version, <code>false</code> to not install Chef Infra Client (assumes the box already has it installed), or a version specifier like <code>15.3.12</code> to install a particular version, or simply <code>15</code> to install the latest 15.x package. When set to <code>true</code> or a version number, the <code>chef_omnibus_url</code> may be used to specify the URL of the <code>install.sh</code> that installs the specified version of Chef Infra Client. Default value: <code>true</code>. <strong>This will be deprecated in a future version.</strong> See the <code>product_version</code> and <code>install_strategy</code> settings.</td>
</tr>
<tr class="odd">
<td><code>roles_path</code></td>
<td>The relative path to the directory in which role data is located. This data must be defined as JSON.</td>
</tr>
<tr class="even">
<td><code>root_path</code></td>
<td>The directory in which Kitchen will stage all content on the target node. This directory should be large enough to store all the content and must be writable. (Typically, this value does not need to be modified from the default value.) Default value: <code>/tmp/kitchen</code>.</td>
</tr>
<tr class="odd">
<td><code>ruby_bindir</code></td>
<td>Chef Infra Client provisioner only.</td>
</tr>
<tr class="even">
<td><code>run_list</code></td>
<td></td>
</tr>
<tr class="odd">
<td><code>solo_rb</code></td>
<td>chef-solo provisioner only.</td>
</tr>
<tr class="even">
<td><code>retry_on_exit_code</code></td>
<td>Takes an array of exit codes to indicate that kitchen should retry the converge command. Default value: <code>[35, 213]</code>.</td>
</tr>
<tr class="odd">
<td><code>max_retries</code></td>
<td>Number of times to retry the converge before passing along the failed status. Defaults value: 1.</td>
</tr>
<tr class="even">
<td><code>wait_for_retry</code></td>
<td>Number of seconds to wait between converge attempts. Default value: 30.</td>
</tr>
</tbody>
</table>

These settings may be added to the `provisioner` section of the
kitchen.yml file when the provisioner is chef-zero or chef-solo.

### New Provisioner Settings

<table>
<colgroup>
<col style="width: 15%" />
<col style="width: 55%" />
<col style="width: 5%" />
<col style="width: 25%" />
</colgroup>
<thead>
<tr class="header">
<th>New Setting</th>
<th>Description</th>
<th>Default</th>
<th>Replaces</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>product_name</code></td>
<td><code>chef</code> or <code>chef-workstation</code>. This setting must be specified in order to use the new settings. Using this setting overrides Test Kitchen's default behavior based on the <code>require_chef_omnibus</code> setting.</td>
<td></td>
<td><code>chef_omnibus_install_options</code></td>
</tr>
<tr class="even">
<td><code>product_version</code></td>
<td>Product version number. Supports partial version numbers.</td>
<td><code>latest</code></td>
<td><code>require_chef_omnibus</code></td>
</tr>
<tr class="odd">
<td><code>channel</code></td>
<td>Artifact repository name. <code>stable</code>, <code>current</code> or <code>unstable</code>.</td>
<td><code>stable</code></td>
<td><code>chef_omnibus_install_options</code></td>
</tr>
<tr class="even">
<td><code>install_strategy</code></td>
<td>Product install strategy. <code>once</code> (Don't install if any product installation detected), <code>always</code> or <code>skip</code>.</td>
<td><code>once</code></td>
<td><code>require_chef_omnibus</code></td>
</tr>
<tr class="odd">
<td><code>download_url</code></td>
<td>Direct package URL. Supports all platforms.</td>
<td></td>
<td><code>install_msi_url</code></td>
</tr>
<tr class="even">
<td><code>checksum</code></td>
<td>Optional setting when using <code>download_url</code>. Validates SHA256 checksum after download.</td>
<td></td>
<td></td>
</tr>
<tr class="odd">
<td><code>platform</code></td>
<td>Override platform.</td>
<td>&lt;auto detected&gt;</td>
<td></td>
</tr>
<tr class="even">
<td><code>platform_version</code></td>
<td>Override platform platform.</td>
<td>&lt;auto detected&gt;</td>
<td></td>
</tr>
<tr class="odd">
<td><code>architecture</code></td>
<td>Override platform architecture.</td>
<td>&lt;auto detected&gt;</td>
<td></td>
</tr>
</tbody>
</table>

{{< note >}}

There are two community provisioners for Kitchen:
[kitchen-dsc](https://github.com/smurawski/kitchen-dsc) and
[kitchen-pester](https://github.com/smurawski/kitchen-pester).

{{< /note >}}

## Transport Settings

Kitchen can configure a transport with the following settings for either
`ssh` or `winrm` transports:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>connection_retries</code></td>
<td>Maximum number of times to retry after a failed attempt to open a connection. The default is 5.</td>
</tr>
<tr class="even">
<td><code>connection_retry_sleep</code></td>
<td>Number of seconds to wait until attempting to make another connection after a failure.</td>
</tr>
<tr class="odd">
<td><code>max_wait_until_ready</code></td>
<td>Maximum number of attempts to determine if the test instance is ready to accept commands. This defaults to 600.</td>
</tr>
<tr class="even">
<td><code>password</code></td>
<td>The password used for authenticating to the test instance.</td>
</tr>
<tr class="odd">
<td><code>port</code></td>
<td>The port used to connect to the test instance. This defaults to <code>22</code> for the <code>ssh</code> transport and <code>5985</code> or <code>5986</code> for <code>winrm</code> using <code>http</code> or <code>https</code> respectively.</td>
</tr>
<tr class="even">
<td><code>username</code></td>
<td>The username used for authenticating to the test instance. This defaults to <code>administrator</code> for the <code>winrm</code> transport and <code>root</code> for the <code>ssh</code> transport. Some drivers may change this default.</td>
</tr>
</tbody>
</table>

These settings may be added to the `transport` section of the
kitchen.yml file when the transport is SSH:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>compression</code></td>
<td>Wether or not to use compression. The default is <code>false</code>.</td>
</tr>
<tr class="even">
<td><code>compression_level</code></td>
<td>The default is 6 if <code>compression</code> is <code>true</code>.</td>
</tr>
<tr class="odd">
<td><code>connection_timeout</code></td>
<td>Defaults to 15.</td>
</tr>
<tr class="even">
<td><code>keepalive</code></td>
<td>Defaults to <code>true</code>.</td>
</tr>
<tr class="odd">
<td><code>keepalive_interval</code></td>
<td>Defaults to 60.</td>
</tr>
<tr class="even">
<td><code>max_ssh_sessions</code></td>
<td>Maximum number of parallel ssh sessions.</td>
</tr>
<tr class="odd">
<td><code>ssh_key</code></td>
<td>Path to an ssh key identity file.</td>
</tr>
</tbody>
</table>

These settings may be added to the `transport` section of the
kitchen.yml file when the transport is WinRM:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Setting</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>elevated</code></td>
<td>When <code>true</code>, all commands are executed via a scheduled task. This may eliminate access denied errors related to double hop authentication, interacting with windows updates and installing some MSIs such as sql server and .net runtimes. Defaults to <code>false</code>.</td>
</tr>
<tr class="even">
<td><code>elevated_password</code></td>
<td>The password used by the identity running the scheduled task. This may be <code>null</code> in the case of service accounts. Defaults to <code>password</code>.</td>
</tr>
<tr class="odd">
<td><code>elevated_username</code></td>
<td>The identity that the task runs under. This may also be set to service accounts such as <code>System</code>. This defaults to <code>username</code>.</td>
</tr>
<tr class="even">
<td><code>rdp_port</code></td>
<td>Port used making <code>rdp</code> connections for <code>kitchen login</code> commands. Defaults to 3389.</td>
</tr>
<tr class="odd">
<td><code>winrm_transport</code></td>
<td>The transport type used by winrm as explained <a href="https://github.com/WinRb/WinRM">here</a>. The default is <code>negotiate</code>. <code>ssl</code> and <code>plaintext</code> are also acceptable values.</td>
</tr>
</tbody>
</table>

### Work with Proxies

{{< readFile_shortcode file="test_kitchen_yml_syntax_proxy.md" >}}

## Chef Infra Client Settings

A kitchen.yml file may define Chef Infra Client-specific settings, such
as whether to require the Chef installer or the URL from which Chef
Infra Client is downloaded, or to override settings in the client.rb
file:

``` yaml
provisioner:
  name: chef_zero *or* chef_solo
  require_chef_omnibus: true
  chef_omnibus_url: https://www.chef.io/chef/install.sh

...

suites:
  - name: config
    run_list:
    ...
    attributes:
      chef_client:
        load_gems:
          chef-handler-updated-resources:
            require_name: "chef/handler/updated_resources"
        config:
          log_level: ":debug"
          ssl_verify_mode: ":verify_peer"
          start_handlers: [{class: "SimpleReport::UpdatedResources", arguments: []}]
          report_handlers: [{class: "SimpleReport::UpdatedResources", arguments: []}]
          exception_handlers: [{class: "SimpleReport::UpdatedResources", arguments: []}]
      ohai:
        disabled_plugins: ["passwd"]
```

where:

-   `require_chef_omnibus` is used to ensure that the Chef installer
    will be used to install Chef Infra Client to all platform instances;
    `require_chef_omnibus` may also be set to `latest`, which means the
    newest version of Chef Infra Client for that platform will be used
    for cookbook testing
-   `chef_omnibus_url` is used to specify the URL from which Chef Infra
    Client is downloaded
-   All of the `attributes` for the `config` test suite contain specific
    client.rb settings for use with this test suite

## Driver Settings

Driver-specific configuration settings may be required. Use a block
similar to:

``` yaml
driver:
  name: driver_name
  optional_settings: values
```

Specific `optional_settings: values` may be specified.

### Bento

{{% bento %}}

### Drivers

{{% test_kitchen_drivers %}}

### kitchen-vagrant

{{% test_kitchen_driver_vagrant %}}

{{% test_kitchen_driver_vagrant_settings %}}

{{% test_kitchen_driver_vagrant_config %}}

## Examples

The following examples show actual kitchen.yml files used in
Chef-maintained cookbooks.

### Chef, Chef Workstation

The following example shows the provisioner settings needed to install
Chef Workstation, and then use the version of Chef that is embedded in
Chef Workstation to converge the node.

To install the latest version of Chef Workstation:

``` yaml
provisioner:
  ...
  chef_omnibus_install_options: -P chef-workstation
  chef_omnibus_root: /opt/chef-workstation
```

and to install a specific version of Chef Workstation:

``` yaml
provisioner:
  ...
  chef_omnibus_install_options: -P chef-workstation
  chef_omnibus_root: /opt/chef-workstation
  require_chef_omnibus: 0.9
```

### Microsoft Windows Platform

The following example shows platform settings for the Microsoft Windows
platform:

``` yaml
platforms:
  - name: eval-win2012r2-standard
    os_type: windows
    transport:
      name: winrm
      elevated: true
```

If `name` begins with `win` then the `os_type` defaults to `windows`.
The `winrm` transport is the default on Windows operating systems. Here
`elevated` is true which runs windows commands via a scheduled task to
imitate a local user.

### Chef Infra Client Cookbook

The following kitchen.yml file is part of the `chef-client` cookbook and
ensures Chef Infra Client is configured correctly.

``` yaml
driver:
  name: vagrant

provisioner:
  name: chef_zero

platforms:
  - name: centos-8
  - name: fedora-latest
  - name: ubuntu-1604
  - name: ubuntu-1804

suites:

- name: service_init
  run_list:
  - recipe[minitest-handler]
  - recipe[chef-client::config]
  - recipe[chef-client_test::service_init]
  - recipe[chef-client::init_service]
  attributes: {}

- name: service_runit
  run_list:
  - recipe[minitest-handler]
  - recipe[runit]
  - recipe[chef-client_test::service_runit]
  - recipe[chef-client::runit_service]
  attributes: {}

- name: service_upstart
  run_list:
  - recipe[minitest-handler]
  - recipe[chef-client_test::service_upstart]
  - recipe[chef-client::upstart_service]
  excludes: ["centos-5.9"]
  attributes: {}

- name: cron
  run_list:
  - recipe[minitest-handler]
  - recipe[chef-client::cron]
  attributes: {}

- name: delete_validation
  run_list:
  - recipe[chef-client::delete_validation]
  attributes: {}
```

### chef-splunk Cookbook

The following kitchen.yml file is part of the `chef-splunk` cookbook and
is used to help ensure the installation of the Splunk client and server
is done correctly.

``` yaml
driver:
  name: vagrant
  customize:
    memory: 1024

provisioner:
  name: chef_zero

platforms:
  - name: ubuntu-16.04
  - name: ubuntu-18.04
  - name: centos-7
  - name: centos-8

suites:
  - name: client
    run_list:
      - recipe[chef-splunk::default]
      - recipe[test::default]
    attributes:
      dev_mode: true
      splunk:
        accept_license: true

  - name: server
    run_list:
      - recipe[chef-splunk::default]
    attributes:
      dev_mode: true
      splunk:
        is_server: true
        accept_license: true
        ssl_options:
          enable_ssl: true
```

### yum Cookbook

The following kitchen.yml file is part of the `yum` cookbook:

``` yaml
driver:
  name: vagrant

provisioner:
  name: chef_zero

platforms:
  - name: centos-7
  - name: centos-8
  - name: fedora-latest

suites:
  - name: default
    run_list:
      - recipe[yum::default]
      - recipe[yum_test::test_repo]
```

### Platform Attributes

The following kitchen.yml file sets up a simple tiered configuration of
the Chef Infra Server, including two front-end servers, a single
back-end server, and two add-ons (Chef Push Jobs and Chef management
console). The `platforms` block uses an `attributes` section to define
Chef server-specific attributes that are used by all three test suites:

``` yaml
driver:
  name: vagrant

provisioner:
  name: chef_zero

platforms:
  - name: ubuntu-16.04
    attributes:
      chef-server:
        api_fqdn: backend.chef-server.com
        backend:
          fqdn: backend.chef-server.com
          ipaddress: 123.456.789.0
        frontends:
          frontend1.chef-server.com: 123.456.789.0
          frontend2.chef-server.com: 123.456.789.0
        urls:
          private_chef: http://123.456.789.0/path/to/private-chef_11.1.4-1_amd64.deb
          manage: http://123.456.789.0/path/to/opscode-manage_1.3.1-1_amd64.deb
          push_jobs: http://123.456.789.0/path/to/opscode-push-jobs-server_1.1.1-1_amd64.deb

suites:
  - name: frontend1
    driver:
      vm_hostname: frontend1.chef-server.com
      network:
      - ["private_network", {ip: "123.456.789.0"}]
      customize:
        memory: 2048
        cpus: 2
    run_list:
      - recipe[chef-server::configfile]
      - recipe[chef-server::ntp]
      - recipe[chef-server::server]
      - recipe[chef-server::frontend]
  - name: frontend2
    driver:
      vm_hostname: frontend2.chef-server.com
      network:
      - ["private_network", {ip: "123.456.789.0"}]
      customize:
        memory: 2048
        cpus: 2
    run_list:
      - recipe[chef-server::configfile]
      - recipe[chef-server::ntp]
      - recipe[chef-server::server]
      - recipe[chef-server::frontend]
  - name: backend
    driver:
      vm_hostname: backend.chef-server.com
      network:
      - ["private_network", {ip: "123.456.789.0"}]
      customize:
        memory: 8192
        cpus: 4
    run_list:
      - recipe[chef-server::configfile]
      - recipe[chef-server::ntp]
      - recipe[chef-server::server]
      - recipe[chef-server::backend]
```

### Kitchen Converge On System Reboot

Test-Kitchen can handle reboots (when initiated from Chef Infra Client)
by setting `retry_on_exit_code`, `max_retries` and `wait_for_retry`
attributes on the provisioner in `kitchen.yml` file as follows :

``` yaml
provisioner:
   name: chef_zero
   retry_on_exit_code:
     - 35 # 35 is the exit code signaling that the node is rebooting
     - 1
   max_retries: 1
   client_rb:
     exit_status: :enabled # Opt-in to the standardized exit codes
     client_fork: false  # Forked instances don't return the real exit code
```

**One note on Linux nodes**: The shutdown command blocks (as opposed to
the Windows variant which registers the reboot and returns right away),
so once the timeout period passes, Chef Infra Client and the node are in
a race to see who can exit/shutdown first - so you may or may not get
the exit code out of Linux instances. In that case, you can add `1` to
the `retry_on_exit_code` array and that should catch both cases.

Please refer [YAML
documentation](https://symfony.com/doc/current/components/yaml/yaml_format.html#collections)
to set `retry_on_exit_code` attribute.
