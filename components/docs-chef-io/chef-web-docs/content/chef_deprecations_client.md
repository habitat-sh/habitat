+++
title = "Chef Deprecation Warnings"
draft = false

aliases = ["/chef_deprecations_client.html"]

[menu]
  [menu.infra]
    title = "Deprecations"
    identifier = "chef_infra/chef_deprecations_client.md Deprecations"
    parent = "chef_infra"
    weight = 90
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_deprecations_client.md)

When we wish to remove a feature or an API in Chef, we try to first mark
it with a deprecation warning that contains a link to a description of
the change and how to fix it. For example:

``` ruby
Deprecated features used!
  JSON auto inflation is not supported (CHEF-1) at (irb):7:in `irb_binding`.
  Please see /chef-client/deprecations/json_auto_inflate.html for further details and information on how to correct this problem.
```

## Testing for Deprecations

To test your code for deprecations, you can put Test Kitchen in a mode
where any deprecations cause the chef run to fail. Ensure your
`kitchen.yml` includes:

``` yaml
provisioner:
  deprecations_as_errors: true
```

and then run Test Kitchen as usual. Test Kitchen will fail if any
deprecation errors are issued.

## Silencing deprecation warnings

Deprecation warnings are great for ensuring cookbooks are kept
up-to-date and to prepare for major version upgrades, sometimes you just
can't fix a deprecation right away. Enabling
`treat_deprecation_warnings_as_errors` mode in Test Kitchen integration
tests often compounds the problem because it does not distinguish
between deprecations from community cookbooks and those in your own
code.

Two new options are provided for silencing deprecation warnings:
`silence_deprecation_warnings` and inline `chef:silence_deprecation`
comments.

The `silence_deprecation_warnings` configuration value can be set in
your `client.rb` or `solo.rb` config file, either to `true` to silence
all deprecation warnings or to an array of deprecations to silence. You
can specify which to silence either by the deprecation key name (e.g.
`"internal_api"`), the numeric deprecation ID (e.g. `25` or <span
class="title-ref">"CHEF-25"</span>), or by specifying the filename and
line number where the deprecation is being raised from (e.g.
`"default.rb:67"`).

An example of setting the `silence_deprecation_warnings` option in your
`client.rb` or `solo.rb`:

``` ruby
silence_deprecation_warnings %w{deploy_resource chef-23 recipes/install.rb:22}
```

or in your \`kitchen.yml\`:

``` yaml
provisioner:
  name: chef_solo
    solo_rb:
      treat_deprecation_warnings_as_errors: true
      silence_deprecation_warnings:
        - deploy_resource
        - chef-23
        - recipes/install.rb:22
```

You can also silence deprecations using a comment on the line that is
raising the warning:

``` ruby
erl_call 'something' do # chef:silence_deprecation
```

We advise caution in the use of this feature, as excessive or prolonged
silencing can lead to difficulty upgrading when the next major release
of Chef comes out.

## All Deprecations

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 57%" />
<col style="width: 10%" />
<col style="width: 20%" />
</colgroup>
<thead>
<tr class="header">
<th>ID</th>
<th>Description</th>
<th>Deprecated</th>
<th>Expected Removal</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="/deprecations_internal_api/">CHEF-0</a></td>
<td>Many internal APIs have been improved.</td>
<td>various</td>
<td>varies</td>
</tr>
<tr class="even">
<td><a href="/deprecations_json_auto_inflate/">CHEF-1</a></td>
<td>Consumers of JSON are now required to be explicit in how it is turned in to a Chef object.</td>
<td>12.7</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_exit_code/">CHEF-2</a></td>
<td>Chef's exit codes are now defined so that it's easy to understand why Chef exited.</td>
<td>12.11</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_chef_gem_compile_time/">CHEF-3</a></td>
<td>When using the <code>chef_gem</code> resource, the phase to install the gem in must be specified.</td>
<td>12.1</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_attributes/">CHEF-4</a></td>
<td>Various improvements have been made to attribute syntax.</td>
<td>various</td>
<td>varies</td>
</tr>
<tr class="even">
<td><a href="/deprecations_custom_resource_cleanups/">CHEF-5</a></td>
<td>Various improvements have been made to custom resource syntax.</td>
<td>various</td>
<td>varies</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_easy_install/">CHEF-6</a></td>
<td>The <code>easy_install</code> resource will be removed.</td>
<td>12.10</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_verify_file/">CHEF-7</a></td>
<td>The <code>verify</code> metaproperty's <code>file</code> substitution will be removed.</td>
<td>12.5</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_supports_property/">CHEF-8</a></td>
<td>The <code>supports</code> metaproperty will be removed.</td>
<td>12.14</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_chef_rest/">CHEF-9</a></td>
<td>The <code>Chef::REST</code> API will be removed.</td>
<td>12.7</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_dnf_package_allow_downgrade/">CHEF-10</a></td>
<td>DNF package provider and resource do not require <code>--allow-downgrade</code> anymore.</td>
<td>12.18</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_property_name_collision/">CHEF-11</a></td>
<td>An exception will be raised if a resource property conflicts with an already-existing property or method.</td>
<td>12.19</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_launchd_hash_property/">CHEF-12</a></td>
<td>An exception will be raised whenever the <code>hash</code> property in the launchd resource is used.</td>
<td>12.19</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_chef_platform_methods/">CHEF-13</a></td>
<td>Deprecated <code>Chef::Platform</code> methods</td>
<td>12.18</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_run_command/">CHEF-14</a></td>
<td>Deprecation of run_command</td>
<td>12.18</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_local_listen/">CHEF-18</a></td>
<td>Deprecation of local mode listening.</td>
<td>13.1</td>
<td>15.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_namespace_collisions/">CHEF-19</a></td>
<td>Deprecation of <code>property_name</code> within actions.</td>
<td>13.2</td>
<td>14.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_deploy_resource/">CHEF-20</a></td>
<td>Deprecation of the <code>deploy</code> resource.</td>
<td>13.6</td>
<td>14.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_chocolatey_uninstall/">CHEF-21</a></td>
<td>Deprecation of the <code>:uninstall</code> action in the <code>chocolatey_package</code> resource.</td>
<td>13.7</td>
<td>14.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_erl_call_resource/">CHEF-22</a></td>
<td>Deprecation of the <code>erl_call</code> resource.</td>
<td>13.7</td>
<td>14.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_legacy_hwrp_mixins/">CHEF-23</a></td>
<td>Deprecation of legacy HWRP mixins.</td>
<td>12.X</td>
<td>14.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_epic_fail/">CHEF-24</a></td>
<td>Deprecation of <code>epic_fail</code> in favor of <code>allow_failure</code></td>
<td>13.7</td>
<td>14.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_map_collision/">CHEF-25</a></td>
<td>Resource(s) in a cookbook collide with the same resource(s) now included in Chef.</td>
<td>XX.X</td>
<td>15.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_locale_lc_all/">CHEF-27</a></td>
<td>Deprecation of lc_all from locale resource</td>
<td>15.0</td>
<td>16.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_resource_name_without_provides/">CHEF-31</a></td>
<td>Deprecation of resource_name declaration without provides</td>
<td>15.13</td>
<td>16.2</td>
</tr>
<tr class="even">
<td><a href="/deprecations_resource_cloning/">CHEF-3694</a></td>
<td>Resource Cloning will no longer work.</td>
<td>10.18</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_legacy_config/">OHAI-1</a></td>
<td>Ohai::Config removal.</td>
<td>12.6</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_sigar_plugins/">OHAI-2</a></td>
<td>Sigar gem based plugins removal.</td>
<td>12.19</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_run_command_helpers/">OHAI-3</a></td>
<td>run_command and popen4 helper method removal.</td>
<td>12.8</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_libvirt_plugin/">OHAI-4</a></td>
<td>Libvirt plugin attributes changes.</td>
<td>12.19</td>
<td>14.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_windows_cpu/">OHAI-5</a></td>
<td>Windows CPU plugin attribute changes.</td>
<td>12.19</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_digitalocean/">OHAI-6</a></td>
<td>DigitalOcean plugin attribute changes.</td>
<td>12.19</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_amazon_linux/">OHAI-7</a></td>
<td>Amazon linux moved to the Amazon platform_family.</td>
<td>13.0</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_cloud/">OHAI-8</a></td>
<td>Cloud plugin replaced by the Cloud_V2 plugin.</td>
<td>13.0</td>
<td>13.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_filesystem/">OHAI-9</a></td>
<td>Filesystem plugin replaced by the Filesystem V2 plugin.</td>
<td>13.0</td>
<td>13.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_v6_plugins/">OHAI-10</a></td>
<td>Removal of support for Ohai version 6 plugins.</td>
<td>11.12</td>
<td>14.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_cloud_v2/">OHAI-11</a></td>
<td>Cloud_v2 attribute removal.</td>
<td>13.1</td>
<td>14.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_filesystem_v2/">OHAI-12</a></td>
<td>Filesystem2 attribute removal.</td>
<td>13.1</td>
<td>14.0</td>
</tr>
<tr class="odd">
<td><a href="/deprecations_ohai_ipscopes/">OHAI-13</a></td>
<td>Removal of IpScopes plugin</td>
<td>13.2</td>
<td>14.0</td>
</tr>
<tr class="even">
<td><a href="/deprecations_ohai_system_profile/">OHAI-14</a></td>
<td>Removal of system_profile plugin</td>
<td>14.6</td>
<td>15.0</td>
</tr>
</tbody>
</table>
