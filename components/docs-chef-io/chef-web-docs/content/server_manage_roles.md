+++
title = "Manage Roles"
draft = false
robots = "noindex"


aliases = ["/server_manage_roles.html"]

[menu]
  [menu.infra]
    title = "Roles"
    identifier = "chef_infra/features/management_console/server_manage_roles.md Roles"
    parent = "chef_infra/features/management_console"
    weight = 80
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_manage_roles.md)

{{% EOL_manage %}}

{{< note >}}

This topic is about using the Chef management console to manage roles.

{{< /note >}}

{{% role %}}

## Manage

Roles can be managed from the Chef management console web user
interface.

### Add Role

To add a role:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Roles**.

4.  Click **Create**.

5.  In the **Create Role** dialog box, enter the name of the role and a
    description.

    ![image](/images/step_manage_webui_policy_role_add.png)

    Click **Next**.

6.  Optional. Build the run-list from the list of available roles and
    recipes:

    ![image](/images/step_manage_webui_policy_role_add_run_list.png)

    Click **Next**.

7.  Optional. Add default attributes as JSON data:

    ![image](/images/step_manage_webui_policy_role_add_default_attribute.png)

    Click **Next**.

8.  Optional. Add override attributes as JSON data:

    ![image](/images/step_manage_webui_policy_role_add_override_attribute.png)

9.  Click **Create Role**.

### Delete Role

To delete a role:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Roles**.

4.  Select a role.

5.  Click **Delete**.

    ![image](/images/step_manage_webui_policy_role_delete.png)

### View All Roles

To view all roles uploaded to the Chef Infra Server organization:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Roles**.

## Run-lists

{{% node_run_list %}}

### Edit Role Run-list

{{% manage_webui_policy_role_edit_run_list %}}

## Default Attributes

{{% node_attribute_type_default %}}

### Edit Default Attributes

To edit default attributes for a role:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Roles**.

4.  Select a role.

5.  Click the **Attributes** tab.

6.  Under **Default Attributes**, click **Edit**.

7.  In the **Edit Role Attributes** dialog box, enter the JSON data that
    defines the attribute (or attributes).

    ![image](/images/step_manage_webui_policy_role_edit_attribute.png)

8.  Click **Save Attributes**.

### View Default Attributes

To view default attributes for a role:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Roles**.
4.  Select a role.
5.  Click the **Attributes** tab.

## Override Attributes

{{% node_attribute_type_override %}}

### Edit Override Attributes

To edit override attributes for a role:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Roles**.

4.  Select a role.

5.  Click the **Attributes** tab.

6.  Under **Override Attributes**, click **Edit**.

7.  In the **Edit Role Attributes** dialog box, enter the JSON data that
    defines the attribute (or attributes).

    ![image](/images/step_manage_webui_policy_role_edit_attribute.png)

8.  Click **Save Attributes**.

### View Override Attributes

To view role details:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Roles**.
4.  Select a role.
5.  Click the **Details** tab.

## Permissions

{{% server_rbac_permissions %}}

{{% server_rbac_permissions_object %}}

### Set

To set permissions list for a role object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Roles**.
4.  Select a role.
5.  Click the **Permissions** tab.
6.  For each group listed under **Name**, select or de-select the
    **Read**, **Update**, **Delete**, and **Grant** permissions.

### Update

To update the permissions list for a role object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Roles**.
4.  Select a role.
5.  Click the **Permissions** tab.
6.  Click the **+ Add** button and enter the name of the user or group
    to be added.
7.  Select or de-select **Read**, **Update**, **Delete**, and **Grant**
    to update the permissions list for the user or group.

### View

To view permissions for a role object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Roles**.
4.  Select a role.
5.  Click the **Permissions** tab.
6.  Set the appropriate permissions: **Delete**, **Grant**, **Read**,
    and/or **Update**.
