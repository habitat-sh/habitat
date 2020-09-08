chef-shell is tool that is run using an Interactive Ruby (IRb) session.
chef-shell currently supports recipe and attribute file syntax, as well
as interactive debugging features. chef-shell has three run modes:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Mode</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Standalone</td>
<td>Default. No cookbooks are loaded, and the run-list is empty.</td>
</tr>
<tr class="even">
<td>Solo</td>
<td>chef-shell acts as a Chef Solo Client. It attempts to load the chef-solo configuration file at <code>~/.chef/config.rb</code> and any JSON attributes passed. If the JSON attributes set a run-list, it will be honored. Cookbooks will be loaded in the same way that chef-solo loads them. chef-solo mode is activated with the <code>-s</code> or <code>--solo</code> command line option, and JSON attributes are specified in the same way as for chef-solo, with <code>-j /path/to/chef-solo.json</code>.</td>
</tr>
<tr class="odd">
<td>Client</td>
<td>chef-shell acts as a Chef Infra Client. During startup, it reads the Chef Infra Client configuration file from <code>~/.chef/client.rb</code> and contacts the Chef Infra Server to get the node's run_list, attributes, and cookbooks. Chef Infra Client mode is activated with the <code>-z</code> or <code>--client</code> options. You can also specify the configuration file with <code>-c CONFIG</code> and the server URL with <code>-S SERVER_URL</code>.</td>
</tr>
</tbody>
</table>