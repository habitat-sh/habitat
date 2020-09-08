+++
title = "Manage Environments"
draft = false
robots = "noindex"


aliases = ["/server_manage_environments.html"]

[menu]
  [menu.infra]
    title = "Environments"
    identifier = "chef_infra/features/management_console/server_manage_environments.md Environments"
    parent = "chef_infra/features/management_console"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_manage_environments.md)

{{< note >}}

This topic is about using the Chef management console to manage
environments.

{{< /note >}}

{{% EOL_manage %}}

{{% environment %}}

## Manage

Environments can be managed from the Chef management console web user
interface.

### Add Environment

To add an environment:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Environments**.

4.  Click **Create**.

5.  In the **Create an Environment** dialog box, enter the name of the
    environment and a description.

    ![image](/images/step_manage_webui_policy_environment_add.png)

    Click **Next**.

6.  Optional. Set a constraint by choosing a name, an operator, and a
    version:

    ![image](/images/step_manage_webui_policy_environment_add_constraint.png)

    Click **Add**. Continue this process until all constraints are
    added. When finished, click **Next**.

7.  Optional. Add default attributes as JSON data:

    ![image](/images/step_manage_webui_policy_environment_add_default_attribute.png)

    Click **Next**.

8.  Optional. Add override attributes as JSON data:

    ![image](/images/step_manage_webui_policy_environment_add_override_attribute.png)

9.  Click **Create Environment**.

### Delete Environment

To delete an environment:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Environments**.

4.  Select an environment.

5.  Click **Delete**.

    ![image](/images/step_manage_webui_policy_environment_delete.png)

### Edit Details

To edit the details of an environment:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Details** tab.
6.  Click **Edit**.

### Set

To set the environment for a node:

1.  Open the Chef management console.

2.  Click **Nodes**.

3.  Select a node.

4.  Click the **Details** tab.

5.  In the top right, from the **Environment** drop-down, select the
    environment:

    ![image](/images/step_manage_webui_node_details_set_environment.png)

6.  Click **Save**.

### View Details

To view environment details:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Details** tab.

## Default Attributes

{{% node_attribute_type_default %}}

### Edit

To edit default attributes for an environment:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Environments**.

4.  Select an environment.

5.  Click the **Attributes** tab.

6.  Under **Default Attributes**, click **Edit**.

7.  In the **Edit Environment Attributes** dialog box, enter the JSON
    data that defines the attribute (or attributes).

    ![image](/images/step_manage_webui_policy_environment_edit_attribute.png)

8.  Click **Save**.

### View

To view default attributes for an environment:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Attributes** tab.

## Override Attributes

{{% node_attribute_type_override %}}

### Edit

To edit override attributes for an environment:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Environments**.

4.  Select an environment.

5.  Click the **Attributes** tab.

6.  Under **Override Attributes**, click **Edit**.

7.  In the **Edit Environment Attributes** dialog box, enter the JSON
    data that defines the attribute (or attributes).

    ![image](/images/step_manage_webui_policy_environment_edit_attribute.png)

8.  Click **Save Attributes**.

### View

To view override attributes for an environment:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Attributes** tab.

## Permissions

{{% server_rbac_permissions %}}

{{% server_rbac_permissions_object %}}

### Set

To set permissions list for an environment object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Permissions** tab.
6.  For each group listed under **Name**, select or de-select the
    **Read**, **Update**, **Delete**, and **Grant** permissions.

### Update

To update the permissions list for an environment object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Permissions** tab.
6.  Click the **+ Add** button and enter the name of the user or group
    to be added.
7.  Select or de-select **Read**, **Update**, **Delete**, and **Grant**
    to update the permissions list for the user or group.

### View

To view permissions for an environment object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Environments**.
4.  Select an environment.
5.  Click the **Permissions** tab.
6.  Set the appropriate permissions: **Read**, **Update**, **Delete**,
    and **Grant**.
