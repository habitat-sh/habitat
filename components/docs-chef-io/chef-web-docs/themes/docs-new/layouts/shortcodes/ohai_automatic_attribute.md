An automatic attribute is a specific detail about a node, such as an IP
address, a host name, a list of loaded kernel modules, and so on.
Automatic attributes are detected by Ohai and are then used by Chef
Infra Client to ensure that they are handled properly during every Chef
Infra Client run. The most commonly accessed automatic attributes are:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Attribute</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>node['platform']</code></td>
<td>The platform on which a node is running. This attribute helps determine which providers will be used.</td>
</tr>
<tr class="even">
<td><code>node['platform_version']</code></td>
<td>The version of the platform. This attribute helps determine which providers will be used.</td>
</tr>
<tr class="odd">
<td><code>node['ipaddress']</code></td>
<td>The IP address for a node. If the node has a default route, this is the IPV4 address for the interface. If the node does not have a default route, the value for this attribute should be <code>nil</code>. The IP address for default route is the recommended default value.</td>
</tr>
<tr class="even">
<td><code>node['macaddress']</code></td>
<td>The MAC address for a node, determined by the same interface that detects the <code>node['ipaddress']</code>.</td>
</tr>
<tr class="odd">
<td><code>node['fqdn']</code></td>
<td>The fully qualified domain name for a node. This is used as the name of a node unless otherwise set.</td>
</tr>
<tr class="even">
<td><code>node['hostname']</code></td>
<td>The host name for the node.</td>
</tr>
<tr class="odd">
<td><code>node['domain']</code></td>
<td>The domain for the node.</td>
</tr>
<tr class="even">
<td><code>node['recipes']</code></td>
<td>A list of recipes associated with a node (and part of that node's run-list).</td>
</tr>
<tr class="odd">
<td><code>node['roles']</code></td>
<td>A list of roles associated with a node (and part of that node's run-list).</td>
</tr>
<tr class="even">
<td><code>node['ohai_time']</code></td>
<td>The time at which Ohai was last run. This attribute is not commonly used in recipes, but it is saved to the Chef Infra Server and can be accessed using the <code>knife status</code> subcommand.</td>
</tr>
</tbody>
</table>