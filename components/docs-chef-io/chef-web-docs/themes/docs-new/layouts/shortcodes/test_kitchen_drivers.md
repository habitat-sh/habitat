Test Kitchen uses a driver plugin architecture to enable Test Kitchen to
test instances on cloud providers such as Amazon EC2, Google Compute
Engine, and Microsoft Azure. You can also test on multiple local
hypervisors, such as VMware, Hyper-V, or VirtualBox.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Chef Workstation includes many common Test Kitchen drivers.

</div>

</div>

Most drivers have driver-specific configuration settings that must be
added to the kitchen.yml file before Test Kitchen will be able to use
that platform during cookbook testing. For information about these
driver-specific settings, please refer to the driver-specific
documentation.

Some popular drivers:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Driver Plugin</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><a href="https://github.com/test-kitchen/kitchen-azurerm">kitchen-azurerm</a></td>
<td>A driver for Microsoft Azure.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/test-kitchen/kitchen-cloudstack">kitchen-cloudstack</a></td>
<td>A driver for CloudStack.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/test-kitchen/kitchen-digitalocean">kitchen-digitalocean</a></td>
<td>A driver for DigitalOcean. This driver ships in Chef Workstation.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/test-kitchen/kitchen-dokken">kitchen-dokken</a></td>
<td>A driver for Docker. This driver ships in Chef Workstation.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/test-kitchen/kitchen-dsc">kitchen-dsc</a></td>
<td>A driver for Windows PowerShell Desired State Configuration (DSC).</td>
</tr>
<tr class="even">
<td><a href="https://github.com/test-kitchen/kitchen-ec2">kitchen-ec2</a></td>
<td>A driver for Amazon EC2. This driver ships in Chef Workstation.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/test-kitchen/kitchen-google">kitchen-google</a></td>
<td>A driver for Google Compute Engine. This driver ships in Chef Workstation</td>
</tr>
<tr class="even">
<td><a href="https://github.com/test-kitchen/kitchen-hyperv">kitchen-hyperv</a></td>
<td>A driver for Microsoft Hyper-V Server. This driver ships in Chef Workstation.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/test-kitchen/kitchen-openstack">kitchen-openstack</a></td>
<td>A driver for OpenStack. This driver ships in Chef Workstation.</td>
</tr>
<tr class="even">
<td><a href="https://github.com/test-kitchen/kitchen-rackspace">kitchen-rackspace</a></td>
<td>A driver for Rackspace.</td>
</tr>
<tr class="odd">
<td><a href="https://github.com/test-kitchen/kitchen-vagrant">kitchen-vagrant</a></td>
<td>A driver for HashiCorp Vagrant. This driver ships in Chef Workstation.</td>
</tr>
</tbody>
</table>