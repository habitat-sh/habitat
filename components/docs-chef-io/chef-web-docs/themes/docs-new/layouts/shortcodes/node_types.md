The types of nodes that can be managed by Chef include, but are not
limited to, the following:

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<thead>
<tr class="header">
<th>Node Type</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><img src="/images/icon_node_type_server.svg" class="align-center" width="100" alt="image" /></td>
<td>A physical node is typically a server or a virtual machine, but it can be any active device attached to a network that is capable of sending, receiving, and forwarding information over a communications channel. In other words, a physical node is any active device attached to a network that can run a Chef Infra Client and also allow that Chef Infra Client to communicate with a Chef Infra Server.</td>
</tr>
<tr class="even">
<td><img src="/images/icon_node_type_cloud_public.svg" class="align-center" width="100" alt="image" /></td>
<td>A cloud-based node is hosted in an external cloud-based service, such as Amazon Web Services (AWS), OpenStack, Rackspace, Google Compute Engine, or Microsoft Azure. Plugins are available for knife that provide support for external cloud-based services. knife can use these plugins to create instances on cloud-based services. Once created, Chef Infra Client can be used to deploy, configure, and maintain those instances.</td>
</tr>
<tr class="odd">
<td><img src="/images/icon_node_virtual_machine.svg" class="align-center" width="100" alt="image" /></td>
<td>A virtual node is a machine that runs only as a software implementation, but otherwise behaves much like a physical machine.</td>
</tr>
<tr class="even">
<td><img src="/images/icon_node_type_network_device.svg" class="align-center" width="100" alt="image" /></td>
<td>A network node is any networking device---a switch, a router---that is being managed by a Chef Infra Client, such as networking devices by Juniper Networks, Arista, Cisco, and F5. Use Chef to automate common network configurations, such as physical and logical Ethernet link properties and VLANs, on these devices.</td>
</tr>
<tr class="odd">
<td><img src="/images/icon_node_type_container.svg" class="align-center" width="100" alt="image" /></td>
<td>Containers are an approach to virtualization that allows a single operating system to host many working configurations, where each working configuration---a container---is assigned a single responsibility that is isolated from all other responsibilities. Containers are popular as a way to manage distributed and scalable applications and services.</td>
</tr>
</tbody>
</table>