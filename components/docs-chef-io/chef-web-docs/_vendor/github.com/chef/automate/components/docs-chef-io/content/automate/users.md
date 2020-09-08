+++
title = "Users"

date = 2018-05-16T16:03:13-07:00
draft = false
[menu]
  [menu.automate]
    title = "Users"
    identifier = "automate/settings/users.md Users"
    parent = "automate/settings"
    weight = 60
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/users.md)

## Overview

Chef Automate supports three different types of users: local users, [LDAP users]({{< relref "ldap.md" >}}), and [SAML users]({{< relref "saml.md" >}}). Manage local users from the **Settings** tab.

Local users can sign in and interact with the system independent of LDAP or SAML. 
Local users will have their Chef Automate sessions refreshed while their Chef Automate browser window remains open or until they sign out directly.

Permission for the `iam:users` action is required to interact with users other than yourself. Any user that is part of the `admins` team or the `Administrator` policy will have this permission. Otherwise, [IAM custom policies]({{< relref "iam_v2_guide.md#creating-custom-policies" >}}) can be created to assign this permission.

## Managing Local Users

### Creating Local Users

Navigate to _Users_ in the **Settings** tab. Select the **Create User** button, which opens a dialog box for entering the user's _display name_, and _password_. A username automatically generates upon creation. If you would like to change the username, use the **Edit Username** button.

![Add Local User](/images/automate/admin-tab-users-list.png)

### Changing Display Names

Navigate to _Users_ in the **Settings** tab and locate the user who needs their display name changed. Navigate to their user page, provide a new display name, and select the **Save** button.

All local users can also change their own display names from the _Profile_ menu.

### Resetting Passwords

Navigate to _Users_ in the **Settings** tab and locate the user who needs a password reset. Navigate to their user page, and then the **Reset Password** tab. Provide a new password, confirm the new password, and then select the **Reset Password** button.

All local users can also reset their own passwords from the _Profile_ menu.

### Deleting Local Users

Navigate to _Users_ in the **Settings** tab. Then open the menu at the end of the table row and select **Delete User**.

## User Self-Maintenance

Local Automate users can manage their own display name and password.
Select the user icon in the top navigation bar, and then select **Profile** from the drop-down menu.

![Navigate to user profile](/images/automate/user-profile-navigation.png)
