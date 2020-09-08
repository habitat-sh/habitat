+++
title = "Manage Data Bags"
draft = false
robots = "noindex"


aliases = ["/server_manage_data_bags.html"]

[menu]
  [menu.infra]
    title = "Data Bags"
    identifier = "chef_infra/features/management_console/server_manage_data_bags.md Data Bags"
    parent = "chef_infra/features/management_console"
    weight = 50
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_manage_data_bags.md)

{{% EOL_manage %}}

{{< note >}}

This topic is about using the Chef management console to manage data
bags.

{{< /note >}}

{{% data_bag %}}

## Manage

Data bags can be managed from the Chef management console web user
interface.

### Add Data Bag

To add a data bag:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Data Bags**.

4.  Click **Create**.

5.  In the **Create a Data Bag** dialog box, enter the name of the data
    bag.

    ![image](/images/step_manage_webui_policy_data_bag_add.png)

6.  Click **Create Data Bag**.

### Delete Data Bag

To delete a data bag:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Data Bags**.

4.  Select a data bag.

5.  Click **Delete**.

    ![image](/images/step_manage_webui_policy_data_bag_delete.png)

## Manage Items

{{% data_bag_item %}}

### Add Item

To add a data bag item:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Data Bags**.

4.  Select a data bag.

5.  Click **Create Item**.

6.  In the **Create a Data Bag Item** dialog box, enter the data bag
    identifier, and then JSON data that defines the data bag item.

    ![image](/images/step_manage_webui_policy_data_bag_add_item.png)

7.  Click **Create Data Bag Item**.

### Delete Item

To delete a data bag item:

1.  Open the Chef management console.

2.  Click **Policy**.

3.  Click **Data Bags**.

4.  Select a data bag.

5.  Select the **Items** tab.

6.  Select an item.

7.  Click **Delete**.

    ![image](/images/step_manage_webui_policy_data_bag_delete_item.png)

### Edit Item

{{% manage_webui_policy_data_bag_edit_item %}}

### View Item

To view data bag items for a data bag:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Data Bags**.
4.  Select a data bag.
5.  Select the **Items** tab.

## Manage Permissions

{{% server_rbac_permissions %}}

{{% server_rbac_permissions_object %}}

### Set

To set permissions list for a data bag object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Data Bags**.
4.  Select a data bag.
5.  Click the **Permissions** tab.
6.  For each group listed under **Name**, select or de-select the
    **Read**, **Update**, **Delete**, and **Grant** permissions.

### Update

To update the permissions list for a data bag object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Data Bags**.
4.  Select a data bag.
5.  Click the **Permissions** tab.
6.  Click the **+ Add** button and enter the name of the user or group
    to be added.
7.  Select or de-select **Read**, **Update**, **Delete**, and **Grant**
    to update the permissions list for the user or group.

### View

To view permissions for a data bag object:

1.  Open the Chef management console.
2.  Click **Policy**.
3.  Click **Data Bags**.
4.  Select a data bag.
5.  Click the **Permissions** tab.
6.  Set the appropriate permissions: **Read**, **Update**, **Delete**,
    and **Grant**.
