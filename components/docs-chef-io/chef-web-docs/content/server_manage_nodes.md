+++
title = "Manage Nodes"
draft = false
robots = "noindex"


aliases = ["/server_manage_nodes.html"]

[menu]
  [menu.infra]
    title = "Nodes"
    identifier = "chef_infra/features/management_console/server_manage_nodes.md Nodes"
    parent = "chef_infra/features/management_console"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_manage_nodes.md)

{{% EOL_manage %}}

{{< note >}}

This topic is about using the Chef management console to manage nodes.

{{< /note >}}

{{% node %}}

## Manage

Nodes can be managed from the Chef management console web user
interface.

### Delete

To delete a node:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node.

4.  Click **Delete**.

5.  Confirm:

    ![image](/images/step_manage_webui_node_delete.png)

### Reset Key

To reset the validation key for a node:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node.

4.  Click **Edit Run List**.

5.  In the **Reset Key** dialog box, confirm that the key should be
    regenerated and click the **Regenerate Key** button:

    ![image](/images/step_manage_webui_admin_organization_reset_key.png)

6.  In the **Reset Key** dialog box, copy the key directly from the
    dialog box or click the **Download** button to download the key to
    your local machine:

    ![image](/images/step_manage_webui_admin_organization_reset_key_regenerated.png)

### Search

To search nodes:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  In the search box in the upper right, enter the search query and
    click the search icon.

    ![image](/images/step_manage_webui_nodes_search.png)

4.  The results will appear in the list below.

## Node Attributes

{{% node_attribute %}}

### Edit Attribute

To edit node attributes:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node.

4.  Click the **Attributes** tab.

5.  Click **Edit**.

6.  In the **Edit Node Attributes** dialog box, make your changes:

    ![image](/images/step_manage_webui_node_attributes_edit.png)

7.  Click **Save Attributes**.

### View Attributes

To view the attributes for a node:

1.  Open the Chef management console.
2.  Click **Nodes**.
3.  Select a node.
4.  Click the **Attributes** tab.
5.  Click **Edit**.

## Run-lists

{{% node_run_list %}}

### Add Recipe to

{{% manage_webui_node_run_list_add_role_or_recipe %}}

### Add Role to

{{% manage_webui_node_run_list_add_role_or_recipe %}}

### Edit

{{% manage_webui_node_run_list_edit %}}

### Remove Recipe from

{{% manage_webui_node_run_list_remove_role_or_recipe %}}

### Remove Role from

{{% manage_webui_node_run_list_remove_role_or_recipe %}}

### View Current

{{% manage_webui_node_run_list_view_current %}}

## Permissions

{{% server_rbac_permissions %}}

{{% server_rbac_permissions_object %}}

### Set

To set permissions list for a node object:

1.  Open the Chef management console.
2.  Click **Nodes**.
3.  Select a node.
4.  Click the **Permissions** tab.
5.  For each group listed under **Name**, select or de-select the
    **Read**, **Update**, **Delete**, and **Grant** permissions.

### Update

To update the permissions list for a node object:

1.  Open the Chef management console.
2.  Click **Nodes**.
3.  Select a node.
4.  Click the **Permissions** tab.
5.  Click the **+ Add** button and enter the name of the user or group
    to be added.
6.  Select or de-select **Read**, **Update**, **Delete**, and **Grant**
    to update the permissions list for the user or group.

### View

To view permissions for a node:

1.  Open the Chef management console.
2.  Click **Nodes**.
3.  Select a node.
4.  Click the **Permissions** tab.
5.  Set the appropriate permissions: **Delete**, **Grant**, **Read**,
    and/or **Update**.

## Manage Tags

{{% chef_tags %}}

### Add

To add tags to a node (or a group of nodes):

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node (or a group of nodes).

4.  Click **Manage Tags**.

5.  In the **Manage Node Tags** dialog box, enter the name of the tag
    and then select **Add Tags** from the drop-down.

    ![image](/images/step_manage_webui_node_tags_add.png)

6.  Click **Update Tags**.

### Delete

To delete tags for a node (or a group of nodes):

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node (or a group of nodes).

4.  Click **Manage Tags**.

5.  In the **Manage Node Tags** dialog box, enter the name of the tag
    and then select **Delete Tags** from the drop-down.

    ![image](/images/step_manage_webui_node_tags_delete.png)

6.  Click **Update Tags**.

### View

To view all of the nodes:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node.

4.  Select the **Details** tab.

5.  The tags for the node appear under the **Tags** header:

    ![image](/images/step_manage_webui_nodes_view_tags.png)
