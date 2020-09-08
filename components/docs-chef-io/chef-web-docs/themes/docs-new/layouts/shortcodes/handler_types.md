There are three types of handlers:

<table>
<colgroup>
<col style="width: 12%" />
<col style="width: 87%" />
</colgroup>
<thead>
<tr class="header">
<th>Handler</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>exception</td>
<td>An exception handler is used to identify situations that have caused a Chef Infra Client run to fail. An exception handler can be loaded at the start of a Chef Infra Client run by adding a recipe that contains the <strong>chef_handler</strong> resource to a node's run-list. An exception handler runs when the <code>failed?</code> property for the <code>run_status</code> object returns <code>true</code>.</td>
</tr>
<tr class="even">
<td>report</td>
<td>A report handler is used when a Chef Infra Client run succeeds and reports back on certain details about that Chef Infra Client run. A report handler can be loaded at the start of a Chef Infra Client run by adding a recipe that contains the <strong>chef_handler</strong> resource to a node's run-list. A report handler runs when the <code>success?</code> property for the <code>run_status</code> object returns <code>true</code>.</td>
</tr>
<tr class="odd">
<td>start</td>
<td>A start handler is used to run events at the beginning of a Chef Infra Client run. A start handler can be loaded at the start of a Chef Infra Client run by adding the start handler to the <code>start_handlers</code> setting in the client.rb file or by installing the gem that contains the start handler by using the <strong>chef_gem</strong> resource in a recipe in the <strong>chef-client</strong> cookbook. (A start handler may not be loaded using the <code>chef_handler</code> resource.)</td>
</tr>
</tbody>
</table>