+++
title = "Chef Infra Client Overview"
draft = false

aliases = ["/chef_client_overview.html", "/chef_client.html"]

[menu]
  [menu.infra]
    title = "Chef Infra Client Overview"
    identifier = "chef_infra/concepts/chef_client_overview.md Chef Infra Client Overview"
    parent = "chef_infra/concepts"
    weight = 10
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/chef_client_overview.md)

{{< note >}}

For the `chef-client` command line tool, see
[chef-client(executable)](/ctl_chef_client/) The Chef Infra Client
executable can be run as a daemon.

{{< /note >}}

<table>
<colgroup>
<col style="width: 19%" />
<col style="width: 80%" />
</colgroup>
<tbody>
<tr class="odd">
<td><p><img src="/images/icon_chef_client.svg" class="align-center" width="100" alt="image" /></p></td>
<td><p>{{< readFile_shortcode file="chef_client_summary.md" >}}</p>
<p>{{< readFile_shortcode file="security_key_pairs_chef_client.md" >}}</p></td>
</tr>
</tbody>
</table>

## The Chef Infra Client Run

{{% chef_client_run %}}

## Related Content

-   [Chef Infra Client (executable)](/ctl_chef_client/)
-   [Chef Infra Server](/server_overview/)
-   [Cookbooks](/cookbooks/)
-   [Nodes](/nodes/)
-   [Run Lists](/run_lists/)

## Next Steps

-   [Install Chef Workstation](/workstation/install_workstation/)
-   [Bootstrap Nodes](/install_bootstrap/)
