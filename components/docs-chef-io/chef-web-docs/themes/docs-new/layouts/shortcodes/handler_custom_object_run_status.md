The `run_status` object is initialized by Chef Infra Client before the
`report` interface is run for any handler. The `run_status` object keeps
track of the status of a Chef Infra Client run and will contain some (or
all) of the following properties:

<table>
<colgroup>
<col style="width: 40%" />
<col style="width: 60%" />
</colgroup>
<thead>
<tr class="header">
<th>Property</th>
<th>Description</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><code>all_resources</code></td>
<td>A list of all resources that are included in the <code>resource_collection</code> property for the current Chef Infra Client run.</td>
</tr>
<tr class="even">
<td><code>backtrace</code></td>
<td>A backtrace associated with the uncaught exception data that caused a Chef Infra Client run to fail, if present; <code>nil</code> for a successful Chef Infra Client run.</td>
</tr>
<tr class="odd">
<td><code>elapsed_time</code></td>
<td>The amount of time between the start (<code>start_time</code>) and end (<code>end_time</code>) of a Chef Infra Client run.</td>
</tr>
<tr class="even">
<td><code>end_time</code></td>
<td>The time at which a Chef Infra Client run ended.</td>
</tr>
<tr class="odd">
<td><code>exception</code></td>
<td>The uncaught exception data which caused a Chef Infra Client run to fail; <code>nil</code> for a successful Chef Infra Client run.</td>
</tr>
<tr class="even">
<td><code>failed?</code></td>
<td>Show that a Chef Infra Client run has failed when uncaught exceptions were raised during a Chef Infra Client run. An exception handler runs when the <code>failed?</code> indicator is <code>true</code>.</td>
</tr>
<tr class="odd">
<td><code>node</code></td>
<td>The node on which a Chef Infra Client run occurred.</td>
</tr>
<tr class="even">
<td><code>run_context</code></td>
<td>An instance of the <code>Chef::RunContext</code> object; used by Chef Infra Client to track the context of the run; provides access to the <code>cookbook_collection</code>, <code>resource_collection</code>, and <code>definitions</code> properties.</td>
</tr>
<tr class="odd">
<td><code>start_time</code></td>
<td>The time at which a Chef Infra Client run started.</td>
</tr>
<tr class="even">
<td><code>success?</code></td>
<td>Show that a Chef Infra Client run succeeded when uncaught exceptions were not raised during a Chef Infra Client run. A report handler runs when the <code>success?</code> indicator is <code>true</code>.</td>
</tr>
<tr class="odd">
<td><code>updated_resources</code></td>
<td>A list of resources that were marked as updated as a result of a Chef Infra Client run.</td>
</tr>
</tbody>
</table>

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

These properties are not always available. For example, a start handler
runs at the beginning of Chef Infra Client run, which means that
properties like `end_time` and `elapsed_time` are still unknown and will
be unavailable to the `run_status` object.



</div>

</div>