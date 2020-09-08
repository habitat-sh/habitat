Atomic updates are used with **file**-based resources to help ensure
that file updates can be made when updating a binary or if disk space
runs out.

Atomic updates are enabled by default. They can be managed globally
using the `file_atomic_update` setting in the client.rb file. They can
be managed on a per-resource basis using the `atomic_update` property
that is available with the **cookbook_file**, **file**,
**remote_file**, and **template** resources.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

On certain platforms, and after a file has been moved into place, Chef
Infra Client may modify file permissions to support features specific to
those platforms. On platforms with SELinux enabled, Chef Infra Client
will fix up the security contexts after a file has been moved into the
correct location by running the `restorecon` command. On the Microsoft
Windows platform, Chef Infra Client will create files so that ACL
inheritance works as expected.



</div>

</div>