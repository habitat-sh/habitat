The Chef Infra Server includes the following object permissions:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Permission</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><strong>Delete</strong></td>
<td>Use the <strong>Delete</strong> permission to define which users and groups may delete an object. This permission is required for any user who uses the <code>knife [object] delete [object_name]</code> argument to interact with objects on the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><strong>Grant</strong></td>
<td>Use the <strong>Grant</strong> permission to define which users and groups may configure permissions on an object. This permission is required for any user who configures permissions using the <strong>Administration</strong> tab in the Chef management console.</td>
</tr>
<tr class="odd">
<td><strong>Read</strong></td>
<td>Use the <strong>Read</strong> permission to define which users and groups may view the details of an object. This permission is required for any user who uses the <code>knife [object] show [object_name]</code> argument to interact with objects on the Chef Infra Server.</td>
</tr>
<tr class="even">
<td><strong>Update</strong></td>
<td>Use the <strong>Update</strong> permission to define which users and groups may edit the details of an object. This permission is required for any user who uses the <code>knife [object] edit [object_name]</code> argument to interact with objects on the Chef Infra Server and for any Chef Infra Client to save node data to the Chef Infra Server at the conclusion of a Chef Infra Client run.</td>
</tr>
</tbody>
</table>