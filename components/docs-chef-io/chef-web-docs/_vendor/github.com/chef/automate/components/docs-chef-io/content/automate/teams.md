+++
title = "Teams"

date = 2018-05-16T16:03:13-07:00
draft = false
[menu]
  [menu.automate]
    title = "Teams"
    identifier = "automate/settings/teams.md Teams"
    parent = "automate/settings"
    weight = 70
+++

[\[edit on GitHub\]](https://github.com/chef/automate/blob/master/components/docs-chef-io/content/automate/teams.md)

## Overview

A Chef Automate team is an assigned grouping of users. You can import existing teams into Chef Automate with [Microsoft AD (LDAP)]({{< ref "ldap.md#microsoft-active-directory" >}}), [generic LDAP]({{< ref "ldap.md" >}}), or [SAML]({{< ref "saml.md" >}}). You can also create local Chef Automate teams that are independent of LDAP or SAML.

Permission for the `iam:teams` action is required to interact with teams. Any user that is part of the `admins` team or the `Administrator` policy will have this permission. Otherwise, [IAM custom policies]({{< relref "iam_v2_guide.md#creating-custom-policies" >}}) can be created to assign this permission.

## Managing Local Teams

### Admins Team

Chef Automate comes with an _Admins_ team by default. Local users can be added directly to this team, which assigns admin permissions to users.

### Creating Local Teams

Navigate to _Teams_ in the **Settings** tab. Select the **Create Team** button, which opens a dialog box to enter the team's _Name_ and optionally assign the team to one or more _Projects_. A team ID automatically generates upon creation. If you would like to change the team ID, use the **Edit ID** button.

![Create Local Team](/images/automate/admin-tab-teams-list.png)

### Adding Local Users to Teams

To add local users to a team, navigate to _Teams_ from the **Settings** tab and locate the team. Navigate to the team's page, and then use the **Add Users** button.

### Removing Local Users from Teams

To remove local users from a team, navigate to _Teams_ from the **Settings** tab and locate the team. Navigate to the team's page, locate the user to remove, and then use the menu at the end of the table row to remove the user.

### Changing Team Details

Teams have both a team name and the projects that a team belongs to. To change, navigate to _Teams_ from the **Settings** tab, select an individual team, and then navigate to the **Details** tab.

### Deleting Local Teams

Navigate to _Teams_ in the **Settings** tab. Then open the menu at the end of the table row and select **Delete Team**.
