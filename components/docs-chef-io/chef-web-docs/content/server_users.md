+++
title = "Users"
draft = false

aliases = ["/server_users.html"]

[menu]
  [menu.infra]
    title = "Users"
    identifier = "chef_infra/managing_chef_infra_server/server_users.md Users"
    parent = "chef_infra/managing_chef_infra_server"
    weight = 140
+++

[\[edit on GitHub\]](https://github.com/chef/chef-web-docs/blob/master/content/server_users.md)

The following tasks are available for user management in Chef Infra
Server:

-   Creating users
-   Editing a user's profile
-   Changing a password
-   Recovering a password
-   Regenerating a private key
-   Viewing a user's profile

## chef-server-ctl

{{% ctl_chef_server_summary %}}

{{% ctl_chef_server_user %}}

### org-create

{{% ctl_chef_server_org_create %}}

**Syntax**

{{% ctl_chef_server_org_create_syntax %}}

**Options**

{{% ctl_chef_server_org_create_options %}}

### org-delete

{{% ctl_chef_server_org_delete %}}

**Syntax**

{{% ctl_chef_server_org_delete_syntax %}}

### org-list

{{% ctl_chef_server_org_list %}}

**Syntax**

{{% ctl_chef_server_org_list_syntax %}}

**Options**

{{% ctl_chef_server_org_list_options %}}

### org-show

{{% ctl_chef_server_org_show %}}

**Syntax**

{{% ctl_chef_server_org_show_syntax %}}

### org-user-add

{{< warning >}}

Early RC candidates for the Chef Server 12 release named this command
`org-associate`. This is the same command, with the exception of the
`--admin` flag, which is added to the command (along with the rename)
for the upcoming final release of Chef Server 12.

{{< /warning >}}

{{% ctl_chef_server_org_user_add %}}

**Syntax**

{{% ctl_chef_server_org_user_add_syntax %}}

**Options**

{{% ctl_chef_server_org_user_add_options %}}

### org-user-remove

{{% ctl_chef_server_org_user_remove %}}

**Syntax**

{{% ctl_chef_server_org_user_remove_syntax %}}

### user-create

{{% ctl_chef_server_user_create %}}

**Syntax**

{{% ctl_chef_server_user_create_syntax %}}

**Options**

{{% ctl_chef_server_user_create_options %}}

### user-delete

{{% ctl_chef_server_user_delete %}}

**Syntax**

{{% ctl_chef_server_user_delete_syntax %}}

### user-edit

{{% ctl_chef_server_user_edit %}}

**Syntax**

{{% ctl_chef_server_user_edit_syntax %}}

### user-list

{{% ctl_chef_server_user_list %}}

**Syntax**

{{% ctl_chef_server_user_list_syntax %}}

**Options**

{{% ctl_chef_server_user_list_options %}}

### user-show

{{% ctl_chef_server_user_show %}}

**Syntax**

{{% ctl_chef_server_user_show_syntax %}}

**Options**

{{% ctl_chef_server_user_show_options %}}
