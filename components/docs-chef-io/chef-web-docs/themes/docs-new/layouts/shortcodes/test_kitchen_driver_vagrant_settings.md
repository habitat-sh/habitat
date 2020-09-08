The following attributes are used to configure `kitchen-vagrant` for
Chef:

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
<td><code>box</code></td>
<td>Required. Use to specify the box on which Vagrant will run. Default value: computed from the platform name of the instance.</td>
</tr>
<tr class="even">
<td><code>box_check_update</code></td>
<td>Use to check for box updates. Default value: <code>false</code>.</td>
</tr>
<tr class="odd">
<td><code>box_url</code></td>
<td>Use to specify the URL at which the configured box is located. Default value: computed from the platform name of the instance, but only when the Vagrant provider is VirtualBox- or VMware-based.</td>
</tr>
<tr class="even">
<td><code>communicator</code></td>
<td>Use to override the <code>config.vm.communicator</code> setting in Vagrant. For example, when a base box is a Microsoft Windows operating system that does not have SSH installed and enabled, Vagrant will not be able to boot without a custom Vagrant file. Default value: <code>nil</code> (assumes SSH is available).</td>
</tr>
<tr class="odd">
<td><code>customize</code></td>
<td>A hash of key-value pairs that define customizations that should be made to the Vagrant virtual machine. For example: <code>customize: memory: 1024 cpuexecutioncap: 50</code>.</td>
</tr>
<tr class="even">
<td><code>guest</code></td>
<td>Use to specify the <code>config.vm.guest</code> setting in the default Vagrantfile.</td>
</tr>
<tr class="odd">
<td><code>gui</code></td>
<td>Use to enable the graphical user interface for the defined platform. This is passed to the <code>config.vm.provider</code> setting in Vagrant, but only when the Vagrant provider is VirtualBox- or VMware-based.</td>
</tr>
<tr class="even">
<td><code>network</code></td>
<td>Use to specify an array of network customizations to be applied to the virtual machine. Default value: <code>[]</code>. For example: <code>network: - ["forwarded_port", {guest: 80, host: 8080}] - ["private_network", {ip: "192.168.33.33"}]</code>.</td>
</tr>
<tr class="odd">
<td><code>pre_create_command</code></td>
<td>Use to run a command immediately prior to <code>vagrant up --no-provisioner</code>.</td>
</tr>
<tr class="even">
<td><code>provider</code></td>
<td>Use to specify the Vagrant provider. This value must match a provider name in Vagrant.</td>
</tr>
<tr class="odd">
<td><code>provision</code></td>
<td>Use to provision Vagrant when the instance is created. This is useful if the operating system needs customization during provisioning. Default value: <code>false</code>.</td>
</tr>
<tr class="even">
<td><code>ssh_key</code></td>
<td>Use to specify the private key file used for SSH authentication.</td>
</tr>
<tr class="odd">
<td><code>synced_folders</code></td>
<td>Use to specify a collection of synchronized folders on each Vagrant instance. Source paths are relative to the Kitchen root path. Default value: <code>[]</code>. For example: <code>synced_folders: - ["data/%{instance_name}", "/opt/instance_data"] - ["/host_path", "/vm_path", "create: true, type: :nfs"]</code>.</td>
</tr>
<tr class="even">
<td><code>vagrantfile_erb</code></td>
<td>Use to specify an alternate Vagrant Embedded Ruby (ERB) template to be used by this driver.</td>
</tr>
<tr class="odd">
<td><code>vagrantfiles</code></td>
<td>An array of paths to one (or more) Vagrant files to be merged with the default Vagrant file. The paths may be absolute or relative to the kitchen.yml file.</td>
</tr>
<tr class="even">
<td><code>vm_hostname</code></td>
<td>Use to specify the internal hostname for the instance. This is not required when connecting to a Vagrant virtual machine. Set this to <code>false</code> to prevent this value from being rendered in the default Vagrantfile. Default value: computed from the platform name of the instance.</td>
</tr>
</tbody>
</table>