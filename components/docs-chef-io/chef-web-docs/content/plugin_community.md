+++
title = "Community Plugins"
draft = false

aliases = "/plugin_community.html"



+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/plugin_community.md)

This page lists plugins for Ohai plugins and Chef Infra Client handlers
that are developed and maintained by the Chef community.

## Ohai

{{% ohai_summary %}}

The following Ohai plugins are available from the open source community:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Plugin</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="https://github.com/demonccc/chef-ohai-plugins/blob/master/dell.rb">dell.rb</a></td>
<td>Adds some useful Dell server information to Ohai. For example: service tag, express service code, storage info, RAC info, and so on. To use this plugin, OMSA and SMBIOS applications need to be installed.</td>
</tr>
<tr class="even">
<td><a href="https://bitbucket.org/retr0h/ohai">ipmi.rb</a></td>
<td>Adds a MAC address and an IP address to Ohai, where available.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/albertsj1/ohai-plugins/blob/master/kvm_extensions.rb">kvm_extensions.rb</a></td>
<td>Adds extensions for virtualization attributes to provide additional host and guest information for KVM.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/demonccc/chef-ohai-plugins/blob/master/linux/ladvd.rb">ladvd.rb</a></td>
<td>Adds ladvd information to Ohai, when it exists.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/jespada/ohai-plugins/blob/master/lxc_virtualization.rb">lxc_virtualization.rb</a></td>
<td>Adds extensions for virtualization attributes to provide host and guest information for Linux containers.</td>
</tr>
<tr class="even">
<td><a href="https://gist.github.com/1040543">network_addr.rb</a></td>
<td>Adds extensions for network attributes with additional <code>ipaddrtype_iface</code> attributes to make it semantically easier to retrieve addresses.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/agoddard/ohai-plugins/blob/master/plugins/network_ports.rb">network_ports.rb</a></td>
<td>Adds extensions for network attributes so that Ohai can detect to which interfaces TCP and UDP ports are bound.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/sbates/Chef-odds-n-ends/blob/master/ohai/parse_host_plugin.rb">parse_host_plugin.rb</a></td>
<td>Adds the ability to parse a host name using three top-level attribute and five nested attributes.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/stevendanna/ohai-plugins/blob/master/plugins/r.rb">r.rb</a></td>
<td>Adds the ability to collect basic information about the R Project.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/spheromak/cookbooks/blob/master/ohai/files/default/sysctl.rb">sysctl.rb</a></td>
<td>Adds sysctl information to Ohai.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/albertsj1/ohai-plugins/blob/master/vserver.rb">vserver.rb</a></td>
<td>Adds extensions for virtualization attributes to allow a Linux virtual server host and guest information to be used by Ohai.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/cloudant/ohai_plugins/blob/master/wtf.rb">wtf.rb</a></td>
<td>Adds the irreverent wtfismyip.com service so that Ohai can determine a machine's external IPv4 address and geographical location.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/spheromak/cookbooks/blob/master/ohai/files/default/xenserver.rb">xenserver.rb</a></td>
<td>Adds extensions for virtualization attributes to load up Citrix XenServer host and guest information.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/timops/ohai-plugins/blob/master/win32_software.rb">win32_software.rb</a></td>
<td>Adds the ability for Ohai to use Windows Management Instrumentation (WMI) to discover useful information about software that is installed on any node that is running Microsoft Windows.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/timops/ohai-plugins/blob/master/win32_svc.rb">win32_svc.rb</a></td>
<td>Adds the ability for Ohai to query using Windows Management Instrumentation (WMI) to get information about all services that are registered on a node that is running Microsoft Windows.</td>
</tr>
</tbody>
</table>

## Handlers

{{% handler %}}

{{% handler_community_handlers %}}
