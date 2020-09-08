It is possible to initiate jobs from Chef Infra Client, such as from
within a recipe based on an action to be determined as the recipe runs.
For a Chef Infra Client to be able to create, initiate, or read jobs,
Chef Infra Client on which Chef Push Jobs is configured must belong to
one (or both) of the following groups:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Group</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>pushy_job_readers</code></td>
<td>Use to view the status of jobs.</td>
</tr>
<tr class="even">
<td><code>pushy_job_writers</code></td>
<td>Use to create and initiate jobs.</td>
</tr>
</tbody>
</table>

These groups do not exist by default, even after Chef Push Jobs has been
installed to the Chef Infra Server. If these groups are not created,
only members of the `admin` security group will be able to create,
initiate, and view jobs.