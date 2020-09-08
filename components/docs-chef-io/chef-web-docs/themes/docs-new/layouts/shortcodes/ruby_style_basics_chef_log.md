`Chef::Log` will print log entries to the default logger that is
configured for the machine on which Chef Infra Client is running. (To
create a log entry that is built into the resource collection, use the
**log** resource instead of `Chef::Log`.)

The following log levels are supported:

<table>
<colgroup>
<col style="width: 25%" />
<col style="width: 75%" />
</colgroup>
<thead>
<tr class="header">
<th>Log Level</th>
<th>Syntax</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td>Fatal</td>
<td><code>Chef::Log.fatal('string')</code></td>
</tr>
<tr class="even">
<td>Error</td>
<td><code>Chef::Log.error('string')</code></td>
</tr>
<tr class="odd">
<td>Warn</td>
<td><code>Chef::Log.warn('string')</code></td>
</tr>
<tr class="even">
<td>Info</td>
<td><code>Chef::Log.info('string')</code></td>
</tr>
<tr class="odd">
<td>Debug</td>
<td><code>Chef::Log.debug('string')</code></td>
</tr>
</tbody>
</table>

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

The parentheses are optional, e.g. `Chef::Log.info 'string'` may be used
instead of `Chef::Log.info('string')`.



</div>

</div>