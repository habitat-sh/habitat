Use to specify that search results only return objects to which an actor
(user, client, etc.) has read access, as determined by ACL settings.
This affects all searches. When `true`, the performance of the Chef
management console may increase because it enables the Chef management
console to skip redundant ACL checks. To ensure the Chef management
console is configured properly, after this setting has been applied with
a `chef-server-ctl reconfigure` run `chef-manage-ctl reconfigure` to
ensure the Chef management console also picks up the setting. Default
value: `false`.

<div class="admonition-warning">

<p class="admonition-warning-title">Warning</p>

<div class="admonition-warning-text">

When `true`, `opscode_erchef['strict_search_result_acls']` affects all
search results and any actor (user, client, etc.) that does not have
read access to a search result will not be able to view it. For example,
this could affect search results returned during a Chef Infra Client
runs if a Chef Infra Client does not have permission to read the
information.



</div>

</div>