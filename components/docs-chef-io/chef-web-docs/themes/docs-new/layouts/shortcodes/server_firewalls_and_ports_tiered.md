For back-end servers in a tiered Chef Infra Server installation, ensure
that ports marked as external (marked as `yes` in the **External**
column) are open and accessible via any firewalls that are in use:

<table>
<colgroup>
<col style="width: 11%" />
<col style="width: 77%" />
<col style="width: 11%" />
</colgroup>
<thead>
<tr class="header">
<th>Port</th>
<th>Service Name, Description</th>
<th>External</th>
</tr>
</thead>
<tbody>
<tr class="odd">
<td><p>80, 443, 9683</p></td>
<td><p><strong>nginx</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_nginx.md" | markdownify }}</p>
<div class="admonition-note">
<p class="admonition-note-title">Note</p>
<div class="admonition-note-text"><p>Port 9683 is used to internally load balance the <strong>oc_bifrost</strong> service.</p>

</div>
</div></td>
<td><p>yes</p></td>
</tr>
<tr class="even">
<td><p>9463</p></td>
<td><p><strong>oc_bifrost</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_bifrost.md" | markdownify }}</p></td>
<td></td>
</tr>
<tr class="odd">
<td><p>8983</p></td>
<td><p><strong>opscode-solr4</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_solr4.md" | markdownify }}</p></td>
<td></td>
</tr>
<tr class="even">
<td><p>5432</p></td>
<td><p><strong>postgresql</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_postgresql.md" | markdownify }}</p></td>
<td></td>
</tr>
<tr class="odd">
<td><p>5672, 15672</p></td>
<td><p><strong>rabbitmq</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_rabbitmq.md" | markdownify }}</p></td>
<td></td>
</tr>
<tr class="even">
<td><p>16379</p></td>
<td><p><strong>redis_lb</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_redis.md" | markdownify }}</p></td>
<td></td>
</tr>
<tr class="odd">
<td><p>4321</p></td>
<td><p><strong>bookshelf</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_bookshelf.md" | markdownify }}</p></td>
<td></td>
</tr>
<tr class="even">
<td><p>8000</p></td>
<td><p><strong>opscode-erchef</strong></p>
<p>{{ readFile "themes/docs-new/layouts/shortcodes/server_services_erchef.md" | markdownify }}</p></td>
<td></td>
</tr>
</tbody>
</table>