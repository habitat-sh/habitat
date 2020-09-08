Use the **log** resource to create log entries. The **log** resource
behaves like any other resource: built into the resource collection
during the compile phase, and then run during the execution phase. (To
create a log entry that is not built into the resource collection, use
`Chef::Log` instead of the **log** resource.)

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

By default, every log resource that executes will count as an updated
resource in the updated resource count at the end of a Chef run. You can
disable this behavior by adding `count_log_resource_updates false` to
your Chef `client.rb` configuration file.



</div>

</div>