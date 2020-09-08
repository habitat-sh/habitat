The `ADDLOCAL` parameter adds two setup options specific to Chef Infra
Client. These options can be passed along with an Msiexec.exe command:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Option</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>ChefClientFeature</code></td>
<td>Use to install Chef Infra Client.</td>
</tr>
<tr class="even">
<td><code>ChefSchTaskFeature</code></td>
<td>Use to configure Chef Infra Client as a scheduled task in Microsoft Windows.</td>
</tr>
<tr class="odd">
<td><code>ChefPSModuleFeature</code></td>
<td>Used to install the chef PowerShell module. This will enable chef command line utilities within PowerShell.</td>
</tr>
</tbody>
</table>

First install Chef Infra Client, and then enable it to run as a
scheduled task. For example:

``` bash
msiexec /qn /i C:\inst\chef-client-15.3.14-1-x64.msi ADDLOCAL="ChefClientFeature,ChefSchTaskFeature,ChefPSModuleFeature"
```