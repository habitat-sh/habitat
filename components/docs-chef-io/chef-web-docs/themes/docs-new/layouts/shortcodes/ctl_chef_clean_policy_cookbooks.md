Use the `chef clean-policy-cookbooks` subcommand to delete cookbooks
that are not used by Policyfile files. Cookbooks are considered unused
when they are not referenced by any policy revisions on the Chef Infra
Server.

<div class="admonition-note">

<p class="admonition-note-title">Note</p>

<div class="admonition-note-text">

Cookbooks that are referenced by orphaned policy revisions are not
removed. Use `chef clean-policy-revisions` to remove orphaned policies.



</div>

</div>