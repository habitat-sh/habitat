The `build_nodes` setting specifies which build nodes to use for
specific phases in the Chef Automate pipeline. The build node may be
defined as well as queried via wildcard search.

<div class="admonition-note">
<p class="admonition-note-title">Note</p>
<div class="admonition-note-text">

This setting should only be used with build nodes that use the previous
push job-based dispatch system. Use the `job_dispatch` setting when
using the new ssh-based job dispatch system.

</div>
</div>
