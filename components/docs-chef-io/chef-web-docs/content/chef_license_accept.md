+++
title = "Accepting the Chef License"
draft = false

aliases = ["/chef_license_accept.html"]

[menu]
  [menu.overview]
    title = "Accepting License"
    identifier = "overview/packages_&_platforms/licensing/chef_license_accept.md Accepting License"
    parent = "overview/packages_&_platforms/licensing"
    weight = 20
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_license_accept.md)

This page aims to document how to accept the Chef license for all Chef
Software products. For an overview of the license, see the [Chef
license](/chef_license/) documentation. There are two types of
license: MLSA and EULA. The MLSA applies to customers with a commercial
contract with Chef Software, and the EULA covers all other cases.

## Accept the Chef MLSA

There are three ways to accept the Chef MLSA:

1.  When running `chef-<PRODUCT-NAME>-ctl reconfigure` the Chef MLSA is
    printed. Type `yes` to accept it. Anything other than typing `yes`
    rejects the Chef MLSA, and the upgrade process will exit. Typing
    `yes` adds a `.license.accepted` file to the
    `/etc/chef/accepted_licenses/<PRODUCT-NAME>` file. As long as this
    file exists in this directory, the Chef MLSA is accepted and the
    reconfigure process will not prompt for `yes`.
2.  Run the `chef-<PRODUCT-NAME>-ctl reconfigure` command using the
    `--chef-license=accept` option. This automatically types `yes` and
    skips printing the Chef MLSA.
3.  Add a `.license.accepted` file to the `/var/opt/<PRODUCT-NAME>/`
    directory. The contents of this file do not matter. As long as this
    file exists in this directory, the Chef MLSA is accepted and the
    reconfigure process will not prompt for `yes`.

## Accept the Chef EULA

Products below are split below into two categories: workstation and
server. Affected product versions which require accepting the EULA to
use are shown. Versions before this do not require accepting the EULA.
More information on supported versions can be seen at the [Supported
Versions](/versions/) documentation.

### Workstation Products

- Chef Workstation \>= 0.4, which also contains:
  -   Chef Infra Client
  -   Chef InSpec
  -   Push Jobs Client
- Chef Infra Client \>= 15.0
- Chef InSpec \>= 4.0
- Chef Habitat \>= 0.80

These products are typically installed on a user's workstation. Two
methods are generally used to accept the license for these products:

1.  `--chef-license <value>` argument passed to the command line
    invocation.
2.  `CHEF_LICENSE="<value>"` as an environment variable.

`<value>` can be specified as one of the following:

1.  `accept` - Accepts the license and attempts to persist a marker file
    locally. Persisting these marker files means future invocations do
    not require accepting the license again.
2.  `accept-silent` - Similar to `accept` except no messaging is sent to
    STDOUT
3.  `accept-no-persist` - Similar to `accept-silent` except no marker
    file is persisted. Future invocation will require accepting the
    license again.

If no command line argument or environment variable is set, these
products will attempt to request acceptance through an interactive
prompt. If the prompt cannot be displayed, then the product will fail
with an exit code 172.

If the product attempts to persist the accepted license and fails, a
message will be sent to STDOUT, but product invocation will continue
successfully. In a future invocation, however, the license would need to
be accepted again.

Please see [License File
Persistence](https://github.com/chef/license-acceptance#license-file-persistence)
for details about persisted marker files.

The `--chef-license` command line argument is not backwards compatible
to older non-EULA versions. If you are managing a multi-version
environment, we recommend using the environment variable as that is
ignored by older versions.

Products with specific features or differences from this general
behavior are documented below.

#### Chef Workstation

Chef Workstation contains multiple Chef Software products. When invoking
the `chef` command line tool and accepting the license, users are
required to accept the license for all the embedded products as well.
The same license applies to all products, but each product must have its
own license acceptance. `chef <command> --chef-license accept` will
accept the license for Chef Workstation, Chef Infra Client, Chef InSpec,
and Push Jobs Client. For example, <span class="title-ref">chef env
--chef-license accept</span>

#### Chef Infra Client

In addition to the above methods, users can specify
`chef_license 'accept'` in their Chef Infra Client and Chef Infra Server
config. On a workstation, this can be specified in `~/.chef/config.rb`
or `~/.chef/knife.rb`, and on a node, it can be specified in
`/etc/chef/client.rb`. This method of license acceptance is
backwards-compatible to non-EULA versions of Chef Infra Client.

#### Habitat

Two methods are generally used to accept the Chef Habitat license:

1.  Users can execute `hab license accept` on the command line.
2.  Alternatively, users can set `HAB_LICENSE="<value>"` as an
    environment variable.

`<value>` can be specified as one of the following:

1.  `accept` - Accepts the license and persists a marker file locally.
    Future invocations do not require accepting the license again.
2.  `accept-no-persist` - accepts the license without persisting a
    marker file. Future invocation will require accepting the license
    again.

If the license isn't accepted through either of these methods, Habitat
will request acceptance through an interactive prompt.

Additionally, to accepting the license in CI or other automation, user
may choose to create an empty file on the filesystem at
`/hab/accepted-licenses/habitat` (if your hab commands run as root) or
at `$HOME/.hab/accepted-licenses/habitat` (if your hab commands run as a
user other than root). For situations where hab commands run as multiple
users, it is advisable to create both files.

**Errors**

If the Chef Habitat License prompt cannot be displayed, then the product
fails with an exit code 172. If Chef Habitat cannot persist the accepted
license, it sends a message STDOUT, but the product invocation will
continue successfully. In a future invocation, however, the user will
need to accept the license again.

**Chef as Habitat packages**

Chef Software products are also distributed as Habitat packages, such as
Chef Infra Client, Chef InSpec, etc. When Chef products are installed as
Habitat, the products request license acceptance at product usage time.
Whether installed as system packages or as Habitat packages, users
accept the licenses in the same way detailed above.

### Server Products

Some Chef products distributed as Habitat packages contain servers. In
these cases, Habitat runs the server products as a supervisor. See the
below sections for information on how to accept the license for these
products when they are distributed as Habitat packages.

<table>
<colgroup>
<col style="width: 20%" />
<col style="width: 79%" />
</colgroup>
<thead>
<tr class="header">
<th>Product</th>
<th>Version</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Chef Infra Server</td>
<td>&gt;= 13.0, which also contains Push Jobs Server</td>
</tr>
<tr class="even">
<td>Chef Automate</td>
<td>&gt;= 2.0</td>
</tr>
<tr class="odd">
<td>Push Jobs Server</td>
<td>&gt;= 3.0</td>
</tr>
<tr class="even">
<td>Supermarket</td>
<td>&gt;= 4.0</td>
</tr>
</tbody>
</table>

Server products are typically installed and managed by some kind of
process supervisor. Chef Software server products do not allow
interactive license acceptance because process supervisors do not easily
allow interactivity. Instead, the license is accepted during the
`reconfigure` command or `upgrade` command for the Omnibus ctl command.
For example:

-   `chef-server-ctl reconfigure --chef-license=accept`
-   `CHEF_LICENSE="accept-no-persist" supermarket-ctl reconfigure`

In addition, the Chef license can be accepted via the omnibus
configuration file. Specify `chef_license 'accept'` in the
`chef-server.rb` or `supermarket.rb` configuration.

#### Chef Automate

Automate has its own reconfigure tool, `automate-ctl`. This tool walks
users through the install and setup of Automate. The Chef license is
accepted after that in the browser. Please follow the in-product
prompts.

#### Chef Infra Server

When installed as a system package, users accept the license with the
ctl command. For example,
`chef-server-ctl reconfigure --chef-license=accept`. Acceptance can also
be set in the configuration file `chef-server.rb` as
`chef_license "accept"`.

Chef Infra Server is also distributed as a Habitat package and ran using
the Habitat supervisor. In this mode, users accept the license by
setting the correct Habitat configuration values. The key is
`chef_license.acceptance`.

For example: Against a supervisor running Chef Infra Server, run
`echo "chef_license.acceptance = accept" | hab config apply server.default 100`.
See the [Habitat config updates
documentation](https://www.habitat.sh/docs/using-habitat/#config-updates)
for more information about how to apply this configuration to a service
group.

### Remote Management Products

-   Test Kitchen
-   `knife bootstrap` in Chef Infra Client
-   `chef-run` in Chef Workstation
-   Packer
-   Terraform Chef Provisioner
-   Terraform Habitat Provisioner
-   Vagrant

These products install or manage Chef on a remote instance. If a user
has accepted the appropriate product license locally, it will be
automatically transferred to the remote instance. For example, if a user
has accepted the Chef Infra Client license locally and converges a Test
Kitchen instance with the Chef provisioner, it will succeed by copying
the acceptance to the remote instance. We aim to support this behavior,
so Workstation users do not have their workflow affected, but any
differences from that behavior are documented below.

#### Test Kitchen

Test Kitchen is not owned by or covered by the Chef license, but
installing Chef Infra Client on a test instance is covered by the EULA.
Without accepting the license, the converge will fail on the test
instance.

The Chef provisioner in Test Kitchen \>= 2.3 has been updated to
simplify accepting this license on behalf of the test instance. Users
can set the `CHEF_LICENSE` environment variable or add
`chef_license: accept` to their provisioner config in their <span
class="title-ref">kitchen.yml</span>. Specifying <span
class="title-ref">accept</span> will attempt to persist the license
acceptance locally. If a local license marker file is detected, no
configuration is required; acceptance is automatically transferred to
the test instance.

To disable this persistence, specify `accept-no-persist` on every test
instance converge.

`kitchen-inspec` uses Chef InSpec as a library, and is not covered by
the EULA when installed as a gem, but is covered by the EULA when
packaged as part of the Chef Workstation installation. Accept the
license in a similar way to the Chef Infra Client license - specify the
`CHEF_LICENSE` environment variable, specify the `chef_license` config
under the verifier section in `kitchen.yml` or persist the acceptance
locally.

**Pin to Chef 14**

You can pin to a specific version of chef in your kitchen.yml:

``` none
provisioner:
  name: chef_zero
  product_name: chef
  product_version: 14.12.3
```

#### `knife bootstrap`

`knife` usage does not require accepting the EULA. A Chef Infra Client
instance does require EULA acceptance. Using `knife bootstrap` to manage
a Chef Infra Client instance will prompt a user to accept the license
locally before allowing for bootstrapping the remote instance. Without
this, `knife bootstrap` would fail.

In most usage cases via Chef Workstation, this license will already have
been accepted and will transfer across transparently. But if a user
installs Chef Workstation and the first command they ever run is
`knife bootstrap`, it will perform the same license acceptance flow as
the Chef Infra Client product.

**`knife bootstrap` in Chef Client 14**

The `knife bootstrap` command in Chef Client 14 cannot accept the Chef
Infra Client 15 EULA on remote nodes unless you use a [custom
template](/workstation/knife_bootstrap/#custom-templates)
and add chef_license "accept" to the client.rb. This applies to
workstations who have Chef Infra Client \<= 14.x, ChefDK \<= 3.x or Chef
Workstation \<= 0.3 installed.

**Pin to Chef 14**

Specify the following argument:

``` bash
knife bootstrap --bootstrap-version 14.12.3
```

#### `chef-run`

`chef-run` in Chef Workstation \>= 0.3 has been updated to add support
for accepting the license locally when remotely running Chef Infra
Client 15. As of Chef Workstation \<= 0.4 there is no way to manage the
version of Chef Infra Client installed on the remote node. It defaults
to the latest stable version available.

To accept the license, complete one of the following three tasks. Either
pass the `--chef-license` command line flag, set the `CHEF_LICENSE`
environment variable, or add the following to your
`~/.chef-workstation/config.toml` file:

``` none
[chef]
chef_license = "accept"
```

#### Packer

Use a custom [Chef configuration
template](https://www.packer.io/docs/provisioners/chef-client.html#chef-configuration).
In your provisioners config, include:

``` json
{
  "type":            "chef-client",
  "config_template": "path/to/client.rb"
}
```

In `path/to/client.rb`, include:

``` ruby
chef_license "accept"
```

You may also add it to the
[execute_command](https://www.packer.io/docs/provisioners/chef-client.html#execute_command),
but this is not backwards-compatible, so it is not suggested.

**Pin to Chef 14**

In your [Packer provisioners
config](https://www.packer.io/docs/provisioners/chef-client.html#install_command),
include:

``` json
{
  "type":            "chef-client",
  "install_command": "curl -L https://omnitruck.chef.io/install.sh | sudo bash -s -- -v 14.12.9"
}
```

#### Terraform Chef Provisioner

The license can be accepted via the Chef Infra Client config file, which
is specified by the `client_options` [Terraform provisioner
config](https://www.terraform.io/docs/provisioners/chef.html#client_options-array-):

``` none
provisioner "chef" {
  client_options = ["chef_license 'accept'"]
}
```

**Pin to Chef 14**

In your [Terraform provisioner
config](https://www.terraform.io/docs/provisioners/chef.html#version-string-),
include:

``` none
provisioner "chef" {
  version = "14.12.3"
}
```

#### Terraform Habitat Provisioner

Default behavior of this provisioner is to install the latest version of
Habitat. [Documentation for this
provisioner](https://www.terraform.io/docs/provisioners/habitat.html)
will be updated in the near future once the provisioner is updated with
options to accept license. For the time being, the provisioner can be
pinned to a prior Habitat version as below.

**Pin to Chef Habitat 0.79**

In your [Terraform provisioner
config](https://www.terraform.io/docs/provisioners/habitat.html#version-string-),
include:

``` none
provisioner "habitat" {
  version = "0.79.1"
}
```

#### Vagrant

This license acceptance can be done via the arguments API:

``` ruby
config.vm.provision "chef_zero" do |chef|
  chef.arguments = "--chef-license accept"
end
```

See
<https://www.vagrantup.com/docs/provisioning/chef_common.html#arguments>
for details. The `--chef-license` argument is not backwards-compatible
to non-EULA Chef Infra Client versions. So instead, users can use the
[custom config
path](https://www.vagrantup.com/docs/provisioning/chef_common.html#custom_config_path)
and point at a local file, which specifies the `chef_license` config.
The environment variable is not currently supported.

**Pin to Chef 14**

This version pinning can be done via the [version
API](https://www.vagrantup.com/docs/provisioning/chef_common.html#version).
In your Chef provisioner config:

``` ruby
config.vm.provision "chef_zero" do |chef|
  chef.version = "14.12.3"
end
```

### Pre-upgrade support

Chef Software aims to make upgrading from a non-EULA version to a EULA
version as simple as possible. For some products (Chef Client 14.12.9,
Chef InSpec 3.9.3), we added backwards-compatible support for the
`--chef-license` command that performs a no-op. This allows customers to
start specifying that argument in whatever way they manage those
products before upgrading.

Alternatively, users can specify the `CHEF_LICENSE` environment variable
when invoking any of the EULA products to accept the license. This
environment variable is ignored by non-EULA products, and so is
backwards-compatible to older versions.

#### `chef-client` cookbook

For users that manage their Chef Infra Client installation using the
`chef-client` cookbook, we added a new attribute that can be specified.
Specify the node attribute
`node['chef_client']['chef_license'] = 'accept'` when running the
cookbook to apply the license acceptance in a backwards-compatible way.

This functionality allows users to set that attribute for a Chef Client
14 install, upgrade to Chef Infra Client 15, and have the product
continue to work correctly.
